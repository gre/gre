import React from "react";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 71;
export const title = "warp2";
export const exportSize = 640;
export const exportStart = 0;
export const exportEnd = 20;
export const exportFramePerSecond = 24;
export const exportSpeed = 1;
export const exportPaletteGenOnce = false;
export const exportPaletteSize = 256;

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;

uniform float time;
#define PI ${Math.PI}

// from https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83
float hash(float n) { return fract(sin(n) * 1e4); }
float hash(vec2 p) { return fract(1e4 * sin(17.0 * p.x + p.y * 0.1) * (0.1 + abs(sin(p.y * 13.0 + p.x)))); }
float noise(float x) {
	float i = floor(x);
	float f = fract(x);
	float u = f * f * (3.0 - 2.0 * f);
	return mix(hash(i), hash(i + 1.0), u);
}
float noise(vec2 x) {
	vec2 i = floor(x);
	vec2 f = fract(x);
	float a = hash(i);
	float b = hash(i + vec2(1.0, 0.0));
	float c = hash(i + vec2(0.0, 1.0));
	float d = hash(i + vec2(1.0, 1.0));
	vec2 u = f * f * (3.0 - 2.0 * f);
	return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}
const mat2 m2 = mat2( 0.6,  0.8, -0.8,  0.6 );
float fbm( in vec2 x ) {
  float f = 2.0;
  float s = 0.55;
  float a = 0.0;
  float b = 0.5;
  for( int i=0; i<9; i++ ) {
    float n = noise(x);
    a += b * n;
    b *= s;
    x = f * x;
  }
	return a;
}

vec3 palette(float t,vec3 a,vec3 b,vec3 c,vec3 d){
  return a+b*cos(6.28318*(c*t+d));
}
vec3 pal(float t){
  return palette(
    t,
    vec3(0.5),
    vec3(0.5),
    vec3(1.),
    vec3(0.1,0.28,0.32)
  );
}

float pattern(in vec2 p, float t) {
  vec2 q = vec2( fbm( p ),
                 fbm( p + vec2(4.3,1.2) ) );
  vec2 r = vec2( fbm( p + 3.*q + vec2(2.7,5.3) ),
                 fbm( p + 3.*q + vec2(3.3 + 0.2 * cos(0.1 * t),11.4 + 0.2 * sin(0.1 * t)) ) );
  return fbm( p + 3.*r + cos(0.05 * t) - sin(0.05 * t) );
}

void main() {
  float l = smoothstep(0.2, 0.4, length(uv-.5));
  vec3 c = pal(
    0.6 * l * pattern(uv, -time * 2. * PI) +
    0.6 * pattern(4.4 + 0.5 * uv, time * 2. * PI)
  );
  gl_FragColor = vec4(c, 1.0);
}
`,
  },
});
