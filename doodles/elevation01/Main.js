import React, { useCallback, useEffect, useMemo, useState } from "react";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, Uniform, LinearCopy } from "gl-react";
import MersenneTwister from "mersenne-twister";

const promiseOfLoad = import("./rust/pkg");
let render;
promiseOfLoad.then((r) => {
  render = r.render;
});

const COLORS = [
  {
    name: "Black",
    main: [0.2, 0.2, 0.2],
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

const MAX = 2048;

const choices = [
  { id: 0, text: "low", freq: 0.3, amp: 1.2 },
  { id: 1, text: "med", freq: 0.5, amp: 1.0 },
  { id: 2, text: "high", freq: 0.7, amp: 0.8 },
  { id: 3, text: "ultra", freq: 1.4, amp: 0.4 },
];

const Main = ({ width, height, viewer, helpOn, setHelpOn }) => {
  const dpr = window.devicePixelRatio || 1;
  const min = Math.min(width, height);
  const w = Math.min(MAX, Math.floor(dpr * min));
  const h = Math.min(MAX, Math.floor(dpr * min));
  const [hover, setHover] = useState(false);
  const [loaded, setLoaded] = useState(false);

  const [r1, r2] = useMemo(() => {
    if (!viewer) return [0, 0];
    const rng = new MersenneTwister([...viewer].map((c) => c.charCodeAt(0)));
    return [rng.random(), rng.random()];
  }, [viewer]);

  const [selectedId, setSelectedId] = useState(Math.floor(r1 * choices.length));
  const [colorI, setColorI] = useState(Math.floor(COLORS.length * r2));

  const opts = useMemo(
    () => ({
      seed: r1 * 100,
      freq: choices[selectedId].freq,
      amp: choices[selectedId].amp,
    }),
    [r1, selectedId]
  );

  const primary = COLORS[colorI];

  const onMouseEnter = useCallback(() => setHover(true), []);
  const onMouseLeave = useCallback(() => setHover(false), []);

  useEffect(() => {
    if (!loaded) promiseOfLoad.then(() => setLoaded(true));
  }, [loaded]);

  const svgBody = useMemo(() => {
    if (!loaded) return "";
    let prev = Date.now();
    const result = render(opts);
    console.log("svg calc time = " + (Date.now() - prev) + "ms");
    return result;
  }, [opts, loaded]);

  const onClickHelp = useCallback(() => {
    setHelpOn((v) => !v);
  }, []);

  const imgSrc = useMemo(
    () =>
      "data:image/svg+xml;base64," +
      btoa(
        `
        <svg xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" xmlns="http://www.w3.org/2000/svg" width="${MAX}px" height="${MAX}px" style="background:white" viewBox="0 0 200 200">` +
          svgBody +
          "</svg>"
      ),
    [svgBody, MAX]
  );

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
            ) +
          "</g></svg>"
      ),
    [svgBody, primary]
  );

  let download = `mountains-circle-${viewer || ""}.svg`;

  const color =
    "rgb(" + primary.main.map((a) => Math.floor(a * 255)).join(",") + ")";

  return (
    <div
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      style={{
        width: "100vw",
        height: "100vh",
        position: "relative",
      }}
    >
      <header>
        <a onClick={onClickHelp} className="help">
          ?
        </a>
      </header>
      <div
        style={{
          width,
          height,
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        <Surface width={min} height={min}>
          <LinearCopy>
            <Post size={{ width: w, height: h }} primary={primary}>
              {imgSrc}
            </Post>
          </LinearCopy>
        </Surface>
      </div>
      <footer
        style={{
          opacity: hover && !helpOn ? 1 : 0,
        }}
      >
        <a
          onClick={() => setSelectedId((i) => (i + 1) % choices.length)}
          className="choice"
        >
          {choices[selectedId].text}
        </a>

        <a
          onClick={() => setColorI((i) => (i + 1) % COLORS.length)}
          className="color"
          style={{ color: color, borderColor: color }}
        >
          {primary.name}
        </a>

        <a className="download" download={download} href={dlSrc}>
          {"SVG"}
        </a>
      </footer>
    </div>
  );
};

const Paper = ({ seed, grain }) => (
  <Node
    shader={shaders.paper}
    uniforms={{ seed, grain, resolution: Uniform.Resolution }}
  />
);

const Post = ({ size, children, primary }) => {
  return (
    <Node
      {...size}
      shader={shaders.main}
      uniforms={{
        t: children,
        paper: <Paper seed={0.2} grain={256} />,
        grainAmp: 0.08,
        resolution: Uniform.Resolution,
        primary: primary.main,
        primaryHighlight: primary.highlight,
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
  main: {
    frag: GLSL`
    precision highp float;
    varying vec2 uv;
    uniform float grainAmp;
    uniform vec2 resolution;
    uniform vec3 primary, primaryHighlight;
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
      vec2 ratio = resolution / min(resolution.x, resolution.y);
      vec2 p = 0.5 + (uv - 0.5) * ratio;
      vec4 v = texture2D(t, p);
      float grain = texture2D(paper, p).r;
      vec3 c1 = pal(v.r, primary, primaryHighlight);
      vec3 c = mix(
        vec3(1.0),
        c1 +
        grainAmp *
        (0.6 + 0.4 * mix(1.0, 0.0, step(0.5, grain))) *
        (grain - 0.5),
        step(0.0, p.x) * step(p.x, 1.0) * step(0.0, p.y) * step(p.y, 1.0)
      );
      gl_FragColor = vec4(c, 1.0);
    }
  `,
  },
});

export default Main;
