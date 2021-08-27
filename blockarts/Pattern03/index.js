// @flow
import React, {
  useEffect,
  useMemo,
  useState,
  useCallback,
  useRef,
} from "react";
import MersenneTwister from "mersenne-twister";
import { Surface } from "gl-react-dom";
import { Bus, GLSL, LinearCopy, Node, Shaders, Uniform } from "gl-react";
import { mix, safeParseInt, smoothstep, useStats, useTime } from "../utils";
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

const COLORS = [
  {
    name: "Black",
    main: [0.1, 0.1, 0.1],
    highlight: [0, 0, 0],
  },
  {
    name: "Bloody Brexit",
    main: [0.02, 0.12, 0.42],
    highlight: [0.18, 0.0, 0.2],
  },
  {
    name: "Indigo",
    main: [0.45, 0.55, 0.7],
    highlight: [0.2, 0.3, 0.4],
  },
  {
    name: "Turquoise",
    main: [0 / 255, 180 / 255, 230 / 255],
    highlight: [0 / 255, 90 / 255, 140 / 255],
  },
  {
    name: "Aurora Borealis",
    main: [0.0, 0.6, 0.6],
    highlight: [0.0, 0.3, 0.4],
  },
  {
    name: "Sherwood Green",
    main: [0.25, 0.5, 0.3],
    highlight: [0.1, 0.3, 0.1],
  },
  {
    name: "Red Dragon",
    main: [0.6, 0.0, 0.0],
    highlight: [0.3, 0.0, 0.0],
  },
  {
    name: "Pumpkin",
    main: [1, 0.5, 0.2],
    highlight: [0.9, 0.3, 0.0],
  },
  {
    name: "Amber",
    main: [1.0, 0.78, 0.28],
    highlight: [1.0, 0.5, 0.0],
  },
  {
    name: "Pink",
    main: [1.0, 0.5, 0.7],
    highlight: [1.0, 0.4, 0.2],
  },
  {
    name: "Imperial Purple",
    main: [0.5, 0.1, 0.9],
    highlight: [0.2, 0.0, 0.4],
  },
];

const pickColor = (f) =>
  COLORS[Math.floor(0.99999 * f * COLORS.length) % COLORS.length];

