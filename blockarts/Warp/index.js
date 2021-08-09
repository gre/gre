import React, { useEffect, useMemo, useState } from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import { Surface } from "gl-react-dom";
import MersenneTwister from "mersenne-twister";

/*
Technical notes
- This is implemented in a shader & gl-react
- This is slightly animated but in a way that the general image is still. so you can have a stable thumbnail but at same time have a very cool viewing experience.

mods feature:
- mod1 and mod2 allows to control the color palette
- mod3 adjust a noise feature (but is kept not too intruisive)

block parameters that impacts the style:
- blocks number that are ending with many 0s (100s, 1000s,...) -> increase the palette density
- lot of transactions => makes more "shapes" (lines/rectangle/circles). it can very rarely become important and do some moiré effect.
- (ex block 12978422) expectionalTxAmountFactor: there is an expectional high value transferred in the block => makes various moiré patterns
- (ex block 12984617) txCountLightFactor: the number of value transferred is expectionally low in the block. => will make very thin waves lines. (very rare can make it very thin)
- block.hash => drives other randomness and the general shape
*/

// UTILITIES //////////////////////

function useTime() {
  const [time, setTime] = useState(0);
  useEffect(() => {
    let startT;
    let h;
    function loop(t) {
      h = requestAnimationFrame(loop);
      if (!startT) startT = t;
      setTime((t - startT) / 1000);
    }
    h = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(h);
  }, []);
  return time;
}

const FBM9 =
  // utilities from https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83
  `
float hash(float n) { return fract(sin(n) * 1e4); }
float hash(vec2 p) { return fract(1e4 * sin(17.0 * p.x + p.y * 0.1) * (0.1 + abs(sin(p.y * 13.0 + p.x)))); }
float noise(float x) {
  float i = floor(x);
  float f = fract(x);
  float u = f * f * (3.0 - 2.0 * f);
  return mix(hash(i), hash(i + 1.0), u);
}
float noise(vec2 x) {
  vec2 i = floor(x);
  vec2 f = fract(x);
  float a = hash(i);
  float b = hash(i + vec2(1.0, 0.0));
  float c = hash(i + vec2(0.0, 1.0));
  float d = hash(i + vec2(1.0, 1.0));
  vec2 u = f * f * (3.0 - 2.0 * f);
  return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}
const mat2 m2 = mat2( 0.6,  0.8, -0.8,  0.6 );
float fbm( in vec2 x ) {
  float f = 2.0;
  float s = 0.55;
  float a = 0.0;
  float b = 0.5;
  for( int i=0; i<9; i++ ) {
    float n = noise(x);
    a += b * n;
    b *= s;
    x = f * x;
  }
  return a;
}
`;
// MAIN CODE ////////////////////////////////////////////////

export const styleMetadata = {
  name: "Warp",
  description: "",
  image:
    "https://raw.githubusercontent.com/gre/gre/master/blockarts/Warp/samples/059.png",
  creator_name: "greweb",
  options: {
    // comment seed when going production!
    // seed: 0, // this was used for debug
    mod1: 0.5,
    mod2: 0.5,
    mod3: 0.5,
  },
};

const CustomStyle = ({ block, attributesRef, mod1, mod2, mod3 }) => {
  const {
    amps,
    shapearg,
    fbmoffset,
    lowcutoff,
    colordelta,
    multicolor,
    attrs,
  } = useBlockDerivedData(block);

  const palarg = [0.5 * mod1, 0.2 + 0.4 * mod2, 0.4 + 0.6 * colordelta];
  const adjust = mod3;

  useEffect(() => {
    const attributes = [
      {
        trait_type: "Shape",
        value: attrs.shape,
      },
      {
        trait_type: "Size",
        value: attrs.size,
      },
      {
        trait_type: "Mood",
        value: attrs.style,
      },
      {
        trait_type: "Complexity",
        value: String(Math.round(attrs.complexity)),
      },
    ];
    attributesRef.current = () => ({
      attributes,
    });
  }, [attributesRef, attrs]);

  const time = useTime();

  return (
    <Node
      shader={shaders.main}
      uniforms={{
        resolution: Uniform.Resolution,
        time,
        amps,
        fbmoffset,
        lowcutoff,
        adjust,
        palarg,
        shapearg,
        multicolor,
      }}
    />
  );
};

