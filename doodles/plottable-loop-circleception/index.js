// @flow
/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Plottable Circleception
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

const fps = 16;
const grid = [4, 2];
const cellSize = [70, 70];
const svgSize = [grid[0] * cellSize[0], grid[1] * cellSize[1]];
const svgOutSize = [297, 210];
const frames = grid[0] * grid[1];
const MAX = 4096;
const ratio = cellSize[0] / cellSize[1];
const svgMMSize = svgSize.map((s) => s + "mm");

// for mobile phone, we need 2
const resolutionDiv = screen?.width < 1000 ? 2 : 1;

let adaptiveSvgWidth = (width) =>
  Math.max(64, Math.ceil(width / 64) * 64) / resolutionDiv;

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
  const widthPx = grid[0] * svgW + "px";
  const heightPx = Math.floor((grid[1] * svgW) / ratio) + "px";

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
        (result.length / (1024 * 1024)).toFixed(2) +
        " Mb"
    );
    window.$fxhashFeatures = generateVariables.inferProps(variables, result);
    if (console && console.table) console.table(window.$fxhashFeatures);
    return result;
  }, [variables.opts, loaded]);

  const renderedSVG = useMemo(
    () =>
      "data:image/svg+xml;base64," +
      btoa(svg.replace(svgMMSize[1], heightPx).replace(svgMMSize[0], widthPx)),
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
              <Post
                ready={!!svg}
                size={{ width: w, height: h }}
                variables={variables}
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
      const defaultViewbox = "0 0 " + svgSize.join(" ");
      const defaultWidth = svgMMSize[0];
      const defaultHeight = svgMMSize[1];
      const newWidth = svgOutSize[0] + "mm";
      const newHeight = svgOutSize[1] + "mm";

      const padx = (svgOutSize[0] - svgSize[0]) / 2;
      const pady = (svgOutSize[1] - svgSize[1]) / 2;
      const newViewbox = `0 0 ` + svgOutSize.join(" ");

      let svgOut = svg
        .replace(defaultViewbox, newViewbox)
        .replace(defaultWidth, newWidth)
        .replace(defaultHeight, newHeight)
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

      const routes = [];
      for (let y = 0; y <= svgSize[1]; y += cellSize[1]) {
        routes.push([
          [0, y],
          [svgSize[0], y],
        ]);
      }
      for (let x = 0; x <= svgSize[0]; x += cellSize[0]) {
        routes.push([
          [x, 0],
          [x, svgSize[1]],
        ]);
      }
      const gridPath = routes
        .map((route) =>
          route.map(([x, y], i) => `${i === 0 ? "M" : "L"}${x},${y}`).join(" ")
        )
        .join(" ");
      const gridSvg = `<g fill="none" inkscape:groupmode="layer" inkscape:label="Grid" stroke="#0FF" stroke-width="0.35"><path d="${gridPath}"/></g>`;

      const iFrom = svgOut.indexOf(">") + 1;
      svgOut =
        svgOut.slice(0, iFrom) +
        `<g transform="translate(${padx},${pady})">` +
        gridSvg +
        svgOut.slice(iFrom);

      const iTo = svgOut.indexOf("</svg>");
      svgOut = svgOut.slice(0, iTo) + "</g>" + svgOut.slice(iTo);

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
        );

      setURI("data:image/svg+xml;base64," + btoa(svgOut));
    }, 500);
    return () => clearTimeout(timeout);
  }, [svg, primary, secondary]);
  return <img style={dlStyle} src={uri} />;
}

function useVariables({ random }) {
  return useMemo(() => generateVariables(random, window.fxhash), []);
}

const Paper = ({ seed, grain, blackPaper }) => (
  <Node
    shader={blackPaper ? shaders.paperBlack : shaders.paper}
    uniforms={{ seed, grain, resolution: Uniform.Resolution }}
  />
);

const PaperCache = React.memo(Paper);

function useTime(ready, frames, fps) {
  const [i, setI] = useState(0);
  useEffect(() => {
    if (!ready) return;
    let startT;
    let h;
    function loop(t) {
      h = requestAnimationFrame(loop);
      if (!startT) startT = t;
      const time = ((fps / frames) * (t - startT)) / 1000;
      const i = Math.floor(time * frames) % frames;
      setI(i);
    }
    h = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(h);
  }, [ready]);
  return i;
}

