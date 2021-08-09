import React, { useEffect } from "react";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";
import MersenneTwister from "mersenne-twister";

/*
Rarity Features

- common: variety of frequencies and colors
- a bit rare: get black color
- rare: harlequin mode! (lot of colors!)
- very rare: get a pattern of high frequency (lot of noise)
- extremely rare: get no pattern (full color)
- extremely extremely rare: you mint a GOLD pattern!

Styles

- swap: finetune coloring and slide the low frequency layer
- soul: highlights more the high frequency patterns.
- lense: it allows to adjust the anti aliasing which can produce a "pixel blur" effect sometimes desired
*/

export const styleMetadata = {
  name: "Pattern 01",
  description:
    "Welcome to the realm of pixel patterns. Explore different harmonies of patterns and color palettes. Block determines most of the pattern, creators are able to finetune things. Black color is uncommon, harlequin palette is rare, high frequency noise is very rare, having a full square color is extremly rare, but you are the luckiest if you mint some GOLD!",
  image: "",
  creator_name: "greweb",
  options: {
    // comment seed when going production!
    // seed: 0.5, // this was used for debug
    swap: 0.5,
    soul: 0.3,
    lense: 0.1,
  },
};

const shaders = Shaders.create({
  main: {
    frag: GLSL`
precision highp float;
varying vec2 uv;

uniform float swap;
uniform float lense;
uniform float soul;
uniform float s1, s2, s3, s4, s5, s6, s7;

#define PI ${Math.PI}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
vec3 pal (float t) {
  return palette(
    t + 0.1 + 0.1 * swap,
    vec3(.5),
    vec3(.5),
    vec3(.9, .5, 0.5),
    vec3(0.0, .6, .2 + 0.2 * pow(s3, 4.0))
  );
}
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
float cell (vec2 p) {
  float v = 0.0;
  float a = cos((1.0 + s1) * p.x);
  float b = sin(p.y);
  v = 2.0 * (0.5 + s2) * a * b;
  v /= mix(1.0, 0.2 * (p.x + 0.5) * (0.2 * p.y - 0.5), (1.0 - s1) * s2);
  float count = 3.0 + 13.0 * step(0.95, s5);
  return mod(floor(v), count);
}
vec2 project (vec2 op, float unzoom) {
  vec2 p = op * unzoom + vec2(2.0 * swap, 0.0);
  pR(p, PI/4.);
  return floor(p);
}
float shade (vec2 op) {
  float s = pow(s4, 0.5) - pow(s7, 4.) + pow(s1 * s2 * s3, 3.0) * 20.0;
  float c1 = cell(project(op, 64. * s));
  float c2 = cell(project(op, 16. * s));
  float c3 = cell(project(op, 8. * s));
  float cR = cell(s3 + project(op, s));
  float a = soul * 0.015;
  float b = a * 0.5;
  float c = 1.0 - a - b;
  float final = a * c1 + b * c2 + c * c3 + cR;
  return final;
}
void main() {
  vec3 c = vec3(0.);
  for (float x=-.5; x<=.5; x += 1.) {
    for (float y=-.5; y<=.5; y += 1.) {
      vec2 uvP = uv;
      uvP += vec2(x, y) * 0.02 * (0.01 + lense * lense);
      c += pal(shade(uvP));
    }
  }
  c /= 4.;

  float gold = s2 * s6 * s6;
  gold *= step(0.8, gold);
  c = mix(
    c,
    mix(vec3(0.0), vec3(1.0, 0.8, 0.0), 1.5 * smoothstep(0.5, 1.0, c.r)),
    gold
  );

  gl_FragColor = vec4(c, 1.0);
}
  `,
  },
});

const CustomStyle = ({ block, attributesRef, swap, lense, seed, soul }) => {
  const { hash } = block;

  const rng = new MersenneTwister(
    // when seed is not provided, it means we're in "production" and the seed is actually the block hash
    (seed ? seed * 0xffffffff : 0) + parseInt(hash.slice(0, 16), 16)
  );
  const s1 = rng.random();
  const s2 = rng.random();
  const s3 = rng.random();
  const s4 = rng.random();
  const s5 = rng.random();
  const s6 = rng.random();
  const s7 = rng.random();

  const s =
    Math.pow(s4, 0.5) - Math.pow(s7, 4) + Math.pow(s1 * s2 * s3, 3.0) * 20.0;

  // generating some qualifier like "much gold" or "very harlequin" =)
  const words = [];
  if (s > 12) {
    words.push("ultimate");
  } else if (s > 5) {
    words.push("extreme");
  } else if (s > 2) {
    words.push("much");
  } else if (s < -0.1) {
    words.push("rare");
  }
  if (s2 * s6 * s6 > 0.8) {
    words.push("gold");
  } else if (s5 > 0.95) {
    words.push("harlequin");
  }

  useEffect(() => {
    attributesRef.current = () => ({
      attributes: [
        {
          trait_type: "Mood",
          value: words.join(" "),
        },
      ],
    });
  }, [swap, lense, soul, words, attributesRef]);

  return (
    <Node
      shader={shaders.main}
      uniforms={{
        swap,
        lense,
        soul,
        s1,
        s2,
        s3,
        s4,
        s5,
        s6,
        s7,
      }}
    />
  );
};

const Outer = function ({ width, height, innerCanvasRef, ...props }) {
  return (
    <Surface width={width} height={height} ref={innerCanvasRef}>
      <CustomStyle width={width} height={height} {...props} />
    </Surface>
  );
};

export default Outer;
