import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import { Shaders, Node, GLSL, Uniform, NearestCopy } from "gl-react";
import { Surface } from "gl-react-dom";
import useDimensions from "react-cool-dimensions";
import MersenneTwister from "mersenne-twister";

const Shader = ({ time, n, w, h }) => {
  const rng = new MersenneTwister(n);
  const a = 0.2 + 3 * rng.random() * rng.random();
  const b = 0.2 + 10 * rng.random() * rng.random();
  const baseC = rng.random();
  const c = [
    baseC + 0.2 - 0.5 * rng.random(),
    baseC + 0.02 - 0.1 * rng.random(),
    baseC - 0.02 + 0.1 * rng.random(),
  ];
  return (
    <Node
      width={w}
      height={h}
      shader={shaders.node}
      uniforms={{
        resolution: Uniform.Resolution,
        time,
        a,
        b,
        c,
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

uniform float a, b;
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
    vec3(0.5),
    vec3(0.5),
    vec3(1.),
    c
  );
}

float pattern(float t, in vec2 p) {
  vec2 q = vec2( fbm( p ),
                 fbm( p + vec2(12.08,1.23) ) );
  vec2 r = vec2( fbm( q + p ),
                fbm( q + vec2(8.37,0.28) ) );
  return fbm( p + a*q + b * r + 0.1 * t );
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec3 c = vec3(0.);
  vec2 p = (uv - 0.5) * ratio;
  p += 0.2 * (vec2(
    pattern(0.2 * time, 0.2 * p + vec2(1., 0.)),
    pattern(0.2 * time, 0.2 * p + vec2(0., 1.)))-0.5);
  float l = length(p);
  float v = pattern(time, p);
  float mul = 3.;
  float f = fract(v * mul);
  float value = floor(v * mul) / mul;
  c += pal(value * step(l, 0.4) * step(0.1, l));
  gl_FragColor = vec4(c, 1.0);
}
`,
  },
});

function useTime() {
  const [time, setTime] = useState(0);
  useEffect(() => {
    let startT;
    let h;
    function loop(t) {
      h = requestAnimationFrame(loop);
      if (!startT) startT = t;
      setTime((t - startT) / 1000);
    }
    h = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(h);
  }, []);
  return time;
}

const viewportStyle = {
  position: "absolute",
  width: "100vw",
  height: "100vh",
  display: "flex",
  flexDirection: "column",
};

const Rendering = () => {
  const { width, height, observe } = useDimensions({});
  const time = useTime();
  if (!height) {
    return <div ref={observe} style={viewportStyle} />;
  }
  const resolutionCap = 512;
  const maxDim = Math.max(width, height);
  const max = Math.min(resolutionCap, maxDim);
  const w = Math.round((max * width) / maxDim);
  const h = Math.round((max * height) / maxDim);

  return (
    <div ref={observe} style={viewportStyle}>
      <Surface width={width} height={height}>
        <NearestCopy>
          <Shader
            w={w}
            h={h}
            time={time}
            n={Math.floor(new Date() / (1000 * 60 * 60 * 24)) - 18766}
          />
        </NearestCopy>
      </Surface>
    </div>
  );
};

const Main = () => <Rendering />;

ReactDOM.render(<Main />, document.getElementById("main"));
