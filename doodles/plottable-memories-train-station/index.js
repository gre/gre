/**
 * Plottable Memories: Train Station
 * @greweb 2022
 */
// constants
const PERF = false;
const DEBUG = false;
const WIDTH = 210;
const HEIGHT = 297;
const ratio = WIDTH / HEIGHT;
const pixelmapheight = 600;
const pixelmapwidth = Math.round(pixelmapheight * ratio);
const TRIANGLE = [-2, 0, 0, -2, 2, 2];
const LINEAR = "linear";
let MASKS = ["#0FF", "#F0F", "#FF0"];
const MAX_GL_SIZE = 4096;

// Colors
const blackPaper = [0.1, 0.1, 0.1];
const gelWhiteOnBlack = {
  name: "Gel White",
  main: [0.9, 0.9, 0.9],
  highlight: [1, 1, 1],
  blackPaper: true,
  bg: blackPaper,
  bgTag: "black",
};

const FOUNTAIN_PRIMARY_CHOICES = [
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
    main: [0.4, 0.5, 0.65],
    highlight: [0.2, 0.3, 0.4],
  },
  {
    name: "Amber",
    main: [1.0, 0.78, 0.28],
    highlight: [1.0, 0.5, 0.0],
  },
  {
    name: "Evergreen",
    main: [0.3, 0.4, 0.2],
    highlight: [0.15, 0.2, 0.1],
  },
  {
    name: "Amazing Amethyst",
    main: [0.6, 0.3, 0.7],
    highlight: [0.3, 0.1, 0.4],
  },
  {
    name: "Poppy Red",
    main: [0.9, 0.2, 0.1],
    highlight: [0.5, 0.0, 0.1],
  },
];

// Global choices with the seed

const paperSeed = 99 * fxrand();
const nbimages = (1 + 4 * fxrand()) | 0;
const totalimages = 65;
const titles = [
  "factory",
  "train",
  "building",
  "entrance",
  "entrance",
  "entrance",
  "ceil",
  "train",
  "ceil",
  "dock",
  "train",
  "track",
  "train",
  "clock",
  "clock",
  "train",
  "train",
  "train",
  "dock",
  "ceil",
  "track",
  "track travel",
  "track travel",
  "track travel",
  "bridge",
  "bridge",
  "track",
  "crane",
  "track travel",
  "holes building",
  "track travel",
  "building",
  "building",
  "ceil",
  "track travel",
  "train",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "track travel",
  "dock",
  "dock",
  "track travel",
  "track travel",
  "bridge",
  "building",
  "building",
  "track travel",
  "track travel",
  "train",
  "factory",
  "track travel",
];
const sources = Array(totalimages)
  .fill(null)
  .map((_, i) => [
    fxrand(),
    i,
    { src: `images/${i + 1}.jpg`, title: titles[i] },
  ])
  .sort((a, b) => 3 * (b[0] - a[0]) + (a[1] - b[1]) / totalimages)
  .map((o) => o[2])
  .slice(0, nbimages);

const primary =
  fxrand() < 0.3
    ? gelWhiteOnBlack
    : FOUNTAIN_PRIMARY_CHOICES[
        (fxrand() * fxrand() * FOUNTAIN_PRIMARY_CHOICES.length) | 0
      ];
const secondary = primary;

const traits = {
  Color: primary.name,
};

// helpers

const PX = (px) => px + "px";
function calcSizes(width, height) {
  let dpr = window.devicePixelRatio || 1;
  let W = width;
  let H = height;
  H = Math.min(H, W / ratio) | 0;
  W = Math.min(W, H * ratio) | 0;
  let w = Math.min(MAX_GL_SIZE, dpr * W);
  let h = Math.min(MAX_GL_SIZE, dpr * H);
  h = Math.min(h, w / ratio) | 0;
  w = Math.min(w, h * ratio) | 0;
  let svgW = adaptiveSvgWidth(w);
  let svgWidth = svgW;
  let svgHeight = (svgW / ratio) | 0;
  return [W, H, w, h, svgWidth, svgHeight];
}
function adaptiveSvgWidth(width) {
  return Math.max(64, Math.ceil(width / 64) * 64);
}

