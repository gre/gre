import React, { useEffect } from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import MersenneTwister from "mersenne-twister";

/*
Rarity Features

- common: variety of zooms and fractals
- a bit rare: seeing great silhouettes
- rare: access to patterns that no longer looks mandelbrot at all!
- very rare: having a non glitched mandelbrot!

Styles

- travel: makes you see dynamically navigate to other places
- love: finetune the mandelbrot precision and coloring
- dark: allows to explore a high variety of palettes. Going into darkness.
*/

export const styleMetadata = {
  name: "Mandelglitch",
  description:
    "Welcome to the realm of mandelbrot fractal glitches. Explore a great variety of fractal patterns. Every block is unique, mesmerizing shapes could appear, sometimes even not recognizable as Mandelbrot's fractal. The rarest item you may hunt for is to find a non-glitched Mandelbrot's original!",
  image: "",
  creator_name: "greweb",
  options: {
    // comment seed when going production!
    seed: -1, // this was used for debug
    travel: 0.1,
    love: 0.5,
    dark: 0.1,
  },
};

const shaders = Shaders.create({
  main: {
    frag: GLSL`
precision highp float;
varying vec2 uv;

uniform vec2 resolution;
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
  float iterations = 2. + 500. * pow(love, 2.0);
  vec2 p = init;
  for (float iter = 0.; iter < 502.; iter += 1.) {
    if (iter >= iterations) break;
    // original mandelbrot formula is: p = vec2(p.x * p.x - p.y * p.y, 2. * p.x * p.y) + init;
    float x2 = p.x * p.x;
    float y2 = p.y * p.y;
    float xy = p.x * p.y;
    float a = 1. + .1 * (s1 - 0.5) * s2 * s2;
    float b = -1. + .1 * (s1 - 0.5) * s2 * s2;
    float c = 0.0 + 2. * (s2 - 0.5) * s3 * s3;
    float d = max(0., pow(s8, 5.) - 0.5) * cos(100. * s7 * s2 * s9 * p.y);
    float e = max(0., pow(s9, 5.) - 0.5) * sin(100. * s2 * s1 * p.x);
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
  float zoom = (0.3 + 12. * s7 * s7 * s7) * (1. + 3. * travel);
  float focusAngle = 4. * travel;
  float focusAmp = 0.4 * s7;
  vec2 init = 2. * (uv - .5) / zoom;
  pR(init, PI * floor(0.5 + 8. * s3) / 4.);
  init -= vec2(.8, .0);
  init += focusAmp * vec2(cos(focusAngle), sin(focusAngle));
  return pal(pow(run(init), .5));
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 uvRatio = 0.5 + (uv - 0.5) * ratio;
  vec3 c = vec3(0.);
  float total = 0.0;
  for (float x=-.5; x<=.5; x += 1.) {
    for (float y=-.5; y<=.5; y += 1.) {
      vec2 uvP = uvRatio;
      uvP += 0.5 * vec2(x, y) / resolution;
      c += shade(uvP);
      total += 1.0;
    }
  }
  c /= total;
  gl_FragColor = vec4(c, 1.0);
}
  `,
  },
});

const CustomStyle = ({
  block,
  attributesRef,
  seed,
  love,
  travel,
  dark,
  lightness,
}) => {
  const { hash } = block;

  const rng = new MersenneTwister(
    // when seed is not provided, it means we're in "production" and the seed is actually the block hash
    (seed || 1) * parseInt(hash.slice(0, 16), 16)
  );
  const s1 = rng.random();
  const s2 = rng.random();
  const s3 = rng.random();
  const s4 = rng.random();
  const s5 = rng.random();
  const s6 = rng.random();
  const s7 = rng.random();
  const s8 = rng.random();
  const s9 = rng.random();

  const zoom = s7;

  useEffect(() => {
    attributesRef.current = () => ({
      attributes: [
        {
          trait_type: "Depth",
          value: zoom,
        },
      ],
    });
  }, [zoom, attributesRef]);

  return (
    <Node
      shader={shaders.main}
      uniforms={{
        resolution: Uniform.Resolution,
        love,
        travel,
        dark,
        lightness,
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
  );
};

export default CustomStyle;
