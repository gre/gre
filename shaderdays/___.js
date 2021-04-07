import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 52;
export const title = "";

export const Shader = ({ time }) => (
  <Node
    shader={shaders.node}
    uniforms={{
      time,
      wood: "/images/seamless-wood2.jpg",
    }}
  />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
uniform sampler2D wood;

#define PI ${Math.PI}

// camera origin
vec3 origin;

#define HIT vec4
HIT map (vec3 p);
vec3 shade (HIT m, vec3 p);
vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir);

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

// ADAPTED from https://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float sdCone( in vec3 p, in float r, float h ) {
  p.y -= h;
  // c is the sin/cos of the angle, h is height
  // Alternatively pass q instead of (c,h),
  // which is the point at the base in 2D
  vec2 q = h*vec2(r/h,-1.0);

  vec2 w = vec2( length(p.xz), p.y );
  vec2 a = w - q*clamp( dot(w,q)/dot(q,q), 0.0, 1.0 );
  vec2 b = w - q*vec2( clamp( w.x/q.x, 0.0, 1.0 ), 1.0 );
  float k = sign( q.y );
  float d = min(dot( a, a ),dot(b, b));
  float s = max( k*(w.x*q.y-w.y*q.x),k*(w.y-q.y)  );
  return sqrt(d)*sign(s);
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
float pModInterval1(inout float p, float size, float start, float stop) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p+halfsize, size) - halfsize;
	if (c > stop) { //yes, this might not be the best thing numerically.
		p += size*(c - stop);
		c = stop;
	}
	if (c <start) {
		p += size*(c - start);
		c = start;
	}
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
float fDisc(vec3 p, float r) {
	float l = length(p.xz) - r;
	return l < 0. ? abs(p.y) : length(vec2(p.y, l));
}
float fCylinder(vec3 p, float r, float height) {
	float d = length(p.xz) - r;
	d = max(d, abs(p.y) - height);
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

HIT marcher (inout vec3 p, vec3 dir) {
  // raymarching perf technique from https://www.shadertoy.com/view/XsyGWV
  HIT hit = HIT(0.);
  float precis = 0.0001;
  float t = 0.;
  for (int i=0; i<80; i++) {
    HIT h = map(p + t * dir);
    precis = t*0.0001;
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
  if (m<10.) return .3;
  return .8;
}

float specular (vec3 n, float m, vec3 ldir, vec3 dir, float p) {
  return specularStrength(m) * pow(max(dot(dir, reflect(ldir, n)), 0.0), p);
}

vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir) {
  vec3 c = vec3(0.);
  vec3 lamp1 = vec3(-4., 0.5, -5.);
  vec3 lamp1dir = normalize(lamp1 - p);
  c +=
    vec3(.8, .9, .4) * (
      // ambient
      0.1
      // diffuse
      + shade(hit, p)
        * (.5 + .5 * diffuse(p, n, lamp1)) // half lambert
      + specular(n, hit.y, lamp1dir, dir, 60.)
    );
  vec3 lamp2 = vec3(4., 2., -3.);
  vec3 lamp2dir = normalize(lamp2 - p);
  c +=
    vec3(1., .4, .3) * (
    // ambient
    0.3
    // diffuse
    + shade(hit, p)
      * (.5 + .5 * diffuse(p, n, lamp2)) // half lambert
      * softshadow(p, lamp2dir, 0.02, 4., 32.)
    + specular(n, hit.y, lamp2dir, dir, 20.)
  );
  vec3 lamp3 = vec3(0., 1.5 + cos(time), 10.);
  vec3 lamp3dir = normalize(lamp3 - p);
  c +=
    vec3(.5) * (
    // ambient
    .1
    // diffuse
    + shade(hit, p)
      * diffuse(p, n, lamp3) // half lambert
    + specular(n, hit.y, lamp3dir, dir, 10.)
  );
  return c;
}

vec3 shade (HIT hit, vec3 _p) {
  float m = hit.y;
  if (m < 1.) return vec3(.8);
  float wFactor = fract(m) * 2.;
  vec2 p = hit.zw;
  vec2 tUV = fract(p);
  float piece = step(10., m);
  vec3 t = texture2D(wood, tUV).r * mix(
    vec3(.1),
    vec3(.9),
    wFactor
  );
  return t;
}

// height of 1m
float sdChessKingOrQueen (vec3 p) {
  float d;
  float body = fOpUnionSoft(
    .1,
    sdCone(p * vec3(1., -1., 1.), .12, .5),
    sdCone(p + vec3(0., 1., 0.), .16, .8)
  );
  d = fOpUnionSoft(.1, body, fDisc(p, .15)-.01);
  vec3 q = p;
  float discs;
  q.y += 0.22;
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

float sdChessKing (vec3 p) {
  float s = sdChessKingOrQueen(p);
  p.y -= .12;
  p.y *= 1.1; // a bit stretched
  float cross = length(p.xy)-.04;
  pModPolar(p.xy, 4.);
  pR(p.xy, .5 * PI);
  p.y += 0.07;

  vec3 q = abs(p);
  float sz = .04;
  cross = fOpUnionSoft(.02, cross, max(.5*q.x+.5*p.y,-p.y)-sz*0.5);
  // cross = min(cross, p.x+p.y-0.2);
  s = fOpUnionSoft(.02, s, max(cross, q.z-.02));
  return s;
}

float sdChessPawn (vec3 p) {
  float d;
  p.y += .5;
  float body = fOpUnionSoft(
    .01,
    fSphere(p, .1),
    sdCone(p + vec3(0., .5, 0.), .16, .5)
  );
  p.y += .11;
  d = min(body, fDisc(p, .08)-.02);
  p.y += 0.27;
  float base = min(fDisc(p, .04) - .08, fDisc(p+vec3(.0, .05, .0), .12) - .04);
  d = fOpUnionSoft(.05, d, base);
  return d;
}

float sdChessRook (vec3 p) {
  p.y += .4;
  // body
  float d = max(
    sdCone(p + vec3(0., .5, 0.), .14, .6),
    p.y + .05
  );
  // crown
  vec3 q = p;
  q.y -= .2;
  q.y *= -1.;
  float cyl = max(
    abs(p.y) - .06,
    sdCone(q, .16, .5)
  );
  q = p - vec3(0., .06, 0.);
  float dif = max(
    abs(q.y) - .02,
    min(
      min(abs(q.z), abs(q.x)) - .02,
      length(q.xz) - .08
    )
  );
  d = min(d, max(cyl, -dif));
  // crown base
  d = min(
    d,
    min(
      fDisc(p + vec3(.0, .07, .0), .08)-.01,
      fDisc(p + vec3(.0, .09, .0), .06)-.02
    ));
  // body base
  p.y += 0.5;
  d = fOpUnionSoft(.03, d,
    min(
      fDisc(p, .05) - .1,
      min(
        fDisc(p-vec3(.0, .05, .0), .1)-.05,
        fDisc(p-vec3(.0, .01, .0), .14)-.03
      )
    )
  );
  return d;
}

// tiles are of 1m x 1m
HIT sdChessboard (vec3 p) {
  float manhattan = max(abs(p.x), abs(p.z));
  float o = step(4., manhattan);
  float m = mix(
    2. + .5 * step(1., mod(floor(p.x)+floor(p.z), 2.)),
    3. + .5 * step(4.38, manhattan),
    o
  );
  return HIT(fBox(p, vec3(4.4, 0.2, 4.4)), m, p.x, p.z);
}

float sdChessPiece(vec3 p, float id) {
  float s = 99.;
  p = (p - vec3(0., 1.4, 0.)) / 1.4;
  if (id == 10.) s = sdChessKing(p);
  if (id == 11.) s = sdChessQueen(p);
  // if (id == 12.) s = sdChessBishop(p);
  // if (id == 13.) s = sdChessKnight(p);
  if (id == 14.) s = sdChessRook(p);
  if (id == 15.) s = sdChessPawn(p);
  return s;
}

float sdChessPieceId(float id, float white) {
  return id + .5 * white;
}

HIT piece (vec3 p, float id, float w) {
  float m = sdChessPieceId(id, w);
  float piece = sdChessPiece(p, id);
  return HIT(piece, m, p.x + .3 * p.y, p.z - .7 * p.y);
}

float idForPhase (float i) {
  i = mod(i, 4.);
  return 10. + i + 2. * step(2., i);
}

HIT board (vec3 p) {
  p.y -= .05;
  HIT s = sdChessboard(p);
  p.y -= .05;
  p.xz += vec2(3.5, 0.5);
  float x = pModInterval1(p.x, 1., 0., 7.);
  float t1 = mod(time + x, 4.);
  float t2 = fract(t1);
  t1 = floor(t1);
  float w = mod(x, 2.);
  HIT a = piece(p, idForPhase(t1), w);
  HIT b = piece(p, idForPhase(t1 + 1.), w);
  s = opU(s, HIT(mix(a.x, b.x, t2), a.y, a.z, a.w));
  return s;
}

HIT map (vec3 p) {
  HIT s = HIT(20. - length(p), 0.1, 0., 0.); // inside sphere
  s = opU(s, board(p));
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
  float zoom = 3.;
  origin = vec3(
    zoom * cos(.2 * time),
    2.,
    zoom * sin(.2 * time)
  );
  vec3 c = vec3(0.);
  vec2 dt = vec2(0.);
  vec2 uvP = uv + dt;
  vec3 dir = normalize(vec3(uvP - .5, .8));
  dir = lookAt(origin, vec3(-0.5, 1., -0.5)) * dir;

  // debug ortho camera
  #if 0
  origin.x = -0.5;
  origin.y = 1.;
  origin.z = -3.;
  origin += vec3(3. * (uvP - .5)- vec2(0., 0.), 0.);
  dir = normalize(vec3(0., 0., 1.));
  #endif

  vec3 p = origin;
  HIT hit = marcher(p, dir);
  vec3 n = normal(p);
  c += lighting(hit, p, n, dir);
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
