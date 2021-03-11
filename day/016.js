

import { Shaders, Node, GLSL } from "gl-react";

export const n = 16;
export const title = "reuleaux";

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

mat2 rot (float a) {
  float c = cos(a);
  float s = sin(a);
  return mat2(c,s,-s,c);
}

float sphere (vec3 p, float r) {
  return length(p)-r;
}

float sdReuleaux(vec3 p, float edge) {
  p.y -= edge / 6.;
  p.z -= edge / 6.;
  float h = edge * sqrt(3.) / 2.;
  float a = sphere(p - vec3(0., h / 2., 0.), edge);
  float b = sphere(p + vec3(.5 * edge, h / 2., 0.), edge);
  float c = sphere(p + vec3(-.5 * edge, h / 2., 0.), edge);
  float d = sphere(p + vec3(0., 0., h), edge);
  return max(max(max(a, b), c), d);
}

float shape(vec3 p, float edge, float diverge) {
  float h = edge * sqrt(3.) / 2.;
  float a = sdReuleaux(p + diverge * vec3(0., -h / 2.-edge / 6., -edge / 6.), edge);
  float b = sdReuleaux(p + diverge * vec3(.5 * edge, h / 2. - edge / 6., -edge / 6.), edge);
  float c = sdReuleaux(p + diverge * vec3(-.5 * edge, h / 2.- edge / 6., -edge / 6.), edge);
  float d = sdReuleaux(p + diverge * vec3(0., 0.-edge / 6., h-edge / 6.), edge);
  return min(min(min(a, b), c), d);
}

float SDF(vec3 p) {
  p.yz *= rot(PI/3.);
  p.y += 3.;
  float m = mod(time, 2.2);
  float t1 = min(m, 1.);
  float t2 = max(0., min(m - 1., 1.));
  t1 *= smoothstep(2.1, 1.9, m);
  t2 *= smoothstep(2.1, 1.9, m);
  p.yz *= rot(-PI/3. * t2);
  return shape(p, 3., 2. * (t1 - t2));
}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float t) {
  return palette(
    t,
    vec3(.8),
    vec3(.8),
    vec3(1., .8, .9),
    vec3(.1, .3, .7)
  );
}

void main() {
  vec3 p = vec3(0., 0., -10.);
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
  vec3 c =
    pow(smoothstep(10., -3., p.z), 4.) *
    sqrt(1. - shad) *
    color(0.5 * time);
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
