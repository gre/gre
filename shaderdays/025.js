

import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";
import { Blur } from "./Blur";

export const n = 25;
export const title = "Mandelbrot";

export const Shader = ({ time }) => {
  return (
    <LinearCopy>
      <Persistence persistence={0.9}>
        <Blur passes={4} factor={0.1}>
          <Node shader={shaders.node} uniforms={{ time }} />
        </Blur>
      </Persistence>
    </LinearCopy>
  );
};

const Persistence = ({ children: t, persistence }) => (
  <Node
    shader={shaders.persistence}
    backbuffering
    uniforms={{ t, back: Uniform.Backbuffer, persistence }}
  />
);

const shaders = Shaders.create({
  persistence: {
    frag: GLSL`
  precision highp float;
  varying vec2 uv;
  uniform sampler2D t, back;
  uniform float persistence;
  void main () {
    gl_FragColor = mix(
      texture2D(t, uv),
      texture2D(back, uv),
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

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float t) {
  float ti = 2. * time + 10.;
  return palette(
    t,
    vec3(.5),
    vec3(.5),
    vec3(.7, .5 + .1 * cos(.01 * ti), .5),
    vec3(.5 + .2 * sin(-.07 * ti), .6 + .1 * cos(.01 * ti), .7 + .1 * sin(.3 + .03 * ti))
  );
}

float mandelbrot (vec2 init) {
  vec2 p = init;
  for (float iter = 0.; iter < 400.; iter += 1.) {
    p = vec2(p.x * p.x - p.y * p.y, 2. * p.x * p.y) + init;
    if (length(p) >= 2.0) {
      return iter / 400.;
    }
  }
  return 1.;
}

void main() {
  float acc = pow(smoothstep(0., 30., time), 1.4); // 0->1 for first 30s
  float zoom = .5 + .2 * pow(time, 1.8) * acc; // zoom in
  vec2 init = 2. * (uv - .5) / zoom;
  pR(init, .1 * time * (1. - acc)); // rotate a bit on the center
  init += vec2(.335, .388); // offset to center
  gl_FragColor = vec4(color(mandelbrot(init)), 1.0);
}
`,
  },
});
