

import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 48;
export const title = "Starry Night";

export const Shader = ({ time }) => (
  <Node
    shader={shaders.node}
    uniforms={{ time, img: "/images/seamless-wood2.jpg" }}
  />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
uniform sampler2D img;

#define PI ${Math.PI}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
vec3 color (float t) {
  return palette(
    t,
    vec3(.5),
    vec3(.5),
    vec3(1.),
    vec3(
      .68 + .03 * cos(2. + .8 * time),
      .75 + .05 * sin(.6 * time),
      .25 + .07 * cos(.5 * time)
    )
  );
}

void main() {
  vec2 p = uv - .5;
  float t = pow(time * .2, 1.4);
  float a = (atan(p.y, p.x)/PI+1.)/2.;
  float b = pow(fract((3. + 2. * cos(.5 * t)) * length(p)- t), 2.);
  vec3 clr = mix(
    color(texture2D(img, vec2(a, b)).r),
    color(.5),
    smoothstep(.09, .08, length(p) + .01 * cos(time + a * 6. * PI + sin(a * 8. * PI - time) - cos(a * 10. * PI + pow(time-10., 1.4))))
  );
  gl_FragColor = vec4(clr, 1.0);
}`,
  },
});
