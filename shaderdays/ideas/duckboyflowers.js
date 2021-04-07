

import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 38;
export const title = "duck, boy and flowers";
// (((x+y)*t)%7-(t+x-2*y)%7)%3

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

void main() {
  vec2 p = floor(uv * 40.);
  float f = mod(
    mod((p.x + p.y) * floor(time), 7.)
  - mod(time + p.x - 2. * p.y, 7.),
  3.);
  vec3 c = vec3(f);
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
