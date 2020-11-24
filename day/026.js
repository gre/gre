import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";
import { Blur } from "../components/Blur";

export const n = 26;
export const title = "fumes";

export const Shader = ({ time }) => {
  return (
    <LinearCopy>
      <Persistence persistence={0.9}>
        <Blur passes={4} factor={Math.min(0.6, 0.01 * time)}>
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

const float PI = ${Math.PI};

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float t) {
  float ti = 2. * time + 60.;
  return palette(
    t,
    vec3(.5),
    vec3(.5),
    vec3(0.5, .5 + .5 * cos(.01 * ti), 1.),
    vec3(.8 + .2 * sin(-.07 * ti), .1 + .05 * cos(.02 * ti), .1 + .1 * sin(.3 + .03 * ti))
  );
}

float gre1 (vec2 init, float t) {
  vec2 p = init;
  for (float iter = 0.; iter < 200.; iter += 1.) {
    p = vec2(
      (1. + cos(t)) * p.x * p.x - 2. *  p.y * p.y + .1 * cos(.1 * t),
      (3. + sin(t)) * p.x * p.y - .2 * cos(.3 * t + p.y)
    ) + init;
    if (length(p) >= 2.0) {
      return iter / 200.;
    }
  }
  return 0.;
}

void main() {
  float t = .1 * time;
  float zoom = 1. + .2 * t;
  vec2 init = 2. * (uv - .5) / zoom;
  pR(init, -PI/2. + .05 * time);
  init -= vec2(.8, .0);
  gl_FragColor = vec4(color(pow(gre1(init, t), .5)), 1.0);
}
`,
  },
});
