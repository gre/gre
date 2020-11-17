import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 19;
export const title = "they were six";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time: time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
const float PI = ${Math.PI};
// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
// from http://glslsandbox.com/e#43182.0 / 007 example
#define SQRT3 1.7320508
const vec2 s = vec2(1.0, SQRT3);
float hex(in vec2 p){
  p = abs(p);
  return max(dot(p, s*.5), p.x);
}
vec4 getHex(vec2 p) {
  vec4 hC = floor(vec4(p, p - vec2(.5, 1))/s.xyxy) + .5;
  vec4 h = vec4(p - hC.xy*s, p - (hC.zw + .5)*s);
  return dot(h.xy, h.xy)<dot(h.zw, h.zw) ? vec4(h.xy, hC.xy) : vec4(h.zw, hC.zw + 9.73);
}
// utilities from classical SDF
float pModPolar(inout vec2 p, float repetitions) {
	float angle = 2.*PI/repetitions;
	float a = atan(p.y, p.x) + angle/2.;
	float r = length(p);
	float c = floor(a/angle);
	a = mod(a,angle) - angle/2.;
	p = vec2(cos(a), sin(a)) * r;
	if (abs(c) >= (repetitions/2.)) c = abs(c);
	return c;
}
float pMod1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize, size) - halfsize;
	return c;
}
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
//////
vec3 color (float t) {
  return palette(
    t,
    vec3(.5),
    vec3(.5),
    vec3(.7, .9, smoothstep(-1., 1., cos(4. * time + uv.y))),
    vec3(.1, .2, .3)
  );
}

vec3 tile (vec2 p, vec2 g) {
  float r1 = pModPolar(p, 6.); // we start by projecting our hex system into 6 triangles
  p.x -= 1./3.; // move to center of the triangle
  pR(p, (mod(r1, 2.) - .5) * PI / 3.); // depending on oddity, we will rotate counter or clockwise
  float r2 = 1. + pModPolar(p, 3.); // then split again the triangle by 3
  // color index in grid system, see explanation at the end of this file
  float index = mod(2. + mod(3. - mod(floor(.5*(r1 + 5.)), 3.), 3.) - r2, 3.);
  return color(0.1 * index); // pick color from palette!
}

float blob (vec2 p, float t) {
  float cycle = sin(11. * t);
  float cycle2 = cos(.1 * t);
  float climb = mod(.2 * t, 1.8);
  float stopAt = .8;
  // bounce jumps
  p -= vec2(
    .04 * sign(cycle) * pow(abs(cycle), 0.25) * step(climb, stopAt),
    .1 + 0.1 * pow(abs(cycle), 0.5) * step(climb, stopAt)
    + min(stopAt, climb)
    - step(stopAt, climb) * (climb - stopAt) * .82
  );
  vec2 disf = vec2(1./(0.9 - 0.3 * abs(cycle) * step(climb, stopAt)), 1.);
  return smoothstep(.024, .025, length(p * disf));
}

void main() {
  float t = time * smoothstep(0., 10., time);
  // hex grid
  vec2 g = uv * 3. + vec2(0., 0.5 * t);
  vec4 r = getHex(g);
  vec3 c = tile(r.xy * vec2(1., -1.), r.zw);
  // adding blobs
  float nb = step(8., t) + step(19., t) + step(25., t) + 3. * step(30., t);
  vec2 p = uv + vec2(.08, .0);
  float f = pMod1(p.x, 1. / 6.);
  p += step(nb + .1, f);
  c = mix(vec3(1.), c, blob(p, t + (1337. * f)));
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});

/*
brainstorm notes
https://twitter.com/greweb/status/1328426850411012096
to get the pattern I want. that I found in a "Barcelona Tile Designs" book,
I figure i need to map these r1,r2 coordinate to this res:

r1, r2, res:
0 0 0
0 1 2
0 2 1
1 0 2
1 1 1
1 2 0
2 0 2
2 1 1
2 2 0
3 0 1
3 1 0
3 2 2
4 0 1
4 1 0
4 2 2
5 0 0
5 1 2
5 2 1

we can identify "group of r1":

r1 = 0 | 5 -> a=2
res = mod(3. - r2, 3.)

r1 = 1 | 2 -> a=0
res = mod(2. - r2, 3.)

r1 = 3 | 4 -> a=1
res = mod(4. - r2, 3.)

then this can all be written as:
~>
res = mod(2. + mod(3. - a, 3.) - r2, 3.)

where a is:
mod(floor(.5*(r1 + 5.)), 3.)
*/
