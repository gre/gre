import React from "react";
import { Shaders, Node, GLSL, Uniform, NearestCopy } from "gl-react";

export const n = 65;
export const title = "Duality";
export const exportSize = 800;
export const exportStart = 4;
export const exportEnd = 8;
export const exportFramePerSecond = 30;
export const exportSpeed = 1;
// render more frames and skip
export const exportSkipFrame = 3;
export const exportPaletteGenOnce = true;
export const exportPaletteSize = 16;

export const Shader = ({ time }) => (
  <NearestCopy>
    <Persistence persistence={0.3}>
      <Node
        shader={shaders.render}
        uniforms={{ resolution: Uniform.Resolution, time }}
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
  float t = 0.5 * PI * time + 0.005 * d;
  float radius = 0.1;
  float smoothing = 0.1;
  p -= 0.5;
  pR(p, t);
  vec2 q = p;
  pR(p, -2. * t + cos(2. * t));
  float s = fOpDifferenceRound(
    fOpUnionRound(
      q.x, // axis
      length(p + vec2(0.3, 0.0)) - radius, // first circle
      smoothing),
    length(p - vec2(0.3, 0.0)) - radius, // second circle (cropped out)
    smoothing);
  return smoothstep(0.0, 1.0 / min(resolution.x, resolution.y), s);
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;  
  float a = shape(base, -1.);
  float b = shape(base, 0.);
  gl_FragColor = vec4(a, b, b, 1.0);
}
`,
  },
});
