import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 6;
export const title = "Bestagons";
// reference to https://www.youtube.com/watch?v=thOifuHs6eY – thanks david

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

const float SEED = ${Math.random()};

// from http://glslsandbox.com/e#43182.0
#define SQRT3 1.7320508
const vec2 s = vec2(1.0, SQRT3);
float hex(in vec2 p){
  p = abs(p);
  return max(dot(p, s*.5), p.x);
}
vec4 getHex(vec2 p) {
  vec4 hC = floor(vec4(p, p - vec2(.5, 1))/s.xyxy) + .5;
  vec4 h = vec4(p - hC.xy*s, p - (hC.zw + .5)*s);
  return dot(h.xy, h.xy)<dot(h.zw, h.zw) ? vec4(h.xy, hC.xy) : vec4(h.zw, hC.zw + 9.73);
}
// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
///////////////////////////////////////////


vec4 scene (vec4 h) {
  vec2 p = h.xy;
  vec2 id = h.zw;
  float one = step(0., p.x+s.y*p.y) * step(0., s.y*p.y-p.x);
  float two = step(0., p.x-s.y*p.y) * step(0., p.x);
  float thr = step(p.x+s.y*p.y, 0.) * step(p.x, 0.);
  float special =
      step(mod(id.y, 4.), mod(id.x, 3.5)) *
      step(mod(id.x + 3.*id.y, 5.), 0.5);
  vec3 c =
  palette(
    0.1 * one + 0.2 * two + 0.3 * thr,
    vec3(0.5),
    vec3(0.5),
    vec3(1.00, 1.00, 1.00),
    vec3(
      mod(id.y * 73.6, 1.) * special * smoothstep(1.0, 0.0, cos(time + 0.1 * uv.x)),
      0.8 * fract((id)*0.01))
  );
  return vec4(c, 1.0);
}

void main() {
  vec2 p = 1000.0 * SEED + 0.3 * time + 5. * uv;
  gl_FragColor = scene(getHex(p));
}`,
  },
});
