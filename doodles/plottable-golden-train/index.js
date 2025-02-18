// @flow
/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Golden train
 */
import React, { useEffect, useMemo, useState } from "react";
import { Surface } from "gl-react-dom";
import { GLSL, LinearCopy, Node, Shaders, Uniform } from "gl-react";
import init, { render } from "./rust/pkg/main";
import wasm from "base64-inline-loader!./rust/pkg/main_bg.wasm";
import generateVariables from "./variables";

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

const svgSize = [297, 210];
const MAX = 4096;
const ratio = svgSize[0] / svgSize[1];
const svgMMSize = svgSize.map((s) => s + "mm");

let adaptiveSvgWidth = (width) => Math.max(64, Math.ceil(width / 64) * 64);

/**
 * Set the NFT traits. Attributes are the NFT attributes seen on marketplaces. Traits are optional numeric
 * representations of attributes to expose to public works.
 */
const setProperties = (attributes, traits = {}) => {
  setWindowProperties("attributes", attributes);
  setWindowProperties("traits", traits);
};

const Main = ({ width, height, random }) => {
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
  const svgW = adaptiveSvgWidth(w);
  const widthPx = svgW + "px";
  const heightPx = Math.floor(svgW / ratio) + "px";

  const [loaded, setLoaded] = useState(wasmLoaded);
  const variables = useVariables({ random });

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
        (result.length / (1024 * 1024)).toFixed(3) +
        " Mb"
    );
    const props = generateVariables.inferProps(variables, result);
    const properties = {};
    const traits = {};
    for (const [k, v] of Object.entries(props)) {
      if (typeof v === "number") {
        traits[k] = v;
      } else {
        properties[k] = v;
      }
    }
    setProperties(properties, traits);
    if (console && console.table) {
      console.table(properties);
      console.table(traits);
    }
    setTimeout(() => setPreviewReady(), 5000);
    return result;
  }, [variables.opts, loaded]);

  const renderedSVG = useMemo(
    () =>
      "data:image/svg+xml;base64," +
      btoa(svg.replace(svgMMSize[1], heightPx).replace(svgMMSize[0], widthPx)),
    [svg, widthPx, heightPx]
  );

  const chimneyPosition = useMemo(() => {
    const match = svg && svg.match(/data-chimney-position="([^"]+)"/);
    if (!match) return [0, 0];
    const [x, y] = match[1].split(",").map(parseFloat);
    return [x, 1 - y];
  }, [svg]);

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
            lineHeight: 0,
          }}
        >
          <Surface
            width={W}
            height={H}
            webglContextAttributes={{
              preserveDrawingBuffer: true,
            }}
          >
            <LinearCopy>
              <Post
                ready={!!svg}
                size={{ width: w, height: h }}
                variables={variables}
                chimneyPosition={chimneyPosition}
              >
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
      let svgOut = svg
        .replace(
          "background:white",
          `background:${
            primary.bg
              ? "rgb(" +
                primary.bg.map((c) => Math.floor(c * 255)).join(",") +
                ")"
              : "white"
          }`
        )
        .replace(/opacity="[^"]*"/g, 'style="mix-blend-mode: multiply"');

      svgOut = svgOut
        .replace(
          /#0FF/g,
          "rgb(" + primary.main.map((n) => Math.round(n * 255)).join(",") + ")"
        )
        .replace(
          /#F0F/g,
          "rgb(" +
            secondary.main.map((n) => Math.round(n * 255)).join(",") +
            ")"
        )
        .replace(
          /#FF0/g,
          "rgb(" +
            secondary.main.map((n) => Math.round(n * 255)).join(",") +
            ")"
        );

      setURI("data:image/svg+xml;base64," + btoa(svgOut));
    }, 500);
    return () => clearTimeout(timeout);
  }, [svg, primary, secondary]);
  return <img style={dlStyle} src={uri} />;
}

function useVariables({ random }) {
  return useMemo(
    () =>
      generateVariables(
        random,
        new URLSearchParams(window.location.search).get("hash") || "",
        new URLSearchParams(window.location.search).get("debug") === "1"
      ),
    []
  );
}

const Paper = ({ seed, grain }) => (
  <Node
    shader={shaders.paper}
    uniforms={{ seed, grain, resolution: Uniform.Resolution }}
  />
);

const PaperCache = React.memo(Paper);

function useTime(ready, fps = 60) {
  const [time, setTime] = useState(0);
  useEffect(() => {
    if (!ready) return;
    let startT;
    let h;
    function loop(t) {
      h = requestAnimationFrame(loop);
      if (!startT) startT = t;
      setTime(Math.floor((fps * (t - startT)) / 1000));
    }
    h = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(h);
  }, [ready]);
  return time;
}

