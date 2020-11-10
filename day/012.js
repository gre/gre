import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 12;
export const title = "floor is lava";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL/* glsl */ `
precision highp float;
varying vec2 uv;
uniform float time;

#define PI ${Math.PI}

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

float SDF(vec3 p) {
  pR(p.yz, 2.5 + 0.3 * abs(0.8 + cos(time)));
  pR(p.xz, 2.);
  float s = 4. * cos(0.2 * p.x + 2. * time) * sin(0.1 * p.z + 1.3 * time);
  pR(p.xz, 1.);
  s += 1.6 * cos(0.2 * p.x + 5. * time) * cos(0.5 * p.z + time);
  pR(p.xz, 0.1 * time);
  s += 0.6 * cos(0.7 * p.x + 1. * time) * sin(0.9 * p.z);
  pR(p.xz, time);
  s += 0.4 * length(p);
  s += -p.y + 3.;
  return s;
}

vec3 color (float d, float shad) {
  vec3 c = palette(
    shad,
    vec3(.5),
    vec3(.5),
    vec3(0.0, 0.8, 0.9),
    vec3(0.1, 0.9, 0.4 * fract(3. * time))
  ) * sqrt(1. - shad);
  return c;
}

void main() {
  vec3 p = vec3 (0., 0., -4.);
  vec3 dir = normalize(vec3((uv - 0.5) * 2.,1.));
  float shad = 1.;
  for (int i=0; i<60; i++) {
    float d = SDF(p);
    if (d<0.001) {
      shad = float(i)/60.;
      break;
    }
    p += d * dir * 0.5;
  }
  gl_FragColor = vec4(
    color(p.z, shad),
    1.0);
}
`,
  },
});
