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

const Persistence = ({ children: t }) => (
  <Node
    shader={shaders.persistence}
    backbuffering
    uniforms={{ t, back: Uniform.Backbuffer }}
  />
);

const Main = ({ time, clickT, values: [s1, s2, s3, s4] }) => (
  <Node
    shader={shaders.main}
    uniforms={{
      resolution: Uniform.Resolution,
      time: time - clickT,
      s1,
      s2,
      s3,
      s4,
    }}
  />
);

const AnimatedMain = animated(Main);

export const Scene = ({ n, time, clickTime }) => {
  const valuesMemo = useMemo(() => {
    const rng = new MersenneTwister(n);
    return Array(4)
      .fill(null)
      .map(() => rng.random());
  }, [n]);
  const { values, clickT } = useSpring({
    clickT: clickTime,
    values: valuesMemo,
    config: {
      mass: 1,
      tension: 10,
      friction: 50,
    },
  });
  return (
    <LinearCopy>
      <Persistence>
        <AnimatedMain clickT={clickT} time={time} values={values} />
      </Persistence>
    </LinearCopy>
  );
};

const Root = () => {
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
  void main () {
    gl_FragColor =
      mix(
        texture2D(t, uv),
        1.01 * texture2D(back, uv) - 0.01,
        0.6
      );
  }
      `,
  },
  main: {
    frag: GLSL`
precision highp float;
varying vec2 uv;

uniform vec2 resolution;
uniform float time;
uniform float s1, s2, s3, s4;

const float PI = ${Math.PI};
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
vec3 pal (float t) {
  return palette(
    t,
    vec3(0.8 - .5 * s3 * s3),
    vec3(.8),
    vec3(1.0),
    vec3(0.9)
  );
}
float run (vec2 init) {
  vec2 p = init;
  float a = 0.5 + 0.1 * s1;
  float b = -0.5;
  float c = 0.1 - s2;
  float d = 0.3;
  float e = 0.6 - 0.01 * time;
  float f = 2.;
  vec2 offset = init;
  float s = 32. - (1. - s3) * s4 * 30.;
  for (float iter = 0.; iter < 32.; iter += 1.) {
    if (iter > s) break;
    float x2 = p.x * p.x;
    float y2 = p.y * p.y;
    float xy = p.x * p.y;
    p = vec2(a * x2 + b * y2 + c * xy + d, f * xy + e) + offset;
    if (length(p) >= 2.0) {
      return iter / s;
    }
  }
  return 0.;
}
vec3 shade (vec2 uv) {
  float zoom = 2. + 8. * s3;
  vec2 init = 2. * (uv - .5) / zoom;
  init.x += 0.2;
  pR(init, -1.3);
  return pal(pow(run(init), .5));
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 uvRatio = 0.5 + (uv - 0.5) * ratio;
  vec3 c = vec3(0.);
  for (float x=-.5; x<=.5; x += 1.) {
    for (float y=-.5; y<=.5; y += 1.) {
      vec2 d = 0.5 * vec2(x,y) / resolution;
      vec2 p = uvRatio + d;
      c += shade(p);
    }
  }
  c /= 4.0;
  gl_FragColor = vec4(c, 1.0);
}
  `,
  },
});

ReactDOM.render(<Root />, document.getElementById("main"));
