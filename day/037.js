import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 37;
export const title = "sdBrick";

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

#define PI ${Math.PI}

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

float vmax(vec2 v) {
	return max(v.x, v.y);
}

float vmax(vec3 v) {
	return max(max(v.x, v.y), v.z);
}

float vmax(vec4 v) {
	return max(max(v.x, v.y), max(v.z, v.w));
}

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

float fBox(vec3 p, vec3 b) {
	vec3 d = abs(p) - b;
	return length(max(d, vec3(0))) + vmax(min(d, vec3(0)));
}

float fCylinder(vec3 p, float r, float height) {
	float d = length(p.xz) - r;
	d = max(d, abs(p.y) - height);
	return d;
}

// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
vec3 getColor (float t) {
  return palette(
    fract(floor(t * 16.)/16.),
    vec3(.5),
    vec3(.5),
    vec3(1.),
    vec3(.1, .4, .7)
  );
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

vec2 opU (vec2 a, vec2 b) {
  if (a.x < b.x) return a;
  return b;
}

float sdBrick(vec3 p, float w, float h) {
  p.y -= .5;
  float s = fBox(p, vec3(w / 2., .5, h / 2.));
  p.x += w / 2. + .5;
  p.z += h / 2. + .5;
  p.y -= .6;
  pModInterval1(p.x, 1., 1., w);
  pModInterval1(p.z, 1., 1., h);
  s = min(s, fCylinder(p, .2, .1));
  return s;
}

vec3 move (vec3 p, float z, float y) {
  float r = step(mod(y, 2.), 0.);
  pR(p.xz, r * PI/2.);
  p.z += z;
  p.y -= y;
  return p;
}

vec2 map (vec3 p) {
  vec2 s = vec2(min(p.y, 40. - p.z), 1.); // ground
  pR(p.xz, PI/4.);
  float speed = .5;
  float P = 50. * speed;
  float phase = floor(time / P);
  for (float y=0.; y<=5.; y+=1.) {
    for (float z=-2.; z<=2.; z+=1.) {
      float id = 2. + z + 5. * y;
      float tId = id * speed;
      float t2 = mod(time, 50. * speed);
      float t = mod(t2>25.*speed ? 50.-t2 : t2, 25. * speed);
      if (t < tId) continue;
      float animating = step(t, tId + speed);
      float m = fract(t / speed) * animating;
      vec3 q = move(p, z, y);
      q.y -= pow((animating - m) * 3., 2.);
      s = opU(s, vec2(sdBrick(q, 5., 1.), 2. + (1. + phase) * id));
    }
  }
  return s;
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
  for (int i=0; i<70; i++) {
    vec2 hit = map(p);
    p += dir * hit.x;
    if (hit.x < 0.001) {
      t = hit;
      break;
    }
  }
  return t;
}


vec3 color (float material) {
  if (material == 0.) return vec3(0.);
  if (material == 1.) return vec3(1.);
  return getColor(.03 * material);
}

void main() {
  vec3 origin = vec3(0., 8., -8.);
  vec3 dir = normalize(vec3(uv - .5, 1.));
  pR(dir.yz, -.6);
  vec3 p = origin;
  vec2 hit = marcher(p, dir);
  vec3 n = normal(p);
  vec3 c = .2 * color(hit.y);
  c += color(hit.y) * vec3(.7, .7, 1.) * diffuse(p, n, vec3(-5., 10., -6.));
  c += color(hit.y) * vec3(1., .7, .7) * diffuse(p, n, vec3(10., 10., -2.));
  c += color(hit.y) * vec3(.5) * diffuse(p, n, vec3(0., 10., -50.));
  c *= ambientOcclusion(p, n, 2., 1.5);
  c += smoothstep(5., 20., p.z);
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
