import React from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";

export const n = 62;
export const title = "Overgrowth";
export const exportEnd = 20;
export const exportFramePerSecond = 40;
export const exportSpeed = 1;

export const Logic = ({ generation, size, n }) => {
  return (
    <Node
      width={size}
      height={size}
      shader={shaders.logic}
      uniforms={{ generation, prev: Uniform.Backbuffer, size, seed: n / 100 }}
      backbuffering
    />
  );
};

const LogicMemo = React.memo(Logic);

const Main = ({ time, generation, n }) => {
  const logicSize = 140;
  return (
    <Node
      shader={shaders.render}
      uniforms={{
        time,
        logic: <LogicMemo generation={generation} size={logicSize} n={n} />,
      }}
      uniformsOptions={{
        logic: {
          interpolation: "nearest",
        },
      }}
    />
  );
};

const MainMemo = React.memo(Main);

const speed = 30;
const seed = 1000 * Math.random();

export const Shader = ({ time, n }) => (
  <MainMemo
    key={n}
    n={n + seed}
    time={time}
    generation={Math.floor(time * speed)}
  />
);

const PERLIN_NOISE =
  // https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83
  `
vec4 permute(vec4 x){return mod(((x*34.0)+1.0)*x, 289.0);}
vec2 fade(vec2 t) {return t*t*t*(t*(t*6.0-15.0)+10.0);}
float cnoise(vec2 P){
  vec4 Pi = floor(P.xyxy) + vec4(0.0, 0.0, 1.0, 1.0);
  vec4 Pf = fract(P.xyxy) - vec4(0.0, 0.0, 1.0, 1.0);
  Pi = mod(Pi, 289.0); // To avoid truncation effects in permutation
  vec4 ix = Pi.xzxz;
  vec4 iy = Pi.yyww;
  vec4 fx = Pf.xzxz;
  vec4 fy = Pf.yyww;
  vec4 i = permute(permute(ix) + iy);
  vec4 gx = 2.0 * fract(i * 0.0243902439) - 1.0; // 1/41 = 0.024...
  vec4 gy = abs(gx) - 0.5;
  vec4 tx = floor(gx + 0.5);
  gx = gx - tx;
  vec2 g00 = vec2(gx.x,gy.x);
  vec2 g10 = vec2(gx.y,gy.y);
  vec2 g01 = vec2(gx.z,gy.z);
  vec2 g11 = vec2(gx.w,gy.w);
  vec4 norm = 1.79284291400159 - 0.85373472095314 * 
    vec4(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
  g00 *= norm.x;
  g01 *= norm.y;
  g10 *= norm.z;
  g11 *= norm.w;
  float n00 = dot(g00, vec2(fx.x, fy.x));
  float n10 = dot(g10, vec2(fx.y, fy.y));
  float n01 = dot(g01, vec2(fx.z, fy.z));
  float n11 = dot(g11, vec2(fx.w, fy.w));
  vec2 fade_xy = fade(Pf.xy);
  vec2 n_x = mix(vec2(n00, n01), vec2(n10, n11), fade_xy.x);
  float n_xy = mix(n_x.x, n_x.y, fade_xy.y);
  return 2.3 * n_xy;
}
`;

const shaders = Shaders.create({
  // RGB component of the cellular automaton:
  // r: red as in the ground earth. if >0.1 this is the solid. we store value to store how solid it is. (0.0 is fagile and will be destroyed)
  // g: "grass". all the vegetation. we use floating value to store the progress too
  // b: negative grow factor
  // a: not used
  logic: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float generation;
uniform float size;
uniform sampler2D prev;
uniform float seed;

${PERLIN_NOISE}

void main() {
  if (generation == 0.0) {
    gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    return;
  }
  float dp = 1. / size;
  // we retrieve previous values of cell and sibling cells
  vec4 vcenter = texture2D(prev, uv);
  vec4 vright = texture2D(prev, uv+vec2(dp,0.0));
  vec4 vleft = texture2D(prev, uv+vec2(-dp,0.0));
  vec4 vtop = texture2D(prev, uv+vec2(0.0,dp));
  vec4 vbottom = texture2D(prev, uv+vec2(0.0,-dp));
  vec4 vtopright = texture2D(prev, uv+vec2(dp,dp));
  vec4 vtopleft = texture2D(prev, uv+vec2(-dp,dp));
  vec4 vbottomleft = texture2D(prev, uv+vec2(-dp,-dp));
  vec3 c = vcenter.rgb;
  vec4 vbottomright = texture2D(prev, uv+vec2(dp,dp));
  float gen = smoothstep(0.0, 50.0, generation);
  if (gen < 1.0) {
    // TERRAIN BUILDING
    // we use perlin noise to modulate terrain with different harmonies
    float freq = 0.5 + 3.0 * gen + fract(seed);
    vec2 disp = vec2(6. * gen + seed, -40. * gen + 50. * fract(seed + 0.4));
    float n = cnoise(disp + uv * freq);
    n += 0.1;
    n *= 1.2 - uv.y; // top part have less ground
    // average ground increase ground propagation
    float rs = vleft.r + vright.r + vbottom.r + vtop.r;
    // accumulate over each step
    c.r = mix(
      mix(n, c.r, 0.8 + 0.2 * gen),
      rs / 4.0,
      0.2 * gen
    );
  }
  else {
    // GROWING EVOLUTION
    float n1 = cnoise(0.233 * uv + vec2(0.109 * generation, 0.0));
    float n2 = cnoise(6.11 * uv + vec2(0.0, 0.2111 * generation));
    float n3 = cnoise(13.82 * uv + vec2(0.0, 0.0211 * generation));
    float n4 = cnoise(55.31 * uv + n1 + n2 + n3 + 0.31 * generation);
    // sprount from ground
    float threshold = 0.14;
    c.g += 0.1 * 
      step(threshold, vbottom.r) *
      step(c.r, threshold) *
      max(0.0, n1);
    // grows up
    c.g += 0.2 * vbottom.g * step(c.b, 0.0) * step(c.r, threshold) * max(0.0, n1 + n2);
    // sometimes can spread from edges
    float edgespread =
    step(0.6, vright.g + vleft.g + vbottomleft.g + vbottomright.g)
      * step(c.r, 0.1)
      * smoothstep(-0.6, -0.7, n4);
    c.g += 0.2 * edgespread * max(0.5, 2. * n3);
    c.b += edgespread;
    c.b -= 0.01;
    // environment constraints
    c.g *= 0.98;
    c.g -= 0.01 *
      step(0.0, vtop.g) *
      step(0.0, c.g) *
      smoothstep(0.8, 1.0, n3 + n2);
  }
  gl_FragColor = vec4(c, 1.0);
}
    `,
  },
  render: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;
uniform sampler2D logic;

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec3 color (vec3 state) {
  float ground = smoothstep(0.1, 0.15, state.r);
  float grass = state.g;
  float phase = cos(0.5 * time);
  return mix(
    palette(
      0.4 + 0.2 * grass + 0.2 * phase,
      vec3(0.0, 1.0, 0.0),
      vec3(.5),
      vec3(1.),
      vec3(0.4, 0.3, 0.1)
    ),
    palette(
      0.1 * phase + 0.6 - 0.4 * ground,
      vec3(.5),
      vec3(.5),
      vec3(1.),
      vec3(0.7, 0.55, 0.4)
    ),
    step(grass, 0.0)
  );
}

void main() {
  vec3 state = texture2D(logic, uv).rgb;
  gl_FragColor = vec4(color(state), 1.0);
}
`,
  },
});
