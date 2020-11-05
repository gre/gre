import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 98;
export const title = "";

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

mat2 rot (float a) {
  float c = cos(a);
  float s = sin(a);
  return mat2(c,s,-s,c);
}

float sdSegment (in vec3 p, in float L, in float R) {
  p.y -= min(L, max(0.0, p.y));
  return length(p) - R;
}

float SDF(vec3 p) {
  // The whole 3D objects are defined in this function
  p.z += 4.;
  p.yz *= rot(-1. + 0.1 * pow(.5 + .5*cos(time), 4.) + 0.2 * cos(.08 * time));
  p.xz *= rot(.1 * time);
  p = abs(p);
  float s = sdSegment(p.xyz, 2., 1.);
  s = min(s, sdSegment(p.xzy, 2., 1.));
  s = min(s, sdSegment(p.yxz, 2., 1.));
  return s;
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
    vec3(1. - shad * smoothstep(0.3, 0.4, shad));
  gl_FragColor = vec4(c,1.0);
}
`,
  },
});
