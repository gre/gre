import { Shaders, Node, GLSL } from "gl-react";

export const n = 61;
export const title = "Binance gradients";
export const exportEnd = 1;
export const exportFramePerSecond = 30;
export const preload = ["/images/shaders/binance.dist.png"];

export const Shader = ({ time }) => (
  <Node
    shader={shaders.node}
    uniforms={{ time, image: "/images/shaders/binance.dist.png" }}
  />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
uniform sampler2D image;

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float t) {
  return palette(
    t,
    vec3(1.0, 1.0, .5),
    vec3(.5),
    vec3(1.),
    vec3(0.2 + 0.1 * cos(0.3 * time), 0.3 + 0.1 * sin(0.2 * time), 0.5 + 0.2 * cos(0.1 * time))
  );
}

void main() {
  float v = texture2D(image, uv).a - 0.5;
  gl_FragColor = vec4(
    step(v, 0.) * color(uv.y + 2. * time) +
    step(0., v) * color(sqrt(max(v, 0.)) - time),
    1.0);
}
`,
  },
});
