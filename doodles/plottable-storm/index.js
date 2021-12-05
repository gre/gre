// @flow
import React, { useEffect, useMemo, useState } from "react";
import { Surface } from "gl-react-dom";
import { GLSL, LinearCopy, Node, Shaders, Uniform } from "gl-react";
import init, { render } from "./rust/pkg/main";
import wasm from "base64-inline-loader!./rust/pkg/main_bg.wasm";

/*
 * interesting hashes: 
oomiFEF6jQRcfNG6HsBowXBs3HZiMgnFAwSXw9LuK8XYGcuyi8w

 * Name: Plottable Storm
 * Tags: plottable, webgl, svg, rust, wasm, A4, physical, phygital
 * Description:
Plottable Storm is a flow field simulating fountain pen ink drawing on paper on its digital form. 10 inks, many rarity features varying noise, size and color positionning. Having only one color is rare.
The digital NFTs can be used to perform a physical action: @greweb plotting on demand a fountain pen plot for those who also want physical originals. Full article: https://greweb.me/2021/11/plottable-storm

Digital and Physical art, hybrid and decoupled:
- The art is made as a regular digital NFT on Tezos blockchain – its digital form rendered with WebGL shaders.
- Token to the physical world: owning each NFT confer the power to request the related physical plot.

A collaborative and open ecosystem:
-> a SVG file can be downloaded (Drag&Drop or right-click save) and plotted with fountain pens physically by plotter artists who can interprete it the way they want in in their own conditions. @greweb offers his service on https://greweb.me/plots/nft

Designed for A4 size. Estimated time of 3 hours of plotting time (25% speed)

@greweb – 2021 – tech: WebGL + Rust + WASM.
 */

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

const RARE_ONE_COLORS = [
  {
    name: "Bloody Brexit",
    main: [0.02, 0.12, 0.42],
    highlight: [0.18, 0.0, 0.2],
  },
  {
    name: "Black",
    main: [0.2, 0.2, 0.2],
    highlight: [0, 0, 0],
  },
];

