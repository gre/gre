import React from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
export const n = 75;
export const title = "moiré square smoke";
export const exportSize = 640;
export const exportStart = 0;
export const exportEnd = 20;
export const exportFramePerSecond = 24;
export const exportSpeed = 1;
export const exportPaletteGenOnce = false;
export const exportPaletteSize = 256;

const BASE = 1000 * 60 * 60;
export const Shader = ({ time }) => {
  const t = Date.now();
  const baseT = t / BASE - Math.floor(t / BASE);
  return (
    <Node
      shader={shaders.node}
      uniforms={{
        resolution: Uniform.Resolution,
        time: time + 1000 * baseT,
        speed: 0.01,
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

uniform float speed;
uniform vec3 c;
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
    vec3(0.7),
    vec3(0.6),
    vec3(1.),
    vec3(0.5, 0.15, 0.25)
  );
}

float pattern(in vec2 p) {
  vec2 q = vec2( fbm( p ),
                 fbm( p + vec2(2.08,0.23) ) );
  vec2 r = vec2( fbm( 1.6 * q + speed * time ),
                fbm( q + vec2(.7,.2) ) );
  return fbm(  p + 2. * r + speed * time );
}

vec2 disp(in vec2 p) {
  vec2 q = vec2( fbm( 0.2 * p ), fbm( 0.2 * p + vec2(12.08,1.23) ) );
  return (0.3 + 0.1 * cos(0.5 * time)) * (0.5-vec2( fbm( 0.3 * p + q + 0.01 * time ), fbm( 0.3 * p + 0.6 * q - 0.01 * time ) ));
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec3 c = vec3(0.);
  vec2 p = (uv - 0.5) * ratio;
  p += disp(p);
  float l = max(abs(p.x),abs(p.y));
  float v = pattern(0.3 * p);
  float mul = 32.;
  float f = fract(v * mul);
  float value = (1. + floor(v * mul)) / (mul + 1.);
  value -= 0.1 * step(0.5, fract(90. * pow(l, 0.9 + 0.05 * cos(0.8 * time)) - time));
  c += pal(value);
  gl_FragColor = vec4(c, 1.0);
}
`,
  },
});
