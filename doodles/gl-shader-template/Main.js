import React from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import { useSpring, animated } from "react-spring";

const Main = ({ time, values }) => {
  const t = time + 0.002 * values[0];
  return (
    <Node
      shader={shaders.node}
      uniforms={{
        resolution: Uniform.Resolution,
        time: t,
      }}
    />
  );
};

const AnimatedMain = animated(Main);


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
    vec3(0.6, 0.1, 0.3)
  );
}

float scene(in vec2 p) {
  vec2 q = vec2( fbm( p ),
                 fbm( p + vec2(6.08,0.23) ) );
  vec2 r = vec2( fbm( 3. * q + 0.03 * time ),
                fbm( q + vec2(.3,.2) ) );
  return fbm( p + 3. * r + 0.02 * time );
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec3 c = vec3(0.);
  vec2 p = (uv - 0.5) * ratio;
  float v = scene(p);
  c += pal(v);
  gl_FragColor = vec4(c, 1.0);
}
`,
  },
});

export const Scene = ({ n, time }) => {
  const { values } = useSpring({
    values: [n],
    config: {
      mass: 1,
      tension: 50,
      friction: 20,
    },
  });
  return (
    <AnimatedMain time={time} values={values} />
  );
};

export default Scene;