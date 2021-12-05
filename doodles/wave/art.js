function art(regl, onImageRendered) {
  const random = window.fxrand;
  let baseColors = Array(3).fill(0.5);
  const colordelta = [
    0.3 + 0.5 * random() * (0.5 - random()),
    0.4 + random() * random() * random(),
    0.5 + 0.5 * random() * (0.5 - random()),
  ];
  let paletteVec3;
  let Palette = "";
  if (random() < 0.3) {
    paletteVec3 = "0.2";
    Palette = "Dark";
  } else if (random() < 0.2) {
    paletteVec3 = "0.8";
    Palette = "Light";
  } else if (random() < 0.4) {
    baseColors[2] = 0.75;
    paletteVec3 = "0.4,0.5,0.6";
    Palette = "Cold";
  } else {
    baseColors[0] = 0.8;
    paletteVec3 = "0.7,0.5,0.45";
    Palette = "Hot";
  }

  const baseColor = `vec3(${baseColors
    .map((n) => n.toFixed(1).toUpperCase())
    .join(",")})`;

  const Amplitude =
    1 - random() * random() + 8 * random() * random() * random();
  const Noise = random();
  const Wind = 8 * random() * random();
  const Waves = 1 + 5 * random();

  const frag = `
precision highp float;varying vec2 uv;uniform vec2 R;uniform float T;
#define PI ${Math.PI}
float vmax(vec2 v) {return max(v.x,v.y);}
float box(vec2 p,vec2 b){vec2 d=abs(p)-b;return length(max(d,vec2(0.)))+vmax(min(d,vec2(0.)));}
float U(float a,float b,float r){vec2 u=max(vec2(r-a,r-b),vec2(0.));return max(r,min(a,b))-length(u);}
float hash(float p){p=fract(p*.1031);p*=p+33.33;p*=p+p;return fract(p);}
float hash(vec2 p){vec3 p3=fract(vec3(p.xyx)*.1031);p3+=dot(p3,p3.yzx+33.33);return fract((p3.x+p3.y)*p3.z);}
float noise(float x){float i=floor(x);float f=fract(x);float u=f*f*(3.-2.*f);return mix(hash(i),hash(i+1.),u);}
float noise(vec2 x){vec2 i=floor(x);vec2 f=fract(x);float a=hash(i);float b=hash(i+vec2(1.,0.));float c=hash(i+vec2(0.,1.));float d=hash(i+vec2(1.));vec2 u=f*f*(3.-2.*f);return mix(a,b,u.x)+(c-a)*u.y*(1.-u.x)+(d-b)*u.x*u.y;}const mat2 m2=mat2(.6,.8,-.8,.6);
float fbm(vec2 x){float f=2.;float s=.55;float a=0.;float b=.5;for(int i=0;i<6;i++){float n=noise(x);a+=b*n;b*=s;x=f*x;}return a;}
vec3 palette(float t,vec3 a,vec3 b,vec3 c,vec3 d){return a+b*cos(6.28318*(c*t+d));}
vec3 pal(float t){return palette(t,${baseColor},vec3(${colordelta
    .map((c) => c.toFixed(2))
    .join(",")}),vec3(1.),vec3(${paletteVec3}));}
float scene(in vec2 p, float t) {
  vec2 q = vec2( fbm( 6. * p ), fbm( 5. * p + vec2(${(10 * random()).toFixed(
    3
  )},${(10 * random()).toFixed(3)}) ) );
  vec2 r = vec2( fbm(10. * q + ${Wind.toFixed(
    3
  )} * vec2(cos(0.2 * PI * t), sin(0.2 * PI * t))),
                fbm( 20. * q + vec2(${(50 * random()).toFixed(3)}, 1.3) ) );
  float v = ${Amplitude.toFixed(3)} * fbm(p + ${(
    0.5 +
    2 * random() * random()
  ).toFixed(3)} * q + 0.1 * ${Noise.toFixed(3)} * r);
  v += ${Waves.toFixed(3)} * p.y + 0.2 * t;
  return .5 * fbm(q) + smoothstep(0.3, 1.0, fract(v));
}
void main(){gl_FragColor=vec4(pal(scene((uv-0.5)*R/min(R.x,R.y),T)),1.);}`;

  const render = regl({
    frag,
    vert: `precision mediump float;attribute vec2 p;varying vec2 uv;void main(){uv=p;gl_Position=vec4(2.*p-1.,0,1);}`,
    attributes: {
      p: [-2, 0, 0, -2, 2, 2],
    },
    uniforms: {
      T: regl.prop("T"),
      R: ({ viewportWidth, viewportHeight }) => [viewportWidth, viewportHeight],
    },
    count: 3,
  });

  let firstCall = onImageRendered;

  regl.frame(({ time }) => {
    render({ T: time });
    if (firstCall) {
      firstCall();
      firstCall = null;
    }
  });

  return {
    destroy: () => regl.destroy(),
    metadata: {
      Palette,
      Amplitude,
      Noise,
      Wind,
      Waves,
    },
  };
}

export default art;
