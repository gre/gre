import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 99;
export const title = "";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL/* glsl */ `
precision highp float;
varying vec2 uv;
uniform float time;

void main() {
  gl_FragColor = vec4(1.0);
}`,
  },
});
