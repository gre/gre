/*
ooc7RqmdsncuGCcK3v2g4xi9rB1QEMi8MKDtHHraAhHmKhfmtaQ

Name: Plottable Letters
Tags: plottable, webgl, svg, rust, wasm, A4, physical, phygital
Description:

Plottable Letters plays with letters, digits, and basic shapes. There is a great chance to obtain a "GM", but also many other letter combination.

More info: https://greweb.me/plots/368

The digital NFT is the recipe to a plottable art: Owning this NFT confers the right to request a physical plot (A4 square, 21cm by 21cm) – this is an optional possibility as you can already enjoy the digital version.

Digital and Physical art, hybrid and decoupled:
- art published via a digital NFT on Tezos – its digital representation simulates fountain pen inks drawing on paper with animated effects.
- Token to the physical world: owning each NFT confer the power to request the related physical plot at https://greweb.me/plots/nft

@greweb – 2021 – tech: WebGL + Rust + WASM – CC BY-SA 4.0 https://creativecommons.org/licenses/by-sa/4.0/
 */
const COLORS = [
  {
    name: "Bloody Brexit",
    main: [0.02, 0.12, 0.42],
    highlight: [0.18, 0.0, 0.2],
    weight: 2,
  },
  {
    name: "Black",
    main: [0.2, 0.2, 0.2],
    highlight: [0, 0, 0],
    weight: 5,
  },
  {
    name: "Indigo",
    main: [0.4, 0.5, 0.65],
    highlight: [0.2, 0.3, 0.4],
    weight: 4,
  },
  {
    name: "Amazing Amethyst",
    main: [0.6, 0.3, 0.7],
    highlight: [0.3, 0.1, 0.4],
    weight: 2,
  },
  {
    name: "FireAndIce",
    main: [0 / 255, 190 / 255, 220 / 255],
    highlight: [0 / 255, 100 / 255, 120 / 255],
    weight: 5,
  },
  {
    name: "Poppy Red",
    main: [0.9, 0.2, 0.1],
    highlight: [0.5, 0.0, 0.1],
    weight: 4,
  },
  {
    name: "Aurora Borealis",
    main: [0.0, 0.6, 0.6],
    highlight: [0.0, 0.3, 0.4],
    weight: 3,
  },
  {
    name: "Pumpkin",
    main: [1, 0.5, 0.2],
    highlight: [0.9, 0.3, 0.0],
    weight: 3,
  },
  {
    name: "Pink",
    main: [1.0, 0.32, 0.46],
    highlight: [0.9, 0.38, 0.3],
    weight: 6,
  },
  {
    name: "Hope Pink",
    main: [1.0, 0.4, 0.75],
    highlight: [0.9, 0.2, 0.6],
    weight: 3,
  },
  {
    name: "Imperial Purple",
    main: [0.5, 0.1, 0.9],
    highlight: [0.2, 0.0, 0.4],
    weight: 1,
  },
  {
    name: "Amber",
    main: [1.0, 0.78, 0.28],
    highlight: [1.0, 0.5, 0.0],
    weight: 7,
  },
  {
    name: "Sherwood Green",
    main: [0.25, 0.5, 0.3],
    highlight: [0.1, 0.3, 0.1],
    weight: 0.5,
  },
];

const colorsWeighted = [];
for (let i = 0; i < COLORS.length; i++) {
  const c = COLORS[i];
  for (let j = 0; j < c.weight; j++) {
    colorsWeighted.push(c);
  }
}

const pickColor = (f) =>
  colorsWeighted[
    Math.floor(0.99999 * f * colorsWeighted.length) % colorsWeighted.length
  ];

const DistribModes = ["constant", "radial", "manhattan", "xcenter", "ycenter"];

