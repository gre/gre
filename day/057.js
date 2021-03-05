import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 57;
export const title = "Virus v2";

export const gifSize = 400;
export const gifStart = 0;
export const gifEnd = 30;
export const gifFramePerSecond = 24;
export const gifSpeed = 0.8;

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

float t = 2. * PI * time / 30.0;

mat2 rot (float a) {
  float c = cos(a);
  float s = sin(a);
  return mat2(c,s,-s,c);
}

float opSmoothUnion( float d1, float d2, float k ) {
    float h = clamp( 0.5 + 0.5*(d2-d1)/k, 0.0, 1.0 );
    return mix( d2, d1, h ) - k*h*(1.0-h); }

float sdSegment (in vec3 p, in float L, in float R) {
  p.y -= min(L, max(0.0, p.y));
  return length(p) - R;
}
float sdBox( vec3 p, vec3 b ) {
  vec3 q = abs(p) - b;
  return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}
float sdSphere( vec3 p, float d ) {
  return length(p) - d;
}

float sdParticle( vec3 p, float d, float dist ) {
  return opSmoothUnion(
    length(p) - d,
    sdSegment(p.yxz+vec3(0.,dist,0.), dist, 0.2 * d),
    0.2
  );
}

// badly failing at making a "good" radial repeat
vec3 opRepeatPolar (vec3 p, float n, float R, float offsetA) {
  float SCALE = n/(2. * PI);
  vec2 pos2d = p.xz;
  float r = length(pos2d) / R;
  pos2d = vec2(log(r), offsetA + atan(p.z, p.x)) * SCALE;
  pos2d.y = fract(pos2d.y) - 0.5;
  return vec3(pos2d, SCALE * p.y / r);
}

vec3 opRep( in vec3 p, in vec3 c ) {
  vec3 q = mod(p+0.5*c,c)-0.5*c;
  return q;
}

float sdVirus (vec3 p, vec3 gp) {
  p.yz *= rot(-1.4 + 0.1 * cos(t));
  p.xz *= rot(0.01 * gp.z + 0.3 * cos(1. + 2. * t + 0.1 * gp.z));
  p.xy *= rot(.02 * gp.z + 0.3 * sin(3. * t));
  float s = sdSphere(p, 2.);
  p.y = abs(p.y);
  s = opSmoothUnion(s, sdParticle(
    opRepeatPolar(p, 18., 2.2, 0.),
    0.2,
    0.4
  ), 0.2);
  s = opSmoothUnion(s, sdParticle(
    opRepeatPolar(p - vec3(0.05 * cos(3. * t), 1., 0.05 * sin(4.*t)), 14., 1.9, 0.),
    0.15,
    0.4
  ), 0.3);
  s = opSmoothUnion(s, sdParticle(
    opRepeatPolar(p - vec3(0.+ 0.05 * cos(5.*t), 1.6,0.05 * sin(6.*t)), 10., 1.5, 0.),
    0.15,
    0.4
  ), 0.3);
  s = opSmoothUnion(s, sdParticle(
    opRepeatPolar(p - vec3(0., 2.1, 0.), 6., 0.6, .2),
    0.18,
    0.0
  ), 0.2);
  return s;
}

float SDF(vec3 p) {
  vec3 gp = p;
  float SCALE = 6./PI;
  float s = sdVirus(p, gp);
  p -= vec3(4., 4., 4.);
  p.xy *= rot(p.z * .07);
  p.yz *= rot(p.z * .05);
  p.x += 0.7 * time;
  p.y += 0.8 * time;
  p.z -= 2.0 * time;
  p += 50.;
  p = opRep(p, vec3(7.4, 5.4, 6.));
  p *= 2.0;
  p.z *= 10.0 * smoothstep(10., 0., abs(mod(time, 30.0)-15.));
  p.z += 10.0 * abs(mod(time, 1.0)) * smoothstep(0., 10., abs(mod(time, 30.0)-15.));
  s = opSmoothUnion(s, sdVirus(p, gp), 1.0);
  return s;
}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float z, float shad) {
  float p = sqrt(1. - shad);
  vec3 c =
  palette(p,
    vec3(.5),
    vec3(.7),
    vec3(0.15, 0.5, 0.35 + 0.05 * cos(8. * t)),
    vec3(1.0, 0.4, 0.5))
  * p
  * smoothstep(30., 5., z);
  return c;
}

void main() {
  vec3 p = vec3 (0., 0., -4.);
  vec3 dir = normalize(vec3((uv - 0.5) * 2.,1.));
  float shad = 1.;
  for (int i=0; i<90; i++) {
    float d = SDF(p);
    if (d<0.001) {
      shad = float(i)/90.;
      break;
    }
    p += d * dir * 0.5;
  }
  float edge = min(min(uv.x, 1.-uv.x), min(uv.y, 1.-uv.y));
  gl_FragColor = vec4(
    mix(
      color(p.z, shad),
      color(0., 0.5),
      step(edge, 0.02)
    )
    , 1.0);
}
`,
  },
});
