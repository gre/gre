import React from "react";
import { Shaders, Node, GLSL, Uniform, NearestCopy } from "gl-react";

export const n = 66;
export const title = "hicetnunc";
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
    <Persistence persistence={0.5}>
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
float sdBox( in vec2 p, in vec2 b ) {
    vec2 d = abs(p)-b;
    return length(max(d,0.0)) + min(max(d.x,d.y),0.0);
}
float fOpUnionRound(float a, float b, float r) {
	vec2 u = max(vec2(r - a,r - b), vec2(0));
	return max(r, min (a, b)) - length(u);
}

float shape (vec2 p, float d) {
  float t = .5 * PI * time + 0.01 * d;
  float radius = 0.1;
  float smoothing = 0.05;
  p -= 0.5;
  vec2 q = p;
  q.x += 0.3 * cos(t);
  q.y += 0.1 * sin(t);
  pR(q, t + sin(freq * t));
  
  p *= 20.0;
  p.x += 5.;
  float s = sdBox(p, vec2(0.5, 1.5));
  p.x -= 1.;
  s = min(s, sdBox(p, vec2(0.5, 0.5)));
  p.x -= 1.;
  s = min(s, sdBox(p, vec2(0.5, 1.5)));
  p.x -= 3.;
  s = min(s, sdBox(p - vec2(0., 1.), vec2(1.5, 0.5)));
  s = min(s, sdBox(p + vec2(0., 1.), vec2(1.5, 0.5)));
  p.x -= 3.;
  s = min(s, sdBox(p, vec2(0.5, 1.5)));
  p.x -= 1.;
  s = min(s, sdBox(p-vec2(0., 1.), vec2(0.5, 0.5)));
  p.x -= 1.;
  s = min(s, sdBox(p, vec2(0.5, 1.5)));
  s /= 20.0;
  
  s = fOpUnionRound(q.x, s, smoothing);
  float v = smoothstep(0.0, 1.0 / min(resolution.x, resolution.y), s);
  return mix(v, 1. - v, step(0., sin(t)));
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;  
  float a = shape(base, -1.);
  float b = shape(base, 1.);
  gl_FragColor = vec4(a, b, b, 1.0);
}
`,
  },
});
