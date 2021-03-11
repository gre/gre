

import { Shaders, Node, GLSL } from "gl-react";

export const n = 10;
export const title = "ring";

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
mat2 rot (float a) {
  float c = cos(a);
  float s = sin(a);
  return mat2(c,s,-s,c);
}

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
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

float pModPolar(inout vec2 p, float repetitions) {
	float angle = 2.*PI/repetitions;
	float a = atan(p.y, p.x) + angle/2.;
	float r = length(p);
	float c = floor(a/angle);
	a = mod(a,angle) - angle/2.;
	p = vec2(cos(a), sin(a)) * r;
	// For an odd number of repetitions, fix cell index of the cell in -x direction
	// (cell index would be e.g. -5 and 5 in the two halves of the cell):
	if (abs(c) >= (repetitions/2.)) c = abs(c);
	return c;
}

float pMod1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize, size) - halfsize;
	return c;
}

float REP;

float SDF(vec3 p, float nb) {
  p.z -= 5. + 2. * cos(3. * time);

  pR(p.yz, abs(0.6 * cos(0.2 * time)));

  pR(p.xy, 0.7 * time);
  
  float middle = sdSphere(p, 1.5);
  float m = pModPolar(p.xy, 3. + mod(floor(1337.9 * floor(nb)), 8.));
  p.x -= time;

  REP = pMod1(p.x, 4.);
  p.y += 20.;
  float s = sdSegment(p, 40., 0.3);
  p.y -= 20.;
  float bounce = cos(REP+ 2. * m + 12. * time);
  s = opSmoothUnion(s, sdSphere(p - vec3(.0, .0, .3 * bounce), (1. + 0.3 * bounce) * smoothstep(.5, .4, sin(0.2 * time))), 0.3);

  p.x -= 2.;

  pR(p.yz, 10. * time);

  s = min(s, sdBox(p, vec3(0.2)));

  return opSmoothUnion(middle, s, 0.3);
}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float d, float shad) {
  vec3 c =
  palette(fract(0.5 + 0.01 * time * REP),
    vec3(.6),
    vec3(.6),
    vec3(1.),
    vec3(0.6, 0.4, 0.2))
  * sqrt(1. - shad);
  return c;
}

void main() {
  vec3 p = vec3 (0., 0., -4.);
  vec3 dir = normalize(vec3((uv - 0.5) * 2.,1.));
  float shad = 1.;
  float nb = 0.2 * time;
  for (int i=0; i<60; i++) {
    float d = SDF(p, nb);
    if (d<0.001) {
      shad = float(i)/60.;
      break;
    }
    p += d * dir * 0.5;
  }
  gl_FragColor = vec4(
    smoothstep(0.02, 0.04, abs(fract(nb+0.02))) *
    color(length(p.xy), shad),
    1.0);
}
`,
  },
});
