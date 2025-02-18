

import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 42;
export const title = "reflection party";

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
vec3 shade (vec2 m);
float glossyness (float m);
vec3 lighting (vec2 hit, vec3 p, vec3 n, vec3 dir);

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

// from HG_SDF
float fOpUnionSoft(float r, float a, float b) {
	float e = max(r - abs(a - b), 0.);
	return min(a, b) - e*e*0.25/r;
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
float fCapsule(vec3 p, float r, float c) {
	return mix(length(p.xz) - r, length(vec3(p.x, abs(p.y) - c, p.z)) - r, step(c, abs(p.y)));
}
float fTorus(vec3 p, float smallRadius, float largeRadius) {
	return length(vec2(length(p.xz) - largeRadius, p.y)) - smallRadius;
}
float fHexagonCircumcircle(vec3 p, vec2 h) {
	vec3 q = abs(p);
	return max(q.y - h.y, max(q.x*sqrt(3.)*0.5 + q.z*0.5, q.z) - h.x);
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

// gre's
vec3 reflection (vec3 p, vec3 n, float maxDist) {
  vec3 o = vec3(0.);
  float bounced = 0.;
  vec3 dir = n;
  vec2 r;
  float total = 0.;
  for (int i=0; i<4; i++) {
    dir = reflect(dir, n);
    r = marcher(p, dir);
    bounced += r.x;
    if (bounced > maxDist) break;
    p += r.x * dir;
    n = normal(p);
    o += lighting(r, p, n, dir) * clamp((maxDist - bounced) / maxDist, 0., 1.);
    total += 1.;
    p += dir; // we need to progress a bit more to not have ray staying at same pos
    if (glossyness(r.y)<=.0) {
      break;
    }
  }
	return o / total;
}

float glossyness(float m) {
  return 0.8 * step(.8, m);
}

vec3 shade (vec2 hit) {
  float m = hit.y;
  if (m < 1.) return vec3(1.);
  return palette(
    (m - 1.) * .3,
    vec3(.5),
    vec3(.5),
    vec3(1.),
    vec3(.0, .33, .66)
  );
}

vec2 opU (vec2 a, vec2 b) {
  if (a.x < b.x) return a;
  return b;
}

vec2 map (vec3 p) {
  vec2 ground = vec2(p.y, 0.1);
  p.y -= 1.5;
  float d = length(p) - .2;
  d = fOpUnionSoft(.5, d, length(p+.6 * vec3(sin(3. + .5 * time), .0, cos(.7 * time))) - .2);
  d = fOpUnionSoft(.5, d, length(p+.4 * vec3(cos(time), sin(time), cos(.6 * time))) - .2);
  d = fOpUnionSoft(.5, d, length(p+.5 * vec3(-sin(.9 * time), cos(1.1 * time), .0)) - .2);
  d = fOpUnionSoft(.5, d, length(p+.6 * vec3(.0, cos(-time), sin(.8 * time))) - .2);
  vec2 metaballs = vec2(d, 1.);

  pR(p.xz, .5 * time);

  p.y += .5;
  vec2 s = opU(ground, metaballs);
  p.x += 1.;
  s = opU(s, vec2(fBox(p, vec3(.3)), 2.));
  p.z += 2.;
  s = opU(s, vec2(fCapsule(p, .3, .5), 3.));
  p.x -= 2.;
  s = opU(s, vec2(fTorus(p.xzy, .1, .3), 4.));
  p.z -= 2.;
  {
    vec3 w = p;
    pR(w.xz, 2. * time);
    s = opU(s, vec2(fHexagonCircumcircle(w, vec2(.3, .4)), 5.));
  }

  return s;
}

float specularStrength (float m) {
  if (m < 1.) return 0.0;
  return 6.0;
}
float specularPow (float m) {
  return 128.0;
}

float specular (vec3 n, float m, vec3 ldir, vec3 dir) {
  return specularStrength(m) * pow(max(dot(dir, reflect(-ldir, n)), 0.0), specularPow(m));
}

float dayTime = .5 + .5 * cos(.2 * time);

vec3 emitColor(vec2 hit) {
  float m = hit.y;
  if (m >= 1. && mod(pow(time, 1.2) + m, 5.) < .9) {
    return mix(0., 1., smoothstep(.2, .15, dayTime)) * shade(hit);
  }
  return vec3(0.);
}

vec3 lighting (vec2 hit, vec3 p, vec3 n, vec3 dir) {
  float lightOn = smoothstep(.1, .5, dayTime);
  vec3 c = emitColor(hit);
  vec3 lamp1 = vec3(-6., 8., -4.);
  vec3 ldir1 = normalize(lamp1 - p);
  c +=
    lightOn *
    vec3(1., .7, .5) * (
      // ambient
      0.1 +
      // diffuse
      shade(hit)
      * diffuse(p, n, lamp1)
      * softshadow(p, ldir1, 0.02, 10., 4.) +
      // specular
      specular(n, hit.y, ldir1, dir)
    );
  vec3 lamp2 = vec3(6., 8., -4.);
  vec3 ldir2 = normalize(lamp2 - p);
  c +=
    lightOn *
    vec3(.5, .6, .7) * (
    // ambient
    0.1 +
    // diffuse
    shade(hit)
    * diffuse(p, n, lamp2)
    * softshadow(p, ldir2, 0.02, 10., 20.) +
    // specular
    specular(n, hit.y, ldir2, dir)
  );
  return c;
}

void main() {
  vec3 origin = vec3(0., 4., -4.);
  vec2 uvP = uv;
  vec3 dir = normalize(vec3(uvP - .5, 1.));
  pR(dir.yz, -.7);
  vec3 p = origin;
  vec2 hit = marcher(p, dir);
  vec3 n = normal(p);
  vec3 c = lighting(hit, p, n, dir);
  float glossy = glossyness(hit.y); // TODO fresnel
  c = mix(c, reflection(p, n, 10.), glossy);
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