async function loadImage({ src, title }) {
  return new Promise((success, error) => {
    const img = new Image();
    img.onerror = error;
    img.onload = () => {
      success({
        img,
        imgdata: null,
        title,
      });
    };
    img.src = src;
  });
}

async function loadImages(source) {
  return Promise.all(source.map(loadImage)).then((images) =>
    images.map((o) => {
      const { img } = o;
      const c = document.createElement("canvas");
      c.width = img.width;
      c.height = img.height;
      const ctx = c.getContext("2d");
      ctx.drawImage(img, 0, 0, c.width, c.height);
      let offset = Math.max(0, fxrand() * fxrand() * fxrand() - 0.1);
      const x1 = (0.49 * offset * img.width) | 0;
      const y1 = (0.49 * offset * img.height) | 0;
      offset = Math.max(0, fxrand() * fxrand() * fxrand() - 0.1);
      const x2 = ((1 - offset) * (img.width - x1)) | 0;
      const y2 = ((1 - offset) * (img.height - y1)) | 0;
      return {
        ...o,
        imgdata: ctx.getImageData(x1, y1, x2, y2),
      };
    })
  );
}

function lookupImage(
  {
    bordersmooth,
    bordereffectpad,
    bordereffect,
    lookupmode,
    img: {
      imgdata: { width, height, data },
    },
  },
  x,
  y
) {
  let overrides = 0;
  switch (lookupmode) {
    case "x-repeat": {
      x = (x + 1000) % 1;
      overrides = smoothstep(bordersmooth, -0.0001, Math.min(y, 1 - y));
      break;
    }
    case "y-repeat": {
      y = (y + 1000) % 1;
      overrides = smoothstep(bordersmooth, -0.0001, Math.min(x, 1 - x));
      break;
    }
    case "repeat": {
      x = (x + 1000) % 1;
      y = (y + 1000) % 1;
      break;
    }
    case "mirror": {
      const xsafe = x + 1000;
      const ysafe = y + 1000;
      const ax = Math.floor(xsafe);
      const ay = Math.floor(ysafe);
      x = (xsafe + 1000) % 1;
      y = (ysafe + 1000) % 1;
      if (ax % 2 == 1) {
        x = 1 - x;
      }
      if (ay % 2 == 1) {
        y = 1 - y;
      }
      break;
    }
    default: {
      overrides = smoothstep(
        bordersmooth,
        -0.0001,
        Math.min(Math.min(x, 1 - x), Math.min(y, 1 - y))
      );
    }
  }
  if (x < 0 || x > 1 || y < 0 || y > 1) return 1;
  switch (bordereffect) {
    case "circle": {
      const dx = x - 0.5;
      const dy = y - 0.5;
      const d = Math.sqrt(dx * dx + dy * dy);
      overrides = smoothstep(
        0.489 - bordersmooth * 0.5,
        0.49,
        d + bordereffectpad
      );
      break;
    }
  }

  const xi = Math.floor(x * (width - 1));
  const yi = Math.floor(y * (height - 1));
  const i = 4 * (yi * width + xi);
  let v = data[i] / 255;
  if (primary.blackPaper) {
    v = 1 - v;
  }
  return mix(v, 1, overrides);
}

