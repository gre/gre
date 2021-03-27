import React, { useEffect } from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import MersenneTwister from "mersenne-twister";

export const styleMetadata = {
  name: "CryptoMinions",
  description: "",
  image: "",
  creator_name: "greweb",
  options: {
    // comment seed when going production!
    seed: -3, // this was used for debug
    mod1: 0.5,
    mod2: 0.5,
    mod3: 0.5,
    mod4: 0.5,
  },
};

const shaders = Shaders.create({
  main: {
    frag: GLSL`
precision highp float;
varying vec2 uv;

uniform vec2 resolution;
uniform float mod1, mod2, mod3, mod4;
uniform float s1, s2, s3, s4, s5, s6, s7;

#define PI ${Math.PI}

// camera origin
vec3 origin;

#define HIT vec2
HIT map (vec3 p);
vec3 shade (HIT m, vec3 p);
vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir);
float specularStrength (float m);

float pMod1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize, size) - halfsize;
	return c;
}
float pModPolar(inout vec2 p, float repetitions) {
	float angle = 2.*PI/repetitions;
	float a = atan(p.y, p.x) + angle/2.;
	float r = length(p);
	float c = floor(a/angle);
	a = mod(a,angle) - angle/2.;
	p = vec2(cos(a), sin(a))*r;
	// For an odd number of repetitions, fix cell index of the cell in -x direction
	// (cell index would be e.g. -5 and 5 in the two halves of the cell):
	if (abs(c) >= (repetitions/2.)) c = abs(c);
	return c;
}
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
float fCylinder(vec3 p, float r, float height) {
	float d = length(p.xz) - r;
	d = max(d, abs(p.y) - height);
	return d;
}
float sdSegment (in vec3 p, in float L, in float R) {
  p.y -= min(L, max(0.0, p.y));
  return length(p) - R;
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

HIT marcher (inout vec3 p, vec3 dir) {
  HIT hit = HIT(0., 2.); // 2. because it's our tree that tends to glitch
  float t = 0.;
  for (int i=0; i<100; i++) {
    HIT h = map(p + t * dir);
    t += min(.3, h.x);
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
  l = vec3(-5., 6., -3.);
  ldir = normalize(l - p);
  c +=
    vec3(.8, .5, .2) * (
      // ambient
      0.1
      // diffuse
      + shade(hit, p)
        * (.5 + .5 * diffuse(p, n, l)) // half lambert
      + specular(n, hit.y, ldir, dir, 100.)
    );
  l = vec3(3., 1., -3.);
  ldir = normalize(l - p);
  c +=
    vec3(.2, .5, 1.) * (
    // ambient
    0.1
    // diffuse
    + shade(hit, p)
      * (.5 + .5 * diffuse(p, n, l)) // half lambert
    + specular(n, hit.y, ldir, dir, 10.)
  );

  l = vec3(0.6, 1.0, 0.0);
  ldir = normalize(l);
  c +=
  vec3(.5) * (
  // ambient
  0.0
  // diffuse
  + shade(hit, p)
    * (.5 + .5 * diffuse(p, n, l)) // half lambert
    * softshadow(p, ldir, 0.2, 0.8, 5.)
  + specular(n, hit.y, ldir, dir, 50.)
  );
  return c;
}

float specularStrength (float m) {
  return 0.1;
}

vec3 shade (HIT hit, vec3 _p) {
  if (hit.y < 2.0) return vec3(1.);
  if (hit.y < 3.0) return vec3(1.0, 0.9, 0.0);
  if (hit.y < 4.0) return vec3(0.6);
  if (hit.y < 5.0) return vec3(1.0);
  if (hit.y < 6.0) return vec3(0.0);
  if (hit.y < 7.0) return vec3(0.0);

  return vec3(0.0);
}

float random (vec2 st) {
  return fract(sin(dot(st.xy, vec2(12.9898,78.233)))*43758.5453123);
}

HIT sdHat (vec3 p) {
  p.y *= 2.0;
  float s = length(p) - 0.5;
  return HIT(s, 6.0);
}

HIT sdEyes (vec3 p, float d, float sz) {
  p.x = abs(p.x);
  p.x -= d / 2.0;
  float s1 = length(p) - sz;
  p.z += 0.1;
  float s2 = length(p) - 0.04;
  return opU(
    HIT(s1, 4.0),
    HIT(s2, 5.0)
  );
}

HIT sdGlasses (vec3 p, float w, float r) {
  vec3 q = p.xzy;
  q.x = abs(q.x);
  // TODO smin
  q.x -= r + w / 2.0 - 0.005;
  float s = max(
    fCylinder(q, r, w),
    -fCylinder(q, r, 1.0)
  ) - w / 2.0;
  // TODO laniere
  return HIT(s, 3.0);
}

HIT sdMinion (vec3 p) {
  p.y -= 0.3;
  // TODO body disformation
  // body
  float body = sdSegment(p, 0.6, 0.4);
  HIT hat = sdHat(p - vec3(0.0, 1.0, 0.0));
  // TODO maybe no glasses
  HIT glasses = sdGlasses(p - vec3(0.0, 0.5, -0.4), 0.03, 0.11);
  // TODO diff eyes
  HIT eyes = sdEyes(p - vec3(0.0, 0.5, -0.35), 0.21, 0.1);

  HIT h = HIT(body, 2.0);
  h = opU(h, glasses);
  h = opU(h, eyes);
  h = opU(h, hat);
  return h;
}

HIT map (vec3 p) {
  HIT s = HIT(6.-length(p), 0.); // inside sphere
  s = opU(s, HIT(p.y, 1.));
  s = opU(s, sdMinion(p));
  return s;
}

vec3 scene(vec2 uv) {
  origin = vec3(
    2. * (mod1-0.5),
    0.8,
    -2.
  );
  vec3 focus = vec3(0.0, .8, 0.);
  vec3 c = vec3(0.);
  vec3 dir = normalize(vec3(uv - .5, 1.));
  dir = lookAt(origin, focus) * dir;
  vec3 p = origin;
  HIT hit = marcher(p, dir);
  vec3 n = normal(p);
  c += lighting(hit, p, n, dir);
  // mist
  c = mix(c, vec3(1.), pow(smoothstep(1., 4., length(p-origin)), .5));

  return c;
}

vec3 render() {
  vec3 c = vec3(0.);
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;
  for (float x=-.5; x<=.5; x += 1.) {
    for (float y=-.5; y<=.5; y += 1.) {
      vec2 d = 0.5 * vec2(x,y) / resolution;
      vec2 p = base + d;
      c += scene(p);
    }
  }
  c /= 4.0;
  return c;
}

void main() {
  vec3 c = render();
  gl_FragColor = vec4(c, 1.0);
}
  `,
  },
});

const CustomStyle = ({
  block,
  attributesRef,
  seed,
  mod1,
  mod2,
  mod3,
  mod4,
}) => {
  const { hash } = block;

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

  useEffect(() => {
    const attributes = [];
    attributesRef.current = () => ({
      attributes,
    });
  }, [attributesRef]);

  return (
    <Node
      shader={shaders.main}
      uniforms={{
        resolution: Uniform.Resolution,
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
      }}
    />
  );
};

export default CustomStyle;
