import React from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";

export const n = 107;
export const title = "SDF fBM";
export const exportSize = 360;
export const exportStart = 20;
export const exportEnd = 36;
export const exportFramePerSecond = 16;
export const exportSpeed = 1;
export const exportPaletteSize = 64;

export const Shader = ({ time }) => {
  return (
    <Node
      shader={shaders.node}
      uniforms={{
        time,
        resolution: Uniform.Resolution,
      }}
    />
  );
};

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

float fOpUnionSoft(float a, float b, float r) {
	float e = max(r - abs(a - b), 0.);
	return min(a, b) - e*e*0.25/r;
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
  if (hit.y < 1.0) return vec3(1.0, 0.0, 0.0);
  if (hit.y < 2.0) return vec3(.8);
  if (hit.y < 3.0) {
    return vec3(0.5, 0.0, 0.0);
  }
  return vec3(0.0);
}

float sph( vec3 i, vec3 f, vec3 c )
{
    vec3  p = 17.0*fract( (i+c)*0.3183099+vec3(0.11,0.17,0.13) );
    float w = fract( p.x*p.y*p.z*(p.x+p.y+p.z) );
    float r = 1.0*w*w;
    return length(f-c) - r; 
}

float sdBase (vec3 p) {
  vec3 i = vec3(floor(p));
   vec3 f =       fract(p);
  float s = min(min(min(sph(i,f,vec3(0.,0.,0.)),
  sph(i,f,vec3(0.,0.,1.))),
min(sph(i,f,vec3(0.,1.,0.)),
  sph(i,f,vec3(0.,1.,1.)))),
min(min(sph(i,f,vec3(1.,0.,0.)),
  sph(i,f,vec3(1.,0.,1.))),
min(sph(i,f,vec3(1.,1.,0.)),
  sph(i,f,vec3(1.,1.,1.)))));
  return s;
}
float sdFbm( vec3 p, float d )
{
   float s = 1.;
   for( int i=0; i<7; i++ )
   {
       float n = s*sdBase(p);
       d = fOpUnionSoft(n,d, 0.3*s);
       p = mat3( 0.00, 1.60, 1.20,
                -1.60, 0.72,-0.96,
                -1.20,-0.96, 1.28 )*p;
       s = 0.5*s;
   }
   return d;
}

HIT map (vec3 p) {
  //HIT s = HIT(sdFbm(p, 10. - length(p)), 0.);
  // HIT s = HIT(sdBase(p), 0.);
  HIT s = HIT(sdFbm(p, 99.), 0.);
  return s;
}

vec3 scene(vec2 uv) {
  origin = vec3(
    time,
    1.0,
    -3.
  );
  vec3 focus = vec3(0.0, .6, 0.);
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