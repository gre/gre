/*
oo4NTpjc5ceCZuVxHsxucNv7EwuktVQX7U4iEiEDH6x8s1qoZXh

Name: Plottable Circles
Tags: plottable, webgl, svg, rust, wasm, A4, physical, phygital
Description:


Plottable Circles explores concentric circles – varies in the different number of circles, noise scales, inks, line widths and includes some rare shapes.

More info: https://greweb.me/plots/349

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

module.exports = function generateVariables(random) {
  const paperSeed = random() + random() + random() + random();
  const ringsf = random() * random() * random();
  const rings = Math.ceil(40 * ringsf);
  let primary = pickColor(random());
  let secondary = rings === 1 ? primary : pickColor(random());
  const ringcenter = random();
  const ring_resolution_multiplier = random() < 0.2 ? 0.4 + 0.4 * random() : 10;
  const zigzag_rep =
    random() > 0.9
      ? Math.floor(1 + (rings - 1) * Math.pow(random(), 0.6))
      : 1000;
  const zigzag_count = Math.floor(2 + 3 * random() * random());
  const zigzag_ring_resolution_multiplier = 0.5 + random();
  const ring_w_lower = Math.max(
    0,
    0.1 +
      0.9 * random() * random() * Math.max(0, 2 * random() - 1) -
      0.16 * random() * random() * random() * random()
  );
  const ring_w_upper = 1.0;
  const ring_max_width = mix(
    60,
    130,
    random() * (1 - 2 * Math.abs(ringcenter - 0.5))
  );
  const size_base = Math.ceil(100 + 30 * ringsf + 10 * random());
  const size = Math.min(size_base / rings, ring_max_width);

  const line_gap_max = 0.8;
  const famp = mix(0.25, 1.2, ringsf);
  const ring_1x = mix(0.1, 4.0, random());
  const ring_1y = mix(0.1, 4.0, random());
  const ring_1xf2x = famp * mix(0.4, 12.0, random());
  const ring_1xf2y = famp * mix(0.4, 12.0, random());
  const ring_1yf2x = famp * mix(0.4, 12.0, random());
  const ring_1yf2y = famp * mix(0.4, 12.0, random());
  const ring_1y3 = mix(0.1, 3, random());
  const ring_1yf3x = famp * mix(1.0, 24.0, random());
  const ring_1yf3y = famp * mix(1.0, 24.0, random());
  const seed = random() * 999;

  const f1 = ring_1x * Math.max(ring_1xf2x, ring_1xf2y);
  const f2 = ring_1y * Math.max(ring_1yf2x, ring_1yf2y);
  const f3 = ring_1y * ring_1y3 * Math.max(ring_1yf3x, ring_1yf3y);
  const f = Math.max(f1, f2, f3);

  const opts = {
    rings,
    seed,
    primary_name: primary.name,
    secondary_name: secondary.name,
    ringcenter,
    ring_resolution_multiplier,
    ring_w_lower,
    ring_w_upper,
    ring_max_width,
    line_gap_max,
    ring_1x,
    ring_1y,
    ring_1xf2x,
    ring_1xf2y,
    ring_1yf2x,
    ring_1yf2y,
    ring_1y3,
    ring_1yf3x,
    ring_1yf3y,
    size,
    zigzag_count,
    zigzag_rep,
    zigzag_ring_resolution_multiplier,
  };

  const props = {
    "Inks Count": primary === secondary ? 1 : 2,
    Inks:
      primary === secondary
        ? primary.name
        : [primary.name, secondary.name].sort().join(" + "),
    ["Ink " + secondary.name]: primary === secondary ? "Both" : "Mountain",
    ["Ink " + primary.name]: primary === secondary ? "Both" : "Stars",
    "Lines Attraction": scoring(ringcenter, [
      "Very Outside",
      0.1,
      "Outside",
      0.3,
      "Balanced",
      0.7,
      "Inside",
      0.9,
      "Very Inside",
    ]),
    "Rings Count": rings,
    "Rings Size": scoring(size, [
      "Very Small",
      6,
      "Small",
      12,
      "Medium",
      36,
      "Large",
      64,
      "Very Large",
      90,
      "Extreme",
    ]),
    Compression: scoring(ring_w_lower, [
      "Very High",
      0.04,
      "High",
      0.08,
      "Normal",
      0.2,
      "Low",
      0.6,
      "Very Low",
    ]),
    Noise: scoring(f, ["Negligible", 0.5, "Small", 4, "Medium", 24, "Large"]),
    Resolution: scoring(ring_resolution_multiplier, ["Low Poly", 3, "Smooth"]),
    Shape:
      zigzag_rep < 999
        ? zigzag_rep === 1
          ? "Only Wireframe"
          : "Wireframe Every " + zigzag_rep
        : "Normal",
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
