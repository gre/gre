import React, { useState, useMemo, useEffect } from "react";
import ReactDOM from "react-dom";
import { Shaders, Node, GLSL, Uniform, LinearCopy } from "gl-react";
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

const Persistence = ({ children: t, time }) => (
  <Node
    shader={shaders.persistence}
    backbuffering
    uniforms={{ t, time, back: Uniform.Backbuffer }}
  />
);

const Mandelglitch = ({
  time,
  clickT,
  values: [travel, palOff, s1, s2, s3, s4, s5, s6, s7, s8, s9],
}) => (
  <Node
    shader={shaders.main}
    uniforms={{
      resolution: Uniform.Resolution,
      time: time - clickT,
      travel,
      palOff,
      s1,
      s2,
      s3,
      s4,
      s5,
      s6,
      s7,
      s8,
      s9,
    }}
  />
);

const AnimatedMandelglitch = animated(Mandelglitch);

export const Scene = ({ n, time, clickTime }) => {
  const valuesMemo = useMemo(() => {
    const rng = new MersenneTwister(n);
    return Array(13)
      .fill(null)
      .map(() => rng.random());
  }, [n]);
  const { values, clickT } = useSpring({
    clickT: clickTime,
    values: valuesMemo,
    config: {
      mass: 1,
      tension: 50,
      friction: 30,
    },
  });
  return (
    <LinearCopy>
      <Persistence time={time}>
        <AnimatedMandelglitch clickT={clickT} time={time} values={values} />
      </Persistence>
    </LinearCopy>
  );
};

const Main = () => {
  const time = useTime();
  const [clickTime, setClickTime] = useState(time);
  const [n, setN] = useState(() => Date.now());
  const { ref, width, height } = useDimensions({});
  function onClick() {
    setN((n) => n + 1);
    setClickTime(time);
  }
  return (
    <div
      onClick={onClick}
      ref={ref}
      style={{ cursor: "pointer", width: "100vw", height: "100vh" }}
    >
      <Surface width={width} height={height}>
        <Scene n={n} time={time} clickTime={clickTime} />
      </Surface>
    </div>
  );
};

const shaders = Shaders.create({
  persistence: {
    frag: GLSL`
  precision highp float;
  varying vec2 uv;
  uniform sampler2D t, back;
  uniform float time;
  void main () {
    gl_FragColor =
      mix(
        texture2D(t, uv),
        (1.04  + 0.02 * cos(3. * time)) * texture2D(back, uv) - 0.01,
        0.9
      );
  }
      `,
  },
  main: {
    frag: GLSL`
  #version 300 es
precision highp float;
in vec2 uv;
out vec4 color;

uniform vec2 resolution;
uniform float time;
uniform float travel, palOff;
uniform float s1, s2, s3, s4, s5, s6, s7, s8, s9;

const float PI = ${Math.PI};
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
vec3 pal (float t) {
  return palette(
    t + 0.45 + 0.1 * palOff,
    vec3(.6),
    vec3(.8),
    vec3(1.0),
    vec3(1.0, 1.0 - 0.1 * s1, 1. - 0.1 * s3)
    // vec3(0.5 + 0.2 * s1, 0.3 + 0.2 * s2, 1.0)
  );
}
float run (vec2 init) {
  float iterations = 300.;
  vec2 p = init;
  for (float iter = 0.; iter < iterations; iter += 1.) {
    // original mandelbrot formula is:
    // p = vec2( p.x * p.x - p.y * p.y, 2. * p.x * p.y) + init;
    float x2 = p.x * p.x;
    float y2 = p.y * p.y;
    float xy = p.x * p.y;
    float axy = abs(xy);
    float a = 1. + .1 * (s1 - 0.5) * s2;
    float b = -1. + .1 * (s1 - 0.5) * s2;
    float c = -0.05 * time + 2. * (s2 - 0.5) * s3;
    float d = 0.5 * max(0., pow(s8, 2.) - 0.5) * cos(100. * s7 * s2 * s9 * p.y);
    float e = 0.5 * max(0., pow(s9, 2.) - 0.5) * sin(100. * s2 * s1 * p.x);
    float f = 2. + s6 - s6 * s6 * s6;
    vec2 offset = init + mix(vec2(0.0), vec2(s4, s5) - .5, s3 * s4 * s5);
    p = vec2(
      a * x2 + b * y2 + c * xy + d,
      f * mix(xy, axy, s2) + e
    ) + offset;
    if (length(p) >= 2.0) {
      return iter / iterations;
    }
  }
  return 0.;
}
vec3 shade (vec2 uv) {
  float zoom = (0.5 + 10. * s7 * s7 * s7);
  float focusAngle = 5. * travel;
  float focusAmp = 0.3 + 0.5 * s7;
  vec2 init = 2. * (uv - .5) / zoom;
  pR(init, 2. + s1);
  init += focusAmp * vec2(cos(focusAngle), sin(focusAngle));
  return pal(pow(run(init), .5));
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 uvRatio = 0.5 + (uv - 0.5) * ratio;
  vec3 c = vec3(0.);
  float total = 0.0;
  vec2 uvP = uvRatio;
  c += shade(uvRatio);
  color = vec4(c, 1.0);
}
  `,
  },
});

ReactDOM.render(<Main />, document.getElementById("main"));
