import React from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
export const n = 81;
export const title = "Red Blue Stripes";
export const exportSize = 512;
export const exportStart = 0;
export const exportEnd = 10;
export const exportFramePerSecond = 16;
export const exportSpeed = 1;
export const exportPaletteGenOnce = false;
export const exportPaletteSize = 64;

export const Shader = ({ time }) => {
  return (
    <Node
      shader={shaders.node}
      uniforms={{
        resolution: Uniform.Resolution,
        time,
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
    vec3(0.6, 0.2, 0.15)
  );
}

float fOpUnionRound(float a, float b, float r) {
	vec2 u = max(vec2(r - a,r - b), vec2(0));
	return max(r, min (a, b)) - length(u);
}

float scene(in vec2 p, float t) {
  vec2 q = vec2( fbm( 2.0 * p ), fbm( 2.0 * p + vec2(0.15,8.3) ) );
  vec2 r = vec2( fbm( 8. * q ),
                fbm( 2. * q + vec2(1.71, 5.1) ) );
  float v = fOpUnionRound(
    length(p+vec2(0.2 * sin(0.5 * PI * t), 0.2 * cos(PI * t))),
    0.5 - abs(fract(3. * p.x) - 0.5),
    0.8) +
    1.0 * fbm( 0.4 * p + 0.5 * r + 8. + 0.2 * t);
  return pow(fract(5.0 * v + 2. * t), 2.0);
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec3 c = vec3(0.);
  vec2 p = (uv - 0.5) * ratio;
  c += pal(scene(p, time));
  gl_FragColor = vec4(c, 1.0);
}
`,
  },
});
