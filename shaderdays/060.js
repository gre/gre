import { Shaders, Node, GLSL } from "gl-react";

export const n = 60;
export const title = "Bitcoin rainbow";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (float t) {
  return palette(
    t,
    vec3(.5),
    vec3(.6 + .1 * cos(.3 * time)),
    vec3(1.),
    vec3(0.22, 0.5, 0.77)
  );
}

const float PI = ${Math.PI};

float sdSegment (in vec3 p, in float L, in float R) {
  p.y -= min(L, max(0.0, p.y));
  return length(p) - R;
}
float sdBox( vec3 p, vec3 b ) {
  vec3 q = abs(p) - b;
  return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}
float sdBox (vec2 p, vec2 sz) {
  return max(abs(p.x) - sz.x, abs(p.y) - sz.y);
}
float sdD (vec2 p, float w, float h) {
  return min(sdBox(p, vec2(w, h)), length(p-vec2(w, .0))-h);
}
float sdUpperD (vec2 p) {
  p.x += .02;
  p.y -= .1;
  float inner = sdD(p + vec2(-0.025, 0.012), 0.037, 0.055);
  float outer = sdD(p, 0.1, 0.1);
  return max(-inner, outer);
}
float sdLowerD (vec2 p) {
  p.x += .01;
  p.y += .085;
  float outer = sdD(p, 0.11, 0.11);
  float inner = sdD(p - vec2(0.023, 0.01), 0.045, 0.058);
  return max(-inner, outer);
}
float sdRevCornerRadius(vec2 p) {
  return max(
    sdBox(p, vec2(.5)),
    -min(
      (p.x - p.y) / 2.,
      length(p + vec2(.5, -.5)) - 1.
    )
  );
}
float sdBitcoin2D (vec2 p) {
  float bottom = sdLowerD(p);
  bottom = min(bottom, max(
    sdBox(p + vec2(.15, .165), vec2(.04, .03)), // bottom-left shape
    -(p.x - .216 * p.y + 0.142)) // 12.5° cut
  );
  bottom = min(bottom, sdRevCornerRadius((p + vec2(0.135, -0.135)) * vec2(1., -1.) * 30.));
  float top = sdUpperD(p);
  top = min(top, sdBox(p - vec2(-.15, .175), vec2(.034, .025)));
  top = min(top, sdRevCornerRadius((p + vec2(0.135, 0.12)) * vec2(1., 1.) * 30.));
  p.x += .01;
  float hash = max(
    sdBox(p, vec2(0.07, .285)),
    -min(
      sdBox(p, vec2(0.022, 1.)),
      sdBox(p, vec2(1., .15))
    )
  );
  return min(min(top, bottom), hash);
}

void main() {
  float v = sdBitcoin2D(uv-.5);
  v = max(-step(fract(time), 0.5), v); // blink 500ms
  gl_FragColor = vec4(
    step(v, 0.) * color(1.6 * (uv.y + time)) +
    step(0., v) * color(sqrt(max(v, 0.)) - 2. * time),
    1.0);
}
`,
  },
});
