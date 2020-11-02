import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 98;
export const title = "cosmos";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL/* glsl */ `
precision highp float;
varying vec2 uv;
uniform float time;

// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

void main() {
  float t = time * smoothstep(0., 8., time);
  float y = 0.4 + 0.8 * uv.y + 0.1 * cos(uv.x + t) + 0.05 * sin(uv.x * 0.1 + 0.2 * t);
  vec3 bg = palette(
    y,
    vec3(0.6, 0.5, 0.5),
    vec3(0.5, 0.4, 0.5),
    vec3(0.8, 1.0, 1.0),
    vec3(0.55+0.1 * sin(0.1 * t), 0.28, 0.2)
  );
  float curve = -.7 + y + 0.1*(sin(t) + 0.5*smoothstep(4., 14., t) * fract((5.*sin((t + 2. * uv.x)) * 04.)))*cos((pow(t*0.2 + 0.1 * cos(t), 1.2) + sin(pow(t*0.1, 0.2)) * uv.x) * 20.0);
  vec3 c = mix(
    bg,
    vec3(1.),
    step(0., curve) * step(curve, 0.01)
  );
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