function generatePixelsMap(images) {
  const pixelmap = document.createElement("canvas");
  const ctx = pixelmap.getContext("2d");
  pixelmap.width = pixelmapwidth;
  pixelmap.height = pixelmapheight;
  const outputPixels = DEBUG
    ? ctx.getImageData(0, 0, pixelmapwidth, pixelmapheight)
    : null;
  const output = new Float32Array(pixelmapwidth * pixelmapheight);

  function gen(retries) {
    const samples = (1.7 + fxrand() * 6) | 0;
    const config = Array(samples)
      .fill(null)
      .map((_, i) => {
        const index = Math.floor(fxrand() % images.length);
        const img = images[index];
        const scale = fxrand() < 0.25 ? 2 + 8 * fxrand() : 1.1 * fxrand();
        const translate = [
          (fxrand() - 0.5) * fxrand(),
          (fxrand() - 0.5) * fxrand(),
        ];
        const rotation =
          fxrand() < 0.5 ? Math.PI * (fxrand() - 0.5) * fxrand() : 0;

        let lookupmode =
          fxrand() < 0.2
            ? ""
            : fxrand() < 0.6
            ? fxrand() < 0.5
              ? "x-repeat"
              : "y-repeat"
            : fxrand() < 0.3
            ? "repeat"
            : "mirror";

        const deformations = [];

        if (fxrand() < 0.5) {
          let count = 4 * fxrand() * fxrand();
          for (let i = 0; i < count; i++) {
            let ampmul = fxrand() * fxrand();
            let amprad = fxrand() * 0.5;
            let ampc = 0.5 - (fxrand() - 0.5) * fxrand();
            let offset = [10 * fxrand(), 10 * fxrand()];
            let blur = Math.max(0, 3 * fxrand() * fxrand() - 0.5);
            const freq = [
              fxrand() * fxrand() * fxrand(),
              fxrand() * fxrand() * fxrand(),
            ];
            if (fxrand() < 0.5) {
              freq[1] = freq[0];
            }
            deformations.push({
              ampmul,
              amprad,
              ampc,
              blur,
              freq,
              f: (x, y) => {
                let dx = x - 0.5;
                let dy = y - 0.5;
                let amp =
                  ampmul *
                  (1 + blur * fxrand()) *
                  Math.max(0, Math.sqrt(dx * dx + dy * dy) + 0.1 - amprad);
                return {
                  offset: [
                    offset[0] + blur * fxrand(),
                    offset[1] + blur * fxrand(),
                  ],
                  freq,
                  amp: [mix(0, amp, ampc), mix(amp, 0, ampc)],
                };
              },
            });
          }
        }

        const bordersmooth = Math.max(0, fxrand() * fxrand() - 0.2);
        const bordereffect = fxrand() < 0.75 ? "" : "circle";
        const bordereffectpad = 0.2 * fxrand() - 0.1;

        let radialamp;
        if (fxrand() < 0.1) {
          radialamp = {
            center: [fxrand(), fxrand()],
            radiusout: 0.9,
            radiusin: 0.5,
          };
        } else if (fxrand() / (i + 1) < 0.2) {
          const r = 1 + 8 * fxrand();
          const transition = 0.5 * fxrand();
          const a = 2 * Math.PI * fxrand();
          radialamp = {
            center: [r * Math.cos(a) + 0.5, r * Math.sin(a) + 0.5],
            radiusout: r - transition,
            radiusin: r + transition,
          };
        }

        const xmirror = fxrand() < 0.5;

        return {
          img,
          scale,
          rotation,
          translate,
          deformations,
          lookupmode,
          bordereffect,
          bordersmooth,
          bordereffectpad,
          radialamp,
          xmirror,
        };
      });

    const powf = 0.9 + fxrand();

    let sum = 0;
    let empties = 0;
    for (let y = 0; y < pixelmapheight; y++) {
      for (let x = 0; x < pixelmapwidth; x++) {
        const ax = x / pixelmapwidth;
        const ay = y / pixelmapheight;
        let v = 0;
        for (let i = 0; i < config.length; i++) {
          const conf = config[i];
          const {
            scale,
            translate,
            deformations,
            rotation,
            radialamp,
            xmirror,
          } = conf;

          let xp = ratio * (scale * (ax - 0.5));
          let yp = scale * (ay - 0.5);
          for (let j = 0; j < deformations.length; j++) {
            const { f } = deformations[j];
            const d = f(ax, ay);
            xp += d.amp[0] * Math.cos(y * d.freq[0] + d.offset[0]);
            yp += d.amp[1] * Math.sin(x * d.freq[1] + d.offset[1]);
          }
          if (xmirror) {
            xp = -xp;
          }
          if (rotation) {
            let cr = Math.cos(rotation);
            let sr = Math.sin(rotation);
            let newx = xp * cr - yp * sr;
            let newy = xp * sr + yp * cr;
            xp = newx;
            yp = newy;
          }
          xp += 0.5 - translate[0];
          yp += 0.5 - translate[1];

          let amp = Math.pow((samples - i) / samples, powf);
          let imgv = lookupImage(conf, xp, yp);
          if (radialamp) {
            const dx = ax - radialamp.center[0];
            const dy = ay - radialamp.center[1];
            const r = Math.sqrt(dx * dx + dy * dy);
            imgv = mix(
              0.8,
              imgv,
              smoothstep(radialamp.radiusout, radialamp.radiusin, r)
            );
          }
          v += amp * imgv;
          v *= 1.15;
          v -= 0.05;
        }
        v = (1.8 * v) / samples - 0.05;
        sum += v;

        if (v >= 0.9) {
          empties++;
        }

        const i = y * pixelmapwidth + x;
        if (outputPixels) {
          const valuepoint = Math.floor(255 * Math.max(0, Math.min(v, 1)));
          outputPixels.data[4 * i] = valuepoint;
          outputPixels.data[4 * i + 1] = valuepoint;
          outputPixels.data[4 * i + 2] = valuepoint;
          outputPixels.data[4 * i + 3] = 255;
        }
        output[i] = v;
      }
    }

    sum /= pixelmapheight * pixelmapwidth;
    empties /= pixelmapheight * pixelmapwidth;

    if (
      retries > 0 &&
      (sum < 0.3 || sum > 1 || (0.5 + 0.5 * fxrand()) * empties > 0.6)
    ) {
      return gen(retries - 1);
    }
    return { config, empties };
  }

  const { config, empties } = gen(4);

  traits.Contrast =
    empties > 0.55
      ? "Very High"
      : empties > 0.36
      ? "High"
      : empties > 0.2
      ? "Normal"
      : "Low";

  config.slice(0, 1).forEach((config, i) => {
    const {
      scale,
      rotation,
      deformations,
      lookupmode,
      bordereffect,
      img: { title },
    } = config;
    const isRotated = Math.abs(rotation) > 0.5;
    const isZoomed = scale < 0.2;
    const isUnzoomed = scale > 2;
    const disformed = deformations.filter((d) => d.ampmul > 0.1).length > 0;
    const blurred =
      deformations.filter((d) => d.ampmul > 0.2 && d.blur > 0.5).length > 0;
    const isCircleCrop = scale > 0.5 && bordereffect === "circle";

    const words = [];
    if (blurred) words.push("blurry");
    else if (disformed) words.push("disformed");
    if (isZoomed) words.push("abstract");
    else {
      if (isRotated) words.push("rotated");
      if (isCircleCrop) words.push("circled");
    }
    words.push(title);
    if (isUnzoomed) {
      switch (lookupmode) {
        case "y-repeat":
        case "x-repeat": {
          words.push("film");
          break;
        }
        case "repeat": {
          words.push("grid");
          break;
        }
        case "mirror": {
          words.push("grid mirrored");
          break;
        }
        default: {
          words.push("frame");
          break;
        }
      }
    }
    let summary = words.join(" ");
    summary = ("aeuioy".includes(summary[0]) ? "An " : "A ") + summary + ".";
    traits.Title = summary;
  });

  // DEBUG
  if (DEBUG) {
    ctx.putImageData(outputPixels, 0, 0);
    pixelmap.style.height = window.innerHeight + "px";
    document.body.appendChild(pixelmap);
  }
  /////////

  // fill up traits

  return output;
}

