import React from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import { useControls } from "leva";

export const n = 63;
export const title = "Relics";

const speed = 20;
const loopMul = 60;
const loop = loopMul * speed;

export const exportSize = 400;
export const exportStart = 0;
export const exportEnd = loopMul;
export const exportFramePerSecond = speed;
export const exportSpeed = 1;
export const exportMP4vb = "5M";

const defaults = {
  colors: [0.7, 0.55, 0.4],
  logicSize: 80,
  amp: 1.0,
  base: 2.0,
  persistance: 0.89,
  delta: 0.5,
  seed: 0.0,
};

export const Logic = ({
  freq,
  g,
  size,
  seed,
  amp,
  base,
  persistance,
  delta,
}) => {
  return (
    <Node
      key={size}
      width={size}
      height={size}
      shader={shaders.logic}
      uniforms={{
        amp,
        base,
        freq,
        g,
        prev: Uniform.Backbuffer,
        size,
        seed,
        persistance,
        delta,
      }}
      backbuffering
    />
  );
};

const LogicMemo = React.memo(Logic);

const Main = ({
  g,
  colors,
  amp,
  base,
  persistance,
  delta,
  logicSize,
  seed,
}) => {
  return (
    <Node
      shader={shaders.render}
      uniforms={{
        colors,
        logic: (
          <LogicMemo
            amp={amp}
            base={base}
            g={g}
            size={logicSize}
            seed={seed}
            persistance={persistance}
            delta={delta}
          />
        ),
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

export const Shader = ({ time }) => {
  const {
    colors,
    logicSize,
    amp,
    base,
    persistance,
    delta,
    seed,
  } = useControls(defaults);

  return (
    <MainMemo
      amp={amp}
      base={base}
      g={Math.floor(time * speed) % loop}
      colors={colors}
      persistance={persistance}
      delta={delta}
      seed={seed}
      logicSize={logicSize}
    />
  );
};

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
  logic: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform sampler2D prev;
uniform float g;
uniform float size;
uniform float seed;
uniform float amp;
uniform float base;
uniform float persistance;
uniform float delta;
#define PI ${Math.PI}

${PERLIN_NOISE}

void main() {
  vec4 vcenter = texture2D(prev, uv);
  // we retrieve previous values of cell and sibling cells
  float dp = 1. / size;
  vec4 vright = texture2D(prev, uv+vec2(dp,0.0));
  vec4 vleft = texture2D(prev, uv+vec2(-dp,0.0));
  vec4 vtop = texture2D(prev, uv+vec2(0.0,dp));
  vec4 vbottom = texture2D(prev, uv+vec2(0.0,-dp));
  vec4 vtopright = texture2D(prev, uv+vec2(dp,dp));
  vec4 vtopleft = texture2D(prev, uv+vec2(-dp,dp));
  vec4 vbottomleft = texture2D(prev, uv+vec2(-dp,-dp));
  vec4 vbottomright = texture2D(prev, uv+vec2(dp,dp));
  vec3 c = vcenter.rgb;
  // we use perlin noise to modulate terrain with different harmonies
  float freq = amp * (base + 1.5 * cos(0.2 * PI * g) + 0.033 * g);
  vec2 disp = vec2(5.2342 * seed, 6.417 - 0.18 * g);
  float n = cnoise(disp + uv * freq);
  n += 0.05;
  float l = length(uv - .5);
  n *= smoothstep(0.4, 0.3, l);
  float rs = vleft.r + vright.r + vbottom.r + vtop.r + vbottomleft.r + vbottomright.r + vtopleft.r + vtopright.r;
  c.r = mix(
    mix(n, c.r, persistance),
    rs / 8.0,
    delta + cos(0.1 * PI * g)
  );
  gl_FragColor = vec4(c, 1.0);
}
    `,
  },
  render: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform sampler2D logic;
uniform vec3 colors;
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
vec3 color (vec3 state) {
  return palette(
    state.r,
    vec3(.5),
    vec3(.5),
    vec3(1.),
    colors
  );
}
void main() {
  vec3 state = texture2D(logic, uv).rgb;
  gl_FragColor = vec4(color(state), 1.0);
}
`,
  },
});
