// @flow
import React, { useEffect, useRef } from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import { GameOfLife } from "../GameOfLife";
import Mandelglitch from "../../blockarts/Mandelglitch";
import MersenneTwister from "mersenne-twister";

export const n = 424242;
export const title = "CryptoAliens";

export const exportSize = 400;
export const exportStart = 0;
export const exportEnd = 10;
export const exportFramePerSecond = 24;
export const exportSpeed = 1;
export const exportMP4vb = "5M";

const SIZE = 20;

let firstTime;
export const Shader = ({ time, n, exporting }) => {
  const mod1 = 0.5;
  const mod2 = Math.sin((time * 2 * Math.PI) / 10);
  const mod3 = 0.5;
  const mod4 = 0.5;
  const seed = n;
  const highQuality = !!exporting;

  const block = { hash: "" + seed };
  const { hash } = block;

  const noopRef = useRef();

  const rng = new MersenneTwister(
    // when seed is not provided, it means we're in "production" and the seed is actually the block hash
    (seed || 1) * parseInt(hash.slice(0, 16), 16)
  );
  const s1 = rng.random();
  const s2 = rng.random();
  const s3 = rng.random();
  const s4 = rng.random();
  const s5 = rng.random();
  const s6 = rng.random();
  const s7 = rng.random();
  const s8 = rng.random();
  const s9 = rng.random();
  const sbg = rng.random();

  let background = [0.1, 0.11, 0.13];
  if (block.number % 2 < 1) {
    background = [0.92, 0.93, 0.96];
  }

  return (
    <Node
      shader={shaders.main}
      uniforms={{
        resolution: Uniform.Resolution,
        t: (
          <Mandelglitch
            block={block}
            seed={seed}
            mod1={mod1}
            mod2={mod2}
            mod3={mod3}
            mod4={mod4}
            attributesRef={noopRef}
          />
        ),
        highQuality,
        background,
        mod1,
        mod2,
        mod3,
        mod4,
        // from seed (block hash)
        s1,
        s2,
        s3,
        s4,
        s5,
        s6,
        s7,
        s8,
        s9,
      }}
    />
  );
};