// SVG ART

function art(pixelmap) {
  // samples points
  const positions = [];
  let total_points = 160000 * (0.8 + 0.3 * fxrand());
  const linelengthfactor = 0.85 + 1.4 * fxrand() * fxrand() * fxrand();
  if (primary.blackPaper) {
    total_points *= 0.8;
  }
  total_points /= linelengthfactor;
  total_points = total_points | 0;
  const samples = 1800000;
  for (let i = 0; i < samples && positions.length < total_points; i++) {
    const x = fxrand();
    const y = fxrand();
    const xi = Math.floor(x * (pixelmapwidth - 1));
    const yi = Math.floor(y * (pixelmapheight - 1));
    const index = pixelmapwidth * yi + xi;
    const v = pixelmap[index];
    const p = fxrand();
    if (v < p * p) {
      positions.push([x, y, v]);
      pixelmap[index] = Math.max(0, v - 0.3);
    }
  }

  // make strokes
  const pad = 10;
  const anglebase = 2 * Math.PI * fxrand();
  const directionalamp = 2 * fxrand() * fxrand() * fxrand();
  const anglenoise = 4 * fxrand() * fxrand();
  const routes = positions.map(([xp, yp, v]) => {
    const x = pad + xp * (WIDTH - 2 * pad);
    const y = pad + yp * (HEIGHT - 2 * pad);
    const r = linelengthfactor * Math.max(0.2, 1 - 0.4 * v);
    const a =
      anglebase +
      Math.cos(directionalamp * x) +
      Math.sin(directionalamp * y) +
      anglenoise * (fxrand() - 0.5);

    const acos = Math.cos(a);
    const asin = Math.sin(a);
    const x1 = x - r * acos;
    const y1 = y - r * asin;
    const x2 = x + r * acos;
    const y2 = y + r * asin;
    return [
      [x, y],
      [x2, y2],
    ];
  });

  const l = positions.length;
  traits["Strokes Quantity"] =
    l < 70000
      ? "Very Low"
      : l < 120000
      ? "Low"
      : l < 170000
      ? "Normal"
      : "High";

  return { routes };
}