const Post = ({
  ready,
  size,
  children,
  variables: { primary, secondary, paperSeed, randoms },
}) => {
  const i = useTime(ready, frames, fps);
  const crop = useMemo(() => {
    const y = Math.floor(i / grid[0]) / grid[1];
    let xi = i % grid[0];
    if (y % 2 === 0) {
      xi = grid[0] - xi - 1;
    }
    let x = xi / grid[0];
    return [x, y, 1 / grid[0], 1 / grid[1]];
  }, [i]);

  const grainAmp = 0.13;
  const lighting = 0.11;
  const baseColor = [
    0.03 * randoms[i % randoms.length] - 0.02,
    0.015 * randoms[(i + 8) % randoms.length] - 0.02,
    0.02 * randoms[(i + 16) % randoms.length] - 0.02,
  ];
  const seed = paperSeed + 13.7 * i;
  const background = primary.bg;

  const uniforms = {
    t: children,
    crop,
    paper: (
      <PaperCache
        width={size.width}
        seed={seed}
        grain={0.5 * cellSize[0]}
        blackPaper={!!primary.blackPaper}
      />
    ),
    primary: primary.main,
    primaryHighlight: primary.highlight,
    secondary: secondary.main,
    secondaryHighlight: secondary.highlight,
    grainAmp,
    lighting,
    baseColor,
  };

  if (background) {
    uniforms.background = background;
  }

  return (
    <Node
      {...size}
      shader={primary.blackPaper ? shaders.mainBlack : shaders.main}
      uniforms={uniforms}
    />
  );
};

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
// https://www.iquilezles.org/www/articles/voronoilines/voronoilines.htm
float voronoiDistance(in vec2 x) {
ivec2 p = ivec2(floor(x));vec2  f = fract(x);
ivec2 mb;vec2 mr;
float res = 8.0;
for( int j=-1; j<=1; j++ )
for( int i=-1; i<=1; i++ ){
  ivec2 b = ivec2(i, j);vec2  r = vec2(b) + hash(vec2(p+b))-f;float d = dot(r,r);
  if( d < res ) { res = d; mr = r; mb = b; }
}
res = 8.0;
for( int j=-2; j<=2; j++ )
for( int i=-2; i<=2; i++ ) {
  ivec2 b = mb + ivec2(i, j);
  vec2  r = vec2(b) + hash(vec2(p+b)) - f;
  float d = dot(0.5*(mr+r), normalize(r-mr));
  res = min( res, d );
}
return res;
}
float paper(vec2 p, float z, float seed) {
  pR(p, seed);
  p += seed;
  float a = smoothstep(0.02, 0.16, 0.13 * fbm(seed + 0.3 * p * z) + voronoiDistance(0.5 * z * p + 3.3 * seed));
  float b = smoothstep(0.0, 0.15, abs(fbm(-2.0 * p * z - seed)-0.5)-0.01);
  return 0.4 * b + 0.6 * a;
}

void main () {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 p = 0.5 + (uv - 0.5) * ratio;
  float t = paper(p, grain, seed);
  gl_FragColor = vec4(t, t, t, 1.0);
}`,
  },
  paperBlack: {
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
    uniform vec4 crop;
    uniform float grainAmp, lighting;
    uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight;
    uniform sampler2D t, paper;
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
      float d = abs(p.y);
      float gain = smoothstep(0.2, 0.0, abs(fract(d) - 0.2));
      vec4 g = texture2D(paper, p);
      float grain = g.r;
      vec4 v = texture2D(t, p * crop.zw + crop.xy);
      vec3 c1 = pal(v.r, primary, primaryHighlight);
      vec3 c2 = pal(v.g, secondary, secondaryHighlight);
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
    uniform vec4 crop;
    uniform float grainAmp, lighting, seed;
    uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight;
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
      float d = abs(p.y);
      float gain = smoothstep(0.2, 0.0, abs(fract(d) - 0.2));
      vec4 g = texture2D(paper, p);
      float grain = g.r;
      vec4 v = texture2D(t, p * crop.zw + crop.xy);
      vec3 c1 = pal(v.r, primary, primaryHighlight);
      vec3 c2 = pal(v.g, secondary, secondaryHighlight);
      vec3 c =
      (c1 + c2) * (1. + lighting * gain) +
        grainAmp * grain +
        baseColor;
      gl_FragColor = vec4(c, 1.0);
    }
  `,
  },
});

export default Main;
