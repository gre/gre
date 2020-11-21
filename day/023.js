import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 23;
export const title = "Hexacubes";

export const Shader = ({ time }) => {
  return <Node shader={shaders.node} uniforms={{ time }} />;
};

const shaders = Shaders.create({
  node: {
    frag: GLSL`
  precision highp float;
  varying vec2 uv;
  uniform float time;
  const float PI = ${Math.PI};
  // https://iquilezles.org/www/articles/palettes/palettes.htm
  vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
    return a + b*cos( 6.28318*(c*t+d) );
  }
  // from http://glslsandbox.com/e#43182.0 / 007 example
  #define SQRT3 1.7320508
  const vec2 s = vec2(1.0, SQRT3);
  float hex(in vec2 p){
    p = abs(p);
    return max(dot(p, s*.5), p.x);
  }
  vec4 getHex(vec2 p) {
    vec4 hC = floor(vec4(p, p - vec2(.5, 1))/s.xyxy) + .5;
    vec4 h = vec4(p - hC.xy*s, p - (hC.zw + .5)*s);
    return dot(h.xy, h.xy)<dot(h.zw, h.zw) ? vec4(h.xy, hC.xy) : vec4(h.zw, hC.zw + 9.73);
  }
  // utilities from classical SDF
  float pModPolar(inout vec2 p, float repetitions) {
    float angle = 2.*PI/repetitions;
    float a = atan(p.y, p.x) + angle/2.;
    float r = length(p);
    float c = floor(a/angle);
    a = mod(a,angle) - angle/2.;
    p = vec2(cos(a), sin(a)) * r;
    if (abs(c) >= (repetitions/2.)) c = abs(c);
    return c;
  }
  void pR(inout vec2 p, float a) {
    p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
  }
    //////
  vec3 color (float t) {
    return palette(
      t,
      vec3(.5),
      vec3(.5),
      vec3(1.),
      vec3(.9, .1, .2)
    );
  }

  float tile (vec2 p, vec2 g, vec2 g2) {
    pR(p, PI / 6.);
    float r1 = pModPolar(p, 3.);
    p.x -= 1./3.;
    float change = smoothstep(-.5, 1., cos(time));
    pR(p, change * PI / 3.);
    float r2 = 1. + pModPolar(p, 3.);
    float index = mod(r1 + r2, 3.);
    return index;
  }

  void main() {
    vec2 p = uv - .5;
    p.x += .1 * time;
    vec2 g = p * 2.;
    vec4 r = getHex(g);
    vec4 r2 = getHex(g + vec2(-.2 * time, .4 * time));
    float i = tile(r.xy, r.zw, r2.zw);
    float sz = 0.03;
    pR(g, PI/3.);
    float m = smoothstep(.48, .52, mod(g.x, sz) / sz);
    vec3 c = color(
      .5 * smoothstep(.9, .95, mod(.4 * time, 1.)) +
      i * .1 +
      0.02 * m +
      -0.03 * (r2.z - r2.w));
    gl_FragColor = vec4(c, 1.0);
  }
      `,
  },
});
