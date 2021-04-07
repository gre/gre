
import { Shaders, Node, GLSL, LinearCopy, NearestCopy } from "gl-react";
import { GameOfLife } from "./GameOfLife";

export const n = 4;
export const title = "Ray of Life";

/**
 * Ray of Life
 * polar projection of Game of Life
 * Reusing the shader from https://gl-react-cookbook.surge.sh/gol
 */

let firstTime;
export const Shader = ({ time }) => {
  if (!firstTime) {
    firstTime = time;
  }
  const t = time - firstTime;
  const refreshEveryTicks = 50;
  const tick = Math.floor(t * 10);
  const tick2 = tick + refreshEveryTicks / 2;
  const size = 10 * (1 + Math.floor(tick / refreshEveryTicks));
  const size2 = 10 * (1 + Math.floor(tick2 / refreshEveryTicks));
  return (
    <Main
      time={time}
      a={
        <NearestCopy>
          <GameOfLife
            refreshEveryTicks={refreshEveryTicks}
            tick={tick}
            size={size}
          />
        </NearestCopy>
      }
      b={
        <NearestCopy>
          <GameOfLife
            refreshEveryTicks={refreshEveryTicks}
            tick={tick2}
            size={size2}
          />
        </NearestCopy>
      }
    />
  );
};

const Main = ({ a, b, time }) => (
  <Node uniforms={{ a, b, time }} shader={shaders.main} />
);

const shaders = Shaders.create({
  main: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
uniform sampler2D a, b;

const float PI = ${Math.PI};

// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

void main() {
  float t = mod(time * 0.2, 2.);
  vec2 p = uv - .5;
  float x = 0.5 + 0.5 * atan(p.y, p.x) / PI;
  float y = mod((sqrt(1.6 * length(p)) - 2. * time / 5.), 2.);
  x = mod(x + 0.1 * y, 1.);
  float y1 = fract(y);
  float y2 = fract(y - 1.);
  float sa = step(1., y);
  float sb = step(y, 1.);
  float wallA = texture2D(a, vec2(x, y1)).r;
  float wallB = texture2D(b, vec2(x, y2)).r;
  float fade = smoothstep(.01, .011, length(p));
  vec3 c =
    palette(
      .5 +
      fade * (
      0.02 * step(fract(20. * x), 0.5) +
      .1 * sa * wallA -
      .1 * sb * wallB ),
      vec3(.8),
      vec3(.5),
      vec3(.3, .9, .9),
      vec3(0.8, 0.3, 0.2)
    );
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
