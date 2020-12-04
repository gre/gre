import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 36;
export const title = "crois en moi";

export const Shader = ({ time }) => (
  <LinearCopy>
    <Persistence>
      <Node shader={shaders.node} uniforms={{ time }} />
    </Persistence>
  </LinearCopy>
);

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
      .6 + .5 * length(uv-.5)
    );
  }
      `,
  },
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
#define PI ${Math.PI}

// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
vec3 color (float t) {
  return palette(
    t,
    vec3(.5),
    vec3(.5),
    vec3(.9, 1., .7),
    vec3(.1, .6, .3)
  );
}

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

float cell (vec2 p) {
  return mod(p.x * p.x + 6. * p.y, 4.);
}

void main() {
  vec2 offset = vec2(0., -time * 3.);
  vec2 center = uv - .5;
  float a = atan(center.y, center.x);
  float l = pow(length(center), .1);
  vec2 p = vec2(a * .9, l * 40.) + offset;
  pR(p, PI/4.);
  float n = .01 * time + .1 * pow(abs(sin(time)), 4.);
  vec3 c = color(2. + .1 * time + n * cell(floor(p)));
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
