

import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 34;
export const title = "⌊2cos(x)sin(y+t)⌋%7";

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
#define PI ${Math.PI}

// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
vec3 color (float t) {
  return palette(
    t,
    vec3(.5),
    vec3(.5),
    vec3(.6, 1., .4),
    vec3(.9, .2, .7)
  );
}

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

float cell (vec2 p) {
  return mod(floor(2. * cos(p.x) * sin(p.y + time)), 7.);
}

void main() {
  float unzoom = 32.;
  vec2 offset = time * vec2(1., -1.);
  vec2 p = uv * unzoom + offset;
  pR(p, PI/4.);
  vec3 c = color(.1 * floor(.2 * time) + (.1 + .005 * time) * cell(floor(p)));
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
