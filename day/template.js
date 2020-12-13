import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 0;
export const title = "";

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
  vec3 clr = vec3(uv.x, uv.y, 0.5);
  gl_FragColor = vec4(clr, 1.0);
}`,
  },
});
