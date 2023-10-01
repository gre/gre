// @flow
/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Spirals
 */
import React, { useEffect, useMemo, useState } from "react";
import { Surface } from "gl-react-dom";
import { GLSL, LinearCopy, Node, Shaders, Uniform } from "gl-react";
import Color from "color";
import init, { render } from "./pkg/main";
import wasm from "base64-inline-loader!./pkg/main_bg.wasm";
import { generateVariables, inferProps, inferPalette } from "./variables";
import { width, height } from "./constants";

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

const svgSize = [width, height];
const MAX = 4096;
const ratio = svgSize[0] / svgSize[1];
const svgMMSize = svgSize.map((s) => s + "mm");

let adaptiveSvgWidth = (width) => Math.max(64, Math.ceil(width / 64) * 64);

const Main = ({ width, height, setProperties, hash, random }) => {
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
  const variables = useVariables({ hash, random });

  useEffect(() => {
    if (!loaded) promiseOfLoad.then(() => setLoaded(true));
  }, [loaded]);

  const svg = useMemo(() => {
    if (!loaded) return "";
    if (!variables) return "";
    if (variables.error) {
      console.error("Please try again", variables.error);
      return "";
    }
    let prev = Date.now();
    const result = render(variables.value);
    console.log(
      "svg calc time = " +
      (Date.now() - prev) +
      "ms – " +
      (result.length / (1024 * 1024)).toFixed(3) +
      " Mb"
    );
    const props = inferProps(result);
    setProperties(props);
    return result;
  }, [variables, loaded]);

  const palette = useMemo(() => svg && inferPalette(svg), [svg]);

  const renderedSVG = useMemo(
    () =>
      "data:image/svg+xml;base64," +
      btoa(svg.replace(svgMMSize[1], heightPx).replace(svgMMSize[0], widthPx)),
    [svg, widthPx, heightPx]
  );

  if (!svg || !palette) return null;

  const background = palette.paper[1];

  return (
    <div
      style={{
        width,
        height,
        position: "relative",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        background
      }}
    >
      <div style={{ position: "relative", width: W, height: H }}>
        <div
          style={{
            zIndex: 1,
            position: "relative",
            pointerEvents: "none",
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
                palette={palette}
              >
                {renderedSVG}
              </Post>
            </LinearCopy>
          </Surface>
        </div>
        <Downloadable svg={svg} palette={palette} />
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
function Downloadable({ svg, palette }) {
  const [uri, setURI] = useState(null);
  useEffect(() => {
    const timeout = setTimeout(() => {
      let svgOut = svg
        .replace("background:white", `background:${palette.paper[1]}`)
        .replace(/opacity="[^"]*"/g, 'style="mix-blend-mode: multiply"');

      svgOut = svgOut
        .replace(/#0FF/g, palette.primary[1])
        .replace(/#F0F/g, palette.secondary[1])
        .replace(/#FF0/g, palette.third[1]);

      setURI("data:image/svg+xml;base64," + btoa(svgOut));
    }, 500);
    return () => clearTimeout(timeout);
  }, [svg, palette]);
  return <img style={dlStyle} src={uri} />;
}

function useVariables({ hash, random }) {
  const promise = useMemo(() => generateVariables(hash, random), [hash]);
  const [result, setResult] = useState(null);
  useEffect(() => {
    setResult(null);
    promise.then(value => setResult({ value }), error => setResult({ error }));
  }, [promise]);
  return result;
}

const paperSeed = 10 * Math.random();

const Paper = ({ grain }) => (
  <Node
    shader={shaders.paper}
    uniforms={{ seed: paperSeed, grain, resolution: Uniform.Resolution }}
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

function useColorRGB(str) {
  return useMemo(() => {
    const c = new Color(str);
    return c
      .rgb()
      .array()
      .map((x) => x / 255);
  }, [str]);
}

const Post = ({ ready, size, children, palette }) => {
  const time = useTime(ready, 30) / 30;

  const isBlackPaper = palette.paper[2];
  const grainAmp = isBlackPaper ? 0.07 : 0.13;
  const lighting = isBlackPaper ? 0.2 : 0.05;
  const baseColor = [-0.003, -0.006, -0.01];
  const background = useColorRGB(palette.paper[1]);
  const primary = useColorRGB(palette.primary[1]);
  const secondary = useColorRGB(palette.secondary[1]);
  const third = useColorRGB(palette.third[1]);
  const primaryHighlight = useColorRGB(palette.primary[2]);
  const secondaryHighlight = useColorRGB(palette.secondary[2]);
  const thirdHighlight = useColorRGB(palette.third[2]);

  return (
    <Node
      {...size}
      shader={isBlackPaper ? shaders.mainBlack : shaders.main}
      uniforms={{
        t: children,
        paper: <PaperCache width={size.width} grain={100} />,
        primary,
        primaryHighlight,
        secondary,
        secondaryHighlight,
        third,
        thirdHighlight,
        grainAmp,
        lighting,
        baseColor,
        time,
        background,
      }}
    />
  );
};

const gainShader = `
float gain = smoothstep(0.3, 1.0, abs(cos(${Math.PI}*(length(p-0.5)-time))));
`;

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
    uniform vec3 baseColor, background;
    uniform float grainAmp, lighting, time;
    uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight, third, thirdHighlight;
    uniform sampler2D t, paper;
    vec3 pal(float t, vec3 c1, vec3 c2){
      float m = smoothstep(0.3, 0.0, t);
      return mix(
        background,
        mix(c1, c2, m),
        smoothstep(1.0, 0.5, t)
      );
    } 
    void main() {
      vec2 p = uv;
      ${gainShader}
      vec4 g = texture2D(paper, p);
      float grain = g.r;
      vec4 v = texture2D(t, p);
      vec3 c1 = pal(v.r, primary, primaryHighlight);
      vec3 c2 = pal(v.g, secondary, secondaryHighlight);
      vec3 c3 = pal(v.b, third, thirdHighlight);
      vec3 c =
        min(vec3(1.), c1 * c2 * c3 * (1. + lighting * gain)) +
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
    uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight, third, thirdHighlight;
    uniform sampler2D t, paper;
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
      vec4 g = texture2D(paper, p);
      float grain = g.r;
      vec4 v = texture2D(t, p);
      vec3 c1 = pal(v.r, primary, primaryHighlight);
      vec3 c2 = pal(v.g, secondary, secondaryHighlight);
      vec3 c3 = pal(v.b, third, thirdHighlight);
      vec3 c =
      (c1 + c2 + c3) * (1. + lighting * gain) +
        grainAmp * grain +/*developed by @greweb*/
        baseColor +
        background * smoothstep(0.5, 1.0, v.r * v.g);
      gl_FragColor = vec4(c, 1.0);
    }
  `,
  },
});

export default Main;
