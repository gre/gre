/*
 * interesting hashes:
ooFGwip9CMF5CKtzJ1n5j89XTpFVngBA5MsNE8VLJsX1PYCBKeX

 * Name: Plottable Mesh
 * Tags: plottable, webgl, svg, rust, wasm, A4, physical, phygital
 * Description:

Plottable Mesh offers a wide variety of patterns made only with lines.
More info: https://greweb.me/plots/341

The digital NFT is the recipe to a plottable: Owning this NFT confers the right to request a physical plot (A4 size) – this is an optional possibility as you can already enjoy the digital version.

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
    weight: 9,
    group: "Dark Blue",
  },
  {
    name: "Black",
    main: [0.2, 0.2, 0.2],
    highlight: [0, 0, 0],
    weight: 20,
    group: "Black",
  },
  {
    name: "Indigo",
    main: [0.4, 0.5, 0.65],
    highlight: [0.2, 0.3, 0.4],
    weight: 4,
    group: "Colored",
  },
  {
    name: "Amazing Amethyst",
    main: [0.6, 0.3, 0.7],
    highlight: [0.3, 0.1, 0.4],
    weight: 6,
    group: "Colored",
  },
  {
    name: "FireAndIce",
    main: [0 / 255, 190 / 255, 220 / 255],
    highlight: [0 / 255, 100 / 255, 120 / 255],
    weight: 8,
    group: "Colored",
  },
  {
    name: "Poppy Red",
    main: [0.9, 0.2, 0.1],
    highlight: [0.5, 0.0, 0.1],
    weight: 8,
    group: "Colored",
  },
  {
    name: "Aurora Borealis",
    main: [0.0, 0.6, 0.6],
    highlight: [0.0, 0.3, 0.4],
    weight: 1,
    group: "Colored",
  },
  {
    name: "Pumpkin",
    main: [1, 0.5, 0.2],
    highlight: [0.9, 0.3, 0.0],
    weight: 1,
    group: "Colored",
  },
  {
    name: "Pink",
    main: [1.0, 0.32, 0.46],
    highlight: [0.9, 0.38, 0.3],
    weight: 2,
    group: "Colored",
  },
  {
    name: "Hope Pink",
    main: [1.0, 0.4, 0.75],
    highlight: [0.9, 0.2, 0.6],
    weight: 4,
    group: "Colored",
  },
  {
    name: "Imperial Purple",
    main: [0.5, 0.1, 0.9],
    highlight: [0.2, 0.0, 0.4],
    weight: 1,
    group: "Colored",
  },
  {
    name: "Amber",
    main: [1.0, 0.78, 0.28],
    highlight: [1.0, 0.5, 0.0],
    weight: 1,
    group: "Colored",
  },
  {
    name: "Sherwood Green",
    main: [0.25, 0.5, 0.3],
    highlight: [0.1, 0.3, 0.1],
    weight: 1,
    group: "Dark Green",
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
  const props = {};
  const paperSeed = random() + random() + random() + random();
  let primary = pickColor(random());
  let secondary = pickColor(random());
  /*
  if (primary === secondary) {
    secondary = pickColor(random());
  }
  */
  if (primary.group !== secondary.group) {
    secondary = primary;
  }
  props["Inks Group"] = primary.group;
  props["Inks Count"] = primary === secondary ? 1 : 2;
  props["Inks"] =
    primary === secondary
      ? primary.name
      : [primary.name, secondary.name].sort().join(" + ");

  props["Ink " + secondary.name] = primary === secondary ? "Both" : "Mountain";
  props["Ink " + primary.name] = primary === secondary ? "Both" : "Stars";

  const precision = 1;
  const vertical = random() < 0.2;
  const shapeamp = mix(0.1, 1.0, random());
  props["Shape Amplitude"] = scoring(shapeamp, [
    "Very Low",
    0.2,
    "Low",
    0.4,
    "Medium",
    0.8,
    "High",
  ]);
  props.Duality = vertical ? "Vertical" : "Horizontal";
  const symmetry = random() < 0.3;
  props.Symmetry = symmetry ? "Yes" : "No";
  const iterations = Math.round(mix(6, 30, random()));
  let k = mix(0, 0.4, random() * random());
  props.Smoothness = scoring(k, [
    "None",
    0.01,
    "Low",
    0.05,
    "Medium",
    0.15,
    "High",
  ]);
  const f1 = mix(0.5, 2, 1 - random() * random());
  const f2x = mix(0.5, 12, random() * random());
  const f2y = mix(0.5, 12, random() * random());
  const f3 = mix(2, 6, random());
  const a1 = mix(0, 1, random());
  const a2 = mix(0.4, 2, random());
  const a3 = mix(0.4, 2, random());
  const samples = Math.ceil(
    200 *
      (1 -
        0.4 * a1 -
        0.1 * Math.min(4, Math.min(a2, a3) * Math.max(f2x, f2y, f3)))
  );
  props["Noise Amplitude"] = scoring(a1, [
    "Very Low",
    0.05,
    "Low",
    0.2,
    "Medium",
    0.7,
    "High",
    0.9,
    "Very High",
  ]);
  props["Noise Perturbation"] = scoring(a2 * a3, [
    "Very Low",
    0.5,
    "Low",
    1,
    "Medium",
    2,
    "High",
    3,
    "Very High",
  ]);

  const f2 = Math.min(f2x, f2y);
  const f2main = f2x > f2y ? "Vertical" : "Horizontal";

  props["Noise Direction"] =
    !a1 > 0.1
      ? "None"
      : f2 > 8
      ? "Both Very High"
      : f2 > 4
      ? "Both High"
      : scoring(Math.max(f2x, f2y), [
          "Normal",
          1,
          f2main + " Medium",
          6,
          f2main + " High",
        ]);

  const seed = 1000 * random();
  const offset = Math.max(0, mix(-0.1, 0.5, random() * random()));
  props["Duality Offset"] = scoring(offset, [
    "None",
    0.005,
    "Low",
    0.05,
    "Low",
    0.1,
    "Medium",
    0.25,
    "High",
  ]);
  const overflowin = Math.max(
    0,
    primary !== secondary
      ? 0.3 * random() - 0.1
      : random() * random() * random() - 0.5
  );
  const overflowout = overflowin;

  props["Duality Overflow"] = scoring(overflowin, [
    "None",
    0.002,
    "Low",
    0.01,
    "Active",
  ]);

  const opts = {
    precision,
    samples,
    iterations,
    shapeamp,
    k,
    seed,
    a1,
    a2,
    a3,
    f1,
    f2x,
    f2y,
    f3,
    vertical,
    symmetry,
    primary_name: primary.name,
    secondary_name: secondary.name,
    offset,
    overflowin,
    overflowout,
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
