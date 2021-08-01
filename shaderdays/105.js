import React from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";

export const n = 105;
export const preload = ["/images/tezos-dist.png"];
export const title = "Tezos swarm";
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
        time,
        resolution: Uniform.Resolution,
        image: "/images/tezos-dist.png"
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
uniform sampler2D image;
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
    vec3(0.6),
    vec3(0.5),
    vec3(1.0),
    vec3(0.9, 0.2, 0.3)
  );
}

float scene(in vec2 p) {
  float t = 0.2 * PI * time;
  vec2 q = vec2( fbm( 2.0 * p ), fbm( 3.0 * p ) );
  vec2 r = vec2( fbm( 20. * q + 1.0 * vec2(cos(t), sin(t))),
                fbm( 20. * q + 2.0 * vec2(cos(t), sin(t))));
  float v = 0.5 * fbm(0.5 * p + 3.0 + 0.6 * r + vec2(0.1 * cos(t + 0.5), 0.2 * sin(t + 0.5)));
  v += 0.5 * texture2D(image, uv).a;
  return .2 + .6 * fbm(q + 0.2 * vec2(cos(t), sin(t))) + 2. * smoothstep(0.3, 1.0, fract(v));
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec3 c = vec3(0.);
  vec2 p = (uv - 0.5) * ratio;
  c += pal(scene(p));
  gl_FragColor = vec4(c, 1.0);
}
`,
  },
});