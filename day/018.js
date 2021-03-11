

import { Shaders, Node, GLSL } from "gl-react";

export const n = 18;
export const title = "queen";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
const float PI = ${Math.PI};
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
float merge (float a, float b) {
  return min(1., a + b);
}
float flower (vec2 p, float n, float s) {
  float a = 0.5 + 0.5 * atan(p.y, p.x) / PI;
  float d = length(p);
  float amp = (1.4 + cos((a * n) * 2. * PI)) / 3.;
  float co = cos(PI/4.);
  float si = sin(PI/4.);
  p *= mat2(co,si,-si,co);
  p = abs(p);
  return smoothstep(0.01 + 0.5 * s, 0.5 * s, max(p.x, p.y)) * smoothstep(d-.01, d+.01, s * amp);
}
float flowerDot (vec2 p, float s) {
  p = abs(p);
  return smoothstep(0.06, 0.05, length(4. * p - s));
}
float circle (vec2 p) {
  return smoothstep(.04, .03, abs(length(p-.5) - .44));
}
float edgeShape (vec2 p) {
  p = 2. * abs(p-.5);
  float a = max(p.x, p.y);
  float b = min(p.x, p.y);
  return smoothstep(.54, .56, a * b * abs(cos(13. * a - 6. * b)));
}

vec3 color (float t) {
  return palette(
    t,
    vec3(.5),
    vec3(.5),
    vec3(1., 1., 0.8),
    vec3(.5, .3, .5)
  );
}

vec3 tile (vec2 p, vec2 g) {
  vec3 c = color(.3);
  c = mix(c, color(.4), merge(circle(p), circle(fract(p + .5))));
  c = mix(c, color(.6), merge(flower(p-.5, 4., 0.35), flowerDot(p-.5, 0.35)));
  c = mix(c, color(.2 + .15 * abs(cos(time + 0.05 * (g.x + g.y)))), edgeShape(p));
  return c;
}

void main() {
  vec2 g = (uv * 2. + vec2(0.01, -0.1) * time) * (1. + 0.01 * cos(time) + 0.01 * time);
  vec2 pos = fract(g);
  gl_FragColor = vec4(tile(pos, g), 1.0);

}`,
  },
});
