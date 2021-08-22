// @flow
import React, { useEffect, useMemo, useState, useCallback } from "react";
import MersenneTwister from "mersenne-twister";
import { mix, safeParseInt, smoothstep, useStats, useTime } from "../utils";
import { Surface } from "gl-react-dom";
import { GLSL, LinearCopy, Node, Shaders, Uniform } from "gl-react";

import init, { blockstyle } from "./blockstyle/pkg/blockstyle";
import wasm from "base64-inline-loader!./blockstyle/pkg/blockstyle_bg.wasm";

function decode(dataURI) {
  const binaryString = atob(dataURI.split(",")[1]);
  var bytes = new Uint8Array(binaryString.length);
  for (var i = 0; i < binaryString.length; i++) {
    bytes[i] = binaryString.charCodeAt(i);
  }
  return bytes.buffer;
}
let wasmLoaded = false;
const promiseOfLoad = init(decode(wasm)).then(() => {
  wasmLoaded = true;
});

const DEBUG_SVG = false;

// IDEAS
// one line of diff color
// diff symmetry of noise
// diff orientation
// make orientation repeat in patterns

const COLORS = [
  {
    name: "Black",
    main: [0, 0, 0],
    highlight: [0, 0, 0],
  },
  {
    name: "Bloody Brexit",
    main: [0.1, 0.1, 0.3],
    highlight: [0.5, 0.0, 0.0],
  },
  {
    name: "Turquoise",
    main: [0.0, 0.5, 1.0],
    highlight: [0.0, 0.2, 0.8],
  },
  {
    name: "Violet",
    main: [0.5, 0.0, 1.0],
    highlight: [0.3, 0.0, 0.5],
  },
  {
    name: "Red Dragon",
    main: [0.8, 0.0, 0.0],
    highlight: [0.2, 0.0, 0.0],
  },
  {
    name: "Pink",
    main: [1.0, 0.5, 0.7],
    highlight: [1.0, 0.5, 0.0],
  },
];

const pickColor = (f) =>
  COLORS[Math.floor(0.99999 * f * COLORS.length) % COLORS.length];

const Main = ({
  innerCanvasRef,
  block,
  attributesRef,
  width,
  height,
  mod1,
  mod2,
  mod3,
  mod4,
  mod5,
  systemContext,
}) => {
  const w = Math.min(2048, Math.floor(width));
  const h = Math.min(2048, Math.floor(height));
  const [hover, setHover] = useState(false);
  const [loaded, setLoaded] = useState(wasmLoaded);
  const variables = useVariables({ block, mod1, mod2, mod3, mod4, mod5 });
  useAttributes(attributesRef, variables);

  const onMouseEnter = useCallback(() => setHover(true), []);
  const onMouseLeave = useCallback(() => setHover(false), []);

  useEffect(() => {
    if (!loaded) promiseOfLoad.then(() => setLoaded(true));
  }, [loaded]);

  const svgBody = useMemo(() => {
    if (!loaded) return "";
    let prev = Date.now();
    const result = blockstyle(variables.opts);
    console.log("svg calc time = " + (Date.now() - prev) + "ms");
    return result;
  }, [variables.opts, loaded]);

  const imgSrc = useMemo(
    () =>
      "data:image/svg+xml;base64," +
      btoa(
        `
        <svg xmlns="http://www.w3.org/2000/svg" width="${w}px" height="${h}px" style="background:white" viewBox="0 0 200 200">` +
          svgBody +
          "</svg>"
      ),
    [svgBody, w, h]
  );

  const dlSrc = useMemo(
    () =>
      "data:image/svg+xml;base64," +
      btoa(
        `
        <svg xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" xmlns="http://www.w3.org/2000/svg" width="200mm" height="200mm" style="background:white" viewBox="0 0 200 200">
        <g inkscape:groupmode="layer" inkscape:label="Plot">` +
          svgBody.replace(/opacity="0\.4"/g, "") +
          "</g></svg>"
      ),
    [svgBody]
  );

  if (DEBUG_SVG) {
    return <img src={imgSrc} />;
  }

  let download = `Pattern03_${String(
    systemContext?.styleEdition || safeParseInt(block.number)
  )}.svg`;

  const aStyle = {
    position: "absolute",
    bottom: 2,
    right: 2,
    background: "white",
    border: "2px solid black",
    padding: "4px 6px",
    color: "black",
    textDecoration: "none",
    fontSize: "16px",
    fontWeight: "bold",
    fontFamily: "Monaco",
    opacity: hover ? 1 : 0,
    transition: "200ms opacity",
  };

  return (
    <div
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      style={{ width, height, position: "relative" }}
    >
      <Surface width={width} height={height} ref={innerCanvasRef}>
        <LinearCopy>
          <Post size={{ width, height }} variables={variables}>
            {imgSrc}
          </Post>
        </LinearCopy>
      </Surface>
      <a style={aStyle} download={download} href={dlSrc}>
        {"SVG"}
      </a>
    </div>
  );
};