function chunk(array, size) {
  let length = array.length;
  if (!length || size < 1) {
    return [];
  }
  let index = 0,
    resIndex = 0,
    result = Array(Math.ceil(length / size));
  while (index < length) {
    result[resIndex++] = array.slice(index, (index += size));
  }
  return result;
}

function makeSVG(routes, rendererMode) {
  let style = rendererMode
    ? 'opacity="0.65"'
    : 'style="mix-blend-mode:multiply"';

  let svgW = rendererMode ? PX(rendererMode[0]) : WIDTH + "mm";
  let svgH = rendererMode ? PX(rendererMode[1]) : HEIGHT + "mm";

  let layers = [primary]
    .map(({ main, name }, ci) => {
      let stroke = rendererMode
        ? MASKS[ci]
        : "rgb(" + main.map((n) => (n * 255) | 0).join(",") + ")";

      let layer = chunk(routes, Math.floor(routes.length / 40))
        .map((routes) => {
          const d = routes
            .map((r) =>
              r
                .map(
                  ([x, y], i) =>
                    `${i === 0 ? "M" : "L"}${x.toFixed(2)},${y.toFixed(2)}`
                )
                .join(" ")
            )
            .join(" ");
          return `<path d="${d}" fill="none" stroke="${stroke}" stroke-width="0.35" ${style} />`;
        })
        .join("\n");
      return `<g inkscape:groupmode="layer" inkscape:label="${name}">${layer}</g>`;
    })
    .join("\n");

  return `<svg data-hash="${fxhash}" data-traits='${JSON.stringify(
    traits
  )}' width="${svgW}" height="${svgH}" style="background:#fff;${
    rendererMode ? "" : "width:100%;height:100%"
  }" viewBox="0 0 ${WIDTH} ${HEIGHT}" xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape">${layers}</svg>`;
}

function smoothstep(min, max, value) {
  var x = Math.max(0, Math.min(1, (value - min) / (max - min)));
  return x * x * (3 - 2 * x);
}
function mix(a, b, x) {
  return (1 - x) * a + x * b;
}

let lastT;
const perf = (tag) => {
  if (!PERF) return;
  let now = window.performance ? performance.now() : Date.now();
  if (lastT) {
    console.log((now - lastT).toFixed(0) + "ms " + tag);
  }
  lastT = now;
};

// MAIN

