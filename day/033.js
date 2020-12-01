import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 33;
export const title = "x(x+2y)%(tN+1)%n";

const SIZE = 100;

let firstTime;
export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float t) {
  return palette(
    t,
    vec3(.5),
    vec3(.5),
    vec3(1.),
    vec3(.1, .3, .4)
  );
}

float cell (vec2 p) {
  float m = 2. + floor(.05 * time);
  return mod(mod(p.x * (p.x + 2. * p.y), m * floor(time) + 1.), m);
}

void main() {
  float unzoom = 32. + 2. * time;
  vec2 offset = vec2(-.25 * unzoom, -2. * pow(time, 1.3));
  vec3 c = color(.1 * cell(floor(uv * unzoom + offset)));
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
