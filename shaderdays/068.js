import React from "react";
import { Shaders, Node, GLSL, Uniform, NearestCopy } from "gl-react";

export const n = 68;
export const title = "ring";
export const exportSize = 800;
export const exportStart = 2;
export const exportEnd = exportStart + 10;
export const exportFramePerSecond = 30;
export const exportSpeed = 1;
// render more frames and skip
export const exportSkipFrame = 7;
export const exportPaletteGenOnce = true;
export const exportPaletteSize = 128;

export const Shader = ({ time }) => (
  <NearestCopy>
    <Persistence persistence={0.9}>
      <Node
        shader={shaders.render}
        uniforms={{ resolution: Uniform.Resolution, time, freq: 2 }}
      />
    </Persistence>
  </NearestCopy>
);

const Persistence = ({ children: t, persistence }) => (
  <Node
    shader={shaders.persistence}
    backbuffering
    uniforms={{ t, back: Uniform.Backbuffer, persistence }}
  />
);

const PERLIN_NOISE =
  // https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83
  `
vec4 permute(vec4 x){return mod(((x*34.0)+1.0)*x, 289.0);}
vec2 fade(vec2 t) {return t*t*t*(t*(t*6.0-15.0)+10.0);}
float cnoise(vec2 P){
  vec4 Pi = floor(P.xyxy) + vec4(0.0, 0.0, 1.0, 1.0);
  vec4 Pf = fract(P.xyxy) - vec4(0.0, 0.0, 1.0, 1.0);
  Pi = mod(Pi, 289.0); // To avoid truncation effects in permutation
  vec4 ix = Pi.xzxz;
  vec4 iy = Pi.yyww;
  vec4 fx = Pf.xzxz;
  vec4 fy = Pf.yyww;
  vec4 i = permute(permute(ix) + iy);
  vec4 gx = 2.0 * fract(i * 0.0243902439) - 1.0; // 1/41 = 0.024...
  vec4 gy = abs(gx) - 0.5;
  vec4 tx = floor(gx + 0.5);
  gx = gx - tx;
  vec2 g00 = vec2(gx.x,gy.x);
  vec2 g10 = vec2(gx.y,gy.y);
  vec2 g01 = vec2(gx.z,gy.z);
  vec2 g11 = vec2(gx.w,gy.w);
  vec4 norm = 1.79284291400159 - 0.85373472095314 * 
    vec4(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
  g00 *= norm.x;
  g01 *= norm.y;
  g10 *= norm.z;
  g11 *= norm.w;
  float n00 = dot(g00, vec2(fx.x, fy.x));
  float n10 = dot(g10, vec2(fx.y, fy.y));
  float n01 = dot(g01, vec2(fx.z, fy.z));
  float n11 = dot(g11, vec2(fx.w, fy.w));
  vec2 fade_xy = fade(Pf.xy);
  vec2 n_x = mix(vec2(n00, n01), vec2(n10, n11), fade_xy.x);
  float n_xy = mix(n_x.x, n_x.y, fade_xy.y);
  return 2.3 * n_xy;
}
`;

const shaders = Shaders.create({
  persistence: {
    frag: GLSL`
  precision highp float;
  varying vec2 uv;
  uniform sampler2D t, back;
  uniform float persistence;
  void main () {
    gl_FragColor =
      mix(
        texture2D(t, uv),
        texture2D(back, uv),
        persistence
      );
  }
      `,
  },
  render: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
uniform float freq;
uniform vec2 resolution;

${PERLIN_NOISE}

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

float shape (vec2 p, float d) {
  float t = 0.2 * PI * (time + d);
  p -= 0.5;
  vec2 p1 = p;
  vec2 p2 = p;
  pR(p1, - 2. * t);
  pR(p2, t);
  float n = cnoise(10.0 + 5. * p1);
  float n2 = cnoise(7. * p2);
  float r = 0.18 + 0.1 * sin(t);
  float s = abs(fOpUnionRound(
    length(p - vec2(0.1 * cos(t), 0.0)) - r - 0.1 * n,
    length(p) - r - 0.1 * n2,
    0.2
  ))-0.05;
  return smoothstep(0.0, 1.0 / min(resolution.x, resolution.y), s);
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;  
  float a = shape(base, -0.05);
  float b = shape(base, 0.0);
  float c = shape(base, 0.05);
  gl_FragColor = vec4(
    a,
    b,
    c,
    1.0
  );
}
`,
  },
});
