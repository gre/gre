import React, { useEffect } from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import MersenneTwister from "mersenne-twister";

export const styleMetadata = {
  name: "Pattern 02",
  description:
    "Welcome to the realm of hexagonal & triangular patterns. Have fun exploring the great variety of patterns and color palettes. The block hash drives the main shape, creators are able to finetune things. Many colors at same time is uncommon, Grayscale is rare, and as in Pattern 01, good luck to mint a GOLD pattern!",
  image: "",
  creator_name: "greweb",
  options: {
    // comment seed when going production!
    // seed: -3, // this was used for debug
    tune: 0.5,
    zoom: 0.5,
    mood: 0.05,
    dephase: 0.3,
  },
};

const ZOOM_POW = 10;

const shaders = Shaders.create({
  main: {
    frag: GLSL`
  #version 300 es
precision highp float;
in vec2 uv;
out vec4 color;

uniform vec2 resolution;
uniform float mood, tune, zoom, dephase;
uniform float mphase;
uniform float s1, s2, s3, s4, s5, s6, s7;

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
vec3 pal (float t) {
  float d = dephase * s3 * 0.3 * length(uv-zoom);
  vec3 c = palette(
    t + (mood + 0.2 * d) * (1. + d),
    vec3(0.5),
    vec3(0.5 + mood),
    vec3(1., 0.5, 0.2),
    vec3(1., mod(s1, .4), s2)
  );
    // grayscale
    c = mix(
      vec3(mod(t + s3, 1.0)),
      c,
      step(s7, 0.9)
    );
    c = mix(
      (0.5 + t + mood) * (vec3(1.0, 0.8, 0.0) + 0.5 * mood),
      c,
      step(s7, 0.99)
    );
    return c;
}

vec3 tile (vec2 p, float t) {
  float r1 = pModPolar(p, 6.);
  p.x -= 1./3.;
  pR(p, t);
  float r2 = 1. + pModPolar(p, 3.);
  float index = mod(r2 + mod(-r1, floor(1. + 5. * s6 * s6)), mphase);
  return pal(1.1 * index / mphase);
}

vec3 shade (vec2 p, vec2 pAbs) {
  float frame = step(0.4 + 0.2 * pow(tune, 2.0), max(abs(pAbs.x-.5), abs(pAbs.y-.5)));
  pR(p, 10. * s4 + zoom);
  vec2 g = p * (2. + 30. * pow(s3, ${ZOOM_POW.toFixed(
    1
  )})) * (1. - 0.5 * zoom) + vec2(0.5 * zoom, s6);
  vec4 r = getHex(g);
  return mix(
    tile(r.xy * vec2(1., -1.), 8. * s5 + 0.2 * tune),
    pal(0.0),
    frame
  );
}

vec3 render(vec2 off) {
  vec3 c = vec3(0.);
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio - .4 + off;
  for (float x=-.5; x<=.5; x += 1.) {
    for (float y=-.5; y<=.5; y += 1.) {
      vec2 d = 0.5 * vec2(x,y) / resolution;
      vec2 p = base + d;
      c += shade(p, uv + d);
    }
  }
  c /= 4.0;
  return c;
}

void main() {
  float dt = 0.02 * max(0., 0.5 * s7 * s7 + dephase - 0.5);
  vec3 c1 = render(vec2(-dt, 0.));
  vec3 c2 = render(vec2(0., 0.));
  vec3 c3 = render(vec2(dt, 0.));
  vec3 c = vec3(c1.r, c2.g, c3.b);
  color = vec4(c, 1.0);
}
  `,
  },
});

const mphasesWord = [
  "zero",
  "solo",
  "duo",
  "trio",
  "quatuor",
  "quintette",
  "sextuor",
  "septuor",
  "octuor",
  "nonette",
  "dixtuor",
];

const CustomStyle = ({
  block,
  attributesRef,
  seed,
  mood,
  tune,
  zoom,
  dephase,
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

  const mphase = Math.floor(3 - s9 * s9 + 5 * s8 * s8 * s8 * s8);
  const zoomFactor = Math.pow(s3, ZOOM_POW);

  useEffect(() => {
    const words = [];
    if (zoomFactor > 0.1) {
      if (zoomFactor < 0.15) {
        words.push("multi");
      } else if (zoomFactor < 0.3) {
        words.push("mega");
      } else if (zoomFactor < 0.6) {
        words.push("ultra");
      } else {
        words.push("noisy");
      }
    }
    if (s7 > 0.99) {
      words.push("gold");
    } else if (s7 > 0.9) {
      words.push("gray");
    }
    if (words.length > 0) {
      words.push(mphasesWord[mphase] || "");
    }
    const attributes = [
      {
        trait_type: "Modulo",
        value: mphase,
      },
    ];
    if (words.length) {
      attributes.push({
        trait_type: "Mood",
        value: words.join(" "),
      });
    }

    attributesRef.current = () => ({
      attributes,
    });
  }, [mphase, zoomFactor, s7, attributesRef]);

  return (
    <Node
      shader={shaders.main}
      uniforms={{
        resolution: Uniform.Resolution,
        mood,
        tune,
        zoom,
        dephase,
        // from seed (block hash)
        s1,
        s2,
        s3,
        s4,
        s5,
        s6,
        s7,
        mphase,
      }}
    />
  );
};

export default CustomStyle;
