import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";
import { Blur } from "../components/Blur";

export const n = 28;
export const title = "No, I'M Batman";

export const Shader = ({ time }) => {
  return (
    <LinearCopy>
      <Persistence
        persistence={0.5 + 0.48 * Math.min(1, time / 60)}
        time={time}
      >
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
    vec2 offset = vec2(0.);
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

const float PI = ${Math.PI};

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float t) {
  return palette(
    t - 0.3 + time / 60.,
    vec3(.5),
    vec3(.5),
    vec3(.3, 1.,.8),
    vec3(.8, .3, .7)
  );
}

float mandelbrot (vec2 init) {
  vec2 p = init;
  for (float iter = 0.; iter < 200.; iter += 1.) {
    p = vec2(
      abs(p.x * p.x - p.y * p.y),
      2. * p.x * p.y
    ) + init;
    if (length(p) >= 2.0) {
      return iter / 200.;
    }
  }
  return -1.;
}

void main() {
  float zooming = pow(smoothstep(0., 30., time), 1.5);
  float rotating = smoothstep(20., 30., time);
  float zoom = .4 + 21.6 * pow(fract(.2 * time * zooming), 2.);
  vec2 init = 2. * (uv - .5);
  init /= zoom;
  pR(init, min(40., time-20.) * time * rotating);
  init += vec2(0., 1.788);
  pR(init, -PI/2.);
  float f = mandelbrot(init);
  vec3 clr = color(mix(step(f, 1.), f, rotating));
  vec3 c = mix(clr, vec3(0.), step(f, -0.1));
  gl_FragColor = vec4(c, 1.0);
}
`,
  },
});
