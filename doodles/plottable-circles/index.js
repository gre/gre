// @flow
import React, { useEffect, useMemo, useState } from "react";
import { Surface } from "gl-react-dom";
import { GLSL, LinearCopy, Node, Shaders, Uniform } from "gl-react";
import init, { render } from "./rust/pkg/main";
import wasm from "base64-inline-loader!./rust/pkg/main_bg.wasm";
import generateVariables from "./variables";

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

const MAX = 4096;

const ratio = 210 / 210;
const svgSize = ["210mm", "210mm"];
const widths = [500, 1000, 2000];

function adaptiveSvgWidth(width) {
  let i = 0;
  let w;
  do {
    w = widths[i];
    i++;
  } while (i < widths.length && width > widths[i]);
  return w;
}

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
  const svgW = adaptiveSvgWidth(w);
  const widthPx = svgW + "px";
  const heightPx = Math.floor(svgW / ratio) + "px";

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

  const renderedSVG = useMemo(
    () =>
      "data:image/svg+xml;base64," +
      btoa(svg.replace(svgSize[1], heightPx).replace(svgSize[0], widthPx)),
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

function useVariables({ random }) {
  return useMemo(() => generateVariables(random), []);
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

const PaperCache = React.memo(Paper);

function useTime(ready) {
  const [time, setTime] = useState(0);
  useEffect(() => {
    if (!ready) return;
    let startT;
    let h;
    function loop(t) {
      h = requestAnimationFrame(loop);
      if (!startT) startT = t;
      setTime((t - startT) / 1000);
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
}) => {
  const time = useTime(ready);
  return (
    <Node
      {...size}
      shader={shaders.main}
      uniforms={{
        t: children,
        paper: <PaperCache width={size.width} seed={paperSeed} grain={100} />,
        time,
        seed: paperSeed,
        primary: primary.main,
        primaryHighlight: primary.highlight,
        secondary: secondary.main,
        secondaryHighlight: secondary.highlight,
        grainAmp: 0.15,
        lighting: 0.13,
        baseColor: [0, -0.005, -0.01],
      }}
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
  pR(p, 2.);
  float a = smoothstep(0.02, 0.16, 0.13 * fbm(seed + 0.3 * p * z) + voronoiDistance(0.5 * z * p));
  float b = smoothstep(0.0, 0.15, abs(fbm(-2.0 * p * z)-0.5)-0.01);
  return 0.4 * b + 0.6 * a;
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
    uniform float grainAmp, lighting, time, seed;
    uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight;
    uniform sampler2D t, paper;
    
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
      vec2 q = p.xy - .5;
      float d = length(q);
      float gain = smoothstep(0.2, 0.0, abs(fract(d - .5*time) - 0.2));
      vec4 g = texture2D(paper, p);
      float grain = g.r;
      vec4 v = mix(vec4(1.0), texture2D(t, p), smoothstep(d, d + 0.01, 0.5 * (time - 0.5)));
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
});

export default Main;
