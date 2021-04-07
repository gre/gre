

import { Shaders, Node, GLSL } from "gl-react";

export const n = 14;
export const title = "seizure";

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

void main() {
  float tri = max(
    0.2 - uv.y,
    max( 0.6 * uv.y - uv.x - 0.,
         0.6 * uv.y + uv.x - 1.));
  tri = max(-step(fract(time), 0.5), tri); // blink 500ms
  gl_FragColor = vec4(
    step(tri, 0.) * color(1.6 * (uv.y + time)) +
    step(0., tri) * color(sqrt(max(tri, 0.)) - 2. * time),
    1.0);
}
`,
  },
});
