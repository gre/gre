import { generatePerlinNoise } from "./perlin.mjs";

function screen(word, createCanvas, makeFillText) {
  const w = 128;
  const h = 64;
  const canvas = createCanvas(w, h);
  const ctx = canvas.getContext("2d");
  ctx.fillStyle = "#fff";
  ctx.fillRect(0, 0, w, h);
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.font =
    (typeof navigator === "undefined" || navigator.userAgent.includes("Mac OS")
      ? ""
      : "bold ") + "22px Arial";
  ctx.fillStyle = "#000";
  const fillText = makeFillText(ctx);
  fillText(word, w / 2, h / 2);
  return canvas;
}

function mix(a, b, x) {
  return a * (1 - x) + b * x;
}

// let imgratio = 383 / 128;
async function metal(word, swivelPlotted, createCanvas, makeFillText) {
  const w = 1200;
  const h = 400;
  const canvas = createCanvas(w, h);
  const ctx = canvas.getContext("2d");
  const fillText = makeFillText(ctx);
  ctx.fillStyle = "#000";
  ctx.fillRect(0, 0, w, h);
  ctx.fillStyle = "#fff";
  let font = "Arial";
  if (swivelPlotted) {
    ctx.strokeStyle = "#fff";
    ctx.lineWidth = 3;
    const octaveCount = Math.floor(3 + 6 * swivelPlotted[4]);
    const perlin = generatePerlinNoise(w, h, {
      octaveCount,
      amplitude: swivelPlotted[3],
      persistence: 0.2,
    });
    let pad = [50, 20];
    let amp = 120 * mix(swivelPlotted[0], swivelPlotted[1], swivelPlotted[2]);
    let incr = Math.floor(3 + 50 * swivelPlotted[1]);
    if (incr < 15 || (octaveCount < 4 && amp > 40)) {
      ctx.fillStyle = "#000";
      font = "Arial Black";
    }
    let heights = Array(w).fill(h);
    for (let y = h - pad[1]; y > pad[1] + amp; y -= incr) {
      ctx.beginPath();
      let up = true;
      for (let x = pad[0]; x < w - pad[0]; x++) {
        let dy = amp * perlin[y * w + x];
        let yy = y - dy;
        let m = heights[x];
        if (yy > m) {
          up = true;
          continue;
        }
        heights[x] = yy;
        if (up) {
          ctx.moveTo(x, yy);
          up = false;
        } else {
          ctx.lineTo(x, yy);
        }
      }
      ctx.stroke();
    }
  }
  //  const s = 84;
  //  ctx.drawImage(img, 60, (h - s) / 2, Math.round(imgratio * s), s);
  if (word) {
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    const lines = word.split("\n");
    const sz = Math.floor(
      20 + 1200 / (3 + Math.max(...lines.map((l) => l.length)))
    );
    ctx.font = sz + "px " + font;
    await Promise.all(
      lines.map((line, i) =>
        fillText(
          line,
          w / 2,
          Math.round(h / 2 + 1.2 * sz * (i + 0.5 - lines.length / 2))
        )
      )
    );
  }
  return canvas;
}
async function sticker(txt, createCanvas, makeFillText) {
  const w = 600;
  const h = 600;
  const canvas = createCanvas(w, h);
  const ctx = canvas.getContext("2d");
  const fillText = makeFillText(ctx);
  if (txt) {
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    ctx.font = "380px Arial";
    ctx.fillStyle = "#fff";
    await fillText(txt, w / 2, 320);
  }
  return canvas;
}

