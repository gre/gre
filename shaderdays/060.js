import { Shaders, Node, GLSL } from "gl-react";

export const n = 60;
export const title = "bitcoin";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float t) {
  return palette(
    t,
    vec3(.5),
    vec3(.6 + .1 * cos(.3 * time)),
    vec3(1.),
    vec3(0.22, 0.5, 0.77)
  );
}

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

void main() {
  float v = bitcoin2d(uv-.5);
  v = max(-step(fract(time), 0.5), v); // blink 500ms
  gl_FragColor = vec4(
    step(v, 0.) * color(1.6 * (uv.y + time)) +
    step(0., v) * color(sqrt(max(v, 0.)) - 2. * time),
    1.0);
}
`,
  },
});
