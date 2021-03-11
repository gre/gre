

import { Shaders, Node, GLSL } from "gl-react";

export const n = 7;
export const title = "Worms party";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

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

mat2 rot (float a) {
  float c = cos(a);
  float s = sin(a);
  return mat2(c,s,-s,c);
}

float sdSegment (in vec3 p, in float L, in float R) {
  p.y -= min(L, max(0.0, p.y));
  return length(p) - R;
}

vec2 id;

float SDF(vec3 p) {
  // The whole 3D objects are defined in this function
  p.z += 3. + cos(0.5 * time);
  p.yz *= rot(-1. + 0.1 * pow(.5 + .5*cos(0.9 * time), 4.) + 0.2 * cos(.08 * time));
  p.xz *= rot(.1 * time);
  vec4 h = getHex(p.xz);
  id = h.zw;
  p.x = h.x;
  p.z = h.y;
  p.x += 0.1 * sin(4.*(p.z + p.y + time - 0.03 * id.x));
  return sdSegment(p.xyz, 3., 0.25);
}

void main() {
  vec3 p = vec3 (0., 0., -10.);
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

  // Coloring
  vec3 c =
    pow(smoothstep(50., 0., p.z), 3.) *
    // it was pretty hard to get a nice palette.
    // not satisfied with current result..
    palette(
      shad,
      vec3(.5),
      vec3(.5),
      vec3(1., 0.2, 0.),
      vec3(
        0.6,
        0.5  + 0.5 * cos(time + 10. * length(id)),
        0.3 + 0.2 * cos(.7*time + 13. * length(id))
      )) *
    sqrt(vec3(1. - shad));
  gl_FragColor = vec4(c,1.0);
}`,
  },
});
