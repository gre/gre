import Head from "next/head";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 1;
export const title = "Hello 2020";

export const Shader = ({ time }) => (
  <Node shader={shaders.day001} uniforms={{ time }} />
);

const shaders = Shaders.create({
  day001: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
float vingt (vec2 p) {
  p *= 1.1;
  p -= 0.1;
  return step(0.3, p.x) * step(p.x, 0.5) * 
  step(0.5, p.y) * step(p.y, 1.0) +
  step(0., p.x) * step(p.x, 0.2) * 
  step(0., p.y) * step(p.y, 0.5) +
  step(0., p.x) * step(p.x, 0.5) * 
  step(0., p.y) * step(p.y, 0.2) +
  step(0., p.x) * step(p.x, 0.5) * 
  step(0.4, p.y) * step(p.y, 0.6) +
  step(0., p.x) * step(p.x, 0.5) * 
  step(0.8, p.y) * step(p.y, 1.0) +
  step(0.6, p.x) * step(p.x, 1.0) * 
  step(0.1, p.y) * step(p.y, 0.9) -
  step(0.7, p.x) * step(p.x, 0.9) * 
  step(0.2, p.y) * step(p.y, 0.8);
}
void main() {
  vec2 p = uv + 0.1 * vec2(cos(uv.x*10.+time*0.1), sin(1.0+uv.y*10.+time*0.2));
  p -= 0.2 * pow(time*0.1, 1.3);
  vec2 r = p * (0.1+pow(0.2*time, 1.2));
  float s = vingt(mod(r, 1.0)) * step(mod(r.x, 4.0), 2.0) * step(mod(r.y, 2.0), 1.0);
  gl_FragColor = vec4(
    mix(
      vec3(1., 1., 1.),
      vec3(uv.x, uv.y, 0.5 + 0.2 * cos(time)),
      step(0.5, s)
    ), 1.0);
}`,
  },
});