function useVariables({ block, mod1, mod2, mod3, mod4, mod5 }) {
  const stats = useStats({ block });

  const primary = pickColor(mod1);
  const border = Math.floor(40 * mod2 * mod2) / 4;
  const marginBase = Math.floor(40 * mod3 * mod3 - 10);
  const line_dir = Math.floor(mod4 * 100) / 100;
  const sdivisions = Math.floor((20 + 180 * mod5) / 10) * 10;

  // then, algos that also needs the mods
  return useMemo(() => {
    const rng = new MersenneTwister(parseInt(block.hash.slice(0, 16), 16));
    let seed = rng.random();

    let margin = [-marginBase, -marginBase];

    let mainPad = Math.max(
      0,
      50 * rng.random() * rng.random() - 10 - Math.max(0, marginBase - 20)
    );
    if (mainPad > 0) {
      mainPad += 10;
    }
    let padding = [mainPad, mainPad];
    if (rng.random() < 0.1) {
      padding[0] = mix(0, mainPad * 2, rng.random());
      padding[1] = mainPad * 2 - padding[0];
    }
    padding = padding.map((v) => Math.max(border, v));

    let osc_freq = 40 + 200 * rng.random();
    let osc_amp = [0, 0];
    if (rng.random() < 0.2) {
      osc_amp[0] += (rng.random() * rng.random() * rng.random()) / osc_freq;
    }
    if (rng.random() < 0.2) {
      osc_amp[1] += (rng.random() * rng.random() * rng.random()) / osc_freq;
    }

    let lines =
      rng.random() < 0.5
        ? 80
        : Math.floor(Math.max(2, Math.min(100 * rng.random(), 100)));

    let densityFactor =
      0.3 -
      0.3 * rng.random() * rng.random() * rng.random() +
      0.7 * smoothstep(0, 400, block.transactions.length);

    let max_density = 600 * densityFactor;

    let sublines = Math.round(Math.max(2, Math.min(max_density / lines, 16)));

    let lines_axis = [];
    if (lines < 20 || rng.random() < 0.8) {
      lines_axis.push(rng.random() < 0.5);
      if (rng.random() < 0.05) {
        lines_axis.push(!lines_axis[0]);
      }
    }

    let mirror_axis = [];
    if (rng.random() < 0.8) {
      mirror_axis.push(rng.random() < 0.5);
      if (rng.random() < 0.2) {
        mirror_axis.push(!mirror_axis[0]);
      }
    }

    let rotation = 0;
    if (rng.random() < 0.2) {
      rotation = Math.PI / 4;
    }

    let lower = 0.1 - 0.3 * rng.random() * rng.random() * rng.random();
    let upper = Math.max(
      lower + 0.1,
      Math.min(1, sublines / 4) -
        0.7 * rng.random() * rng.random() * Math.max(0, rng.random() - 0.5)
    );

    const off = [
      0.1 * rng.random() * rng.random(),
      0.1 * rng.random() * rng.random(),
    ];

    return {
      primary,
      opts: {
        opacity: 0.4,
        border,
        padding,
        lines,
        seed,
        sdivisions,
        sublines,
        osc_amp,
        osc_freq,
        margin,
        lines_axis,
        mirror_axis,
        line_dir,
        mirror_axis_weight: 1 + 1 * Math.cos((5 * Math.PI * border) / 10),
        off,
        lower,
        upper,
        lowstep: -0.3,
        highstep: 0.5,
        rotation,
        m: 3 + 5 * rng.random() - 2 * rng.random() * rng.random(),
        k: 3 + 3 * rng.random() - 2 * rng.random() * rng.random(),
        k1: 1.0 + rng.random(),
        k2: 1.0 + rng.random(),
        k3: 1.0 + 5 * rng.random(),
        k4: 1.0 + 5 * rng.random(),
        k5: 2.0 + 10 * rng.random(),
        k6: 2.0 + 10 * rng.random(),
      },
    };
  }, [stats, block, primary, border, marginBase, sdivisions, line_dir]);
}

