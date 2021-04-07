

import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";
import { Blur } from "./Blur";

export const n = 27;
export const title = "real burning ship";

export const exportSize = 400;
export const exportStart = 0;
export const exportEnd = 60;
export const exportFramePerSecond = 12;
export const exportSpeed = 2;

const GIF = 0;

export const Shader = ({ time }) => {
  return (
    <LinearCopy>
      <Persistence persistence={0.8 + 0.05 * Math.random()} time={time}>
        <Node shader={shaders.node} uniforms={{ time }} />
      </Persistence>
    </LinearCopy>
  );
};

const Persistence = ({ children: t, persistence, time }) => (
  <Node
    shader={shaders.persistence}
    backbuffering
    uniforms={{ t, back: Uniform.Backbuffer, persistence, time }}
  />
);

const shaders = Shaders.create({
  persistence: {
    frag: GLSL`
  precision highp float;
  varying vec2 uv;
  uniform sampler2D t, back;
  uniform float persistence;
  uniform float time;
  void main () {
    vec2 offset = vec2(0.002 * (cos(2. * time) + 0.4 * sin(5. * time)), -0.006);
    gl_FragColor = mix(
      texture2D(t, uv),
      texture2D(back, uv + offset),
      persistence
    );
  }
      `,
  },
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

float tt = time;

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float t) {
  return palette(
    t,
    vec3(.5),
    vec3(.5),
    vec3(1.),
    vec3(.7, .2 + min(.1, .01 * tt), .2)
  );
}

float mandelbrot (vec2 init) {
  vec2 p = init;
  for (float iter = 0.; iter < 400.; iter += 1.) {
    p = vec2(
      p.x * p.x - p.y * p.y,
      2. * abs(p.x * p.y) + 0.0003 * tt
    ) + init;
    if (length(p) >= 2.0) {
      return iter / 400.;
    }
  }
  return -1.;
}

void main() {
  float zoom = 20.;
  vec2 init = 2. * (uv - .5) / zoom;
  init.x *= -1.;
  pR(init, -3.14);
  init += vec2(-1.76, -.03 - 0.0003 * tt);
  float f = mandelbrot(init);
  vec3 clr = color(f); // vec3(1.-pow(f, 0.5)))
  vec3 c = mix(clr, vec3(0.), step(f, -0.1));

  #if ${GIF}
  c *= smoothstep(59., 58., time);
  #endif
  gl_FragColor = vec4(c, 1.0);
}
`,
  },
});
