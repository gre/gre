import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import { Surface } from "gl-react-dom";
import useDimensions from "react-cool-dimensions";
import { useSpring, animated } from "react-spring";

export function useTime() {
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

const r = Math.random();

const Main = ({ time, ni }) => (
  <Node
    shader={shaders.main}
    uniforms={{
      resolution: Uniform.Resolution,
      time: time + ni,
      r,
    }}
  />
);

const AnimatedMain = animated(Main);

export const Scene = ({ time, n }) => {
  const { ni } = useSpring({
    ni: n,
    config: {
      mass: 1,
      tension: 30,
      friction: 20,
    },
  });
  return <AnimatedMain time={time} ni={ni} />;
};

const Root = () => {
  const time = useTime();
  const [n, setN] = useState(() => 0);
  const { observe, width, height } = useDimensions({});
  function onClick() {
    setN((n) => n + 1);
  }
  return (
    <div
      onClick={onClick}
      ref={observe}
      style={{ cursor: "pointer", width: "100vw", height: "100vh" }}
    >
      <Surface width={width} height={height}>
        <Scene n={n} time={time} />
      </Surface>
    </div>
  );
};

const shaders = Shaders.create({
  main: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform vec2 resolution;
uniform float time;
uniform vec4 values;
uniform float r;

#define PI ${Math.PI}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float t) {
  return palette(
    t,
    vec3(.5),
    vec3(.6 + .1 * cos(.2 * time)),
    vec3(1.),
    vec3(0.22, 0.5, 0.77)
  );
}

float sdCircle(vec2 p, float r) {
  return length(p) - r;
}
float sdBox( in vec2 p, in vec2 b ) {
    vec2 d = abs(p)-b;
    return length(max(d,0.0)) + min(max(d.x,d.y),0.0);
}
float sdTri(vec2 p, float h) {
  vec2 q = abs(p);
  return max(q.x*0.866025+p.y*0.5,-p.y)-h*0.5;
}
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
float fOpUnionSoft(float a, float b, float r) {
	float e = max(r - abs(a - b), 0.);
	return min(a, b) - e*e*0.25/r;
}
float shape (vec2 p) {
  p -= 0.5;
  float t = 0.5 * time - 40. * r;
  float scale = 0.5 + 0.3 * sin(0.3 * t + 500. * r);
  float k = 0.2 + 0.1 * abs(cos(0.5 * t));
  float t2 = t * 1.2 + sin(t - r);
  float t3 = t * 0.8 - 0.5 * cos(t2 + 100. * r);
  float a1 = 0.2 - 0.15 * sin(t2 - 3. * r);
  float a2 = 0.2 - 0.15 * cos(t3);
  float a3 = 0.2 - 0.15 * sin(t + r);
  vec2 p1 = p + a1 * vec2(cos(t), sin(t));
  vec2 p2 = p + a2 * vec2(cos(t2), sin(t2));
  vec2 p3 = p + a3 * vec2(cos(t3), sin(t3));
  pR(p1, 0.2 * t3);
  pR(p2, 0.6 * t2);
  pR(p3, 0.1 * t);
  float s = sdCircle(p3, 0.25 * scale);
  s = fOpUnionSoft(s, sdBox(p2, vec2(0.2 * scale)), k);
  s = fOpUnionSoft(s, sdTri(p1, 0.3 * scale), 0.5 * k);
  return s;
}
void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;  
  float v = shape(base);
  v = max(-step(fract(0.25 * time), 0.5), v);
  gl_FragColor = vec4(
    step(v, 0.) * color(1.6 * (uv.y + time)) +
    step(0., v) * color(sqrt(max(v, 0.)) - 0.5 * time),
    1.0);
}
  `,
  },
});

ReactDOM.render(<Root />, document.getElementById("main"));
