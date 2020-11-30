import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";
import { GameOfLife } from "./GameOfLife";

export const n = 32;
export const title = "GoL valley";

const SIZE = 100;

let firstTime;
export const Shader = ({ time }) => {
  if (!firstTime) {
    firstTime = time;
  }
  const t = time - firstTime;
  const refreshEveryTicks = 100;
  const tick = Math.floor(t * 12);
  return (
    <Node
      shader={shaders.node}
      uniforms={{
        time,
        t: (
          <GameOfLife
            refreshEveryTicks={refreshEveryTicks}
            tick={tick}
            size={SIZE}
          />
        ),
      }}
    />
  );
};

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
uniform sampler2D t;

vec2 map (vec3 p);

#define PI ${Math.PI}
#define SIZE_F ${SIZE}.

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

// GRE's

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

float sdSegment(vec3 p, float s, float L) {
  p.y -= min(L, max(0.0, p.y));
  return length(p) - s;
}

vec2 map (vec3 p) {
  p.y -= .02 * pow(abs(.5 * SIZE_F - p.x + .3 * sin(.3 * p.z + time)), 2.);
  float s = p.y; // ground
  float x = pModInterval1(p.x, 1., 0., SIZE_F);
  p.z += .5 * mod(x, 2.);
  vec2 id = vec2(x, pModInterval1(p.z, 1., 0., SIZE_F));
  vec4 lk = texture2D(t, id / SIZE_F);
  s = min(s, sdSegment(p, .4, lk.r));
  return vec2(s, 1.);
}

vec3 normal (in vec3 p) {
	vec3 eps = vec3(0.001, 0.0, 0.0);
	return normalize(vec3(
		map(p+eps.xyy).x - map(p-eps.xyy).x,
		map(p+eps.yxy).x - map(p-eps.yxy).x,
		map(p+eps.yyx).x - map(p-eps.yyx).x
	));
}

float getDiff(vec3 p, vec3 n, vec3 lpos) {
  vec3 l = normalize(lpos-p);
  float dif = clamp(dot(n, l), 0.01, 1.);
  return dif;
}

vec2 marcher (inout vec3 p, vec3 dir) {
  vec2 t = vec2(999., 0.);
  for (int i=0; i<100; i++) {
    vec2 hit = map(p);
    p += dir * hit.x * .7;
    if (hit.x < 0.001) {
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
  float zoom = cos(.2 * time);
  vec3 origin = vec3(SIZE_F/2., 9. - 5. * zoom, .5 * zoom - 2.);
  vec3 dir = normalize(vec3(uv - .5, 1. + .5 * zoom));
  pR(dir.yz, -.3 + .1 * zoom);
  vec3 p = origin;
  vec2 hit = marcher(p, dir);
  vec3 n = normal(p);
  vec3 c = vec3(0.);
  c += color(hit.y) * vec3(.5, .2, .1) * getDiff(p, n, vec3(0., 5., -4.));
  c += color(hit.y) * vec3(.5, .5, .7) * getDiff(p, n, vec3(SIZE_F, 5., -4.));
  c += color(hit.y) * vec3(2., 1.6, 1.) * getDiff(p, n, vec3(.5 * SIZE_F, 5., .3 * SIZE_F));
  c += .5;
  float a = ambientOcclusion(p, n, 1.5, 0.8);
  c *= a;
  c = mix(c, vec3(.9) + .1 * a, pow(clamp(.025 * length(origin - p), 0., 1.), 1.4));
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
