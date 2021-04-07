

import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 31;
export const title = "alien tower";

const numberInCirculation = 18557031;

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time, numberInCirculation }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
uniform float numberInCirculation;

vec2 map (vec3 p);

#define PI ${Math.PI}

// FROM https://www.shadertoy.com/view/4sdGWN

#define HASHSCALE1 .1031

float hash(float p) {
	vec3 p3  = fract(vec3(p) * HASHSCALE1);
  p3 += dot(p3, p3.yzx + 19.19);
  return fract((p3.x + p3.y) * p3.z);
}
vec3 randomSphereDir(vec2 rnd) {
	float s = rnd.x*PI*2.;
	float t = rnd.y*2.-1.;
	return vec3(sin(s), cos(s), t) / sqrt(1.0 + t * t);
}
vec3 randomHemisphereDir(vec3 dir, float i) {
	vec3 v = randomSphereDir( vec2(hash(i+1.), hash(i+2.)) );
	return v * sign(dot(v, dir));
}

float ambientOcclusion( in vec3 p, in vec3 n, in float maxDist, in float falloff ) {
  const int nbIte = 12;
  const float nbIteInv = 1./float(nbIte);
  const float rad = 1.-1.*nbIteInv;
  float ao = 0.0;
  for( int i=0; i<nbIte; i++ ) {
    float l = hash(float(i))*maxDist;
    vec3 rd = normalize(n+randomHemisphereDir(n, l )*rad)*l;
    ao += (l - max(map( p + rd ).x, 0.)) / maxDist * falloff;
  }
  return clamp( 1.-ao*nbIteInv, 0., 1.);
}

// https://mercury.sexy/hg_sdf/

float pModInterval1(inout float p, float size, float start, float stop) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p+halfsize, size) - halfsize;
	if (c > stop) {
		p += size*(c - stop);
		c = stop;
	}
	if (c <start) {
		p += size*(c - start);
		c = start;
	}
	return c;
}

// GRE's

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

float sdReuleaux(vec3 p, float edge, float L) {
  p.y -= min(L, max(0.0, p.y));
  p.z -= edge / 6.;
  float h = edge * sqrt(3.) / 2.;
  float a = length(p - vec3(0., 0., h / 2.)) - edge;
  float b = length(p + vec3(.5 * edge, 0., h / 2.)) - edge;
  float c = length(p + vec3(-.5 * edge, 0., h / 2.)) - edge;
  return max(max(max(a, b), c), abs(p.y));
}

float sdSegment(vec3 p, float s, float L) {
  p.y -= min(L, max(0.0, p.y));
  return max(length(p)-s, abs(p.y));
}

vec2 map (vec3 p) {
  float s = p.y; // ground
  for (float f=0.; f<20.; f+=1.) {
    float t = .4 * time - f * pow(1. + .6 * time, .5) + f * pow(1. + .8 * time - sin(time), .2);
    p.xz -= .075 * vec2(cos(-t), sin(-t));
    pR(p.xz, t / 3. + PI / 2.);
    float cut = .55 * smoothstep(.8, 1., sin(time - (.03 + 0.0002 * time) * f))-length(p.xz);
    s = min(s, max(sdReuleaux(p, 1., .1), cut));
    p.y -= .11 + 0.0005 * time;
  }
  return vec2(s, 1.);
}

vec3 normal (in vec3 p) {
	vec3 eps = vec3(0.001, 0.0, 0.0);
	return normalize(vec3(
		map(p+eps.xyy).x-map(p-eps.xyy).x,
		map(p+eps.yxy).x-map(p-eps.yxy).x,
		map(p+eps.yyx).x-map(p-eps.yyx).x
	));
}

float getDiff(vec3 p, vec3 n, vec3 lpos) {
  vec3 l = normalize(lpos-p);
  float dif = clamp(dot(n, l), 0.01, 1.);
  return dif;
}

vec2 marcher (inout vec3 p, vec3 dir) {
  float glitch = 0.0001 * time; // NB intentional glitch over time of the raymarcher
  vec2 t = vec2(999., 0.);
  for (int i=0; i<70; i++) {
    vec2 hit = map(p);
    p += dir * hit.x;
    if (hit.x < 0.01 + glitch) {
      t = hit;
      break;
    }
  }
  return t;
}

vec3 color (float material) {
  if (material == 0.) return vec3(0.);
  return vec3(1.);
}

void main() {
  float zoom = sin(.2 * time);
  vec3 origin = vec3(0., 4., -3.2 + .4 * zoom);
  vec3 dir = normalize(vec3(uv - .5, 1.));
  pR(dir.yz, -.6 + .05 * zoom);
  vec3 p = origin;
  vec2 hit = marcher(p, dir);
  vec3 n = normal(p);
  vec3 c = vec3(0.5);
  c += color(hit.y) * vec3(.6, .4, .3) * getDiff(p, n, vec3(-2., 5., -4.));
  c += color(hit.y) * vec3(.2, .4, .5) * getDiff(p, n, vec3(2., 5., -4.));
  c *= ambientOcclusion(p, n, 2., 1.);
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