module.exports = function generateVariables(random) {
  const paperSeed = random() + random() + random() + random();
  let primary = pickColor(random());
  let secondary = primary;

  const Mode = scoring(random(), [
    "lowercase",
    0.2,
    "uppercase",
    0.6,
    "oneletter",
    0.9,
    "digit",
  ]);

  const word =
    Mode === "lowercase"
      ? random() < 0.5
        ? "gm"
        : String.fromCharCode(97 + Math.floor(random() * 26)) +
          String.fromCharCode(97 + Math.floor(random() * 26))
      : Mode === "uppercase"
      ? random() < 0.5
        ? "GM"
        : String.fromCharCode(65 + Math.floor(random() * 26)) +
          String.fromCharCode(65 + Math.floor(random() * 26))
      : Mode === "oneletter"
      ? String.fromCharCode(65 + Math.floor(random() * 26))
      : Mode === "digit"
      ? String(Math.floor(random() * 10))
      : "";

  const fontMul = random() < 0.4 ? 1.3 : 1.6;

  const voronoi_size = Math.floor(400 + 600 * random());

  let Circles = Math.floor(random() * random() * random() * 8);
  const Rectangles = Math.floor(random() * random() * random() * 8);
  const Polygons = Math.floor(random() * random() * random() * 8);

  const circles = [];
  const rects = [];
  const polys = [];

  for (let i = 0; i < Circles; i++) {
    const r = 0.45 * (0.3 + 0.7 * random());
    const x = 0.5 + (0.5 - r) * (0.5 - random());
    const y = 0.5 + (0.5 - r) * (0.5 - random());
    circles.push([x, y, r]);
  }
  for (let i = 0; i < Rectangles; i++) {
    const r = 0.45 * (0.2 + 0.8 * random());
    const cx = 0.5 + (0.5 - r) * (0.5 - random());
    const cy = 0.5 + (0.5 - r) * (0.5 - random());
    const w = 2 * (random() < 0.5 ? r : (0.2 + 0.8 * random()) * r);
    const h = 2 * (random() < 0.5 ? r : (0.2 + 0.8 * random()) * r);
    const x = cx - w / 2;
    const y = cy - h / 2;
    rects.push([x, y, w, h]);
  }
  for (let i = 0; i < Polygons; i++) {
    const count = 3 + Math.floor(6 * random() * random() * random());
    const points = [];
    for (let j = 0; j < count; j++) {
      const x =
        random() < 0.5
          ? 0.95 * random() * random()
          : 0.95 - 0.9 * random() * random();
      const y =
        random() < 0.5
          ? 0.95 * random() * random()
          : 0.95 - 0.9 * random() * random();
      points.push([x, y]);
    }
    polys.push(points);
  }

  const distribmode = Math.floor(random() * random() * 4);

  const bakeImageOpts = {
    word,
    fontMul,
    circles,
    rects,
    polys,
  };

  const seed = random() * 999;

  const opts = {
    seed,
    primary_name: primary.name,
    secondary_name: secondary.name,
    distribmode,
    voronoi_size,
  };

  const props = {
    Ink: primary.name,
    "Word Kind": Mode,
    "voronoi size": scoring(voronoi_size, [
      "Normal",
      600,
      "High",
      900,
      "Very High",
    ]),
    Word: word,
    Complexity:
      Number(Circles > 0) + Number(Rectangles > 0) + Number(Polygons > 0),
    Circles,
    Rectangles,
    Polygons,
    Shapes: Circles + Rectangles + Polygons,
    "Distribution Mode": DistribModes[distribmode],
  };

  // eslint-disable-next-line no-undef
  if (process.env.NODE_ENV !== "production" && typeof window !== "undefined") {
    console.log(window.fxhash);
    Object.keys(opts).forEach((key) => console.log(key + " =", opts[key]));
    Object.keys(props).forEach((key) => console.log(key + " =", props[key]));
  }

  return {
    opts,
    primary,
    secondary,
    paperSeed,
    props,
    bakeImageOpts,
  };
};

module.exports.bakeImage = function bakeImage({
  word,
  fontMul,
  circles,
  rects,
  polys,
}) {
  if (typeof window === "undefined") return { width: 0, height: 0, data: [] };
  const dim = 400;
  const canvas = document.createElement("canvas");
  canvas.width = dim;
  canvas.height = dim;
  const ctx = canvas.getContext("2d");
  ctx.fillStyle = "#fff";
  ctx.fillRect(0, 0, dim, dim);
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";

  ctx.globalCompositeOperation = "xor";

  ctx.font = Math.floor((fontMul * dim) / (word.length + 1)) + "px ArialBlack";

  rects.forEach(([x, y, w, h]) => {
    ctx.fillRect(dim * x, dim * y, dim * w, dim * h);
  });
  circles.forEach(([x, y, r]) => {
    ctx.beginPath();
    ctx.arc(dim * x, dim * y, r * dim, 0, 2 * Math.PI);
    ctx.fill();
  });
  polys.forEach((points) => {
    ctx.beginPath();
    points.forEach(([x, y], i) => {
      if (i === 0) ctx.moveTo(x * dim, y * dim);
      else ctx.lineTo(x * dim, y * dim);
    });
    ctx.fill();
  });

  ctx.fillText(word, dim / 2, dim / 2);
  const imageData = ctx.getImageData(0, 0, dim, dim);
  var binary = new Array(imageData.data.length);
  for (var i = 0; i < binary.length; i++) {
    binary[i] = imageData.data[i];
  }

  return {
    width: dim,
    height: dim,
    data: binary,
  };
};

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