const COLORS = [
  {
    name: "Indigo",
    main: [0.45, 0.55, 0.7],
    highlight: [0.2, 0.3, 0.4],
  },
  {
    name: "FireAndIce",
    main: [0 / 255, 190 / 255, 240 / 255],
    highlight: [0 / 255, 100 / 255, 150 / 255],
  },
  {
    name: "Sherwood Green",
    main: [0.25, 0.5, 0.3],
    highlight: [0.1, 0.3, 0.1],
  },
  {
    name: "Aurora Borealis",
    main: [0.0, 0.6, 0.6],
    highlight: [0.0, 0.3, 0.4],
  },
  {
    name: "Poppy Red",
    main: [0.9, 0.0, 0.1],
    highlight: [0.5, 0.0, 0.1],
  },
  {
    name: "Amber",
    main: [1.0, 0.78, 0.28],
    highlight: [1.0, 0.5, 0.0],
  },
  {
    name: "Pumpkin",
    main: [1, 0.5, 0.2],
    highlight: [0.9, 0.3, 0.0],
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

const pickColor = (f, col) =>
  col[Math.floor(0.99999 * f * col.length) % col.length];

const MAX = 4096;

const ratio = 297 / 210;
const Main = ({ attributesRef, width, height, random }) => {
  const dpr = window.devicePixelRatio || 1;
  let W = width;
  let H = height;
  H = Math.min(H, W / ratio);
  W = Math.min(W, H * ratio);
  W = Math.floor(W);
  H = Math.floor(H);
  let w = Math.min(MAX, dpr * W);
  let h = Math.min(MAX, dpr * H);
  h = Math.min(h, w / ratio);
  w = Math.min(w, h * ratio);
  w = Math.floor(w);
  h = Math.floor(h);
  const [loaded, setLoaded] = useState(wasmLoaded);
  const variables = useVariables({ random });
  useAttributes(attributesRef, variables);

  useEffect(() => {
    if (!loaded) promiseOfLoad.then(() => setLoaded(true));
  }, [loaded]);

  const svg = useMemo(() => {
    if (!loaded) return "";
    let prev = Date.now();
    const result = render(variables.opts);
    console.log(
      "svg calc time = " +
        (Date.now() - prev) +
        "ms – " +
        result.length +
        " bytes"
    );
    return result;
  }, [variables.opts, loaded]);

  let widthPx, heightPx;
  if (W < 600) {
    widthPx = "600px";
    heightPx = "424px";
  } else {
    widthPx = "1200px";
    heightPx = "848px";
  }

  const renderedSVG = useMemo(
    () =>
      "data:image/svg+xml;base64," +
      btoa(svg.replace("210mm", heightPx).replace("297mm", widthPx)),
    [svg, widthPx, heightPx]
  );

  return (
    <div
      style={{
        width,
        height,
        position: "relative",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <div style={{ position: "relative", width: W, height: H }}>
        <div
          style={{
            zIndex: 1,
            position: "relative",
            pointerEvents: "none",
            background: "white",
          }}
        >
          <Surface width={W} height={H}>
            <LinearCopy>
              <Post size={{ width: w, height: h }} variables={variables}>
                {renderedSVG}
              </Post>
            </LinearCopy>
          </Surface>
        </div>
        <Downloadable
          svg={svg}
          primary={variables.primary}
          secondary={variables.secondary}
        />
      </div>
    </div>
  );
};

const dlStyle = {
  opacity: 0,
  width: "100%",
  height: "100%",
  zIndex: 0,
  position: "absolute",
  top: 0,
  left: 0,
};
function Downloadable({ svg, primary, secondary }) {
  const [uri, setURI] = useState(null);
  useEffect(() => {
    const timeout = setTimeout(() => {
      setURI(
        "data:image/svg+xml;base64," +
          btoa(
            svg
              .replace(/opacity="[^"]*"/g, 'style="mix-blend-mode: multiply"')
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
              )
          )
      );
    }, 500);
    return () => clearTimeout(timeout);
  }, [svg, primary, secondary]);
  return <img style={dlStyle} src={uri} />;
}

function mix(a, b, x) {
  return (1 - x) * a + x * b;
}

function scoring(value, sizes) {
  let i = 0;
  for (; i < sizes.length - 1; i += 2) {
    if (value < sizes[i + 1]) return sizes[i];
  }
  return sizes[i];
}

function useVariables({ random }) {
  return useMemo(() => {
    const paperSeed = 100 * random();
    const props = {};
    let primary = pickColor(random(), COLORS);
    let secondary = pickColor(random(), COLORS);
    if (random() < 0.02) {
      secondary = primary = pickColor(random(), RARE_ONE_COLORS);
    }
    props["Inks Count"] = primary === secondary ? 1 : 2;
    props["Inks"] =
      primary === secondary
        ? primary.name
        : [primary.name, secondary.name].sort().join(" + ");
    const f1 = 0.01 + 0.12 * random() * random();
    props["Noise Frequency"] = scoring(f1, [
      "Low",
      0.03,
      "Medium",
      0.05,
      "High",
      0.09,
      "Very High",
    ]);
    const f2 = f1 * 2.0;
    const f3 = f1 * 4.0;
    const max_scale = mix(10, 160, random() * random());
    props["Size"] = scoring(max_scale, [
      "Normal",
      30,
      "Medium",
      70,
      "Large",
      120,
      "Very Large",
    ]);
    const fading = random() < 0.1 ? 20 : random() < 0.15 ? 60 : 35;
    props["Color Separation"] = scoring(fading, [
      "Separated",
      30,
      "Normal",
      50,
      "Melted",
    ]);
    const gravity_dist = 40 + 70 * random() * (0.5 - random());
    props["Gravity"] = scoring(gravity_dist, [
      "Very Low",
      20,
      "Low",
      30,
      "Normal",
      40,
      "High",
      50,
      "Very High",
    ]);
    const spiral_pad = 8.0;
    const a1 = 0.9 * random();
    props["Noise"] = scoring(a1, [
      "Very Small",
      0.05,
      "Small",
      0.25,
      "Medium",
      0.6,
      "High",
    ]);
    const a2 = 0.2 + random() * random();
    const a3 = 0.2 + random() * random();
    props["Noise Aggressivity"] = scoring(a2 * a3, [
      "Low",
      0.1,
      "Medium",
      0.5,
      "High",
      1,
      "Extreme",
    ]);
    const yfactor = random() * random();
    props["Vertical Separation"] =
      primary === secondary
        ? "NA"
        : scoring(yfactor, [
            "Low",
            0.3,
            "Medium",
            0.6,
            "High",
            0.8,
            "Very High",
          ]);
    const seed = 1000 * random();
    const desired_count = Math.ceil(300 - max_scale);
    const samples = 5200;
    const particle_size = 30;
    const opts = {
      seed,
      max_scale,
      desired_count,
      samples,
      particle_size,
      fading,
      gravity_dist,
      spiral_pad,
      a1,
      a2,
      a3,
      f1,
      f2,
      f3,
      yfactor,
      primary_name: primary.name,
      secondary_name: secondary.name,
    };

    // eslint-disable-next-line no-undef
    if (process.env.NODE_ENV !== "production") {
      console.log(window.fxhash);
      Object.keys(props).forEach((key) => console.log(key + " =", props[key]));
    }
    return {
      opts,
      primary,
      secondary,
      paperSeed,
      props,
    };
  }, []);
}

function useAttributes(attributesRef, variables) {
  useEffect(() => {
    attributesRef.current = () => variables.props;
  }, [variables]);
}

const Paper = ({ seed, grain }) => (
  <Node
    shader={shaders.paper}
    uniforms={{ seed, grain, resolution: Uniform.Resolution }}
  />
);

const Post = ({
  size,
  children,
  variables: { primary, secondary, paperSeed },
}) => {
  return (
    <Node
      {...size}
      shader={shaders.main}
      uniforms={{
        t: children,
        paper: <Paper seed={paperSeed} grain={256} />,
        grainAmp: 0.1,
        primary: primary.main,
        primaryHighlight: primary.highlight,
        secondary: secondary.main,
        secondaryHighlight: secondary.highlight,
      }}
    />
  );
};

const shaders = Shaders.create({
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
      for( int i=0; i<3; i++ ) {
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
      float t = fbm(seed + p * grain + 1.0 * fbm(-4.0 * p * grain));
      t = smoothstep(0.0, 0.1, abs(t-0.5)-0.01);
      gl_FragColor = vec4(vec3(t), 1.0);
    }`,
  },
  main: {
    frag: GLSL`
    precision highp float;
    varying vec2 uv;
    uniform float grainAmp;
    uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight;
    uniform sampler2D t, paper;
    vec3 pal(float t, vec3 c1, vec3 c2){
      float m = smoothstep(0.3, 0.15, t);
      return mix(
        vec3(1.0, 1.0, 1.0),
        mix(c1, c2, m),
        smoothstep(1.0, 0.5, t)
      );
    } 
    void main() {
      vec4 v = texture2D(t, uv);
      float grain = texture2D(paper, uv).r;
      vec3 c1 = pal(v.r, primary, primaryHighlight);
      vec3 c2 = pal(v.g, secondary, secondaryHighlight);
      vec3 c =
        c1 * c2 +
        grainAmp * /*developed by @greweb*/
        (0.6 + 0.4 * mix(1.0, 0.0, step(0.5, grain))) *
        (grain - 0.5);
      gl_FragColor = vec4(c, 1.0);
    }
  `,
  },
});

export default Main;