const TX_UPPER_BOUND = 500;
const TX_MIN_THRESHOLD = 100 * Math.pow(10, 18);
const TX_LIGHT_VALUE = 10 * Math.pow(10, 18);
const safeParseInt = (a) => {
  const v = parseInt(a);
  if (isNaN(v) || !isFinite(a)) return 0;
  return v;
};

function useBlockDerivedData(block) {
  return useMemo(() => {
    const { hash, transactions } = block;
    const blockNumber = parseInt(block.number);
    const txsCount = transactions.length;
    const txCountHeavyFactor = Math.pow(
      Math.min(TX_UPPER_BOUND, txsCount) / TX_UPPER_BOUND,
      2.0
    );

    let expectionalTxAmountFactor = 0;
    let txCountLightFactor = 0;
    const allGas = transactions.map(
      (t) => safeParseInt(t.gas) * safeParseInt(t.gasPrice)
    );
    const allNonZeroValues = transactions
      .map((t) => safeParseInt(t.value))
      .filter(Boolean);
    allGas.sort((a, b) => a - b);
    allNonZeroValues.sort((a, b) => a - b);
    if (allNonZeroValues.length > 50) {
      let valueMax = allNonZeroValues[allNonZeroValues.length - 1];
      if (valueMax > TX_MIN_THRESHOLD) {
        expectionalTxAmountFactor = Math.pow(
          (valueMax - TX_MIN_THRESHOLD) / valueMax,
          8
        );
      }
      if (valueMax < TX_LIGHT_VALUE) {
        txCountLightFactor = Math.pow(1 - valueMax / TX_LIGHT_VALUE, 0.5);
      }
    }
    const rng = new MersenneTwister(parseInt(hash.slice(0, 16), 16));
    const amps = [
      (20 + 50 * rng.random()) * rng.random() * rng.random(),
      (1 + 30 * rng.random() * rng.random()) * rng.random(),
      (1 + 8 * rng.random() * rng.random()) * rng.random(),
      rng.random() + rng.random() * rng.random(),
      1 +
        (5 * rng.random() - 0.5) * (1 + 2 * rng.random()) +
        30 * expectionalTxAmountFactor,
    ];
    const complexity = amps[0] * amps[1] * amps[2] * amps[3];

    const multicolor =
      rng.random() *
      (Number(blockNumber % 100 === 0) +
        Number(blockNumber % 1000 === 0) +
        Number(blockNumber % 10000 === 0) +
        Number(blockNumber % 100000 === 0));
    const colordelta = rng.random();

    const swtch = +(rng.random() < 0.5);
    const shapearg = [
      rng.random() + 6 * txCountHeavyFactor * (rng.random() - 0.5),
      rng.random() * (1 + 10 * expectionalTxAmountFactor * swtch),
      rng.random() * (1 + 10 * expectionalTxAmountFactor * (1 - swtch)),
      rng.random(),
    ];

    let inf = 0.2;
    let sup = 0.8;
    let size = "normal";
    let style = "normal";
    let shape = "unknown";

    if (amps[4] > 28) {
      size = "extreme";
    }
    if (amps[4] > 23) {
      size = "immense";
    } else if (amps[4] > 16) {
      size = "huge";
    } else if (amps[4] > 8) {
      size = "great";
    }

    if (complexity > 1000) {
      style = "noisy";
    } else if (txCountLightFactor > 0.5) {
      style = "thin";
    } else if (complexity < 4) {
      style = "smooth";
    } else if (multicolor > 0.5) {
      style = "harlequin";
    } else if (complexity > 200) {
      style = "complex";
    }

    if (amps[3] > 1 && complexity < 500) {
      if (amps[0] > 32) {
        style = "RARE1";
      } else if (amps[1] > 16) {
        style = "RARE2";
      } else if (amps[2] > 4) {
        style = "RARE3";
      }
    }

    if (amps[4] > 2) {
      // shape itself
      if (shapearg[2] < inf) {
        if (shapearg[3] < inf) {
          shape = (shapearg[3] < 0.05 ? "rare " : "") + "square";
        } else if (shapearg[3] > sup) {
          shape = (shapearg[3] > 0.95 ? "rare " : "") + "cross";
        } else {
          shape = "polygon";
        }
      } else if (shapearg[2] > sup) {
        if (shapearg[1] < inf) {
          shape = (shapearg[3] < 0.05 ? "rare " : "") + "circle";
        } else if (shapearg[1] > sup) {
          if (shapearg[0] < inf) {
            shape = (shapearg[3] < 0.05 ? "rare " : "") + "vertical";
          } else if (shapearg[0] > sup) {
            shape = (shapearg[3] > 0.95 ? "rare " : "") + "horizontal";
          } else {
            shape = "lines";
          }
        } else {
          shape = "unknown";
        }
      } else {
        shape = "unknown";
      }
    } else {
      shape = "marble";
    }

    const fbmoffset = 100 * rng.random();
    const lowcutoff = rng.random() * txCountLightFactor;
    return {
      amps,
      shapearg,
      fbmoffset,
      lowcutoff,
      multicolor,
      colordelta,
      attrs: {
        shape,
        size,
        style,
        complexity,
      },
    };
  }, [block]);
}

