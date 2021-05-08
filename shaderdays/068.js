import React from "react";
import { Shaders, Node, GLSL, Uniform, NearestCopy } from "gl-react";

export const n = 68;
export const title = "Jumping blob";
export const exportSize = 800;
export const exportStart = 2;
export const exportEnd = exportStart + 8;
export const exportFramePerSecond = 30;
export const exportSpeed = 1;
// render more frames and skip
export const exportSkipFrame = 7;
export const exportPaletteGenOnce = true;
export const exportPaletteSize = 16;

export const Shader = ({ time }) => (
  <NearestCopy>
    <Persistence persistence={0.6}>
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
  float t = 0.5 * PI * time + d;
  float radius = 0.18;
  float smoothing = 0.2;
  float dist = 0.2;
  p -= 0.5;
  pR(p, PI / 2.0);
  vec2 q = p;
  pR(p, -2. * t + cos(t));
  float s = fOpDifferenceRound(
    fOpUnionRound(
      max(q.x, 0.1 + q.x),
      length(p + vec2(dist, 0.0)) - radius,
      smoothing),
    length(p - vec2(dist, 0.0)) - radius,
    smoothing);
  return smoothstep(0.0, 1.0 / min(resolution.x, resolution.y), s);
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;  
  float a = shape(base, -0.02);
  float b = shape(base, 0.0);
  float c = shape(base, 0.01);
  float m = min(min(a, b), c);
  gl_FragColor = vec4(
    b,
    m,
    m,
    1.0
  );
}
`,
  },
});
