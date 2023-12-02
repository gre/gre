/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
import paperGLSL from "./shaders/paper.glsl";
import mainGLSL from "./shaders/main.glsl";
import mainDarkGLSL from "./shaders/main-dark.glsl";

Promise.all([
  import("./pkg"),
  fetch("./PrinceValiant.ttf").then(r => r.arrayBuffer()).then(r => new Uint8Array(r)),
]).then(([rust, fontdata]) => {
  const hash = $fx.hash;
  const width = 210;
  const height = 297;
  const pad = 5;
  const precision = 0.2;
  const debug = new URLSearchParams(window.location.search).has("debug");

  // Generate the SVG

  const prev = Date.now();
  const svg = rust.render(hash, width, height, pad, precision, fontdata, true, debug);
  console.log("generated in " + (Date.now() - prev) + "ms");

  if (debug) {
    const perf = JSON.parse(svg.match("data-perf='([^']+)'")[1]);
    console.table(perf.per_label);
  }

  const palette = JSON.parse(svg.match("data-palette='([^']+)'")[1]);

  const props = {};
  const _props = JSON.parse(svg.match("data-traits='([^']+)'")[1]);
  for (let k in _props) {
    if (_props[k]) {
      props[camelCaseFeature(k)] = _props[k];
    }
  }
  $fx.features(props);

  const effectsHot = svg.match("data-effects-hot='([^']+)'")[1];
  const effectsWater = svg.match("data-effects-water='([^']+)'")[1];

  const effectsHotImg = new Image();
  effectsHotImg.src = effectsHot;
  const effectsWaterImg = new Image();
  effectsWaterImg.src = effectsWater;

  if (debug) {
    document.body.appendChild(effectsHotImg);
    document.body.appendChild(effectsWaterImg);
  }

  // Generate the WebGL

  let MAX = 4096;
  let ratio = width / height;
  let paperSeed = $fx.rand() * 999;
  let { ceil, min, max } = Math;
  let WINDOW = window;
  let LINEAR = "linear";
  let PX = (px) => px + "px";
  let MM = (mm) => mm + "mm";
  let TRIANGLE = [-2, 0, 0, -2, 2, 2];
  let DOC = document;
  let BODY = DOC.body;
  let assign = Object.assign;
  let ABSOLUTE = "absolute";
  let CENTER = "center";
  let HUNDREDPC = "100%";
  let sharedStyle = { position: ABSOLUTE, opacity: 0 };

  let createElement = (e) => DOC.createElement(e);
  let append = (n, e) => n.appendChild(e);
  let appendBody = (e) => append(BODY, e);
  let makeSVGDataImage = (svg) => "data:image/svg+xml;base64," + btoa(svg);
  let adaptiveSvgWidth = (width) => max(64, ceil(width / 64) * 64);

  let calcSizes = (width, height) => {
    let dpr = WINDOW.devicePixelRatio || 1;
    let W = width;
    let H = height;
    H = min(H, W / ratio) | 0;
    W = min(W, H * ratio) | 0;
    let w = min(MAX, dpr * W);
    let h = min(MAX, dpr * H);
    h = min(h, w / ratio) | 0;
    w = min(w, h * ratio) | 0;
    let svgW = adaptiveSvgWidth(w);
    let svgWidth = svgW;
    let svgHeight = (svgW / ratio) | 0;
    return [W, H, w, h, svgWidth, svgHeight];
  };

  assign(BODY.style, {
    display: "flex",
    alignItems: CENTER,
    justifyContent: CENTER,
    backgroundColor: palette.paper[1],
  });

  let container = createElement("div");

  let bgImage = createElement("img");
  bgImage.src = makeSVGDataImage(
    svg
      .replace("background:white", `background:${palette.paper[1]}`)
      .replace(/opacity="[^"]*"/g, 'style="mix-blend-mode: multiply"')
      .replace(/#0FF/g, palette.primary[1])
      .replace(/#F0F/g, palette.secondary[1])
      .replace(/#FF0/g, palette.third[1]),
  );
  assign(container.style, sharedStyle);
  assign(bgImage.style, {
    top: 0,
    left: 0,
    width: HUNDREDPC,
    height: HUNDREDPC,
    ...sharedStyle,
  });

  let V = ({ viewportWidth, viewportHeight }) => [
    viewportWidth,
    viewportHeight,
  ];

  let canvas = createElement("canvas");
  canvas.style.pointerEvents = "none";

  append(container, bgImage);
  appendBody(container);
  appendBody(canvas);

  let regl = createREGL(canvas);
  let prop = regl.prop;

  let vert = `precision mediump float;attribute vec2 p;varying vec2 uv;void main(){uv=p;gl_Position=vec4(2.*p-1.,0,1);}`;

  let framebuffer = regl.framebuffer();

  let paper = regl({
    framebuffer,
    frag: paperGLSL.sourceCode,
    vert,
    attributes: {
      p: TRIANGLE,
    },
    uniforms: {
      [paperGLSL.uniforms.seed.variableName]: paperSeed,
      [paperGLSL.uniforms.grain.variableName]: 100,
      [paperGLSL.uniforms.resolution.variableName]: V,
    },
    count: 3,
  });

  const isBlackPaper = palette.paper[2];
  const g = isBlackPaper ? mainDarkGLSL : mainGLSL;
  const grainAmp = isBlackPaper ? 0.07 : 0.13;
  const lighting = isBlackPaper ? 0.2 : 0.05;
  const baseColor = [-0.003, -0.006, -0.01];
  const effect = $fx.isPreview ? 0 : 1;

  let render = regl({
    frag: g.sourceCode,
    vert,
    attributes: {
      p: TRIANGLE,
    },
    uniforms: {
      [g.uniforms.time.variableName]: prop("T"),
      [g.uniforms.t.variableName]: prop("t"),
      [g.uniforms.waterT.variableName]: prop("waterT"),
      [g.uniforms.hotT.variableName]: prop("hotT"),
      [g.uniforms.effect.variableName]: effect,
      [g.uniforms.paper.variableName]: framebuffer,
      [g.uniforms.primary.variableName]: colorRgb(palette.primary[1]),
      [g.uniforms.primaryHighlight.variableName]: colorRgb(palette.primary[2]),
      [g.uniforms.secondary.variableName]: colorRgb(palette.secondary[1]),
      [g.uniforms.secondaryHighlight.variableName]: colorRgb(
        palette.secondary[2],
      ),
      [g.uniforms.third.variableName]: colorRgb(palette.third[1]),
      [g.uniforms.thirdHighlight.variableName]: colorRgb(palette.third[2]),
      [g.uniforms.grainAmp.variableName]: grainAmp,
      [g.uniforms.lighting.variableName]: lighting,
      [g.uniforms.baseColor.variableName]: baseColor,
      [g.uniforms.background.variableName]: colorRgb(palette.paper[1]),
    },
    count: 3,
  });

  let img = createElement("img");
  let lastPaperWidth;
  let lastSVGWidth;
  let txParam = (data) => ({ data, min: LINEAR, mag: LINEAR, flipY: true });

  let tex = regl.texture(txParam(img));
  let resize = (width, height) => {
    let [W, H, w, h, svgWidth, svgHeight] = calcSizes(width, height);
    canvas.width = w;
    canvas.height = h;
    container.style.width = canvas.style.width = PX(W);
    container.style.height = canvas.style.height = PX(H);
    if (lastPaperWidth !== w) {
      framebuffer.resize(w, h);
      paper();
    }
    if (lastSVGWidth !== svgWidth) {
      lastSVGWidth = svgWidth;
      img.onload = () => tex(txParam(img));
      img.src = makeSVGDataImage(svg);
      img.width = svgWidth;
      img.height = svgHeight;
    }
  };

  let waterT = regl.texture(txParam(effectsWaterImg));
  let hotT = regl.texture(txParam(effectsHotImg));
  effectsHotImg.onload = () => hotT(txParam(effectsHotImg));
  effectsWaterImg.onload = () => waterT(txParam(effectsWaterImg));

  let r = (onresize = () => resize(WINDOW.innerWidth, WINDOW.innerHeight));
  r();

  let startT;
  regl.frame(({ time }) => {
    if (!startT) startT = time;
    let T = time - startT;
    render({ T, t: tex, waterT, hotT });
  });

  // global helpers
  function camelCaseFeature(key) {
    let keyInCamelCase = "";
    let shouldUppercase = true;
    for (let i = 0; i < key.length; i++) {
      const c = key[i];
      if (shouldUppercase) {
        keyInCamelCase += c.toUpperCase();
        shouldUppercase = false;
      } else if (c === "_") {
        shouldUppercase = true;
        keyInCamelCase += " ";
      } else {
        keyInCamelCase += c;
      }
    }
    return keyInCamelCase;
  }

  function colorRgb(str) {
    let r, g, b;

    // Check for #rgb or #rrggbb format
    if (str[0] === "#") {
      if (str.length === 4) {
        // Convert #rgb to #rrggbb
        str = "#" + str[1] + str[1] + str[2] + str[2] + str[3] + str[3];
      }
      r = parseInt(str.substr(1, 2), 16);
      g = parseInt(str.substr(3, 2), 16);
      b = parseInt(str.substr(5, 2), 16);
    }
    // Check for rgb(r, g, b) format
    else if (str.startsWith("rgb(")) {
      const parts = str.match(/rgb\((\d+),\s*(\d+),\s*(\d+)\)/);
      [r, g, b] = parts.slice(1, 4).map(Number);
    }
    // Convert to normalized RGB values
    return [r, g, b].map((x) => x / 255);
  }
});
