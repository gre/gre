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

const Main = ({ time, ni }) => (
  <Node
    shader={shaders.main}
    uniforms={{
      resolution: Uniform.Resolution,
      time: time + ni,
    }}
  />
);

const AnimatedMain = animated(Main);

export const Scene = ({ time, n }) => {
  const { ni } = useSpring({
    ni: n,
    config: {
      mass: 1,
      tension: 50,
      friction: 50,
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
#define PI ${Math.PI}

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
float shape (vec2 p, float d) {
  p -= 0.5;
  float t = time + 0.001 * d;
  float scale = 0.5 + 0.3 * sin(0.3 * t);
  float k = 0.2 + 0.1 * abs(cos(0.5 * t));
  float t2 = t * 1.1 + sin(t);
  float t3 = t * 0.9 - 0.5 * cos(t2);
  float a1 = 0.2 - 0.15 * cos(t2);
  float a2 = 0.2 - 0.15 * cos(t3);
  float a3 = 0.2 - 0.15 * cos(t);
  vec2 p1 = p + a1 * vec2(cos(t), sin(t));
  vec2 p2 = p + a2 * vec2(cos(t2), sin(t2));
  vec2 p3 = p + a3 * vec2(cos(t3), sin(t3));
  pR(p1, 0.2 * t3);
  pR(p2, 0.6 * t2);
  pR(p3, 0.1 * t);
  float s = sdCircle(p3, 0.25 * scale);
  s = fOpUnionSoft(s, sdBox(p2, vec2(0.2 * scale)), k);
  s = fOpUnionSoft(s, sdTri(p1, 0.3 * scale), 0.5 * k);
  return smoothstep(0.0, 0.0005, s);
}
void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;  
  gl_FragColor = vec4(
    shape(base, 4.0),
    shape(base, -2.0),
    shape(base, -2.0),
    1.0);
}
  `,
  },
});

ReactDOM.render(<Root />, document.getElementById("main"));
