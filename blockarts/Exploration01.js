import React, { useEffect, useMemo, useState } from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import { Surface } from "gl-react-dom";
import MersenneTwister from "mersenne-twister";
import { words } from "lodash";

// MAIN CODE ////////////////////////////////////////////////

export const styleMetadata = {
  name: "",
  description: "",
  image: "",
  creator_name: "greweb",
  options: {
    // comment seed when going production!
    seed: 0, // this was used for debug
    mod1: 0.5,
    mod2: 0.5,
    mod3: 0.1,
  },
};

const CustomStyle = ({ block, attributesRef, mod1, mod2, mod3 }) => {
  const {} = useBlockDerivedData(block);

  useEffect(() => {
    const attributes = [
      {
        trait_type: "Foo",
        value: "Bar",
      },
    ];
    attributesRef.current = () => ({
      attributes,
    });
  }, [attributesRef]);

  return (
    <Node
      shader={shaders.main}
      uniforms={{
        resolution: Uniform.Resolution,
      }}
    />
  );
};

function useBlockDerivedData(block) {
  return useMemo(() => {
    const { hash, transactions } = block;

    let MAX = 50 * 50;

    const words = [];
    const all = [];
    transactions
      .slice(0)
      .sort((a, b) => (b.input || "").length - (a.input || "").length)
      .forEach((t) => {
        if (all.length >= MAX) return;
        let input = String(t.input || "");
        if (input.startsWith("0x")) {
          input = input.slice(2);
        }
        let word = "";
        for (let i = 0; i < input.length; i += 2) {
          const c = parseInt(input.slice(i, i + 2), 16);
          if (c && all.length < MAX) {
            all.push(c);
          }
          if (
            (97 <= c && c <= 122) ||
            (65 <= c && c <= 90) ||
            (48 <= c && c <= 57)
          ) {
            word += String.fromCharCode(c);
          } else {
            if (word.length > 6) {
              words.push(word);
            }
            word = "";
          }
        }
      });

    words.sort((a, b) => b.length - a.length);

    console.log(Math.floor(Math.sqrt(all.length)));
    console.log(words);

    const rng = new MersenneTwister(parseInt(hash.slice(0, 16), 16));

    console.log(block);

    return {};
  }, [block]);
}

const shaders = Shaders.create({
  main: {
    frag: GLSL`
    precision highp float;
    varying vec2 uv;
    uniform vec2 resolution;

    #define PI ${Math.PI}
    
    vec3 palette(float t,vec3 a,vec3 b,vec3 c,vec3 d){
      return a+b*cos(6.28318*(c*t+d));
    }
    vec3 pal(float t){
      return palette(
        t,
        vec3(0.5),
        vec3(0.5),
        vec3(0.9, 1., 1.1),
        vec3(0.0, 0.33, 0.66)
      );
    }

    void main() {
      vec2 ratio = resolution / min(resolution.x, resolution.y);
      vec3 c = vec3(0.);
      vec2 p = (uv - 0.5) * ratio;
      c += pal(uv.x + floor(10. * uv.y));
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
