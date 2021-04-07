import { Shaders, Node, GLSL } from "gl-react";

export const n = 5;
export const title = "Here We Go Again";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

float opRepF(in float p, in float s) {
  return mod(p+s*0.5,s)-s*0.5;
}
vec2 opRep(in vec2 p, in float s) {
  return mod(p+s*0.5,s)-s*0.5;
}

mat2 rot (float a) {
  float c = cos(a);
  float s = sin(a);
  return mat2(c,s,-s,c);
}

float sphere (vec3 p, float r) {
  return length(p)-r;
}

float box (vec3 p, vec3 c) {
  return length(max(abs(p)-c,0.));
}

float smin( float a, float b, float k ) {
  float h = clamp( 0.5+0.5*(b-a)/k, 0.0, 1.0 );
  return mix( b, a, h ) - k*h*(1.0-h);
}

float smax(float a,float b, float k) {
    return -smin(-a,-b,k);
}

float shape (vec3 p) {
  float a = box(p, vec3(3.));
  float b = sphere(p, 3.8);
  return max(-b, a);
}

float shape2 (vec3 p) {
  vec3 w = vec3(p);
  p.xy = opRep(p.xy, 2.);
  p.z = opRepF(p.z, 2.);
  float s = sphere(p, 1.4 + 0.6 * cos(6. * time));
  s = max(s, box(w, vec3(3.)));
  return s;
}

float SDF(vec3 p) {
  float s = 99.;
  p.x -= 3.;
  p.zy *= rot(cos(time / 5.));
  p.zx *= rot(cos(time / 2.));
  p.yx *= rot(0.5 * sin(time / 3.));
  p.z = opRepF(p.z, 20.);
  p.z -= 3.;
  p.x = opRepF(p.x, 20.);
  p.x -= 3.;
  float s1 = min(s, shape(p));
  p.z += 7.;
  p.x += 7.;
  float s2 = min(s, shape2(p));
  return min(s1, s2);
}

void main() {
  vec3 p = vec3 (0., 0., -14.);
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
    pow(smoothstep(50., 0., p.z), 3.) *
    sqrt(vec3(1. - shad));
  gl_FragColor = vec4(c,1.0);
}`,
  },
});