const Post = ({
  ready,
  size,
  children,
  variables: { primary, secondary, paperSeed },
  chimneyPosition,
}) => {
  const time = useTime(ready, 30) / 30;

  const grainAmp = primary.blackPaper ? 0.05 : 0.13;
  const lighting = 0.1;
  const baseColor = [-0.003, -0.006, -0.01];
  const seed = paperSeed;
  const background = primary.bg;

  return (
    <Node
      {...size}
      shader={primary.blackPaper ? shaders.mainBlack : shaders.main}
      uniforms={{
        t: children,
        paper: <PaperCache width={size.width} seed={seed} grain={100} />,
        primary: primary.main,
        primaryHighlight: primary.highlight,
        secondary: secondary.main,
        secondaryHighlight: secondary.highlight,
        grainAmp,
        lighting,
        baseColor,
        time,
        chimneyPosition,
        ...(background ? { background } : {}),
      }}
    />
  );
};

const gainShader = `
float gain = smoothstep(.5,1.,abs(cos(${0.5 * Math.PI}*(p.x+time))));
`;

const sharedVariables = `
vec4 g = texture2D(paper, p);
float grain = g.r;
vec4 v = texture2D(t, p);
float motion = .003*smoothstep(0.,.05,length(p-chimneyPosition))*sin(-${
  0.5 * Math.PI
}*(time+p.x-3.*p.y+4.*cos(3.*p.x-10.*p.y)));
${/* float phase = cos(5.*p.x+time+cos(p.x-6.*p.y));*/ ""}
vec4 v2 = texture2D(t, p+vec2(motion,0.));
vec3 c1 = pal(v.r,primary, primaryHighlight);
vec3 c2 = pal(min(v.g,v2.b), secondary, secondaryHighlight);
`;

const clrmask = `v.r * v.g * v2.b`;
// mix(v2.b,1.0,smoothstep(0.45,0.51,abs(phase)))

const shaders = Shaders.create({
  paper: {
    frag: `precision highp float;
    varying vec2 uv;
    uniform vec2 resolution;
    uniform float grain, seed;
    

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
float paper(vec2 p, float z, float seed) {
  pR(p, seed);
  p += seed;
  float n = 0.7 * smoothstep(-0.1, 0.2, abs(fbm(-3.0 * p * z - seed)-0.5)-0.01) +
  0.3 * smoothstep(0.0, 0.1, abs(fbm(-6.0 * p * z + seed)-0.5)-0.01);
  return n;
}

void main () {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 p = 0.5 + (uv - 0.5) * ratio;
  float t = paper(p, grain, seed);
  gl_FragColor = vec4(t, t, t, 1.0);
}`,
  },
  main: {
    frag: GLSL`
    precision highp float;
    varying vec2 uv;
    uniform vec3 baseColor;
    uniform float grainAmp, lighting, time;
    uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight;
    uniform sampler2D t, paper;
    uniform vec2 chimneyPosition;
    vec3 pal(float t, vec3 c1, vec3 c2){
      float m = smoothstep(0.3, 0.0, t);
      return mix(
        vec3(1.0, 1.0, 1.0),
        mix(c1, c2, m),
        smoothstep(1.0, 0.5, t)
      );
    } 
    void main() {
      vec2 p = uv;
      ${gainShader}
      ${sharedVariables}
      vec3 c =
        min(vec3(1.), c1 * c2 * (1. + lighting * gain)) +
        grainAmp * /*developed by @greweb*/
        mix(1.0, 0.5, step(0.5, grain)) *
        (0.5 - grain) +
        baseColor;
      gl_FragColor = vec4(c, 1.0);
    }
  `,
  },
  mainBlack: {
    frag: GLSL`
    precision highp float;
    varying vec2 uv;
    uniform vec3 baseColor, background;
    uniform float grainAmp, lighting, time;
    uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight;
    uniform sampler2D t, paper;
    uniform vec2 chimneyPosition;
    vec3 pal(float t, vec3 c1, vec3 c2) {
      float m = smoothstep(0.3, 0.0, t);
      return mix(
        vec3(0.0),
        mix(c1, c2, m),
        smoothstep(1.0, 0.5, t)
      );
    } 
    void main() {
      vec2 p = uv;
      ${gainShader}
      ${sharedVariables}
      vec3 c =
      (c1 + c2) * (1. + lighting * gain) +
        grainAmp * grain +/*developed by @greweb*/
        baseColor +
        background * smoothstep(0.5, 1.0, ${clrmask});
      gl_FragColor = vec4(c, 1.0);
    }
  `,
  },
});

export default Main;
