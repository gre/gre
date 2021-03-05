import Head from "next/head";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 2;
export const title = "stripes";

export const gifSize = 400;
export const gifStart = 0;
export const gifEnd = 60;
export const gifFramePerSecond = 24;
export const gifSpeed = 1;

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

#define GIF
 
float shape (vec2 p, float d) {
  float t = 0.5 + time * 0.05;
  t = pow(t, 2.);
  t += d * 0.003 * pow(t, 1.1);
  p -= 0.5;
  p /= 2.0;
  p *= mat2(cos(t), -sin(t), sin(t), cos(t));
  p += 0.5;
  t *= 2.;
  vec2 c = p - vec2(.5+0.2*cos(t), .5-0.2*sin(t));
  p *= length(c);
  float m = mod(p.x * 5.* smoothstep(0., 5., t) + t * c.y, 1.0);
  return step(length(vec2(m, p.y) - .5), .42);
}

void main() {
  vec3 c = vec3(0.);
  for (float x=-.5; x<=.5; x += 1.) {
    for (float y=-.5; y<=.5; y += 1.) {
      vec2 uvP = uv;
      uvP += vec2(x, y) / 800.0;
      c += vec3(
        shape(uvP, -1.),
        shape(uvP, 0.),
        shape(uvP, 1.)
      );
    }
  }
  c /= 4.;
  #ifdef GIF
  c *= smoothstep(60., 59., time) * smoothstep(0., 1., time);
  #endif
  gl_FragColor = vec4(c, 1.0);
}`,
  },
});
