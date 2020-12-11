import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 40;
export const title = "pen-o-plasma";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

vec2 map (vec3 p);

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec2 opU (vec2 a, vec2 b) {
  if (a.x < b.x) return a;
  return b;
}

#define PI ${Math.PI}

// HG_SDF
float pMod1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize, size) - halfsize;
	return c;
}

float vmax(vec2 v) {
	return max(v.x, v.y);
}

float vmax(vec3 v) {
	return max(max(v.x, v.y), v.z);
}

float vmax(vec4 v) {
	return max(max(v.x, v.y), max(v.z, v.w));
}

float vmin(vec2 v) {
	return min(v.x, v.y);
}

float vmin(vec3 v) {
	return min(min(v.x, v.y), v.z);
}

float vmin(vec4 v) {
	return min(min(v.x, v.y), min(v.z, v.w));
}

float fBox(vec3 p, vec3 b) {
	vec3 d = abs(p) - b;
	return length(max(d, vec3(0))) + vmax(min(d, vec3(0)));
}

float fOpUnionSoft(float r, float a, float b) {
	float e = max(r - abs(a - b), 0.);
	return min(a, b) - e*e*0.25/r;
}

vec3 normal (in vec3 p) {
	vec3 eps = vec3(0.001, 0.0, 0.0);
	return normalize(vec3(
		map(p+eps.xyy).x-map(p-eps.xyy).x,
		map(p+eps.yxy).x-map(p-eps.yxy).x,
		map(p+eps.yyx).x-map(p-eps.yyx).x
	));
}

float diffuse(vec3 p, vec3 n, vec3 lpos) {
  vec3 l = normalize(lpos-p);
  float dif = clamp(dot(n, l), 0.01, 1.);
  return dif;
}

vec2 marcher (inout vec3 p, vec3 dir) {
  vec2 t = vec2(999., 0.);
  for (int i=0; i<80; i++) {
    vec2 hit = map(p);
    p += dir * hit.x;
    if (hit.x < 0.001) {
      t = hit;
      break;
    }
  }
  return t;
}

vec2 map (vec3 p) {
  float t = .3 * time;
  vec2 s = vec2(p.y, 0.); // ground
  p.y -= 2.;
  float d = length(p) - .2;
  d = fOpUnionSoft(.5, d, length(p+.8 * vec3(cos(t), sin(.9 * t), 0.)) - .1);
  d = fOpUnionSoft(.5, d, length(p+.8 * vec3(cos(.8 * t), 0., -sin(t))) - .1);
  d = fOpUnionSoft(.5, d, length(p+.8 * vec3(0., -cos(t), sin(.7 * t))) - .1);
  pR(p.xy, t);
  pR(p.xz, t);
  d = fOpUnionSoft(.3, d, fBox(p, vec3(.3)));
  s = opU(s, vec2(d, 1.));
  return s;
}

vec3 color (float t, float m) {
  vec2 d = uv * 50.;
  vec2 id = floor(d);
  vec2 c = d - id;
  float l = .1 * mix(1.5, .9, sqrt(t));
  float s1 = smoothstep(.8 * l, l, abs(c.x-c.y));
  float s2 = smoothstep(.8 * l, l, abs(c.x-1.+c.y));
  float mul = 1. + step(t, .5);
  l *= mul;
  float s3 = smoothstep(.8 * l, l, length(fract(c * mul) - .5));
  float s = 1.;
  if (t < .4) s=min(s, s1);
  if (t < .6) s=min(s, s2);
  if (t < .2 || t > .6) s = min(s, s3);
  if (mod(t, .2)>.1 && mod(id.x+id.y, 2.)<1.) s=1.;
  return vec3(s);
}

// https://www.iquilezles.org/www/articles/rmshadows/rmshadows.htm
float softshadow( in vec3 ro, in vec3 rd, in float mint, in float tmax, in float k) {
	float res = 1.0;
  float t = mint;
  float ph = 1e10; // big, such that y = 0 on the first iteration
  for( int i=0; i<32; i++ ) {
		float h = map( ro + rd*t ).x;
    float y = h*h/(2.0*ph);
    float d = sqrt(h*h-y*y);
    res = min( res, k*d/max(0.0,t-y) );
    ph = h;
    t += h;
    if( res<0.0001 || t>tmax ) break;
  }
  return clamp( res, 0.0, 1.0 );
}

void main() {
  vec3 origin = vec3(0., 4., -2.);
  vec3 clr = vec3(0.);
  vec2 uvP = uv;
  vec3 dir = normalize(vec3(uvP - .5, 1.));
  pR(dir.yz, -.8);
  vec3 p = origin;
  vec2 hit = marcher(p, dir);
  vec3 n = normal(p);
  vec3 c = vec3(0.);
  float d = 2. + 1.6 * cos(.2 * time);
  vec3 lamp1 = d * vec3(0., 10., 2.);
  float v = 1.;
  c +=
    vec3(v, .0, .0)
    * diffuse(p, n, lamp1)
    * softshadow(p, normalize(lamp1 - p), 2., 10., 8.);

  vec3 lamp2 = d * vec3(-4., 5., -3.);
  c +=
    vec3(.0, .0, v)
    * diffuse(p, n, lamp2)
    * softshadow(p, normalize(lamp2 - p), 2., 10., 8.);

  vec3 lamp3 = d * vec3(4., 5., -3.);
  c +=
    vec3(.0, v, .0)
    * diffuse(p, n, lamp3)
    * softshadow(p, normalize(lamp3 - p), 2., 10., 8.);

  c = mix(c, vec3(1.), color((c.r+c.g+c.b)/3., hit.y));

  clr += c;

  gl_FragColor = vec4(clr, 1.0);
}`,
  },
});
