

import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 35;
export const title = "aie confiance";

export const Shader = ({ time }) => (
  <LinearCopy>
    <Persistence persistence={0.6}>
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
#define PI ${Math.PI}

// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
vec3 color (float t, float l) {
  return palette(
    t,
    vec3(.5),
    vec3(.5),
    vec3(.3, .5, .7),
    vec3(.5 * (time - .4 * l), .5, .3)
  );
}

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

float cell (vec2 p) {
  return mod(p.x * p.y, 3.);
}

void main() {
  vec2 offset = time * vec2(.5, -2. - .01 * time);
  vec2 center = uv - .5;
  float a = atan(center.y, center.x);
  float l = pow(length(center), 1. / (4. + min(50., .2 * time)));
  vec2 p = vec2(a * 2.025, l * 40.) + offset;
  pR(p, PI/4.);
  vec3 c = color(cell(floor(p)), l);
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