loadImages(sources).then((images) => {
  perf();
  const pixelmap = generatePixelsMap(images);
  perf("generatePixelsMap");

  if (!DEBUG) {
    const { routes } = art(pixelmap);
    perf("art");

    if (console && console.table) console.table(traits);
    window.$fxhashFeatures = traits;

    // bake svg
    const svgText = makeSVG(routes);
    perf("makeSVG");

    let DOC = document;
    let BODY = DOC.body;
    let createElement = (e) => DOC.createElement(e);
    let append = (n, e) => n.appendChild(e);
    let appendBody = (e) => append(BODY, e);

    let container = createElement("div");

    let bgImage = createElement("img");
    bgImage.src = makeSVGDataImage(svgText);
    let ABSOLUTE = "absolute";
    let CENTER = "center";
    let HUNDREDPC = "100%";
    let sharedStyle = { position: ABSOLUTE, opacity: 0 };
    Object.assign(container.style, sharedStyle);
    Object.assign(bgImage.style, {
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

    Object.assign(BODY.style, {
      display: "flex",
      alignItems: CENTER,
      justifyContent: CENTER,
    });

    append(container, bgImage);
    appendBody(container);
    appendBody(canvas);

    let regl = createREGL(canvas);

    let vert = `precision mediump float;attribute vec2 p;varying vec2 uv;void main(){uv=p;gl_Position=vec4(2.*p-1.,0,1);}`;

    let framebuffer = regl.framebuffer();

    const shaders = {
      paper: `precision highp float;
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
      paperBlack: `precision highp float;
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
      main: `
    precision highp float;
    varying vec2 uv;
    uniform vec3 baseColor;
    uniform float grainAmp, lighting, time;
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
      float d = p.y;
      float gain = smoothstep(0.1, 0.0, abs(fract(d - .2 * time) - 0.2));
      vec4 g = texture2D(paper, p);
      float grain = g.r;
      vec4 v = texture2D(t, p);
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
      mainBlack: `
    precision highp float;
    varying vec2 uv;
    uniform vec3 baseColor, background;
    uniform float grainAmp, lighting, time;
    uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight;
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
      float d = p.y;
      float gain = smoothstep(0.1, 0.0, abs(fract(d - .2 * time) - 0.2));
      vec4 g = texture2D(paper, p);
      float grain = g.r;
      vec4 v = texture2D(t, p);
      vec3 c1 = pal(v.r, primary, primaryHighlight);
      vec3 c2 = pal(v.g, secondary, secondaryHighlight);
      vec3 c =
      (c1 + c2) * (1. + lighting * gain) +
        grainAmp * grain +
        baseColor +
        background * smoothstep(0.5, 1.0, v.r * v.g);
      gl_FragColor = vec4(c, 1.0);
    }
  `,
    };

    let paper = regl({
      framebuffer,
      frag: primary.blackPaper ? shaders.paperBlack : shaders.paper,
      vert,
      attributes: {
        p: TRIANGLE,
      },
      uniforms: {
        seed: paperSeed,
        grain: 100,
        resolution: V,
      },
      count: 3,
    });

    let prop = regl.prop;

    const background = primary.bg;
    const grainAmp = primary.blackPaper ? 0.05 : 0.08;
    const lighting = 0.1;
    const baseColor = [-0.003, -0.006, -0.01];

    let render = regl({
      frag: primary.blackPaper ? shaders.mainBlack : shaders.main,
      vert,
      attributes: {
        p: TRIANGLE,
      },
      uniforms: {
        t: prop("t"),
        time: prop("T"),
        paper: framebuffer,
        primary: primary.main,
        primaryHighlight: primary.highlight,
        secondary: secondary.main,
        secondaryHighlight: secondary.highlight,
        grainAmp,
        lighting,
        baseColor,
        ...(background ? { background } : {}),
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
        img.src = makeSVGDataImage(makeSVG(routes, [svgWidth, svgHeight]));
        img.width = svgWidth;
        img.height = svgHeight;
      }
    };

    let r = (onresize = () => resize(window.innerWidth, window.innerHeight));
    r();
    perf("end");

    let startT;
    regl.frame(({ time }) => {
      if (!startT) startT = time;
      else if (time - startT < 0.1) return;
      render({ T: time - startT, t: tex });
    });
  }
});

function makeSVGDataImage(svg) {
  return "data:image/svg+xml;base64," + btoa(svg);
}