const shaders = Shaders.create({
  main: {
    frag: GLSL`

    #version 300 es
precision highp float;
in vec2 uv;
out vec4 color;
uniform vec2 resolution;

uniform vec3 background;
uniform float s1,s2,s3,s4,s5,s6,s7,s8,s9;
uniform float mod1,mod2,mod3,mod4;
uniform sampler2D t;
uniform bool highQuality;

#define PI ${Math.PI}

#define HIT vec4
HIT map (vec3 p);
vec3 shade (HIT m, vec3 p);
vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir);

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
void pR(inout vec2 p, float a) {
p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
float fOpUnionSoft(float r, float a, float b) {
  float e = max(r - abs(a - b), 0.);
  return min(a, b) - e*e*0.25/r;
}
float sdRoundedCylinder( vec3 p, float ra, float rb, float h ) {
  vec2 d = vec2( length(p.xz)-2.0*ra+rb, abs(p.y) - h );
  return min(max(d.x,d.y),0.0) + length(max(d,0.0)) - rb;
}
float sdSegment (in vec3 p, in float L, in float R) {
  p.y -= min(L, max(0.0, p.y));
  return length(p) - R;
}

// https://www.iquilezles.org/www/articles/rmshadows/rmshadows.htm
float softshadow( in vec3 ro, in vec3 rd, float mint, float maxt, float k ) {
  float res = 1.0;
  float ph = 1e20;
  for(float t=mint; t<maxt; ) {
    float h = map(ro + rd*t).x;
    if( h<0.001) return 0.0;
    float y = h*h/(2.0*ph);
    float d = sqrt(h*h-y*y);
    res = min( res, k*d/max(0.0,t-y) );
    ph = h;
    t += h;
  }
  return res;
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

HIT marcher (inout vec3 p, vec3 dir) {
  // raymarching perf technique from https://www.shadertoy.com/view/XsyGWV
  HIT hit = HIT(0.);
  float precis = 0.0001;
  float t = 0.;
  for (int i=0; i<120; i++) {
    HIT h = map(p + t * dir);
    precis = t*0.0002;
    float rl = max(t*.02, 1.);
    t += h.x * rl;
    if (abs(h.x) < precis || p.z > 20.) {
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
  if (m<1.) return .0;
  return 0.8;
}

float specular (vec3 n, float m, vec3 ldir, vec3 dir, float p) {
  return specularStrength(m) * pow(max(dot(dir, reflect(ldir, n)), 0.0), p);
}

vec3 light (float id) {
  return 0.6 * palette(
    id + s1,
    vec3(0.8),
    vec3(0.5),
    vec3(1.0),
    // vec3(0.8, 0.0, 0.2)
    vec3(0.6, 0.8, 0.)
  );
}

vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir) {
  vec3 l, ldir;
  vec3 c = vec3(0.);
  l = vec3(-4., 4., -2.);
  ldir = normalize(l - p);
  c +=
  light(0.0) * (
    // ambient
    0.1
    // diffuse
    + shade(hit, p)
      * (.5 + .5 * diffuse(p, n, l)) // half lambert
    + specular(n, hit.y, ldir, dir, 20.)
  );
  l = vec3(4., 3., -2.);
  ldir = normalize(l - p);
  c +=
  light(0.5) * (
  // ambient
  0.1
  // diffuse
  + shade(hit, p)
    * (.5 + .5 * diffuse(p, n, l)) // half lambert
    * (0.6 + 0.4 * softshadow(p, ldir, 0.1, 16., 50.))
  + specular(n, hit.y, ldir, dir, 40.)
  );
  // adding ambient
  l = vec3(0., 6., -5.);
  ldir = normalize(l - p);
  c += vec3(.2) * (0.05 + shade(hit, p) * diffuse(p, n, l));
  return c;
}

vec3 shade (HIT hit, vec3 g) {
  float m = hit.y;
  if (m < 1.) {
    return background;
  }
  vec2 p = hit.zw;
  vec2 tUV = fract(p);
  return palette(
    s6 + mod3 * s5 * s5 * texture(t, tUV).r,
    vec3(0.5),
    vec3(0.5),
    vec3(1.0),
    vec3(0.6, 0.4, 0.3)
  );
}

float random1 (float a) {
  return fract(sin(a * 12.9898) * 43758.5453123); // very very light version of randomness
}

float worm (
  inout vec3 p,
  float w,
  float h,
  float k,
  int iterations,
  inout float ss1,
  inout float ss2
) {
  float s = sdSegment(p, h, w);
  for (int i = 0; i < iterations; i++) {
    pR(p.xy, 8. * s4 * (ss2-.5));
    pR(p.xz, 6. * s5 * (ss1-.5));
    s = fOpUnionSoft(k, s, sdSegment(p, h, w));
    ss1 = random1(ss1);
    ss2 = random1(ss2);
    h *= .9;
    w *= .9;
    p.y -= 1.2 * h;
  }
  s = fOpUnionSoft(k + 0.1 * s3, s, length(p) - pow(s4 * s5 * s6, 4.0));
  return s;
}

HIT obj (vec3 p) {
  vec2 xy = .5 + vec2(0.5, 1.0) * (p.xz + p.xy) / 2.0;
  // displacement
  p += 0.006 * s3 * s4 * vec3(cos(20. * p.y), cos(20. * p.x), cos(20. * p.x));
  p.y -= 0.1;
  float s = sdRoundedCylinder(p, (0.2 + 0.6 * s3) / 2.0, 0.02, 0.1);
  // random twist
  float twistAmp = 0.2 * pow(s8, 10.0);
  float twistFreq = s7 * 20.0 * p.y;
  p.x += 0.1 * twistAmp * cos(twistFreq);
  p.z += 0.1 * twistAmp * sin(twistFreq);

  float k = (0.05 + 0.2 * pow(s6, 3.0)) * (0.1 + mod1);
  float ss1 = s1;
  float ss2 = s2;

  float stepR = (s3 - 0.5) * pow(s4, 8.0) + (mod2 - .5);
  float stepR2 = s3 * 7.;
  float w = 0.04 + 0.05 * s6 * s6;
  float h = 0.3 + 0.2 * s5;
  float incr = 0.1 + 0.2 * pow(s6, 3.0);
  int iterations = int(2. + 20. * pow(1. - s6, 4.));
  float initialL = incr + s5;
  float arms = sdSegment(p, initialL, 0.1);
  p.y -= initialL;
  vec3 q;
  for (float f = 0.0; f<1.0; f+=0.1) {
    pR(p.xy, stepR);
    pR(p.xz, stepR2);
    s = fOpUnionSoft(0.1, s, sdSegment(p, incr, 0.1));
    q = p;
    pR(q.xy, PI / 2.0);
    if (abs(f-s4) < s5) {
      arms = fOpUnionSoft(0.1, arms, worm(q, w, h, k, iterations, ss1, ss2));
    }
    p.y -= incr;
  }
  s = fOpUnionSoft(0.01 + mod4 * mod4, s, arms);

  float sz = 0.2 * (pow(s6, 2.0)-0.2) + 0.3 * pow(s3, 8.0);
  if (sz > 0.0) {
    pR(q.xy, 100.0 * s2);
    q.y -= sz;
    q.y += 0.05 * s5 * cos(30. * s4 * q.x);
    q.z += 0.05 * s5 * cos(30. * s4 * q.x);
    s = fOpUnionSoft(0.2, s, length(q) - sz);
  }
  return HIT(s, 2.0, xy);
}

HIT map (vec3 p) {
  HIT s = HIT(min(20. - length(p),p.y), 0.1, 0., 0.);
  return opU(s, obj(p));
}

mat3 lookAt (vec3 ro, vec3 ta) {
  float cr = 0.;
  vec3 ww = normalize( ta - ro );
  vec3 uu = normalize( cross(ww,vec3(sin(cr),cos(cr),0.0) ) );
  vec3 vv =          ( cross(uu,ww));
  return mat3(uu,vv,ww);
}

vec3 scene(vec2 uvP) {
  float dy = 0.2 * sin(4. * PI * mod1);
  vec3 origin = vec3(16. * (mod1 - 0.5), 3.0 + dy, -5.0);
  vec3 poi = vec3(0.0, 1.5 + 2. * dy, 0.0);
  vec3 c = vec3(0.);
  vec3 dir = normalize(vec3(uvP - .5, 1.5));
  dir = lookAt(origin, poi) * dir;
  vec3 p = origin;
  HIT hit = marcher(p, dir);
  vec3 n = normal(p);
  c += lighting(hit, p, n, dir);
  // mist
  c = mix(c, background, smoothstep(8.0, 16.0, length(origin - p)));
  return c;
}

void main() {
  vec3 c = vec3(0.);
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;
  c = scene(base);
  if (highQuality) {
    for (float x=-.5; x<=.5; x += 1.) {
      for (float y=-.5; y<=.5; y += 1.) {
        vec2 d = 0.5 * vec2(x,y) / resolution;
        vec2 p = base + d;
        c += scene(p);
      }
    }
    c /= 5.0;
  }
  color = vec4(c, 1.0);
}
    `,
  },
});
