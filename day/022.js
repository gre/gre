

import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 22;
export const title = "atoms";

export const Shader = ({ time }) => (
  <Node
    shader={shaders.node}
    uniforms={{ time, t: "/images/einstein-tongue-out.jpg" }}
  />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
uniform sampler2D t;

const float PI = ${Math.PI};

// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float t) {
  return palette(
    .8 - .8 * t,
    vec3(.5),
    vec3(.5),
    vec3(.4, .7, .6),
    vec3(.1, .1, .2)
  );
}

void main() {
  float rez = max(1., 64. - 8. * floor(.03 * time));
  vec2 g = mod(uv * rez, rez);
  g -= rez / 2.;
  g += .5 * rez * vec2(cos(1.5 + .5 * time), sin(.5 * time)) * smoothstep(20., 10., time);
  g *= min(1., pow(.02 + .012 * time, 1.8));
  g += rez / 2.;
  vec2 l = fract(g);
  vec2 gf = floor(g) + .5;
  g -= rez / 2.;
  g *= .8 + .2 * pow(time * .05, 1.2) * cos(.3 * time) * length(g / rez) * .5 * (1. + pow(time * .01, 1.3) * vec2(cos(g.x + .9 * time), sin(g.y + 1.1 * time)));
  g += rez / 2.;
  float r = texture2D(t, g / rez).r;
  float rf = texture2D(t, gf / rez).r;
  vec3 c = smoothstep(.01, .0, length(l-.5) - r * 0.5) * mix(color(rf), vec3(rf), min(1., .002 * time));
  gl_FragColor = vec4(c, 1.);
}`,
  },
});
