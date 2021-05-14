import React from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";

export const n = 70;
export const title = "warp1";
export const exportSize = 400;
export const exportStart = 0;
export const exportEnd = 20;
export const exportFramePerSecond = 20;
export const exportSpeed = 1;
export const exportPaletteGenOnce = false;
export const exportPaletteSize = 256;

export const Shader = ({ time }) => (
  <Node
    shader={shaders.node}
    uniforms={{ time, resolution: Uniform.Resolution }}
  />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;

uniform vec2 resolution;
uniform float time;

// adapted work from https://www.iquilezles.org/
float hash1( vec2 p ) {
  p = 50.0*fract( p*0.3183099 );
  return fract( p.x*p.y*(p.x+p.y) );
}
float noise( in vec2 x ) {
  vec2 p = floor(x);
  vec2 w = fract(x);
  vec2 u = w*w*w*(w*(w*6.0-15.0)+10.0);
  float a = hash1(p+vec2(0,0));
  float b = hash1(p+vec2(1,0));
  float c = hash1(p+vec2(0,1));
  float d = hash1(p+vec2(1,1));
  return -1.0+2.0*( a + (b-a)*u.x + (c-a)*u.y + (a - b - c + d)*u.x*u.y );
}
const mat2 m2 = mat2( 0.4,  1.0, -1.2,  0.5 );
float fbm( in vec2 x ) {
  float f = 1.8;
  float s = 0.5;
  float a = 0.0;
  float b = 0.4;
  for( int i=0; i<9; i++ ) {
    float n = noise(x);
    a += b*n;
    b *= s;
    x = f * m2 * x;
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
    vec3(0.9,0.1,0.2)
  );
}

float pattern( in vec2 p ) {
  vec2 q = vec2( fbm( p + vec2(1.0, 0.01 * time) ),
                 fbm( p + vec2(4.9,2.9) ) );
  vec2 r = vec2( fbm( p + 3.*q + vec2(2.0,5.5) ),
                 fbm( p + 3.*q + vec2(3.,1.) ) );
  return fbm( p + 4.*r + vec2(4.0, 0.5)+ vec2(1.0, 0.1 * time) );
}

void main() {
  vec3 c = pal(0.1 * time + smoothstep(0.5, 0.2, length(uv - .5)) * pattern(0.5 * uv));
  gl_FragColor = vec4(c, 1.0);
}
`,
  },
});
