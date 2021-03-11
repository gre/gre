

import { Shaders, Node, GLSL } from "gl-react";
import { GameOfLife } from "../GameOfLife";

export const n = 97;
export const title = "Hexagon of Life";

const SIZE = 20;

let firstTime;
export const Shader = ({ time }) => {
  if (!firstTime) {
    firstTime = time;
  }
  const t = time - firstTime;
  const refreshEveryTicks = 50;
  const tick = Math.floor(t * 5);
  return (
    <Node
      shader={shaders.node}
      uniforms={{
        time,
        t: (
          <GameOfLife
            refreshEveryTicks={refreshEveryTicks}
            tick={tick}
            size={SIZE}
          />
        ),
      }}
    />
  );
};

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
uniform sampler2D t;

#define PI ${Math.PI}
#define SIZE ${SIZE}.

// from http://glslsandbox.com/e#43182.0
#define SQRT3 1.7320508
const vec2 s = vec2(1.0, SQRT3);
float hex(in vec2 p){
  p = abs(p);
  return max(dot(p, s*.5), p.x);
}
vec4 getHex(vec2 p) {
  vec4 hC = floor(vec4(p, p - vec2(.5, 1))/s.xyxy) + .5;
  vec4 h = vec4(p - hC.xy*s, p - (hC.zw + .5)*s);
  return dot(h.xy, h.xy)<dot(h.zw, h.zw) ? vec4(h.xy, hC.xy) : vec4(h.zw, hC.zw - .27);
}
// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
///////////////////////////////////////////

mat2 rot (float a) {
  float c = cos(a);
  float s = sin(a);
  return mat2(c,s,-s,c);
}

float sdSegment (in vec3 p, in float L, in float R) {
  p.y -= min(L, max(0.0, p.y));
  return length(p) - R;
}

float sdSphere (in vec3 p, in float R) {
  return length(p) - R;
}

vec2 id;
float inBound;
float active;

// IDEA : hexagon tower with a game of life in it? and that repeat itself recursively?

float SDF(vec3 p) {
  // The whole 3D objects are defined in this function
  p.z -= 8.;
  //p.yz *= rot(-1. + 0.1 * pow(.5 + .5*cos(0.9 * time), 4.) + 0.2 * cos(.08 * time));
  //p.xz *= rot(.1 * time);
  p.yz *= rot(-1.6);
  vec4 h = getHex(p.xz);
  id = h.zw;
  vec2 u = id / vec2(SIZE, SIZE / 2.);
  inBound = step(0., u.x) * step(0., u.y) * step(u.x, 1.) * step(u.y, 1.);
  active = step(0.1, texture2D(t, u).r);
  p.x = h.x;
  p.z = h.y;
  float s = sdSphere(p.xyz, 0.1 + 0.3 * active);
  return s;
}

void main() {
  vec3 p = vec3(SIZE/2., SIZE/2., -0.);
  vec3 dir = normalize(vec3((uv - 0.5) * 2.,1.));
  float shad = 1.;
  for (int i=0; i<60; i++) {
    float d = SDF(p);
    if (d<0.001) {
      shad = float(i)/60.;
      break;
    }
    p += d * dir * 0.5;
  }

  // Coloring
  vec3 c = inBound *
    sqrt(vec3(1. - shad));
  gl_FragColor = vec4(c,1.0);
}`,
  },
});
