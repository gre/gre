import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 20;
export const title = "trippy tiles";

export const Shader = ({ time }) => (
  <LinearCopy>
    <Persistence persistence={0.79 + Math.min(.2, .003 * time)}>
      <Node shader={shaders.node} uniforms={{ time: time }} />
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
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
//////
vec3 color (float t) {
  return palette(
    t,
    vec3(.5),
    vec3(.5),
    vec3(1.),
    vec3(.5, .2 + mod(.03 * time + 0.2, .3), .8 - mod(.01 * time, .4))
  );
}

vec3 tile (vec2 p, float t) {
  float r1 = pModPolar(p, 6.);
  p.x -= 1./3.;
  pR(p, t);
  float r2 = 1. + pModPolar(p, 3.);
  float index = mod(r2 + mod(-r1, floor(2. + mod(3. * t, 3.))), 3.);
  return color(0.25 * index);
}

void main() {
  float t = pow(0.1 * time, 1.6);
  vec2 p = uv - .5;
  pR(p, .1 * t);
  vec2 g = p * (2. + floor(mod(.2 * time, 4.))) + vec2(0., .2 * t);
  vec4 r = getHex(g);
  vec3 c = tile(r.xy * vec2(1., -1.), t);
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
