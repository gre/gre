import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 11;
export const title = "magic mouse";

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
float pMod1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize, size) - halfsize;
	return c;
}
float pModPolar(inout vec2 p, float repetitions) {
	float angle = 2.*PI/repetitions;
	float a = atan(p.y, p.x) + angle/2.;
	float r = length(p);
	float c = floor(a/angle);
	a = mod(a,angle) - angle/2.;
	p = vec2(cos(a), sin(a)) * r;
	if (abs(c) >= (repetitions/2.)) c = abs(c);
	return c;
}

float opSmoothUnion( float d1, float d2, float k ) {
    float h = clamp( 0.5 + 0.5*(d2-d1)/k, 0.0, 1.0 );
    return mix( d2, d1, h ) - k*h*(1.0-h); }

float sdSphere( vec3 p, float d ) {
  return length(p) - d;
}

float opMouse (vec3 p) {
  return opSmoothUnion(
    sdSphere(p, 1.),
    min(
      sdSphere(p + vec3(-.8, 0., .8), 0.5),
      sdSphere(p + vec3(.8, 0., .8), 0.5)
    ),
    0.1
  );
}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

float REP;
float POL;
float BIG;

float SDF(vec3 p) {
  vec3 bp = p;
  pR(bp.yz, 0.9);
  pR(bp.xy, 2. * time);
  float bigMouse = opMouse(bp);
  BIG = step(bigMouse, 0.001);
  p.x -= 20. + 3. * cos(time) + 0.2 * time;
  p.y -= 4.;
  p.z -= 7. + 0.1 * time;
  pR(p.yz, 0.6 + abs(0.4 * cos(0.15 * time)));
  pR(p.xy, 0.8 * time);
  float n = 10. + time;
  POL = pModPolar(p.xy, n);
  p.x -= time;
  REP = pMod1(p.x, 10. - 4. * smoothstep(0., 30., time));
  p.z += abs(2. * cos(
    time *
    (2. + 0.2 * REP + POL/n)
  ));
  return opSmoothUnion(
    opMouse(p),
    bigMouse,
    3.
  );
}

vec3 color (float d, float shad) {
  vec3 c =
  mix(
  palette((0.2) * POL,
    vec3(1.),
    vec3(0.5),
    vec3(1., 1., 1. - 0.7 * REP),
    vec3(0.9, 0.2, 0.6)),
    vec3(1.2),
    BIG
  )
  * sqrt(1. - shad);
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
    color(length(p.xy), shad),
    1.0);
}
`,
  },
});
