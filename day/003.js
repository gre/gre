import Head from "next/head";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 3;
export const title = "zephyr";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL/* glsl */ `
precision highp float;
varying vec2 uv;
uniform float time;

const float PI = ${Math.PI};

float flower (vec2 p, float n, float s, float r) {
  float a = 0.5 + 0.5 * atan(p.y, p.x) / PI;
  float d = length(p);
  float amp = (1. + 0.5 * cos((r + a * n) * 2. * PI)) / 3.;
  return step(d, s * amp);
}

float helios (vec2 p, float n, float s, float r) {
  return step(
    length(p) - step(length(p), 0.03),
    s * fract(r + n * (0.5 + 0.5 * atan(p.y, p.x) / PI)));
}

vec2 pattern (vec2 p, vec2 m) {
  float f = 0.5 * m.y * step(1., mod(0.5 + p.x / m.x, 2.));
  p.y += f;
  p = mod(p + m / 2., m) - m / 2.;
  return p;
}

// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 clr (float f) {
  float c = cos(time);
  f += 0.2 * (1. + sign(c) * pow(abs(c), 0.2));
  return palette(
    f,
    vec3(0.5),
    vec3(0.5),
    vec3(1.00, 1.00, 1.00),
    vec3(0.0, 0.2, 0.3)
  );
}

void main() {
  vec3 c = mix(
    mix(
      clr(0.2),
      clr(0.18),
      helios(uv - 0.5, 5., 0.4, 0.2 * time)
    ),
    clr(.2 + .1 * cos(6. * time + floor(10.0 * uv.y + 0.5))),
    flower(pattern(uv, vec2(0.2)), 5., 0.1, time)
  );
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