const MAX = 2048;

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
  const dpr = window.devicePixelRatio || 1;
  const w = Math.min(MAX, Math.floor(dpr * width));
  const h = Math.min(MAX, Math.floor(dpr * height));
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
        <svg xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" xmlns="http://www.w3.org/2000/svg" width="${MAX}px" height="${MAX}px" style="background:white" viewBox="0 0 200 200">` +
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
        <svg xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" xmlns="http://www.w3.org/2000/svg" width="210mm" height="210mm" style="background:white" viewBox="0 0 210 210">
        <g transform="translate(5,5)">` +
          svgBody
            .replace(/opacity="[^"]*"/g, "")
            .replace(
              /#0FF/g,
              "rgb(" +
                primary.main.map((n) => Math.round(n * 255)).join(",") +
                ")"
            )
            .replace(
              /#F0F/g,
              "rgb(" +
                secondary.main.map((n) => Math.round(n * 255)).join(",") +
                ")"
            ) +
          "</g></svg>"
      ),
    [svgBody, primary, secondary]
  );

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
    fontFamily: "Monaco, sans-serif",
    opacity: hover ? 1 : 0,
    transition: "200ms opacity",
    userSelect: "none",
  };

  return (
    <div
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      style={{ width, height, position: "relative" }}
    >
      <Surface width={width} height={height} ref={innerCanvasRef}>
        <LinearCopy>
          <Post
            size={{ width: w, height: h }}
            mod6={mod6}
            variables={variables}
          >
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
  const paperSeed = (blockNumber / 100) % 1;

  const mod2rounded = Math.round(mod2 * 100) / 100;
  const mod3rounded = Math.round(mod3 * 100) / 100;
  const mod4rounded = Math.floor(mod4 * 100) / 100;
  const mod5rounded = Math.floor(mod5 * 100) / 100;
  const primary = pickColor(mod1);
  const secondary = pickColor((blockNumber % 10) / 10);
  const borderBase = Math.floor(Math.max(0, 80 * (mod2 - 0.5))) / 4;
  const marginBase = Math.floor(
    30 * mod3 * mod3 - 30 * (1 - mod3) * (1 - mod3)
  );

  const shouldEnableBorderCross = useMemo(() => {
    const rng = new MersenneTwister(safeParseInt(block.hash.slice(0, 16)));
    return (
      (blockNumber % 1000000 === 0 ||
        (blockNumber % 100000 === 0 && rng.random() < 0.2) ||
        rng.random() < 0.01) &&
      Math.floor(rng.random() * COLORS.length) === COLORS.indexOf(primary)
    );
  }, [primary, blockNumber]);

  // then, algos that also needs the mods
  const opts = useMemo(() => {
    const rng = new MersenneTwister(safeParseInt(block.hash.slice(0, 16)));
    let seed = 1000 * rng.random();

    const sdivisions = Math.floor(10 + 210 * mod5rounded);
    const line_dir = mod4rounded;

    let margin = [-marginBase, -marginBase];

    const ratioEthTransfer = !stats.totalUsd
      ? 1
      : stats.totalEthUsd / stats.totalUsd;

    let border = borderBase;
    let border_cross = "";
    let r = [rng.random(), rng.random(), rng.random(), rng.random()];
    if (shouldEnableBorderCross) {
      if (r[0] < 0.3) {
        border_cross += "\\";
      }
      if (r[3] < 0.3) {
        border_cross += "/";
      }
      if (r[1] < 0.3) {
        border_cross += "|";
      }
      if (r[2] < 0.3 || border_cross.length === 0) {
        border_cross += "-";
      }
      border *= 2;
    }

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

    let sublines =
      8 - 7 * Math.pow(rng.random(), 8) + 8 * Math.pow(rng.random(), 3);
    let lines =
      1.5 +
      89 *
        mix(smoothstep(0, 400, block.transactions.length), rng.random(), 0.2);

    sublines *= 1 + 3 * rng.random() * smoothstep(20, 2, lines);

    sublines = Math.floor(sublines);
    lines = Math.floor(lines);

    let lines_axis = [];
    if (
      lines < 10 ||
      ratioEthTransfer < 0.99 ||
      stats.transactions.length === 0
    ) {
      lines_axis.push(ratioEthTransfer < 0.5);
      if (ratioEthTransfer < 0.02) {
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

    let lower = 0.1 - 0.3 * Math.pow(rng.random(), 10);
    let upper = Math.max(
      lower + 0.1,
      Math.min(1, sublines / 4) -
        rng.random() *
          rng.random() *
          rng.random() *
          Math.max(0, rng.random() - 0.5) +
        Math.pow(rng.random(), 14)
    );

    const off = [0, 0];
    off[0] = Math.max(
      0,
      0.3 * rng.random() * rng.random() * rng.random() - 0.05
    );
    if (!off[0] || rng.random() < 0.2) {
      off[1] = Math.max(0, 0.2 * rng.random() * rng.random() - 0.05);
    }
    off[0] *= mod4rounded;
    off[1] *= mod4rounded;

    let disp0 = 3 * rng.random() * rng.random() + mod3rounded;
    let disp1 = rng.random() + mod2rounded * (1 - mod3rounded);

    let f1 = 3 * rng.random() * rng.random() + mod2rounded;
    let f2 = 3 * rng.random() * rng.random() + mod3rounded;

    let k1 = f1 * disp0;
    let k2 = disp0;
    let k3 = disp1;
    let k4 = f2 * disp1;

    let second_color_div =
      lines_axis.length > 0 && lines > 1 && rng.random() < 0.1
        ? Math.floor(1 + 60 * Math.pow(rng.random(), 8))
        : 0;

    let second_color_blind = second_color_div > 0 ? rng.random() > 0.5 : false;

    const mirror_axis_weight = 1 + 1 * Math.cos(2 * 2 * Math.PI * mod2rounded);

    const lowstep =
      -0.3 - (rng.random() < 0.05 ? 4 * rng.random() * rng.random() : 0);

    const m = 3 + 5 * rng.random() - 2 * rng.random() * rng.random();

    const k = 3 + 3 * rng.random() - 2 * rng.random() * rng.random();

    const mod3sawtooth = (mod3rounded * 2) % 1;
    const highstep = 0.45 + mod3sawtooth * rng.random();

    let radius_amp =
      4 *
      mod5rounded *
      (rng.random() - 0.5) *
      rng.random() *
      rng.random() *
      Math.max(0.0, rng.random() - 0.2);
    let radius_freq =
      1 + 20 * rng.random() * rng.random() * rng.random() * rng.random();
    let radius_offset = rng.random();

    return {
      opacity: 0.55,
      opacity_fade: 0.15,
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
      mirror_axis_weight,
      off,
      lower,
      upper,
      lowstep,
      highstep,
      rotation,
      m,
      k,
      k1,
      k2,
      k3,
      k4,
      second_color_div,
      second_color_blind,
      border_cross,
      radius_amp,
      radius_freq,
      radius_offset,
    };
  }, [
    stats,
    block,
    borderBase,
    mod2rounded,
    mod3rounded,
    mod4rounded,
    mod5rounded,
    marginBase,
    shouldEnableBorderCross,
  ]);

  return useMemo(
    () => ({
      opts,
      primary,
      secondary,
      paperSeed,
    }),
    [opts, primary, secondary]
  );
}

function useAttributes(attributesRef, variables) {
  useEffect(() => {
    attributesRef.current = () => {
      const { opts, primary, secondary } = variables;
      const attributes = [
        {
          trait_type: "Density",
          value: opts.sublines * opts.lines,
        },
        {
          trait_type: "Lines",
          value: opts.lines,
        },
        {
          trait_type: "Shape",
          value:
            opts.lines_axis.length === 0
              ? "spiral"
              : opts.lines_axis.length === 1
              ? !opts.lines_axis[0]
                ? "vertical"
                : "horizontal"
              : "cloth",
        },
        {
          trait_type: "Ink",
          value:
            primary.name +
            (opts.second_color_div > 0 && !opts.second_color_blind
              ? ", " + secondary.name
              : ""),
        },
      ];
      if (opts.border_cross && opts.border > 0) {
        attributes.push({
          trait_type: "Rare Shape",
          value: opts.border_cross,
        });
      }

      if (opts.mirror_axis.length) {
        attributes.push({
          trait_type: "Symmetry",
          value: opts.rotation
            ? opts.mirror_axis.length === 2
              ? "two diagonals"
              : "diagonal"
            : opts.mirror_axis.length === 2
            ? "cross"
            : !opts.mirror_axis[0]
            ? "horizontal"
            : "vertical",
        });
      }

      return {
        attributes,
      };
    };
  }, [variables]);
}

const BlurV1D = ({
  width,
  height,
  map,
  factor,
  aa,
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
      factor,
      aa,
    }}
  />
);

const NORM = Math.sqrt(2) / 2;

const directionForPass = (p, total) => {
  const f = (2 * Math.ceil(p / 2)) / total;
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
        factor={factor}
        aa={0.1 * Math.floor(1 + (0.5 * passes) / pass)}
        direction={directionForPass(pass, passes)}
      >
        {rec(pass - 1)}
      </BlurV1D>
    );
  return rec(passes);
};

const Blurred = ({ size, children, mod6, blurMap }) => (
  <BlurV {...size} map={blurMap} factor={4 * mod6} passes={4}>
    {children}
  </BlurV>
);

const BlurredMemo = React.memo(Blurred);

const Paper = ({ seed, grain }) => (
  <Node
    shader={shaders.paper}
    uniforms={{ seed, grain, resolution: Uniform.Resolution }}
  />
);
const PaperMemo = React.memo(Paper);

const BlurGradient = ({ mod6 }) => (
  <Node
    shader={shaders.blurGradient}
    uniforms={{
      narrow: 0.3 * mod6 * mod6,
      resolution: Uniform.Resolution,
    }}
  />
);
const BlurGradientMemo = React.memo(BlurGradient);

const Post = ({
  size,
  children,
  variables: { primary, secondary, paperSeed },
  mod6,
}) => {
  const time = useTime();
  const blurMapBusRef = useRef();
  return (
    <Node
      {...size}
      shader={shaders.main}
      uniforms={{
        t: (
          <BlurredMemo
            blurMap={() => blurMapBusRef.current}
            size={size}
            mod6={mod6}
          >
            {children}
          </BlurredMemo>
        ),
        blurMap: () => blurMapBusRef.current,
        paper: <PaperMemo seed={paperSeed} grain={256} />,
        phasing: 0.15 + 0.1 * mod6,
        grainAmp: 0.08,
        time,
        resolution: Uniform.Resolution,
        primary: primary.main,
        primaryHighlight: primary.highlight,
        secondary: secondary.main,
        secondaryHighlight: secondary.highlight,
      }}
    >
      <Bus ref={blurMapBusRef}>
        <BlurGradientMemo mod6={mod6} />
      </Bus>
    </Node>
  );
};

const shaders = Shaders.create({
  blurGradient: {
    frag: `precision highp float;
    varying vec2 uv;
    uniform float narrow;
    uniform vec2 resolution;
    void main () {
      vec2 ratio = resolution / min(resolution.x, resolution.y);
      vec2 p = 0.5 + (uv - 0.5) * ratio;
      float t = smoothstep(0.4, 0.8, 2. * abs(p.y-0.5) * length(p-.5) + narrow);
      gl_FragColor = vec4(vec3(t), 1.0);
    }`,
  },
  paper: {
    frag: `precision highp float;
    varying vec2 uv;
    uniform vec2 resolution;
    uniform float grain;
    uniform float seed;
    void pR(inout vec2 p, float a) {
      p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
    }
    float hash(float p) {
      p = fract(p * .1031);
      p *= p + 33.33;
      p *= p + p;
      return fract(p);
    }
    float hash(vec2 p) {
      vec3 p3  = fract(vec3(p.xyx) * .1031);
      p3 += dot(p3, p3.yzx + 33.33);
      return fract((p3.x + p3.y) * p3.z);
    }
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
    const mat2 m2 = mat2( 0.4,  0.7, -0.7,  0.4 );
    float fbm( in vec2 x ) {
      float f = 2.0;
      float s = 0.55;
      float a = 0.0;
      float b = 0.5;
      for( int i=0; i<4; i++ ) {
        float n = noise(x);
        a += b * n;
        b *= s;
        x = f * x;
      }
      return a;
    }
    void main () {
      vec2 ratio = resolution / min(resolution.x, resolution.y);
      vec2 p = 0.5 + (uv - 0.5) * ratio;
      pR(p, 2.);
      float t = 0.5 * fbm(seed + p * grain) + 0.5 * fbm((p + vec2(7.7 * seed, 3.3 - seed)) * grain * 2.0);
      t = smoothstep(0.05, 0.15, abs(t-0.5));
      gl_FragColor = vec4(vec3(t), 1.0);
    }`,
  },
  blur1D: {
    // blur9: from https://github.com/Jam3/glsl-fast-gaussian-blur
    frag: `precision highp float;
