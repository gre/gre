

import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

const GIF = 0;
export const n = 55;
export const title = "cube d'or";

export const exportSize = 400;
export const exportStart = 0;
export const exportEnd = 20;
export const exportFramePerSecond = 24;
export const exportSpeed = 1;

export const Shader = ({ time }) => (
  <Node
    shader={shaders.node}
    uniforms={{
      time,
    }}
  />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`#version 300 es
precision highp float;
in vec2 uv;
out vec4 color;
uniform float time;

#define PI ${Math.PI}

// camera origin
vec3 origin;

#define HIT vec2
HIT map (vec3 p);
vec3 shade (HIT m, vec3 p);
vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir);

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

// from HG_SDF
float vmax(vec3 v) {
	return max(max(v.x, v.y), v.z);
}
float fBoxCheap(vec3 p, vec3 b) {
	return vmax(abs(p) - b);
}

mat3 lookAt (vec3 ro, vec3 ta) {
  float cr = 0.;
  vec3 ww = normalize( ta - ro );
  vec3 uu = normalize( cross(ww,vec3(sin(cr),cos(cr),0.0) ) );
  vec3 vv =          ( cross(uu,ww));
  return mat3(uu,vv,ww);
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
	vec3 eps = vec3(0.0005, 0.0, 0.0);
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

HIT marcher (inout vec3 p, vec3 dir) {
  HIT hit = HIT(0.);
  float t = 0.;
  for (int i=0; i<100; i++) {
    HIT h = map(p + t * dir);
    t += h.x;
    if (abs(h.x) < .0005) {
      hit = h;
      break;
    }
  }
  p += t * dir;
  return hit;
}

HIT opU (HIT a, HIT b) {
  if (a.x < b.x) return a;
  return b;
}

float specularStrength (float m) {
  return step(.5, m);
}

float specular (vec3 n, float m, vec3 ldir, vec3 dir, float p) {
  return specularStrength(m) * pow(max(dot(dir, reflect(ldir, n)), 0.0), p);
}

vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir) {
  vec3 l, ldir;
  vec3 c = vec3(0.);
  l = vec3(-3., 2., 2.);
  ldir = normalize(l - p);
  c +=
    vec3(.9, .6, .3) * (
      // ambient
      0.2
      // diffuse
      + shade(hit, p)
        * (.5 + .5 * diffuse(p, n, l)) // half lambert
      + specular(n, hit.y, ldir, dir, 20.)
    );
  l = vec3(3., 1., 3.);
  ldir = normalize(l - p);
  c +=
    vec3(.3, .5, .9) * (
    // ambient
    0.1
    // diffuse
    + shade(hit, p)
      * (.5 + .5 * diffuse(p, n, l)) // half lambert
    + specular(n, hit.y, ldir, dir, 40.)
  );
  l = vec3(4., 3., -4.);
  ldir = normalize(l - p);
  c +=
    vec3(.4) * (
    // diffuse
    + shade(hit, p)
      * (.5 + .5 * diffuse(p, n, l)) // half lambert
      * softshadow(p, ldir, 0.02, 4., 18.)
    + specular(n, hit.y, ldir, dir, 60.)
  );
  return c;
}

vec3 shade (HIT hit, vec3 _p) {
  return mix(
    vec3(.9, .65, .0),
    vec3(.8),
    step(hit.y, .5)
  );
}

HIT sdObject (vec3 p) {
  p.y += .5;
  p.y = abs(p.y);
  p.y -= .45;
  vec3 size = vec3(.5, .02, .02);
  float s = 99.;
  pR(p.xy, -PI/2.);
  p -= .5;
  for (int i = 0; i <= 32; i++) {
    size.x -= .013;
    s = min(s, fBoxCheap(p + size, size));
    p = vec3(p.y, p.z, -p.x - 2. * size.x);
  }
  size.x += .3;
  s = min(s, fBoxCheap(p + size, size));
  return HIT(s, 1.);
}

HIT map (vec3 p) {
  HIT s = HIT(min(p.y+.5, 20.-length(p)), 0.); // inside sphere
  p.y -= .9;
  s = opU(s, sdObject(p));
  return s;
}

void main() {
  float t = .2 * PI * time;
  float zoom = .4;
  float h = cos(.5 * t);
  origin = zoom * vec3(
    4. * cos(t),
    2.5 + h,
    -4. * sin(t)
  );
  vec3 focus = vec3(0., .5 + .5 * h, 0.);
  vec3 c = vec3(0.);

  vec2 uvP = uv;
  #if ${GIF}
  for (float x=-.5; x<=.5; x += 1.) {
    for (float y=-.5; y<=.5; y += 1.) {
      uvP = uv + vec2(x, y) / 800.0;
  #endif
      vec3 dir = normalize(vec3(uvP - .5, 1.));
      dir = lookAt(origin, focus) * dir;
      vec3 p = origin;
      HIT hit = marcher(p, dir);
      vec3 n = normal(p);
      c += lighting(hit, p, n, dir);
  #if ${GIF}
    }
  }
  c /= 4.;
  #endif
  color = vec4(c, 1.0);
}`,
  },
});
