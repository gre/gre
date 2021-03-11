

import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 47;
export const title = "glow experiment";

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
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

#define PI ${Math.PI}

// camera origin
vec3 origin;

vec2 map (vec3 p);
vec3 shade (vec2 m);
vec3 lighting (vec2 hit, vec3 p, vec3 n, vec3 dir);

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

// from HG_SDF
float vmax(vec3 v) {
	return max(max(v.x, v.y), v.z);
}
float fBox(vec3 p, vec3 b) {
	vec3 d = abs(p) - b;
	return length(max(d, vec3(0))) + vmax(min(d, vec3(0)));
}
float fSphere(vec3 p, float r) {
	return length(p) - r;
}
float sdSegment (in vec3 p, in float L, in float R) {
  p.y -= min(L, max(0.0, p.y));
  return length(p) - R;
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

// glowing is an output that contains the rgb color that is glowing with a alpha that contains how much it's glowing (not bounded)
vec2 marcher (inout vec3 p, vec3 dir, inout vec4 glowing) {
  vec2 hit = vec2(999., 0.);
  float t = 0.;
  float ph = 0.;
  float tA = 0.;
  vec3 tC = vec3(0.);

  for (int i=0; i<150; i++) {
    vec2 h = map(p + t * dir);
    h.x = min(.3, h.x);
    t += h.x;
    if (h.x < .001 || p.z > 20.) {
      hit = h;
      break;
    }
    if (h.y > 1.) {
      float a = .8 * ph * pow(smoothstep(.8, .0, h.x), 16.);
      ph = h.x;
      tC += a * shade(h);
      tA += a;
    }
  }

  glowing = vec4(tC / tA, min(1., tA));

  p += t * dir;
  return hit;
}

vec2 opU (vec2 a, vec2 b) {
  if (a.x < b.x) return a;
  return b;
}

vec3 lighting (vec2 hit, vec3 p, vec3 n, vec3 dir) {
  vec3 clr = shade(hit);
  float glow = step(1., hit.y);
  vec3 c = vec3(0.);
  vec3 lamp1 = vec3(-6., 12., -4.);
  vec3 lamp1dir = normalize(lamp1 - p);
  vec3 lamp2 = vec3(6., 6., -4.);
  vec3 lamp2dir = normalize(lamp2 - p);
  c = .8 * glow * clr;
  c +=
    .6 *
    (.45 * cos(time) + .5) *
    vec3(1., .7, .5) * (
      // ambient
      0.1 +
      // diffuse
      clr *
      // glow have half lambert
      mix(diffuse(p, n, lamp1), 1., .5 * glow)  *
      // glow don't receive shadows
      mix(softshadow(p, lamp1dir, 0.02, 8., 20.), 1., glow)
    );
  c +=
    .8 *
    (.45 * sin(2. * time) + .5) *
    vec3(.1, .4, .6) * (
      // ambient
      0.1 +
      // diffuse
      clr *
      // glow have half lambert
      mix(diffuse(p, n, lamp2), 1., .5 * glow)  *
      // glow don't receive shadows
      mix(softshadow(p, lamp2dir, 0.02, 8., 10.), 1., glow)
    );
  return c;
}

// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
vec3 color (float t) {
  return palette(
    t + time,
    vec3(.5),
    vec3(.5),
    vec3(1., 1.1, 1.),
    vec3(.1, .3, .5)
  );
}

vec3 shade (vec2 hit) {
  float m = hit.y;
  if (m<1.) return vec3(m);
  return vec3(color(fract(m)));
}

vec2 map (vec3 p) {
  vec2 s = vec2(p.y, .7); // ground
  p.y -= .5;
  // non glowing objects
  s = opU(s, vec2(fBox(p, vec3(.5)), .4));
  s = opU(s, vec2(fBox(p+vec3(1.2, 0.1, 1.2), vec3(.4)), 2.2));
  s = opU(s, vec2(fBox(p+vec3(-1.2, 0.1, 1.2), vec3(.4)), 2.7));

  // glowing objects
  p.y -= .6;
  s = opU(s, vec2(fSphere(p, .3), 2.));
  s = opU(s, vec2(fSphere(p+vec3(1.2, -1. + .8 * cos(time), 1.2), .3), 2.2));
  s = opU(s, vec2(fSphere(p+vec3(-1.2, -1. + .8 * sin(time), 1.2), .3), 2.7));
  s = opU(s, vec2(sdSegment((p+vec3(1., 0.6, 1.2)).zxy, 2., .05), 2.2+.5*smoothstep(-1., 1., p.x + .5 * cos(time))));
  pR(p.xz, time);
  pR(p.xy, .5);
  s = opU(s, vec2(sdSegment(p.zxy, 4., .05), 2.));
  return s;
}

mat3 lookAt (vec3 ro, vec3 ta) {
  float cr = 0.;
  vec3 ww = normalize( ta - ro );
  vec3 uu = normalize( cross(ww,vec3(sin(cr),cos(cr),0.0) ) );
  vec3 vv =          ( cross(uu,ww));
  return mat3(uu,vv,ww);
}

void main() {
  origin = vec3(0., 10., -9.);
  vec3 c = vec3(0.);
  vec2 dt = vec2(0.);
  vec2 uvP = uv + dt;
  vec3 dir = normalize(vec3(uvP - .5, 2.5));
  origin.x += 6. * sin(.5 + .2 * time);
  dir = lookAt(origin, vec3(0., 0., 0.)) * dir;
  vec3 p = origin;
  vec4 glowing = vec4(0.);
  vec2 hit = marcher(p, dir, glowing);
  vec3 n = normal(p);
  c += lighting(hit, p, n, dir);
  if (glowing.a>0.) {
    c = mix(c, glowing.rgb, glowing.a);
  }
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
