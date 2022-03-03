// @flow
import React, { useEffect, useMemo, useState } from "react";
import { Surface } from "gl-react-dom";
import { GLSL, LinearCopy, Node, Shaders, Uniform } from "gl-react";
import config from "./config";

const MAX = 4096;
const ratio = config.width / config.height;
const svgSize = [config.width + "mm", config.height + "mm"];
const widths = [500, 1000, 2000];

function useFetchSvg(path) {
  const [svg, setSvg] = useState("");
  useEffect(() => {
    fetch(path)
      .then((r) => r.text())
      .then(setSvg);
  }, []);
  return svg;
}

function adaptiveSvgWidth(width) {
  let i = 0;
  let w;
  do {
    w = widths[i];
    i++;
  } while (i < widths.length && width > widths[i]);
  return w;
}

const Main = ({ width, height }) => {
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

  const svg = useFetchSvg("image.svg");

  const renderedSVG = useMemo(
    () =>
      "data:image/svg+xml;base64," +
      btoa(
        svg
          .replace(svgSize[1], heightPx)
          .replace(svgSize[0], widthPx)
          .replaceAll("mix-blend-mode: multiply;", "opacity:" + config.opacity)
          .replaceAll(config.primaryMatch, "#0FF")
          .replaceAll(config.secondaryMatch, "#F0F")
          .replaceAll(config.thirdMatch, "#FF0")
      ),
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
            background: "white",
          }}
        >
          <Surface width={W} height={H}>
            <LinearCopy>
              <Post ready={!!svg} size={{ width: w, height: h }}>
                {renderedSVG}
              </Post>
            </LinearCopy>
          </Surface>
        </div>
      </div>
    </div>
  );
};

const Paper = ({ seed, grain }) => (
  <Node
    shader={shaders.paper}
    uniforms={{ seed, grain, resolution: Uniform.Resolution }}
  />
);

const PaperCache = React.memo(Paper);

const Post = ({ size, children }) => {
  const primary = config.colors[0] || config.colors[0];
  const secondary = config.colors[1] || config.colors[0];
  const third = config.colors[2] || config.colors[0];
  const paperSeed = Math.random();
  return (
    <Node
      {...size}
      shader={shaders.main}
      uniforms={{
        t: children,
        paper: <PaperCache width={size.width} seed={paperSeed} grain={100} />,
        time: 0,
        seed: paperSeed,
        primary: primary.main,
        primaryHighlight: primary.highlight,
        secondary: secondary.main,
        secondaryHighlight: secondary.highlight,
        third: third.main,
        thirdHighlight: third.highlight,
        grainAmp: 0.15,
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
    uniform float grainAmp, seed;
    uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight, third, thirdHighlight;
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
      vec2 q = p.xy - .5;
      float d = length(q);
      vec4 g = texture2D(paper, p);
      float grain = g.r;
      vec4 v = texture2D(t, p);
      vec3 c1 = pal(v.r, primary, primaryHighlight);
      vec3 c2 = pal(v.g, secondary, secondaryHighlight);
      vec3 c3 = pal(v.b, third, thirdHighlight);
      vec3 c =
        min(vec3(1.), c1 * c2 * c3) +
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
