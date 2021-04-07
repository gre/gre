import { useMemo } from "react";

import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";
import MersenneTwister from "mersenne-twister";

const duration = 10;

export const exportSize = 400;
export const exportStart = 0;
export const exportEnd = 3 * duration;
export const exportFramePerSecond = 20;
export const exportSpeed = 1;
export const exportMP4vb = "5M";

export const n = 58;
export const title = "Mandelglitch";

export const nfts = [
  { url: "https://ethblock.art/create/17", text: "Mint on ethblock.art" },
  {
    url:
      "https://ghostmarket.io/asset/pha/ghost/3008841254969814369262311336331954453497120774334547905246474374493804042898/",
    text: "ghostmarket.io: 400 KCAL",
  },
  {
    url: "https://www.hicetnunc.xyz/objkt/3063",
    text: "hicetnunc.xyz: Mandelglitch #1 (5 XTZ)",
  },
  {
    url: "https://www.hicetnunc.xyz/objkt/3068",
    text: "hicetnunc.xyz: Mandelglitch #2 (5 XTZ)",
  },
  {
    url: "https://www.hicetnunc.xyz/objkt/3071",
    text: "hicetnunc.xyz: Mandelglitch #3 (5 XTZ)",
  },
  {
    url: "https://www.hicetnunc.xyz/objkt/3072",
    text: "hicetnunc.xyz: Mandelglitch #4 (5 XTZ)",
  },
  {
    url: "https://www.hicetnunc.xyz/objkt/3073",
    text: "hicetnunc.xyz: Mandelglitch #5 (5 XTZ)",
  },
  {
    url: "https://www.hicetnunc.xyz/objkt/3077",
    text: "hicetnunc.xyz: Mandelglitch #6 (5 XTZ)",
  },
  {
    url: "https://www.hicetnunc.xyz/objkt/3088",
    text: "hicetnunc.xyz: Mandelglitch #7 (5 XTZ)",
  },
  {
    url: "https://www.hicetnunc.xyz/objkt/3574",
    text: "hicetnunc.xyz: Mandelglitch #8 (5 XTZ)",
  },
];

function interp(a, b, x) {
  return a.map((v, i) => v * (1 - x) + b[i] * x);
}

const easeInOutQuad = (t) => (t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t);

function calc(exporting, n = 0) {
  const seeds = [];
  const length = exporting ? 3 : 50;
  const rng = new MersenneTwister(n);
  const values = Array(length)
    .fill(null)
    .map(() => {
      return Array(12)
        .fill(null)
        .map(() => rng.random());
    });
  return values;
}

export const Shader = ({ time, n, exporting }) => {
  const values = useMemo(() => calc(exporting, n), [n, exporting]);

  const t = (time / duration) % values.length;
  const index = Math.floor(t);
  const progress = t - index;
  let [travel, love, dark, s1, s2, s3, s4, s5, s6, s7, s8, s9] = interp(
    values[index % values.length],
    values[(index + 1) % values.length],
    easeInOutQuad(progress)
  );

  dark = easeInOutQuad(dark);

  return (
    <LinearCopy>
      <Persistence persistence={exporting ? 0 : 0.8}>
        <Node
          shader={shaders.main}
          uniforms={{
            aa: exporting ? 2 : 0,
            time,
            resolution: Uniform.Resolution,
            travel,
            love,
            dark,
            rot: exporting ? (2 * Math.PI) / exportEnd : 0.1,
            s1,
            s2,
            s3,
            s4,
            s5,
            s6,
            s7,
            s8,
            s9,
          }}
        />
      </Persistence>
    </LinearCopy>
  );
};

const shaders = Shaders.create({
  persistence: {
    frag: GLSL`
  precision highp float;
  varying vec2 uv;
  uniform sampler2D t, back;
  uniform float persistence;
  void main () {
    gl_FragColor =
      mix(
        texture2D(t, uv),
        texture2D(back, uv),
        persistence
      );
  }
      `,
  },
  main: {
    frag: GLSL`
  #version 300 es
precision highp float;
in vec2 uv;
out vec4 color;

uniform vec2 resolution;
uniform float aa;
uniform float time;
uniform float rot;
uniform float love, travel, dark;
uniform float s1, s2, s3, s4, s5, s6, s7, s8, s9;

const float PI = ${Math.PI};
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
vec3 pal (float t) {
  return palette(
    t + 0.5 * dark * dark,
    vec3(.85 - .5 * dark),
    vec3(.5),
    vec3(1.),
    vec3(0.8 + 0.2 * s1, 0.2 * s2, .2)
  );
}
float run (vec2 init) {
  float iterations = 200. + 300. * love;
  vec2 p = init;
  for (float iter = 0.; iter < iterations; iter += 1.) {
    // original mandelbrot formula is:
    // p = vec2( p.x * p.x - p.y * p.y, 2. * p.x * p.y) + init;
    float x2 = p.x * p.x;
    float y2 = p.y * p.y;
    float xy = p.x * p.y;
    float a = 1. + .1 * (s1 - 0.5) * s2;
    float b = -1. + .1 * (s1 - 0.5) * s2;
    float c = 0.0 + 2. * (s2 - 0.5) * s3;
    float d = max(0., pow(s8, 2.) - 0.5) * cos(100. * s7 * s2 * s9 * p.y);
    float e = max(0., pow(s9, 2.) - 0.5) * sin(100. * s2 * s1 * p.x);
    float f = 2. + s6 - s6 * s6 * s6;
    vec2 offset = init + mix(vec2(0.0), vec2(s4, s5) - .5, s3 * s4 * s5);
    p = vec2(
      a * x2 + b * y2 + c * xy + d,
      f * xy + e
    ) + offset;
    if (length(p) >= 2.0) {
      return iter / iterations;
    }
  }
  return 0.;
}
vec3 shade (vec2 uv) {
  float zoom = (0.5 + 12. * s7 * s7 * s7) * (1.5 + 0.5 * travel);
  float focusAngle = 4. * travel;
  float focusAmp = 0.1 + 0.4 * s7;
  vec2 init = 2. * (uv - .5) / zoom;
  pR(init, rot * time);
  init -= vec2(.6, .0);
  init += focusAmp * vec2(cos(focusAngle), sin(focusAngle));
  return pal(pow(run(init), .5));
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 uvRatio = 0.5 + (uv - 0.5) * ratio;
  vec3 c = vec3(0.);
  float total = 0.0;
  vec2 uvP = uvRatio;
  c += shade(uvRatio);
  total += 1.0;
  if (aa > 0.) {
    for (float x=-.5; x<=.5; x += 1. / aa) {
      for (float y=-.5; y<=.5; y += 1. / aa) {
        uvP = uvRatio;
        uvP += 0.5 * vec2(x, y) / resolution;
        c += shade(uvP);
        total += 1.0;
      }
    }
  }
  c /= total;
  color = vec4(c, 1.0);
}
  `,
  },
});

const Persistence = ({ children: t, persistence }) => (
  <Node
    shader={shaders.persistence}
    backbuffering
    uniforms={{ t, back: Uniform.Backbuffer, persistence }}
  />
);
