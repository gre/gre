

import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 21;
export const title = "HODL";

export const Shader = ({ time }) => (
  <LinearCopy>
    <Persistence persistence={Math.min(.99, .015 * time)} time={time}>
      <Node shader={shaders.node} uniforms={{ time }} />
    </Persistence>
  </LinearCopy>
);

const Persistence = ({ children: t, persistence, time }) => (
  <Node
    shader={shaders.shakePersistence}
    backbuffering
    uniforms={{ t, back: Uniform.Backbuffer, persistence,time }}
  />
);

const shaders = Shaders.create({
  shakePersistence: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform sampler2D t, back;
uniform float time;
uniform float persistence;
mat2 rot (float a) {
  float c = cos(a);
  float s = sin(a);
  return mat2(c,s,-s,c);
}
void main () {
  gl_FragColor = 1.02 * mix(
    texture2D(t, uv),
    texture2D(back, uv + vec2(.0, 0.005) * rot(pow(time, 1.5))),
    persistence
  );
}
    `,
  },
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
const float PI = ${Math.PI};

float rect2d (vec2 p, vec2 sz) {
  return max(abs(p.x) - sz.x, abs(p.y) - sz.y);
}
float d2d (vec2 p, float w, float h) {
  return min(rect2d(p, vec2(w, h)), length(p-vec2(w, .0))-h);
}
float bitcoin2d (vec2 p) {
  p.y -= 0.1;
  p.x += .02;
  float inner = d2d(p, 0.04, 0.06);
  float outer = d2d(p, 0.1, 0.1);
  float top = max(-inner, outer);
  top = min(top, rect2d(p - vec2(-.12, .08), vec2(.02)));
  top = min(top, rect2d(p - vec2(-.06, .14), vec2(.02, .04)));
  top = min(top, rect2d(p - vec2(.04, .14), vec2(.02, .04)));
  p.x -= .01;
  p.y += 0.2;
  inner = d2d(p, 0.04, 0.06);
  outer = d2d(p, 0.11, 0.12);
  float bottom = max(-inner, outer);
  bottom = min(bottom, rect2d(p - vec2(-.13, -.09), vec2(.03)));
  bottom = min(bottom, rect2d(p - vec2(-.06, -.16), vec2(.02, .04)));
  bottom = min(bottom, rect2d(p - vec2(.04, -.16), vec2(.02, .04)));
  float f = min(top, bottom);
  return f;
}

vec3 cBase = vec3(.8, .5, .0);
vec3 cMain = vec3(1., .8, .6);
vec3 cSec = vec3(.9, .6, .1);

void main() {
  vec2 p = uv - .5;
  p.y -= 0.05 * min(1., time * .02);
  p *= 1.2;
  float phase = cos(pow(.2 * time, 1.8));
  p.x /= .02 + .98 * abs(phase);
  float shape = mix(bitcoin2d(p), 1., step(phase, .0));
  float d = length(p);
  vec3 c =
    smoothstep(.3, .7, length(uv-.5)) * cBase * .3 +
    step(shape, 0.01) * step(0., shape) * cMain +
    step(0.013, shape) * step(d, 0.41) * mix(cBase, cSec, smoothstep(.003, .002, abs(mod(d, .02)-.01))) +
    step(0.013, shape) * step(d, 0.48) * smoothstep(.011, .01, abs(d-.43)) * cMain;
  gl_FragColor = vec4(c * ((.3 * phase + .7 * abs(phase)) + .4), 1.0);
}`,
  },
});