function useAttributes(attributesRef, variables) {
  useEffect(() => {
    attributesRef.current = () => {
      return {
        attributes: [
          {
            trait_type: "Ink",
            value: variables.primary.name,
          },
        ],
      };
    };
  }, [variables]);
}

const BlurV1D = ({
  width,
  height,
  map,
  pixelRatio,
  direction,
  children: t,
}) => (
  <Node
    shader={shaders.blur1D}
    width={width}
    height={height}
    pixelRatio={pixelRatio}
    uniforms={{
      direction,
      resolution: Uniform.Resolution,
      t,
      map,
    }}
  />
);

const NORM = Math.sqrt(2) / 2;

const directionForPass = (p, factor, total) => {
  const f = (factor * 2 * Math.ceil(p / 2)) / total;
  switch (
    (p - 1) %
    4 // alternate horizontal, vertical and 2 diagonals
  ) {
    case 0:
      return [f, 0];
    case 1:
      return [0, f];
    case 2:
      return [f * NORM, f * NORM];
    case 3:
      return [f * NORM, -f * NORM];
  }
};

const BlurV = ({
  width,
  height,
  map,
  pixelRatio,
  factor,
  children,
  passes,
}) => {
  const rec = (pass) =>
    pass <= 0 ? (
      children
    ) : (
      <BlurV1D
        width={width}
        height={height}
        map={map}
        pixelRatio={pixelRatio}
        direction={directionForPass(pass, factor, passes)}
      >
        {rec(pass - 1)}
      </BlurV1D>
    );
  return rec(passes);
};

const Post = ({ size, children, variables: { primary } }) => {
  const time = useTime();
  return (
    <BlurV
      {...size}
      map={<Node shader={shaders.blurGradient} uniforms={{ time }} />}
      factor={3}
      passes={4}
    >
      <Node
        shader={shaders.main}
        uniforms={{
          t: children,
          time,
          resolution: Uniform.Resolution,
          primary: primary.main,
          primaryHighlight: primary.highlight,
        }}
      />
    </BlurV>
  );
};

// TODO paper effect

const shaders = Shaders.create({
  blurGradient: {
    // TODO add a slight blur for ALL? for aa?
    frag: `precision highp float;
    varying vec2 uv;
    uniform float time;
    void main () {
      float phase = 0.2 * cos(4. * time);
      float phase2 = 0.2 * cos(2. * time);
      float t = smoothstep(0.3 + phase2, 0.7, 2. * abs(uv.y-0.5) * length(uv-.5)) * (0.7 + 0.3 * phase);
      gl_FragColor = vec4(vec3(t), 1.0);
    }`,
  },
  blur1D: {
    // blur9: from https://github.com/Jam3/glsl-fast-gaussian-blur
    frag: `precision highp float;
varying vec2 uv;
uniform sampler2D t, map;
uniform vec2 direction, resolution;
vec4 blur9(sampler2D image, vec2 uv, vec2 resolution, vec2 direction) {
  vec4 color = vec4(0.0);
  vec2 off1 = vec2(1.3846153846) * direction;
  vec2 off2 = vec2(3.2307692308) * direction;
  color += texture2D(image, uv) * 0.2270270270;
  color += texture2D(image, uv + (off1 / resolution)) * 0.3162162162;
  color += texture2D(image, uv - (off1 / resolution)) * 0.3162162162;
  color += texture2D(image, uv + (off2 / resolution)) * 0.0702702703;
  color += texture2D(image, uv - (off2 / resolution)) * 0.0702702703;
  return color;
}
void main () {
  gl_FragColor = blur9(t, uv, resolution, direction * texture2D(map, uv).rg);
}`,
  },
  main: {
    frag: GLSL`
    precision highp float;
    varying vec2 uv;
    uniform float time;
    uniform vec2 resolution;
    uniform vec3 primary, primaryHighlight;
    uniform sampler2D t;

    vec3 palette(float t,vec3 a,vec3 b,vec3 c,vec3 d){
      return a+b*cos(6.28318*(c*t+d));
    }
    vec3 pal(float t){
      return mix(
        vec3(1.0, 1.0, 1.0),
        mix(
          primary,
          primaryHighlight,
          smoothstep(0.11, 0.1, t) * 0.9 + 0.1 * cos(4. * time)),
        smoothstep(0.99, 0.5, t)
      );
    }
    
    void main() {
      vec2 ratio = resolution / min(resolution.x, resolution.y);
      vec2 p = 0.5 + (uv - 0.5) * ratio;
      vec3 c = pal(texture2D(t, p).r);
      // c = pal(uv.x);
      gl_FragColor = vec4(c, 1.0);
    }
  `,
  },
});

export default Main;
