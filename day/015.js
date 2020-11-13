import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, Uniform, LinearCopy } from "gl-react";
import { Blur } from "../components/Blur";
// Kudos to 0xB0nnaz for the idea and the formula

export const n = 15;
export const title = "parametric";

export const Shader = ({ time }) => (
  <LinearCopy>
    <Persistence
      persistence={
        0.98 + // base persistence
        0.02 * Math.cos(time) + // cycle between "clean" phases and "more persistence" phases
        -0.8 * Math.exp(-time) // increase over time
      }
    >
      <Blur passes={4} factor={0.2}>
        <Effect time={time} />
      </Blur>
    </Persistence>
  </LinearCopy>
);

const Effect = ({ time }) => <Node shader={shaders.node} uniforms={{ time }} />;

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
  gl_FragColor =
    texture2D(t, uv) + persistence*texture2D(back, uv);
}
    `,
  },
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

const float PI = ${Math.PI};

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec2 parametric (in float t) {
  return vec2(
    sin(PI * t) + 0.8 * sin(4. * PI * t) + sin(128. * PI * t) * 0.5,
    cos(PI * t) + 0.8 * cos(4. * PI * t) + cos(128. * PI * t) * 0.5
  );
}

vec3 color (float t) {
  return palette(
    t,
    vec3(.4),
    vec3(.4),
    vec3(1.),
    vec3(.0, .33, .66)
  );
}

void main() {
  // this implement some variations of scale/speed/size to make it more trippy
  float base = mod(time, 30.);
  vec2 p = mix(2., 6., smoothstep(0., 10., base)) * (uv - .5);
  float speed = smoothstep(0., 5., base) * 3.;
  float size = 0.01 + 0.1 * smoothstep(8., 0., base);
  // interpolation of parametric function
  vec3 clr = vec3(0.);
  for (float f = 0.; f<1.; f+=1./500.) {
    float t = time + f/60.;
    vec2 c = parametric(speed * t);
    float m = smoothstep(1.1 * size, size, length(p - c));
    if (m > .0) {
      clr = m * color(t);
      break;
    }
  }
  gl_FragColor = vec4(clr, 1.0);
}
`,
  },
});
