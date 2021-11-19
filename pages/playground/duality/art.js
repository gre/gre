function art(regl, onImageRendered) {
  const random = window.fxrand;

  let tMul = Math.round(5 * random() - 2);
  if (tMul === 0) tMul++;

  const Smoothing =
    Math.floor(1.5 - random() * random() + 10 * Math.pow(random(), 8)) / 10;

  const Polar =
    random() < 0.2 ? Math.floor(3 + 15 * random() * random() * random()) : 0;

  let preX = random() < 0.1;
  let preY = random() < 0.1;
  let postX = random() < 0.1;
  let postY = random() < 0.1;
  const Repetitions = [
    preX && "preX",
    preY && "preY",
    postX && "postX",
    postY && "postY",
  ]
    .filter(Boolean)
    .join(" ");

  const Rotating = random() < 0.2;

  let circle = random() < 0.6;
  const rectangle = true || random() < 0.2;
  if (!rectangle) {
    circle = true;
  }
  const Shapes = [circle && "Circle", rectangle && "Rectangle"]
    .filter(Boolean)
    .join(" ");

  const orientationV = random() < 0.2;
  const Orientation = orientationV ? "Vertical" : "Horizontal";

  let Palette = "Monochrome";
  let palette;

  if (random() < 0.02) {
    const p = [
      Math.floor(random() * 3),
      2 + Math.floor(random() * 3),
      1 + Math.floor(random() * 3),
    ];
    palette = p.map((n) => 0.2 + 0.1 * n).join(",");
    Palette = p.join("");
  }

  let aberration = 0.02;
  let Aberration = "normal";
  let r = random();
  if (r < 0.03) {
    aberration = 0.25;
    Aberration = "Extreme";
  } else if (r < 0.1) {
    aberration = 0.1;
    Aberration = "Very High";
  } else if (r < 0.2) {
    aberration = 0.05;
    Aberration = "High";
  }

  const metadata = {
    Palette,
    Smoothing,
    Polar,
    Shapes,
    Rotating,
    Repetitions,
    Orientation,
    Aberration,
  };

  const frag = `

  precision highp float;
varying vec2 uv;
uniform float time;
uniform vec2 resolution;
#define PI ${Math.PI}
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
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
float pMod1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize, size) - halfsize;
	return c;
}
float pModPolar(inout vec2 p, float repetitions) {
	float angle = 2.*PI/repetitions;
	float a = atan(p.y, p.x) + angle/2.;
	float r = length(p);
	float c = floor(a/angle);
	a = mod(a,angle) - angle/2.;
	p = vec2(cos(a), sin(a))*r;
	if (abs(c) >= (repetitions/2.)) c = abs(c);
	return c;
}
float vmax(vec2 v) {
  return max(v.x, v.y);
}
float fBox2(vec2 p, vec2 b) {
	vec2 d = abs(p) - b;
	return length(max(d, vec2(0))) + vmax(min(d, vec2(0)));
}
float shape (vec2 p, float d) {
  float t = 0.5 * PI * time + ${aberration.toFixed(3)} * d;
  float smoothing = ${Smoothing.toFixed(2)};
  p -= 0.5;

  ${Rotating ? `pR(p, t);` : ""}
  ${Polar ? `pModPolar(p, ${Polar}.);` : ""}

  vec2 q = p;
  ${preX ? `pMod1(p.x, ${(0.1 + 0.8 * random()).toFixed(2)});` : ""}
  ${preY ? `pMod1(p.y, ${(0.1 + 0.8 * random()).toFixed(2)});` : ""}
  pR(p, ${tMul}. * t + cos(${Math.round(5 * random() - 2)}. * t));
  ${preX ? `pMod1(p.x, ${(0.1 + 0.8 * random()).toFixed(2)});` : ""}
  ${preY ? `pMod1(p.y, ${(0.1 + 0.8 * random()).toFixed(2)});` : ""}
  vec2 dist = vec2(0.0);
  float crop = 99.0;
  float s = 99.0;;
  s = fOpUnionRound(${orientationV ? "q.x" : "q.y"}, s, smoothing);
  
  ${
    !circle
      ? ""
      : `\
dist = vec2(${(0.3 + 0.3 * random() * random() * (0.5 - random())).toFixed(
          2
        )}, 0.0);
float radius = ${(0.1 + 0.1 * random() * random() * (0.5 - random())).toFixed(
          2
        )};
s = fOpUnionRound(s, length(p + dist) - radius, smoothing);
crop = fOpUnionRound(crop, length(p - dist) - radius, smoothing);`
  }${
    !rectangle
      ? ""
      : `\
  dist = vec2(${(0.6 * random()).toFixed(2)}, ${(
          0.5 *
          random() *
          random()
        ).toFixed(2)});
  vec2 sz = vec2(${(0.1 + 0.1 * random() * random() * (0.5 - random())).toFixed(
    2
  )},${(0.1 + 0.1 * random() * random() * (0.5 - random())).toFixed(2)});
  crop = fOpUnionRound(crop, fBox2(p + dist, sz), smoothing);
  s = fOpUnionRound(s, fBox2(p - dist, sz), smoothing);`
  }

  s = fOpDifferenceRound(s, crop, smoothing);
  return smoothstep(0.0, 1.0 / min(resolution.x, resolution.y), s);
}
vec3 palette(float t,vec3 a,vec3 b,vec3 c,vec3 d){
  return a+b*cos(6.28318*(c*t+d));
}
void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;  
  float b = shape(base, 0.);
  float a = shape(base, -1.);
  gl_FragColor = ${
    palette
      ? "vec4(palette(0.4 * b + 0.1 * a, vec3(.5), vec3(.8), vec3(1.), vec3(" +
        palette +
        ")), 1.0)"
      : "vec4(a, b, b, 1.0)"
  };
}
`;

  const render = regl({
    frag,
    vert: `precision mediump float;attribute vec2 p;varying vec2 uv;void main(){uv=p;gl_Position=vec4(2.*p-1.,0,1);}`,
    attributes: {
      p: [-2, 0, 0, -2, 2, 2],
    },
    uniforms: {
      time: regl.prop("time"),
      resolution: ({ viewportWidth, viewportHeight }) => [
        viewportWidth,
        viewportHeight,
      ],
    },
    count: 3,
  });

  let firstCall = onImageRendered;

  regl.frame(({ time }) => {
    render({ time });
    if (firstCall) {
      firstCall();
      firstCall = null;
    }
  });

  return {
    destroy: () => regl.destroy(),
    metadata,
  };
}

export default art;
