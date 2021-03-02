import React, { useEffect } from "react";
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

- swap: it allows to ajust the alignment of the "low frequency" pattern
- soul: it highlights even more the high frequency patterns. use with caution!
- lense: it allows to adjust the anti aliasing which can produce a "pixel blur" effect sometimes desired
*/

export const styleMetadata = {
  name: "Pattern 01",
  description: "",
  image: "",
  creator_name: "gre",
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
  #version 300 es
precision highp float;
in vec2 uv;
out vec4 color;

uniform float swap;
uniform float lense;
uniform float soul;
uniform float s1, s2, s3, s4, s5, s6;

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
vec3 shade (vec2 op) {
  float s = 0.8 * pow(s4, 0.5) + pow(s1 * s2 * s3, 3.0) * 20.0;
  float c1 = cell(project(op, 64. * s));
  float c2 = cell(project(op, 16. * s));
  float c3 = cell(project(op, 8. * s));
  float cR = cell(s3 + project(op, s));
  float a = soul * 0.02;
  float b = a * 0.5;
  float c = 1.0 - a - b;
  float final = a * c1 + b * c2 + c * c3 + cR;
  return pal(final);
}
void main() {
  vec3 c = vec3(0.);
  for (float x=-.5; x<=.5; x += 1.) {
    for (float y=-.5; y<=.5; y += 1.) {
      vec2 uvP = uv;
      uvP += vec2(x, y) * 0.02 * (0.1 + lense * lense);
      c += shade(uvP);
    }
  }
  c /= 4.;

  float gold = pow(s6, 6.0);
  gold *= step(0.8, gold);
  c = mix(
    c,
    mix(vec3(0.0), vec3(1.0, 0.8, 0.0), 1.5 * smoothstep(0.5, 1.0, c.r)),
    gold
  );

  color = vec4(c, 1.0);
}
  `,
  },
});

const CustomStyle = ({ block, attributesRef, swap, lense, seed, soul }) => {
  useAttributes(attributesRef);

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
      }}
    />
  );
};

function useAttributes(ref) {
  // Update custom attributes related to style when the modifiers change
  useEffect(() => {
    ref.current = () => {
      return {
        // This is called when the final image is generated, when creator opens the Mint NFT modal.
        // should return an object structured following opensea/enjin metadata spec for attributes/properties
        // https://docs.opensea.io/docs/metadata-standards
        // https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1155.md#erc-1155-metadata-uri-json-schema

        attributes: [
          {
            trait_type: "your trait here text",
            value: "replace me",
          },
        ],
      };
    };
  }, [ref]);
}

export default CustomStyle;
