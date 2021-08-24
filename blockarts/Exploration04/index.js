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
    main: [0.1, 0.1, 0.1],
    highlight: [0, 0, 0],
  },
  {
    name: "Indigo",
    main: [0.3, 0.3, 0.4],
    highlight: [0, 0, 0],
  },
  {
    name: "Bloody Brexit",
    main: [0.1, 0.1, 0.4],
    highlight: [0.5, 0.0, 0.0],
  },
  {
    name: "Turquoise",
    main: [0.0, 0.5, 1.0],
    highlight: [0.0, 0.2, 0.8],
  },
  {
    name: "Aurora Borealis",
    main: [0.0, 0.65, 0.7],
    highlight: [0.0, 0.3, 0.4],
  },
  {
    name: "Sherwood Green",
    main: [0.1, 0.35, 0.1],
    highlight: [0.0, 0.3, 0.0],
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
    name: "Pumpkin",
    main: [1.0, 0.55, 0.25],
    highlight: [0.8, 0.3, 0.0],
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
  mod6,
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

  const { primary, secondary } = variables;

  const dlSrc = useMemo(
    () =>
      "data:image/svg+xml;base64," +
      btoa(
        `
        <svg xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" xmlns="http://www.w3.org/2000/svg" width="200mm" height="200mm" style="background:white" viewBox="0 0 200 200">
        <g inkscape:groupmode="layer" inkscape:label="Plot">` +
          svgBody
            .replace(/opacity="{\\"}*"/g, "")
            .replace(
              /#F00/g,
              "rgb(" +
                primary.main.map((n) => Math.round(n * 255)).join(",") +
                ")"
            )
            .replace(
              /#0F0/g,
              "rgb(" +
                secondary.main.map((n) => Math.round(n * 255)).join(",") +
                ")"
            ) +
          "</g></svg>"
      ),
    [svgBody, primary, secondary]
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
          <Post mod6={mod6} size={{ width, height }} variables={variables}>
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
  const blockNumber = safeParseInt(block.number);

  const mod2rounded = Math.round(mod2 * 100) / 100;
  const primary = pickColor(mod1);
  const secondary = pickColor((blockNumber % 10) / 10);
  const border = Math.floor(Math.max(0, 80 * (mod2 - 0.5))) / 4;
  const marginBase = Math.floor(40 * mod3 * mod3 - 10);
  const line_dir = Math.floor(mod4 * 100) / 100;
  const sdivisions = Math.floor((10 + 200 * mod5) / 10) * 10;

  // then, algos that also needs the mods
  const opts = useMemo(() => {
    const rng = new MersenneTwister(parseInt(block.hash.slice(0, 16), 16));
    let seed = 1000 * rng.random();

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
      osc_amp[0] += (4 * rng.random() * rng.random() * rng.random()) / osc_freq;
    }
    if (rng.random() < 0.2) {
      osc_amp[1] += (4 * rng.random() * rng.random() * rng.random()) / osc_freq;
    }

    let lines =
      rng.random() < 0.5
        ? 80
        : Math.floor(Math.max(2, Math.min(100 * rng.random(), 100)));

    let densityFactor =
      0.3 -
      0.3 * rng.random() * rng.random() * rng.random() +
      0.7 * smoothstep(0, 400, block.transactions.length);

    let max_density = 640 * densityFactor;

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
        rng.random() *
          rng.random() *
          rng.random() *
          Math.max(0, rng.random() - 0.5)
    );

    const off = [0, 0];
    off[0] = Math.max(
      0,
      0.15 * rng.random() * rng.random() * rng.random() - 0.05
    );
    if (!off[0] || rng.random() < 0.2) {
      off[1] = Math.max(0, 0.1 * rng.random() * rng.random() - 0.05);
    }

    let disp0 = 3 * rng.random() * rng.random();
    let disp1 = rng.random();

    let f1 = 3 * rng.random() * rng.random();
    let f2 = 3 * rng.random() * rng.random();

    let k1 = f1 * disp0;
    let k2 = disp0;
    let k3 = disp1;
    let k4 = f2 * disp1;

    let second_color_div =
      rng.random() < 0.01 ? Math.floor(1 + 60 * Math.pow(rng.random(), 8)) : 0;

    return {
      opacity: 0.5,
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
      mirror_axis_weight: 1 + 1 * Math.cos(2 * 2 * Math.PI * mod2rounded),
      off,
      lower,
      upper,
      lowstep: -0.3,
      highstep: 0.5,
      rotation,
      m: 3 + 5 * rng.random() - 2 * rng.random() * rng.random(),
      k: 3 + 3 * rng.random() - 2 * rng.random() * rng.random(),
      k1,
      k2,
      k3,
      k4,
      second_color_div,
    };
  }, [stats, block, border, mod2rounded, marginBase, sdivisions, line_dir]);

  return useMemo(
    () => ({
      opts,
      primary,
      secondary,
    }),
    [opts, primary, secondary]
  );
}

function useAttributes(attributesRef, variables) {
  useEffect(() => {
    attributesRef.current = () => {
      return {
        // TODO lines
        // TODO sublines
        // TODO shape
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

const Post = ({ size, children, variables: { primary, secondary }, mod6 }) => {
  const time = useTime();
  return (
    <BlurV
      {...size}
      map={
        <Node
          shader={shaders.blurGradient}
          uniforms={{ time, narrow: 0.3 * mod6 * mod6 }}
        />
      }
      factor={3 * mod6}
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
          secondary: secondary.main,
          secondaryHighlight: secondary.highlight,
        }}
      />
    </BlurV>
  );
};

const shaders = Shaders.create({
  blurGradient: {
    frag: `precision highp float;
    varying vec2 uv;
    uniform float time;
    uniform float narrow;
    void main () {
      float phase = 0.1 * cos(2. * time);
      float t = smoothstep(0.4, 0.8, 2. * abs(uv.y-0.5) * length(uv-.5) + narrow + phase);
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
    uniform vec3 secondary, secondaryHighlight;
    uniform sampler2D t;

    vec3 palette(float t,vec3 a,vec3 b,vec3 c,vec3 d){
      return a+b*cos(6.28318*(c*t+d));
    }
    vec3 pal(float t, vec3 c1, vec3 c2){
      float m = smoothstep(0.2, 0.15, t) * 0.95 + 0.05 * cos(4. * time);
      return mix(
        vec3(1.0, 1.0, 1.0),
        mix(c1, c2, m),
        smoothstep(0.99, 0.5, t)
      );
    }
    
    void main() {
      vec2 ratio = resolution / min(resolution.x, resolution.y);
      vec2 p = 0.5 + (uv - 0.5) * ratio;
      vec4 v = texture2D(t, p);
      vec3 c1 = pal(v.r, primary, primaryHighlight);
      vec3 c2 = pal(v.g, secondary, secondaryHighlight);
      vec3 c = c1 * c2;
      gl_FragColor = vec4(c, 1.0);
    }
  `,
  },
});

export default Main;
