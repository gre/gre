import React, { useEffect, useState } from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import { Surface } from "gl-react-dom";

const res = 500;

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

const colordelta = [
  0.5 + Math.random() * (0.5 - Math.random()),
  0.4 + 0.5 * Math.random() * Math.random(),
  0.5 + Math.random() * (0.5 - Math.random()),
];
const s1 = 100 * Math.random();
const s2 = 100 * Math.random();
const s3 = 100 * Math.random();

const Shader = () => {
  const time = useTime();
  return (
    <Node
      shader={shaders.node}
      uniforms={{
        resolution: Uniform.Resolution,
        time,
        colordelta,
        s1,
        s2,
        s3,
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
uniform float s1, s2, s3;
uniform vec3 colordelta;
#define PI ${Math.PI}


// Maximum/minumum elements of a vector
float vmax(vec2 v) {
	return max(v.x, v.y);
}

float vmax(vec3 v) {
	return max(max(v.x, v.y), v.z);
}

float vmax(vec4 v) {
	return max(max(v.x, v.y), max(v.z, v.w));
}

float vmin(vec2 v) {
	return min(v.x, v.y);
}

float vmin(vec3 v) {
	return min(min(v.x, v.y), v.z);
}

float vmin(vec4 v) {
	return min(min(v.x, v.y), min(v.z, v.w));
}
float fBox2(vec2 p, vec2 b) {
	vec2 d = abs(p) - b;
	return length(max(d, vec2(0.))) + vmax(min(d, vec2(0.)));
}
float fOpUnionRound(float a, float b, float r) {
	vec2 u = max(vec2(r - a,r - b), vec2(0.));
	return max(r, min (a, b)) - length(u);
}
float fOpIntersectionRound(float a, float b, float r) {
	vec2 u = max(vec2(r + a,r + b), vec2(0));
	return min(-r, max (a, b)) + length(u);
}
float fOpDifferenceRound (float a, float b, float r) {
	return fOpIntersectionRound(a, -b, r);
}
// Repeat space along one axis. Use like this to repeat along the x axis:
// <float cell = pMod1(p.x,5);> - using the return value is optional.
float pMod1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize, size) - halfsize;
	return c;
}

// Same, but mirror every second cell so they match at the boundaries
float pModMirror1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize,size) - halfsize;
	p *= mod(c, 2.0)*2. - 1.;
	return c;
}
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

float hash(float p) {
    p = fract(p * .1031);
    p *= p + 33.33;
    p *= p + p;
    return fract(p);
  }
  float hash(vec2 p) {
      vec3 p3  = fract(vec3(p.xyx) * .1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
  }
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
  for( int i=0; i<12; i++ ) {
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
    vec3(0.5, 0.5, 0.5),
    colordelta,
    vec3(1.0),
    vec3(0.49, 0.5, 0.51)
  );
}

vec3 scene(in vec2 p) {
  vec2 q = vec2( fbm( p + vec2(4.2, 1.8) ), fbm( 2. * p ) );
  vec2 r = vec2( fbm(7. * q),
                fbm(s1 + p + 80. * q));
  float v = fbm(0.8 * p + .2 * r + s2);

  vec2 o = p;

  p.x = abs(p.x);
  // pModMirror1(p.x, 0.4);
  // pModMirror1(p.y, 0.4);

  // v -= p.x * 4.;
  // v += p.y * 10.;

  float s = fbm(
    4.0 * p +
    s3 +
    fbm(vec2(fbm(3.4 * p + s1), fbm(4. * p + s2)))
  ) + length(p) - 0.7 + 0.01 * cos(6. * time);
  float diff = length(p - vec2(0.15, 0.0))-0.05;

  // s = min(s, 0.7 * p.x + 0.3 * abs(p.y) - 0.1);
  // s = min(s, p.x);

  float k = 0.15;

  p.x -= 0.05;
  p.y -= 0.1 * cos(1.3 * time);
  pR(p, s1 + 0.2 * time);
  s = fOpUnionRound(s, fBox2(p, vec2(0.03, 0.1)), k);
  p.x += 0.2;
  p.y += 0.1;
  pR(p, 1.0 + 0.2 * time);
  s = fOpUnionRound(s, fBox2(p, vec2(0.03, 0.2)), k);
  p.x -= 0.1 * sin(0.3 * time);
  p.y -= 0.1 * cos(time);
  pR(p, 2.0 - 0.4 * time);
  s = fOpUnionRound(s, fBox2(p, vec2(0.03, 0.2)), k);
  p.x += 0.1 * cos(.3 * time);
  p.y += 0.1 * sin(.2 * time);
  pR(p, 0.4 * time);
  s = fOpUnionRound(s, fBox2(p, vec2(0.03, 0.1)), k);
 
  /*
  p = o;
  pR(p, 0.3 * time);
  s = fOpUnionRound(s, fBox2(p, vec2(0.1, 0.4)), k);
  */

  s = fOpDifferenceRound(s, diff, k);

  float d = smoothstep(0.0, 0.2, s);
  v = mix(v, 0.5, smoothstep(0.01, 0.0, s));
  v -= .5 * pow(d, 0.2);

  return pal(v) + smoothstep(0.0, -0.2, s);
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec3 c = vec3(0.);
  vec2 p = (uv - 0.5) * ratio;
  c += scene(p);
  gl_FragColor = vec4(c, 1.0);
}
`,
  },
});

function Home() {
  return (
    <Surface pixelRatio={1} width={res} height={res}>
      <Shader />
    </Surface>
  );
}

export default Home;
