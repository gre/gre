import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 53;
export const title = "cross waves";

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
    vec3(1., .2, .3)
  );
}

void main() {
  float ci = 0.;
  float z = 5. + .01 * time;
  vec2 p = uv;
  p.y -= .1 * time;
  vec2 id = floor(p * z);
  p *= z;
  float even = mod(id.y, 2.);
  p.x += even * .5;
  p = fract(p);
  p.y = mix(p.y, 1.-p.y, even);
  float y = p.y;
  p.y = min(y, 1.-y);
  float alt = step(y, p.y);
  p.x = fract(p.x + mix(-time, time, alt));
  float l = length(p - vec2(.5, .0));
  float smooth = .01;
  float a = smoothstep(-smooth, smooth, abs(l-0.1)-.05);
  float b = smoothstep(-smooth, smooth, abs(l-0.4)-.05);
  float c = smoothstep(smooth, -smooth, abs(l-0.25)-.08);
  ci += a * b;
  ci += (1. + alt) * c;
  float mul = .2 + .3 * cos(.2 * time);
  vec3 from = color(floor(ci) * mul);
  vec3 to = color(ceil(ci) * mul);
  gl_FragColor = vec4(mix(from, to, fract(ci)), 1.0);
  /*
  // debug palette
  if (uv.y < .05) {
    gl_FragColor = vec4(color(uv.x), 1.0);
  }
  */
}`,
  },
});