const shaders = Shaders.create({
  main: {
    frag: GLSL`
    precision highp float;
    varying vec2 uv;
    uniform vec2 resolution;
    
    uniform float time;
    uniform float amps[5];
    uniform float shapearg[4];
    uniform float fbmoffset;
    uniform float lowcutoff;
    uniform float adjust;
    uniform vec3 palarg;
    uniform float multicolor;

    #define PI ${Math.PI}
    
    ${FBM9}
    vec3 palette(float t,vec3 a,vec3 b,vec3 c,vec3 d){
      return a+b*cos(6.28318*(c*t+d));
    }
    vec3 pal(float t){
      return palette(
        t,
        vec3(0.5),
        vec3(0.5 + 0.2 * adjust),
        vec3(1. + multicolor, 1.0, 1.0),
        palarg
      );
    }
    float fOpUnionRound(float a, float b, float r) {
      vec2 u = max(vec2(r - a,r - b), vec2(0));
      return max(r, min (a, b)) - length(u);
    }
    float shape (in vec2 p) {
      float l = mix(
        mix(
          max(abs(p.x), abs(p.y)),
          min(abs(p.x), abs(p.y)),
          shapearg[3]
        ),
        mix(
          length(p),
          mix(p.x, p.y, shapearg[0]),
          shapearg[1]
        ),
        shapearg[2]);
      return l;
    }
    float scene(in vec2 p, float t) {
      float adj = 0.8 + 0.2 * adjust;
      float a0 = mix(0.0, amps[0], adj);
      float a1 = mix(0.0, amps[1], adj);
      float a2 = mix(0.0, amps[2], adj);
      float a3 = mix(0.0, amps[3], adj);
      vec2 q = vec2( fbm( a0 * p ), fbm( a0 * p + vec2(3.1,3.32) ) );
      vec2 r = vec2( fbm( 0.2 * a1 * q ),
                    fbm( a1 * q) );
      float v = a3 * fbm(p + a2 * r + fbmoffset);
      v *= 2. - length(p);
      v += 0.1 * fbm(q + p + r + vec2(cos(t), sin(t)));
      v += amps[4] * shape(p);
      return smoothstep(lowcutoff, 1.0, fract(v));
    }
    void main() {
      vec2 ratio = resolution / min(resolution.x, resolution.y);
      vec3 c = vec3(0.);
      vec2 p = (uv - 0.5) * ratio;
      c += pal(scene(p, 0.3 * PI * time));
      gl_FragColor = vec4(c, 1.0);
    }
  `,
  },
});

const Outer = function ({ width, height, innerCanvasRef, ...props }) {
  return (
    <Surface width={width} height={height} ref={innerCanvasRef}>
      <CustomStyle width={width} height={height} {...props} />
    </Surface>
  );
};

export default Outer;
