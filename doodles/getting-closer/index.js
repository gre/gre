import React, { useState, useMemo, useEffect } from "react";
import ReactDOM from "react-dom";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import { Surface } from "gl-react-dom";
import useDimensions from "react-cool-dimensions";
import { useSpring, animated } from "react-spring";
import MersenneTwister from "mersenne-twister";

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

const Main = ({ time, values, ni }) => (
  console.log(ni),
  (
    <Node
      shader={shaders.main}
      uniforms={{
        resolution: Uniform.Resolution,
        time: time + ni,
        values,
      }}
    />
  )
);

const AnimatedMain = animated(Main);

export const Scene = ({ time, n }) => {
  const valuesMemo = useMemo(() => {
    const rng = new MersenneTwister(n);
    return Array(4)
      .fill(null)
      .map(() => rng.random());
  }, [n]);
  const { values, ni } = useSpring({
    values: valuesMemo,
    ni: n,
    config: {
      mass: 1,
      tension: 50,
      friction: 50,
    },
  });
  return <AnimatedMain values={values} time={time} ni={ni} />;
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
#define PI ${Math.PI}

float sdCircle(vec2 p, float r) {
  return length(p) - r;
}
float sdBox( in vec2 p, in vec2 b ) {
    vec2 d = abs(p)-b;
    return length(max(d,0.0)) + min(max(d.x,d.y),0.0);
}
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
float fOpUnionSoft(float a, float b, float r) {
	float e = max(r - abs(a - b), 0.);
	return min(a, b) - e*e*0.25/r;
}
float pModMirror1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize,size) - halfsize;
	p *= mod(c, 2.0)*2. - 1.;
	return c;
}
float shape (vec2 p, float d) {
  p -= 0.5;
  float t = 0.5 * pow(time, 1.2) + 0.01 * d;
  float t2 = 0.1 * t;
  p += 0.01 * t;
  p *= .1 * pow(max(1.,time), 0.5) + 1.0 + 0.2 * cos(0.1 * t);
  pR(p, -0.5);
  vec2 q = p;
  pR(q, 0.05 * time);
  float s = 0.03 - abs(fract(
    .6 * sin(0.1 * t + (3.4 + values.w) * q.y - 7. * q.x)
    + sin(5. + q.y + q.x - 0.1 * t + 0.5 * sin(30.41 * q.x + 5. * t) - 4.23 * t2 + 6.471 * values.y * q.x - 14.239 * q.y)
    )-0.5);
  float y = pModMirror1(p.y, 0.4 + 0.1 + 0.1 * cos(0.3 * t));
  float x = pModMirror1(p.x, 0.5 + 0.05 + 0.05 * sin(0.4 * t));
  p.x += (0.05 + 0.03 * cos(x * 4.64+y*94.3+t)) * cos(6. * values.x + t + 6. * fract(461.23 * y));
  p.y += 0.03 * sin(t + 2.31 * x + 46.23 * y);
  pR(p, t);
  vec2 off = vec2(0.0, 0.05);
  pR(off, 3.5 * y - 0.5 * x + 2. * t + 10. * values.z);
  float phase = cos(t + 5.3 * y - 34.12 * x);
  float sc = fOpUnionSoft(
    sdCircle(p, 0.05 + 0.05 * phase),
    sdBox(p + off, vec2(0.03 + 0.03 * (1. - phase))),
    0.1
  );
  s = min(-s, sc);
  return smoothstep(0.0, 0.0005, s);
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;  
  gl_FragColor = vec4(
    shape(base, -1.0),
    shape(base, 1.0),
    shape(base, 4.0),
    1.0);
}
  `,
  },
});

ReactDOM.render(<Root />, document.getElementById("main"));
