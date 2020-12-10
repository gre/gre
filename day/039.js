import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 39;
export const title = "metaballs";

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

// from HG_SDF
float fOpUnionSoft(float r, float a, float b) {
	float e = max(r - abs(a - b), 0.);
	return min(a, b) - e*e*0.25/r;
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
  float ground = p.y;
  p.y -= 1.5;
  float d = length(p) - .2;
  d = fOpUnionSoft(.5, d, length(p+.3 * vec3(sin(3. + .5 * time), sin(.6 * time), cos(.7 * time))) - .2);
  d = fOpUnionSoft(.5, d, length(p+.4 * vec3(cos(time), sin(time), cos(.6 * time))) - .2);
  d = fOpUnionSoft(.5, d, length(p+.5 * vec3(-sin(.9 * time), cos(1.1 * time), -sin(.8 * time))) - .2);
  d = fOpUnionSoft(.5, d, length(p+.6 * vec3(sin(1.+time), cos(-time), sin(.8 * time))) - .2);
  return vec2(min(ground, d), 0.);
}

void main() {
  vec3 origin = vec3(0., 4., -3.);
  vec3 clr = vec3(0.);
  // anti aliasing
  for (float x=-.5; x<=.5; x += 1.) {
    for (float y=-.5; y<=.5; y += 1.) {
      vec2 uvP = uv;
      uvP += vec2(x, y) / 800.;
      vec3 dir = normalize(vec3(uvP - .5, 1.));
      pR(dir.yz, -.8);
      vec3 p = origin;
      vec2 hit = marcher(p, dir);
      vec3 n = normal(p);
      vec3 c = vec3(0.2);
      vec3 lamp1 = vec3(-6., 8., -4.);
      c += vec3(1., .7, .5) * diffuse(p, n, lamp1)
      * softshadow(p, normalize(lamp1 - p), 0.02, 10., 3.);
      vec3 lamp2 = vec3(6., 8., -4.);
      c += vec3(.5, .6, .7) * diffuse(p, n, lamp2)
        * softshadow(p, normalize(lamp2 - p), 0.02, 10., 20.);
      clr += c;
    }
  }
  clr /= 4.;
  gl_FragColor = vec4(clr, 1.0);
}`,
  },
});