varying vec2 uv;
uniform sampler2D t, map;
uniform vec2 direction, resolution;
uniform float factor, aa;
vec4 blur9(sampler2D image, vec2 uv, vec2 resolution, vec2 direction, float f) {
  vec4 color = vec4(0.0);
  vec2 off1 = vec2(1.3846153846) * direction * f;
  vec2 off2 = vec2(3.2307692308) * direction * f;
  color += texture2D(image, uv) * 0.2270270270;
  color += texture2D(image, uv + (off1 / resolution)) * 0.3162162162;
  color += texture2D(image, uv - (off1 / resolution)) * 0.3162162162;
  color += texture2D(image, uv + (off2 / resolution)) * 0.0702702703;
  color += texture2D(image, uv - (off2 / resolution)) * 0.0702702703;
  return color;
}
void main () {
  gl_FragColor = blur9(t, uv, resolution, direction, aa + factor * texture2D(map, uv).r);
}`,
  },
  main: {
    frag: GLSL`
    precision highp float;
    varying vec2 uv;
    uniform float time, grainAmp, phasing;
    uniform vec2 resolution;
    uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight;
    uniform sampler2D t, paper, blurMap;

    vec3 pal(float t, vec3 c1, vec3 c2, float phase){
      float m = mix(
        smoothstep(0.3, 0.15, t),
        phase,
        phasing
      );
      return mix(
        vec3(1.0, 1.0, 1.0),
        mix(c1, c2, m),
        smoothstep(1.0, 0.5, t)
      );
    }
    
    void main() {
      vec2 ratio = resolution / min(resolution.x, resolution.y);
      vec2 p = 0.5 + (uv - 0.5) * ratio;
      float phase = abs(cos(2. * (time + uv.y)));
      vec4 v = texture2D(t, p);
      float grain = texture2D(paper, p).r;
      float blur = texture2D(blurMap, p).r;
      vec3 c1 = pal(v.r, primary, primaryHighlight, phase);
      vec3 c2 = pal(v.g, secondary, secondaryHighlight, phase);
      vec3 c = mix(
        vec3(1.0),
        c1 * c2 +
        grainAmp *
        (1. - blur) *
        (0.6 + 0.4 * mix(1.0, -phase, step(0.5, grain))) *
        (grain - 0.5),
        step(0.0, p.x) * step(p.x, 1.0) * step(0.0, p.y) * step(p.y, 1.0)
      );
      gl_FragColor = vec4(c, 1.0);
    }
  `,
  },
});

export default Main;