export async function art(
  regl,
  opts,
  frameTime,
  onFrame,
  createCanvas,
  makeFillText,
  createImageTexture,
  flipY,
  AA,
  speed
) {
  const screenCanvas = screen(opts.word, createCanvas, makeFillText);
  const metalCanvas = await metal(
    opts.sentence,
    opts.swivelPlotted,
    createCanvas,
    makeFillText
  );
  const stickerCanvas = await sticker(opts.sticker, createCanvas, makeFillText);

  //return document.body.appendChild(metalCanvas);

  const bgColor = opts.bgColor;

  const frag = `
  precision highp float;
  varying vec2 uv;
  uniform sampler2D text, metalText, stickerText;

  uniform vec2 resolution;
  uniform float time;

const vec3 Purple = vec3(0.765, 0.478, 0.99);
const vec3 Orange = vec3(0.988, 0.31, 0.0196);
const vec3 Black = vec3(0.1);
const vec3 White = vec3(0.9);
const vec3 Gold = vec3(1.0, 0.73, 0.2);

vec3 sceneBgColor;

#define PI ${Math.PI}
#define HIT vec2
HIT map (vec3 p);
vec3 shade (HIT m, vec3 p);
vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir);
float specularStrength (float m, vec3 n, vec3 p);

float powof3 (float t) {
  return t * t * t;
}

float cubicInOut(float t) {
  return t < 0.5
    ? 4.0 * t * t * t
    : 0.5 * powof3(2.0 * t - 2.0) + 1.0;
}

float hash(float p) {
  p = fract(p * .1031);
  p *= p + 33.33;
  p *= p + p;
  return fract(p);
}
float hash(vec2 p) {
  vec3 p3  = fract(vec3(p.xyx) * .1031);
  p3 += dot(p3, p3.yzx + 33.33);
  return fract((p3.x + p3.y) * p3.z);
}
float noise(float x) {
  float i = floor(x);
  float f = fract(x);
  float u = f * f * (3.0 - 2.0 * f);
  return mix(hash(i), hash(i + 1.0), u);
}
float noise(vec2 x) {
  vec2 i = floor(x);
  vec2 f = fract(x);
  float a = hash(i);
  float b = hash(i + vec2(1.0, 0.0));
  float c = hash(i + vec2(0.0, 1.0));
  float d = hash(i + vec2(1.0, 1.0));
  vec2 u = f * f * (3.0 - 2.0 * f);
  return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}
const mat2 m2 = mat2( 0.4,  0.7, -0.7,  0.4 );
float fbm( in vec2 x ) {
  float f = 2.0;
  float s = 0.55;
  float a = 0.0;
  float b = 0.5;
  for( int i=0; i<4; i++ ) {
	float n = noise(x);
	a += b * n;
	b *= s;
	x = f * x;
  }
  return a;
}
float fbm9( in vec2 x ) {
  float f = 2.0;
  float s = 0.55;
  float a = 0.0;
  float b = 0.5;
  for( int i=0; i<9; i++ ) {
	float n = noise(x);
	a += b * n;
	b *= s;
	x = f * x;
  }
  return a;
}

float fOpUnionRound(float a, float b, float r) {
	vec2 u = max(vec2(r - a,r - b), vec2(0));
	return max(r, min (a, b)) - length(u);
}
float fOpIntersectionRound(float a, float b, float r) {
	vec2 u = max(vec2(r + a,r + b), vec2(0));
	return min(-r, max (a, b)) + length(u);
}
float fOpDifferenceRound (float a, float b, float r) {
	return fOpIntersectionRound(a, -b, r);
}
float opSmoothSubtraction( float d1, float d2, float k ) {
  float h = clamp( 0.5 - 0.5*(d2+d1)/k, 0.0, 1.0 );
  return mix( d2, -d1, h ) + k*h*(1.0-h);
}
float sdCylinder( vec3 p, vec3 c ) {
  return length(p.xz-c.xy)-c.z;
}
float sdCappedCylinder( vec3 p, float h, float r )
{
  vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(h,r);
  return min(max(d.x,d.y),0.0) + length(max(d,0.0));
}
float sdBox( vec3 p, vec3 b ) {
  vec3 q = abs(p) - b;
  return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}
float sdBox2(in vec2 p, in vec2 b) {
  vec2 d = abs(p) - b;
  return length(max(d, vec2(0))) + min(max(d.x, d.y), 0.0);
}
float sdBoxRoundZ(vec3 p, vec3 b, float r) {
  return max(sdBox2(p.xy, b.xy-r)-r, abs(p.z)-b.z);
}
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

${
  !opts.screenAnimation
    ? ""
    : `
float shape (vec2 p, float t) {
  float smoothing = 0.15;
  p -= 0.5;
  vec2 q = p;
  pR(p, t + cos(${Math.round(5 * opts.screenAnimation[0] - 2)}. * t));
  vec2 dist = vec2(0.0);
  float crop = 99.0;
  float s = 99.0;;
  s = fOpUnionRound(q.y, s, smoothing);

dist = vec2(0.31, 0.0);
float radius = 0.11;
s = fOpUnionRound(s, length(p + dist) - radius, smoothing);
crop = fOpUnionRound(crop, length(p - dist) - radius, smoothing);

  s = fOpDifferenceRound(s, crop, smoothing);
  return smoothstep(0.0, 1.0 / min(resolution.x, resolution.y), s);
}
`
}

mat3 lookAt (vec3 ro, vec3 ta) {
  float cr = 0.;
  vec3 ww = normalize( ta - ro );
  vec3 uu = normalize( cross(ww,vec3(sin(cr),cos(cr),0.0) ) );
  vec3 vv =          ( cross(uu,ww));
  return mat3(uu,vv,ww);
}
vec3 normal (in vec3 p) {
	vec3 eps = vec3(0.0005, 0.0, 0.0);
	return normalize(vec3(
		map(p+eps.xyy).x-map(p-eps.xyy).x,
		map(p+eps.yxy).x-map(p-eps.yxy).x,
		map(p+eps.yyx).x-map(p-eps.yyx).x
	));
}
float diffuse(vec3 p, vec3 n, vec3 lpos) {
  vec3 l = normalize(lpos-p);
  float dif = clamp(dot(n, l), 0.01, 1.);
  return dif;
}
// https://www.iquilezles.org/www/articles/rmshadows/rmshadows.htm
float softshadow( in vec3 ro, in vec3 rd, float mint, float maxt, float k ) {
  float res = 1.0;
  float ph = 1e20;
  float t = mint;
  for (int i=0; i<20; i++) {
    float h =  map(ro + rd*t).x;
    if (t>=maxt) break;
    if( h<0.001) return 0.0;
    float y = h*h/(2.0*ph);
    float d = sqrt(h*h-y*y);
    res = min( res, k*d/max(0.0,t-y) );
    ph = h;
    t += h;
  }
  return res;
}
HIT marcher (inout vec3 p, vec3 dir) {
  HIT hit = HIT(0.);
  float t = 0.;
  for (int i=0; i<120; i++) {
    HIT h = map(p + t * dir);
    t += h.x;
    if (abs(h.x) < .0001) {
      hit = h;
      break;
    }
  }
  p += t * dir; 
  return hit;
}
HIT opU (HIT a, HIT b) {
  if (a.x < b.x) return a;
  return b;
}
float specular (vec3 n, vec3 pos, float m, vec3 ldir, vec3 dir, float p) {
  return specularStrength(m, n, pos) * pow(max(dot(dir, reflect(ldir, n)), 0.0), p);
}
float grayscale (vec3 c) {
  return 0.299 * c.r + 0.587 * c.g + 0.114 * c.b;
}

vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir) {
  vec3 l, ldir;
  vec3 c = vec3(0.);
  l = vec3(${opts.lightPos.toFixed(1)}, 1.5, -3.4);
  vec3 obj = shade(hit, p);
  ldir = normalize(l - p);
  c +=
  0.92 * vec3(0.9, 0.7, 0.6) * (
    // ambient
    0.1
    // diffuse
    + obj
      * (.5 + .5 * diffuse(p, n, l)) // half lambert
      * (0.5 + 0.5 * softshadow(p, ldir, 0.05, 5., 8.))
    + specular(n, p, hit.y, ldir, dir, 10.)
  );
  l = vec3(${(-opts.lightPos).toFixed(1)}, 5., -2.);
  ldir = normalize(l - p);
  c +=
  0.92 * vec3(0.3, 0.5, 0.6) * (
  // ambient
  0.1
  // diffuse
  + obj
    * (.5 + .5 * diffuse(p, n, l)) // half lambert
  + specular(n, p, hit.y, ldir, dir, 20.)
  );

  l = vec3(0., 2., 8.);
  ldir = normalize(l - p);
  c += ${
    bgColor === "White" || bgColor === "Black"
      ? opts.bgOrangeNoise
        ? "Orange*"
        : "0.3*"
      : "0.8*"
  }${bgColor} * (
  + obj
    * diffuse(p, n, l)
  + specular(n, p, hit.y, ldir, dir, 20.)
  );
  return c;
}
float noiseMetal;
float specularStrength (float m, vec3 n, vec3 p) {
  if (m < 2.1) {
  	float v =
  	  n.z * fbm(600. * p.xy) +
  	  n.x * fbm(600. * p.yz) +
  	  n.y * fbm(600. * p.xz);
  	return 0.4 + 0.3 * v;
  }
  if (m < 2.2) {
    return 2.0;
  }
  if (m < 4.) {
    return 0.6 - 0.5 * noiseMetal + 1. * ceil(m-2.21);
  }
  return 0.4;
}

vec3 sceneBg(vec2 p) {
  vec2 motion = vec2(
    cos(2. * PI * time),
    sin(2. * PI * time));
  vec2 c = p - 0.5;
  float r = length(c);
  float n = fbm9(
    ${opts.bgRadial.toFixed(2)} * r +
    0.5 * p +
    ${opts.bgNoiseSeed.toFixed(2)} +
    0.5 * fbm9(
      -${opts.bgNoiseSeed.toFixed(2)} +
      p * 3.0 +
      ${opts.bgNoiseMotion.toFixed(2)} * motion +
      ${opts.bgNoiseFreq.toFixed(2)} * vec2(
        fbm9(vec2(3.7, 7.7) + p * 20.0),
        fbm9(p * 30.0 - vec2(7.7, 3.3) + ${opts.bgNoiseSeed.toFixed(2)})
      )
    )
  );
  ${
    !opts.bgOrangeNoise
      ? `return ${bgColor} + ${opts.bgNoise.toFixed(2)} * (n - 0.5);`
      : `return mix(${bgColor}, Orange, smoothstep(-0.0001,${opts.bgNoise.toFixed(
          2
        )},n-0.5));`
  }
}

vec3 sticker_color;

vec3 shade (HIT hit, vec3 p) {
  if (hit.y < 2.0) return sceneBgColor;
  if (hit.y < 4.0) {
    if (hit.y < 2.1) {
    	return ${opts.colorName};
    }
    if (hit.y < 2.2) {
      vec2 coord = fract(fract(vec2(-0.2, 0.5) + vec2(3.6) * p.xy / vec2(-2.25, 1.0)) + ${
        opts.scrollingScreen ? "vec2(0.5+floor(time*15.0)/15.0, 0.)" : "0.0"
      });
      ${flipY ? "coord.y = 1.0 - coord.y;" : ""}
      vec2 a = coord * vec2(128.,64.);
      float edge = min(fract(a.x), fract(a.y));
      coord = floor(a) / vec2(128.,64.);
      float m = step(texture2D(text, coord).x, 0.5) * (1.0 - 0.5 * step(edge, 0.25));
      ${opts.blinkingScreen ? "m*=step(fract(2.*time),0.5);" : ""}
      ${opts.halfnegativeScreen ? "m=mix(m,1.-m,step(coord.y, 0.5));" : ""}
${
  !opts.screenAnimation
    ? ""
    : `
      float sz = ${(
        1 -
        opts.screenAnimation[3] * opts.screenAnimation[3]
      ).toFixed(2)};
      coord -= 0.5;
      coord *= vec2(2.,1.) * ${(
        1 -
        opts.screenAnimation[3] * opts.screenAnimation[3]
      ).toFixed(2)};
      coord += 0.5;
      ${
        opts.screenAnimation[1] < 0.2
          ? `coord.y${opts.screenAnimation[1] < 0.1 ? "+" : "-"}=time;`
          : ""
      }
      ${opts.screenAnimation[2] < 0.2 ? `coord.x-=time;` : ""}
      coord=fract(coord);
      m=mix(m,1.-m,step(shape(coord,2.*PI*time), 0.5));
    `
}
      return mix(
        vec3(0.01),
        vec3(1.0),
        ${opts.negativeScreen ? "1.-" : ""}m
      );
    }

    return vec3(0.7 - 0.1 * noiseMetal - 0.2 * (hit.y - 2.2));
  }
  if (hit.y < 5.0) {
    return sticker_color;
  }
  return vec3(0.0);
}

// ref: 5mm -> 0.1
HIT sdLedgerNanoSPlus (vec3 p, float rot) {
  float btn = sdBoxRoundZ(vec3(abs(p.x - 0.18) - 0.22, p.z, p.y - 0.155 + ${
    opts.btnAnimate
      ? "0.01*cubicInOut(1.-abs(cos(PI*(4.*time+0.5*step(0.,p.x-0.18)))))"
      : "0."
  }), vec3(0.06, 0.03, 0.04), 0.03);
  float case2d = sdBox2(p.xy, vec2(0.624, 0.174)-0.08)-0.08;
  float swivel_hook = sdCylinder(p.xzy, vec3(-0.44, 0.0, 0.074));
  float zcrop = abs(p.z)-0.101;
  float front_carving = abs(p.z+0.12)-0.015;
  float screen2 = sdBox2(
    p.xy - vec2(0.18, 0.),
    vec2(0.27, 0.12));
  HIT s = HIT(max(
    min(
      opSmoothSubtraction(
		    min(
          max(case2d+0.015, front_carving), // main casing carving
          btn-0.004 // btns carving
        ),
        max(case2d, zcrop) - 0.01, // main casing
        0.008
      ),
      min(
        btn,
        max(swivel_hook-0.015, abs(p.z)-0.12) // plastic in the casing for the swivel
      )
    ),
    -swivel_hook // carve the swivel hook out
  ), 2.05);
  // screen
  s = opU(s, HIT(max(s.x, screen2), 2.1));
  // swivel
  p.x += 0.04;
  p.x += 0.4;
  pR(p.xy, rot);
  p.x -= 0.4;
  float w = 0.54;
  float x = p.x + 0.8;
  float z = abs(p.z) - 0.12;
  float swivel_radius = 0.192;
  float swivel_metal_width = 0.006;
  float rounding = 0.003;
  float swivel = opSmoothSubtraction(
    sdCylinder(p.xzy, vec3(-0.4, 0.0, 0.08)), // carved
    min(
      sdCappedCylinder(vec3(p.y, z, x - 0.4), swivel_radius, swivel_metal_width),
      sdBox(vec3(x - 0.41 + w, p.y, z), vec3(w, swivel_radius, swivel_metal_width))
    )-rounding,
    0.04
  );
  // metal to close the swivel end
  swivel = fOpUnionRound(swivel,
    sdBox(vec3(x + 0.135 + w, p.y, p.z), vec3(swivel_metal_width, swivel_radius, 0.123))
  ,0.01);
  
  noiseMetal = fbm(vec2(40.0, 1000.) * p.xy);
  vec2 coord = fract(vec2(1.0, -3.0) * p.xy + vec2(0.5));
  ${flipY ? "coord.y = 1.0 - coord.y;" : ""}
  vec4 mt = texture2D(metalText, coord);
  float t = mix(0., grayscale(mt.rgb),mt.a * step(p.z, 0.) * step(p.x, -0.5) * step(abs(p.y), 0.16));
  float swivelM = 2.2 + t;
  s = opU(s, HIT(swivel, swivelM));
  ${
    !opts.sticker
      ? ""
      : `
      vec2 q = p.xy + vec2(${opts.stickerPosX.toFixed(
        2
      )}, ${opts.stickerPosY.toFixed(2)});
      float sticker_size = 0.15;
      float sticker_border = 0.01;
      coord = fract(vec2(.5,-.5) * q / sticker_size - 0.5);
      ${flipY ? "coord.y = 1.0 - coord.y;" : ""}
      vec4 v = texture2D(stickerText, coord);
      sticker_color = mix(vec3(1.), v.rgb, v.a);
      float l = length(q.xy)-sticker_size;
      s = opU(s, HIT(max(
        abs(p.z + 0.13)-0.005,
        l-sticker_border
      ), 4.2 - step(0.0, l)));
  `
  }
  return s;
}
HIT map (vec3 p) {
  HIT s = HIT(10. - length(p), 0.);
  float t = 3. * fract(${opts.slowSwivelRotate ? "0.5" : "1."} * time);
  float rot = ${
    opts.swivelRotate
      ? `PI * (
    1. +
    cubicInOut(min(1.0, t)) +
    cubicInOut(min(1.0, max(t - 1.8, 0.0)))
  )`
      : opts.swivelAngle.toFixed(2)
  };
  s = opU(s, sdLedgerNanoSPlus(p, rot));
  return s;
}
vec3 scene(vec2 uv) {
  vec3 focus = vec3(0.0, 0.0, 0.);
  vec3 c = vec3(0.);
  vec3 origin = vec3(
    ${opts.motion[0].toFixed(2)}*${
    opts.motionPhase[0]
  }(${opts.motionAcc[0].toFixed(2)}*PI*time${
    opts.motionComplex ? `+cos(PI*${opts.motionComplex.toFixed(1)}*time)` : ""
  }),
   0.5+${opts.motion[1].toFixed(2)}*${
    opts.motionPhase[1]
  }(${opts.motionAcc[1].toFixed(2)}*PI*time${
    opts.motionComplex ? `+cos(PI*${opts.motionComplex.toFixed(1)}*time)` : ""
  }),
   -1.8-${opts.motion[2].toFixed(2)}*${
    opts.motionPhase[2]
  }(${opts.motionAcc[2].toFixed(2)}*PI*time${
    opts.motionComplex ? `+cos(PI*${opts.motionComplex.toFixed(1)}*time)` : ""
  })
  );
  vec3 p = origin;

  vec3 dir = normalize(vec3(uv - .5, 1.));
  dir = lookAt(origin, focus) * dir;
  
  HIT hit = marcher(p, dir);
  vec3 n = normal(p);
  c += lighting(hit, p, n, dir);
  c = mix(c, sceneBgColor, pow(smoothstep(4., 10., length(p-origin)), .5));
  return c;
}
vec3 render() {
  vec3 c = vec3(0.);
  vec2 ratio = resolution / max(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio${flipY ? "*vec2(1.,-1.)" : ""};

${
  AA
    ? `
  for (float x=-.5; x<=.5; x += 1.) {
    for (float y=-.5; y<=.5; y += 1.) {
      vec2 d = 0.5 * vec2(x,y) / resolution;
      vec2 p = base + d;
      sceneBgColor = sceneBg(p);
      c += scene(p);
    }
  }
  c /= 4.0;`
    : `
  sceneBgColor = sceneBg(base);
  c += scene(base);
  `
}
  return c;
}
void main() {
  vec3 c = render();
  gl_FragColor = vec4(c, 1.0);
}
  `;

  const render = regl({
    frag,
    vert: `precision mediump float;attribute vec2 p;varying vec2 uv;void main(){uv=p;gl_Position=vec4(2.*p-1.,0,1);}`,
    attributes: {
      p: [-2, 0, 0, -2, 2, 2],
    },
    uniforms: {
      text: regl.texture(createImageTexture(screenCanvas)),
      metalText: regl.texture(createImageTexture(metalCanvas)),
      stickerText: regl.texture(createImageTexture(stickerCanvas)),
      time: regl.prop("time"),
      resolution: ({ viewportWidth, viewportHeight }) => [
        viewportWidth,
        viewportHeight,
      ],
    },
    count: 3,
  });

  let t = 0;
  regl.frame((v) => {
    regl.clear({
      depth: 1,
      color: [0, 0, 0, 1],
    });
    const time = speed * frameTime(t, v);
    render({ time });
    onFrame(t, time);
    t++;
  });

  return () => regl.destroy();
}
