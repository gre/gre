import React from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";

export const n = 69;
export const title = "Bananas paradize";
export const exportSize = 400;
export const exportStart = 0;
export const exportEnd = 20;
export const exportFramePerSecond = 20;
export const exportSpeed = 1;

export const Shader = ({ time }) => (
  <Node
    shader={shaders.node}
    uniforms={{ time, resolution: Uniform.Resolution }}
  />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;

uniform vec2 resolution;
uniform float time;

#define PI ${Math.PI}

// camera origin
vec3 origin;

#define HIT vec2
HIT map (vec3 p);
vec3 shade (HIT m, vec3 p);
vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir);
float specularStrength (float m);

vec2 pMod2(inout vec2 p, vec2 size) {
	vec2 c = floor((p + size*0.5)/size);
	p = mod(p + size*0.5,size) - size*0.5;
	return c;
}
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
float fOpUnionSoft(float a, float b, float r) {
	float e = max(r - abs(a - b), 0.);
	return min(a, b) - e*e*0.25/r;
}
float vmax(vec2 v) {
	return max(v.x, v.y);
}
float vmax(vec3 v) {
	return max(max(v.x, v.y), v.z);
}
float fBox(vec3 p, vec3 b) {
	vec3 d = abs(p) - b;
	return length(max(d, vec3(0.))) + vmax(min(d, vec3(0.)));
}
float sdSegment (in vec3 p, in float L, in float R) {
  p.y -= min(L, max(0.0, p.y));
  return length(p) - R;
}
float mod289(float x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 mod289(vec4 x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 perm(vec4 x){return mod289(((x * 34.0) + 1.0) * x);}
float noise(vec3 p){
    vec3 a = floor(p);
    vec3 d = p - a;
    d = d * d * (3.0 - 2.0 * d);
    vec4 b = a.xxyy + vec4(0.0, 1.0, 0.0, 1.0);
    vec4 k1 = perm(b.xyxy);
    vec4 k2 = perm(k1.xyxy + b.zzww);
    vec4 c = k2 + a.zzzz;
    vec4 k3 = perm(c);
    vec4 k4 = perm(c + 1.0);
    vec4 o1 = fract(k3 * (1.0 / 41.0));
    vec4 o2 = fract(k4 * (1.0 / 41.0));
    vec4 o3 = o2 * d.z + o1 * (1.0 - d.z);
    vec2 o4 = o3.yw * d.x + o3.xz * (1.0 - d.x);
    return o4.y * d.y + o4.x * (1.0 - d.y);
}

mat3 lookAt (vec3 ro, vec3 ta) {
  float cr = 0.;
  vec3 ww = normalize( ta - ro );
  vec3 uu = normalize( cross(ww,vec3(sin(cr),cos(cr),0.0) ) );
  vec3 vv =          ( cross(uu,ww));
  return mat3(uu,vv,ww);
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

// https://www.iquilezles.org/www/articles/rmshadows/rmshadows.htm
float softshadow( in vec3 ro, in vec3 rd, float mint, float maxt, float k ) {
  float res = 1.0;
  float ph = 1e20;
  float t = mint;
  for (int i=0; i<40; i++) {
    float h = 0.5 * map(ro + rd*t).x;
    if (t>=maxt) break;
    if( h<0.001) return 0.0;
    float y = h*h/(2.0*ph);
    float d = sqrt(h*h-y*y);
    res = min( res, k*d/max(0.0,t-y) );
    ph = h;
    t += h;
  }
  return res;
}

HIT marcher (inout vec3 p, vec3 dir) {
  HIT hit = HIT(0., 2.); // 2. because it's our tree that tends to glitch
  float t = 0.;
  for (int i=0; i<120; i++) {
    HIT h = map(p + t * dir);
    t += 0.8 * min(.3, h.x);
    if (abs(h.x) < .0001) {
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

float specular (vec3 n, float m, vec3 ldir, vec3 dir, float p) {
  return specularStrength(m) * pow(max(dot(dir, reflect(ldir, n)), 0.0), p);
}

vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir) {
  vec3 l, ldir;
  vec3 c = vec3(0.);
  l = vec3(-2., 4., -1.);
  ldir = normalize(l - p);
  c +=
  vec3(0.9, 0.7, 0.5) * (
    // ambient
    0.1
    // diffuse
    + shade(hit, p)
      * (.5 + .5 * diffuse(p, n, l)) // half lambert
      * (0.5 + 0.5 * softshadow(p, ldir, 0.05, 5., 20.))
    + specular(n, hit.y, ldir, dir, 20.)
  );
  l = vec3(2., 6., -2.);
  ldir = normalize(l - p);
  c +=
  vec3(0.3, 0.5, 0.6) * (
  // ambient
  0.1
  // diffuse
  + shade(hit, p)
    * (.5 + .5 * diffuse(p, n, l)) // half lambert
  + specular(n, hit.y, ldir, dir, 40.)
  );
  return c;
}

float specularStrength (float m) {
  return 0.1;
}

vec3 shade (HIT hit, vec3 _p) {
  if (hit.y < 2.0) return vec3(.8);
  if (hit.y < 3.0) {
    float m = hit.y - 2.0;
    return mix(
      vec3(1.0, 0.9, 0.0),
      vec3(0.3, 1.0, 0.0),
      0.6 * m
    );
  }
  if (hit.y < 4.0) {
    return mix(vec3(0.6, 1.0, 0.0), vec3(0.5, 0.3, 0.2), fract(hit.y));
  }
  return vec3(0.0);
}

HIT banana (vec3 p, vec2 id) {
  float r1 = fract(id.y * 44.15 - id.x * 745.739);
  float r2 = fract(id.x * 133.6109);
  float r3 = fract(id.y * 56.857);
  p.y -= 0.5;
  pR(p.xy, p.y * (0.9 + r2 + r3));
  p.y += 0.5;
  float m = 0.999 * max(smoothstep(0.8, 1.0, p.y) + smoothstep(0.001, 0., 0.02 + p.y - 0.04 * r1), 0.1 * r3 * r3); // material
  float n1 = noise(111. * (p - id.y));
  float n2 = noise(5. * (p + id.x - id.y));
  m += 0.999 * smoothstep(0.9, 1.0, m);
  m += 2. * smoothstep(0.01, 0.0, 0.6 - n1 * n2);
  m = min(1.999, m);
  float k = 0.2 + 0.1 * r1;
  float s = 0.25 + 0.15 * r2;
  float body = fOpUnionSoft(
    sdSegment(p, 1.0, 0.04),
    fBox(p - vec3(0., 0.45, 0.), vec3(0.06, s, 0.04)),
    k
  );
  HIT h = HIT(body, 2.0 + m);
  return h;
}


HIT map (vec3 p) {
  HIT s = HIT(10. - length(p), 0.); // inside sphere
  s = opU(s, HIT(p.y, 1.));
  p.x -= 0.5;
  p.z += time * smoothstep(20., 60., time);
  vec2 id = pMod2(p.xz, vec2(1.0));
  id.x += 7. * id.y;
  p.y -= 0.1 + 0.2 * cos(0.1 * PI * time + id.x);
  pR(p.xz, PI * 0.2 * time);
  pR(p.xy, -0.5);
  s = opU(s, banana(p, id));
  return s;
}

vec3 scene(vec2 uv) {
  origin = vec3(
    0.1 * cos(.1 * PI * time),
    1.0 + 0.5 * sin(.1 * PI * time),
    -2.
  );
  vec3 focus = vec3(0.5 * sin(.2 * PI * time), .6, 0.);
  vec3 c = vec3(0.);
  vec3 dir = normalize(vec3(uv - .5, 1.));
  dir = lookAt(origin, focus) * dir;
  vec3 p = origin;
  HIT hit = marcher(p, dir);
  vec3 n = normal(p);
  c += lighting(hit, p, n, dir);
  c = mix(c, vec3(0.9), pow(smoothstep(4., 10., length(p-origin)), .5));
  return c;
}

vec3 render() {
  vec3 c = vec3(0.);
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;
  c += scene(base);
  return c;
}

void main() {
  vec3 c = render();
  gl_FragColor = vec4(c, 1.0);
}
`,
  },
});
