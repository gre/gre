

import { Shaders, Node, GLSL } from "gl-react";

export const n = 13;
export const title = "chip";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
const float PI = ${Math.PI};

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float t) {
  return palette(
    t,
    vec3(.6),
    vec3(.3),
    vec3(1.),
    vec3(.2, .55, .75)
  );
}

void main() {
  vec2 p = uv - .5;
  vec2 ap = abs(p);
  float rect = max(ap.x, ap.y) - .2; // rect
  float m = 8.;
  float squircle = pow(pow(ap.x, m) + pow(ap.y, m), 1./m) - .2; // squircle
  float a = 0.5 + 0.5 * atan(p.y, p.x) / PI;
  vec3 glow =
    step(0., rect) *
    ( pow(smoothstep(0.3, 0., squircle), 8.)
      + 0.2 * smoothstep(0.02, 0., rect)
      - 0.3 ) *
    color(a + 0.8 + 0.1 * time);
  vec3 shade =
    step(rect, 0.) * 0.4 * (.05 + vec3(p.y - p.x));
  gl_FragColor = vec4(shade + glow, 1.0);
}
`,
  },
});
