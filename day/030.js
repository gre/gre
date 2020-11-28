import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 30;
export const title = "21 millions";

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

#define SIZE 275.

float sdBox (vec2 p, vec2 sz) {
  return max(abs(p.x) - sz.x, abs(p.y) - sz.y);
}
float sdD (vec2 p, float w, float h) {
  return min(sdBox(p, vec2(w, h)), length(p-vec2(w, .0))-h);
}
float sdUpperD (vec2 p) {
  p.x += .02;
  p.y -= .1;
  float inner = sdD(p + vec2(-0.025, 0.012), 0.037, 0.055);
  float outer = sdD(p, 0.1, 0.1);
  return max(-inner, outer);
}
float sdLowerD (vec2 p) {
  p.x += .01;
  p.y += .085;
  float outer = sdD(p, 0.11, 0.11);
  float inner = sdD(p - vec2(0.023, 0.01), 0.045, 0.058);
  return max(-inner, outer);
}
float sdRevCornerRadius(vec2 p) {
  return max(
    sdBox(p, vec2(.5)),
    -min(
      (p.x - p.y) / 2.,
      length(p + vec2(.5, -.5)) - 1.
    )
  );
}
float sdBitcoin2D (vec2 p) {
  float bottom = sdLowerD(p);
  bottom = min(bottom, max(
    sdBox(p + vec2(.15, .165), vec2(.04, .03)), // bottom-left shape
    -(p.x - .216 * p.y + 0.142)) // 12.5° cut
  );
  bottom = min(bottom, sdRevCornerRadius((p + vec2(0.135, -0.135)) * vec2(1., -1.) * 30.));
  float top = sdUpperD(p);
  top = min(top, sdBox(p - vec2(-.15, .175), vec2(.034, .025)));
  top = min(top, sdRevCornerRadius((p + vec2(0.135, 0.12)) * vec2(1., 1.) * 30.));
  p.x += .01;
  float hash = max(
    sdBox(p, vec2(0.07, .285)),
    -min(
      sdBox(p, vec2(0.022, 1.)),
      sdBox(p, vec2(1., .15))
    )
  );
  return min(min(top, bottom), hash);
}

float sdBitcoin (in vec3 p, in float L, in float sz) {
  p.y += 0.05;
  p.y -= min(L, max(0.0, p.y));
  float plane = abs(p.y);
  return max(sdBitcoin2D(p.xz / sz), plane);
}

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

const vec3 rep = vec3(.65, .3, .5);

float unzooming = (.5 + .5 * cos(2.8 + .2 * time)) * smoothstep(5., 10., time);

vec2 map (vec3 p) {
  vec3 id = vec3(
    pModInterval1(p.x, rep.x, 0., SIZE),
    pModInterval1(p.y, rep.y, -SIZE, 0.),
    pModInterval1(p.z, rep.z, 0., SIZE)
  );
  float a = numberInCirculation/SIZE;
  float b = a/SIZE;
  float circ = step(SIZE + id.y - b, 0.);
  float d = sdBitcoin2D(.8 * (id.zx / SIZE - .5));
  float sz = 1. - 0.5 * step(d, 0.) - 0.3 * abs(cos(3. * time + d * 20.)) - .23* unzooming;
  p.y += 0.05 * cos(.8 * id.x + .7 *  time) * sin(.7 * id.z + .5 * time);
  p.x += 0.05 * cos(8. * id.y + .5 * time) * sin(4.7 * id.z + .3 * time);
  p.z += 0.05 * cos(7. * id.x + .6 * time) * sin(8. * id.y + .4 * time);
  return vec2(
    sdBitcoin(p.zyx, .1, sz)
    , 1. + circ);
}

vec3 color (float material) {
  if (material == 0.) return vec3(0.);
  return mix(
    vec3(1.),
    vec3(246./255., 145./255., 29./255.),
    step(material, 1.5)
  );
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

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

mat3 lookAt (vec3 ro, vec3 ta) {
  float cr = 0.;
  vec3 ww = normalize( ta - ro );
  vec3 uu = normalize( cross(ww,vec3(sin(cr),cos(cr),0.0) ) );
  vec3 vv =          ( cross(uu,ww));
  return mat3(uu,vv,ww);
}

void main() {
  vec3 origin = vec3(-.1, 1., .1);
  origin += vec3(-100. - time, 200. - 2. * time, time) * pow(unzooming, 2.);
  vec3 dir = normalize(vec3(uv - .5, 1.));
  vec3 poi = rep * vec3(SIZE, -SIZE, SIZE) * smoothstep(.0, .5, unzooming);
  dir = lookAt(origin, poi) * dir;
  float material = 0.;
  vec3 p = origin;
  for (int i=0; i<400; i++) {
    vec2 hit = map(p);
    p += dir * hit.x * mix(.5, .9, unzooming);
    if (hit.x < 0.001) {
      material = hit.y;
      break;
    }
  }
  vec3 n = normal(p);
  vec3 lpos = vec3(0., 5., -4.);
  vec3 c = color(material) * getDiff(p, n, lpos);
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
