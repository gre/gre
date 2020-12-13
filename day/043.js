import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 43;
export const title = "irreconcilable queens";

export const Shader = ({ time }) => (
  <LinearCopy>
    <Persistence persistence={0.4}>
      <Node shader={shaders.node} uniforms={{ time }} />
    </Persistence>
  </LinearCopy>
);

const Persistence = ({ children: t, persistence }) => (
  <Node
    shader={shaders.persistence}
    backbuffering
    uniforms={{ t, back: Uniform.Backbuffer, persistence }}
  />
);

const shaders = Shaders.create({
  persistence: {
    frag: GLSL`
    precision highp float;
    varying vec2 uv;
    uniform sampler2D t, back;
    uniform float persistence;
    void main () {
      gl_FragColor = mix(
        texture2D(t, uv),
        texture2D(back, uv),
        persistence
      );
    }
        `,
  },
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

#define PI ${Math.PI}

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
float fOpUnionSoft(float r, float a, float b) {
	float e = max(r - abs(a - b), 0.);
	return min(a, b) - e*e*0.25/r;
}
float fBox(vec3 p, vec3 b) {
	vec3 d = abs(p) - b;
	return length(max(d, vec3(0))) + vmax(min(d, vec3(0)));
}
float fSphere(vec3 p, float r) {
	return length(p) - r;
}
float fTorus(vec3 p, float smallRadius, float largeRadius) {
	return length(vec2(length(p.xz) - largeRadius, p.y)) - smallRadius;
}
float fDisc(vec3 p, float r) {
	float l = length(p.xz) - r;
	return l < 0. ? abs(p.y) : length(vec2(p.y, l));
}
float fCone(vec3 p, float radius, float height) {
	vec2 q = vec2(length(p.xz), p.y);
	vec2 tip = q - vec2(0., height);
	vec2 mantleDir = normalize(vec2(height, radius));
	float mantle = dot(tip, mantleDir);
	float d = max(mantle, -q.y);
	float projected = dot(tip, vec2(mantleDir.y, -mantleDir.x));

	// distance to tip
	if ((q.y > height) && (projected < 0.)) {
		d = max(d, length(tip));
	}

	// distance to base ring
	if ((q.x > radius) && (projected > length(vec2(height, radius)))) {
		d = max(d, length(q - vec2(radius, 0.)));
	}
	return d;
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
    if (hit.x < 0.001 || p.z > 40.) {
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
  return 0.2 * step(.8, m);
}

vec3 shade (vec2 hit) {
  float m = hit.y;
  if (m < 1.) return vec3(1.);
  return vec3(mix(.05, 1., fract(m) * 2.));
}

vec2 opU (vec2 a, vec2 b) {
  if (a.x < b.x) return a;
  return b;
}

float sdChessKingOrQueen (vec3 p) {
  float d;
  float body = fOpUnionSoft(
    .1,
    fCone(p * vec3(1., -1., 1.), .12, .5),
    fCone(p + vec3(0., 1., 0.), .16, .8)
  );
  d = fOpUnionSoft(.1, body, fDisc(p, .15)-.01);
  vec3 q = p;
  float discs;
  q.y += 0.02;
  /*
  fTorus(q, .02, .12);
  */
  q.y += 0.2;
  discs = fDisc(q, .06) - .01;
  q.y += 0.06;
  discs = min(discs, fDisc(q, .07) - .01);
  q.y += 0.04;
  discs = min(discs, fDisc(q, .08) - .03);
  q.y += 0.49;
  discs = min(discs, fDisc(q, .11) - .02);
  q.y += 0.11;
  discs = min(discs, fDisc(q, .12) - .05);
  d = fOpUnionSoft(.015, d, discs);
  return d;
}

// height of 1m + crawn
float sdChessQueen (vec3 p) {
  float s = sdChessKingOrQueen(p);
  float ball = fSphere((p-vec3(0., 0.05, 0.)) * vec3(1., 2., 1.), .05);
  s = min(s, ball);
  pModPolar(p.xz, 14.);
  p.y -= .08;
  p.x -= .2;
  s = max(s, -fSphere(p, 0.1));
  return s;
}

// tiles are of 1m x 1m
vec2 sdChessboard (vec3 p) {
  float manhattan = max(abs(p.x), abs(p.z));
  float o = step(4., manhattan);
  float m = mix(
    2. + .5 * step(1., mod(floor(p.x)+floor(p.z), 2.)),
    3. + .5 * step(4.1, manhattan),
    o
  );
  return vec2(fBox(p, vec3(4.4, 0.04, 4.4)), m);
}

vec2 sdChessPiece(vec3 p, float id, float white) {
  return vec2(sdChessQueen((p - vec3(0., 1.4, 0.)) / 1.4), id + .5 * white);
}

float inOutCubic (float t) {
  return mix(4.*t*t*t, (t-1.)*(2.*t-2.)*(2.*t-2.)+1., step(.5, t));
}

vec2 map (vec3 p) {
  vec2 ground = vec2(p.y, 0.1);
  vec2 s = ground;
  p.y -= .04;
  s = opU(s, sdChessboard(p));
  p.y -= .04;
  p.xz += vec2(3.5);
  float phase = mod(time, 4.);
  float whiteMove = inOutCubic(min(
      1.5 * min(1., phase),
      max(0., 1. - 1.5 * max(0., phase-2.))
    ));
  phase -= 1.;
  float blackMove = inOutCubic(min(
      min(1., 1.5 * max(0., phase)),
      max(0., 1. - 1.5 * max(0., phase-2.))
    ));
  p.x -= 3.;
  s = opU(s, sdChessPiece(p - vec3(whiteMove, 0., 0.), 10., 0.5));
  p.z -= 7.;
  s = opU(s, sdChessPiece(p - vec3(blackMove, 0., 0.), 10., 0.));
  return s;
}

float specularStrength (float m) {
  if (m<1.) return .1;
  return 5.0;
}
float specularPow (float m) {
  return 32.0;
}

float specular (vec3 n, float m, vec3 ldir, vec3 dir) {
  return specularStrength(m) * pow(max(dot(dir, reflect(-ldir, n)), 0.0), specularPow(m));
}

vec3 emitColor(vec2 hit) {
  float m = hit.y;
  return vec3(0.);
}

vec3 lighting (vec2 hit, vec3 p, vec3 n, vec3 dir) {
  vec3 c = emitColor(hit);
  vec3 lamp1 = vec3(-4., 1., -6.);
  vec3 ldir1 = normalize(lamp1 - p);
  c +=
    vec3(1., .7, .5) * (
      // ambient
      0.2 +
      // diffuse
      shade(hit)
      * (.5 + .5 * diffuse(p, n, lamp1)) // half lambert
      * softshadow(p, ldir1, 0.02, 8., 16.) +
      // specular
      specular(n, hit.y, ldir1, dir)
    );
  vec3 lamp2 = vec3(4., 8., -7.);
  vec3 ldir2 = normalize(lamp2 - p);
  c +=
    vec3(.6, .7, .9) * (
    // ambient
    0.1 +
    // diffuse
    shade(hit)
    * (.5 + .5 * diffuse(p, n, lamp2)) // half lambert
    * softshadow(p, ldir2, 0.02, 8., 12.) +
    // specular
    specular(n, hit.y, ldir2, dir)
  );
  return c;
}

void main() {
  vec3 origin = vec3(0., 2., -6.);
  origin.x += sin(2. * PI * time / 4.);
  vec3 c = vec3(0.);
  vec2 uvP = uv;
  vec3 dir = normalize(vec3(uvP - .5, 1.));
  pR(dir.yz, -.4);
  vec3 p = origin;
  vec2 hit = marcher(p, dir);
  vec3 n = normal(p);
  c += lighting(hit, p, n, dir);
  // float glossy = glossyness(hit.y); // TODO fresnel
  // c = mix(c, reflection(p, n, 10.), glossy);
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
