/*
 * interesting hashes:
ooDehiZ5cXPzUt6S9zDug9N59RDbFGu9nT6j9HJqPcah8ukx17p

// BUGS
// ??? oovD1y5mRYSQmVp4gwEXS2EA4J6YkjtqVxuRDMx8ANw79ECX4zP

 * Name: Plottable Mountain Moons
 * Tags: plottable, webgl, svg, rust, wasm, A4, physical, phygital
 * Description:

What makes this NFT have value? not only the digital is cool but it is the recipe to a plottable. Owning this NFT confers the right to request a physical plot (21cm square). @greweb will ask extra fees for this service => https://greweb.me/plots/nft <=

Full article: https://greweb.me/2021/12/plottable-mountain-moons

Plottable Mountain Moons generalize many explorations done this year. There are 13 inks, 9 different shape primitives with recursion, various intensity, and frequencies of noise,... and many rare cases to discover!

Digital and Physical art, hybrid and decoupled:
- The art is published via a digital NFT on Tezos blockchain – its digital representation simulates fountain pen inks drawing on paper with animated effects.
- Token to the physical world: owning each NFT confer the power to request the related physical plot: https://greweb.me/plots/nft

@greweb – 2021 – tech: WebGL + Rust + WASM.
 */

const COLORS = [
  {
    name: "Bloody Brexit",
    main: [0.02, 0.12, 0.42],
    highlight: [0.18, 0.0, 0.2],
    weight: 6,
  },
  {
    name: "Black",
    main: [0.2, 0.2, 0.2],
    highlight: [0, 0, 0],
    weight: 6,
  },
  {
    name: "Indigo",
    main: [0.45, 0.55, 0.7],
    highlight: [0.2, 0.3, 0.4],
    weight: 4,
  },
  {
    name: "Amazing Amethyst",
    main: [0.6, 0.3, 0.7],
    highlight: [0.3, 0.1, 0.4],
    weight: 4,
  },
  {
    name: "FireAndIce",
    main: [0 / 255, 190 / 255, 220 / 255],
    highlight: [0 / 255, 100 / 255, 120 / 255],
    weight: 3,
  },
  {
    name: "Poppy Red",
    main: [0.9, 0.2, 0.1],
    highlight: [0.5, 0.0, 0.1],
    weight: 3,
  },
  {
    name: "Aurora Borealis",
    main: [0.0, 0.6, 0.6],
    highlight: [0.0, 0.3, 0.4],
    weight: 2,
  },
  {
    name: "Pumpkin",
    main: [1, 0.5, 0.2],
    highlight: [0.9, 0.3, 0.0],
    weight: 2,
  },
  {
    name: "Pink",
    main: [1.0, 0.32, 0.46],
    highlight: [0.9, 0.38, 0.3],
    weight: 2,
  },
  {
    name: "Hope Pink",
    main: [1.0, 0.4, 0.75],
    highlight: [0.9, 0.2, 0.6],
    weight: 2,
  },
  {
    name: "Imperial Purple",
    main: [0.5, 0.1, 0.9],
    highlight: [0.2, 0.0, 0.4],
    weight: 1,
  },
  {
    name: "Sherwood Green",
    main: [0.25, 0.5, 0.3],
    highlight: [0.1, 0.3, 0.1],
    weight: 1,
  },
  {
    name: "Amber",
    main: [1.0, 0.78, 0.28],
    highlight: [1.0, 0.5, 0.0],
    weight: 1,
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

const primitiveNames = [
  "Waveballs",
  "Spiral Strokes",
  "Random Strokes",
  "Diagonal Strokes",
  "Circles Packing",
  "Spirals",
  "Crosshatch",
  "X Strokes",
  "Y Strokes",
];

module.exports = function generateVariables(random) {
  const props = {};
  const paperSeed = random() + random() + random() + random();
  let weights = [
    0.01 + 3 * random(),
    Math.max(0, 3 * random() * random() - 1),
    Math.max(0, 2 * random() * random() - 1),
    Math.max(0, 3 * random() * random() - 1),
    Math.max(0, 3 * random() * random() - 1),
    Math.max(0, 3 * random() * random() - 1),
    Math.max(0, 2 * random() * random() - 1),
    Math.max(0, 2 * random() * random() - 1),
    Math.max(0, 2 * random() * random() - 1),
  ];
  let base_offset = Math.max(
    -1.5,
    0.1 + mix(-2, 1.5, random()) * random() // * random()
  );
  if (base_offset < -0.8) {
    weights = [1];
    base_offset -= 0.5;
  } else if (base_offset > 0.8) {
    base_offset += 0.5;
  }
  let primary = pickColor(random());
  let secondary = random() < 0.2 ? primary : pickColor(random());
  const hasWaveSplit =
    primary === secondary || base_offset > 0.3
      ? false
      : random() < 0.01 + smoothstep(0.3, -1.1, base_offset);
  let wave_split_color = hasWaveSplit
    ? Math.max(-0.5, base_offset + 0.5)
    : -10.0;
  props["Inks Count"] = primary === secondary ? 1 : 2;
  props["Inks"] =
    primary === secondary
      ? primary.name
      : [primary.name, secondary.name].sort().join(" + ");

  props["Ink " + secondary.name] = primary === secondary ? "Both" : "Mountain";
  props["Ink " + primary.name] = primary === secondary ? "Both" : "Stars";

  const primitives = weights.filter((w) => w > 0).length;
  props["Primitives"] = primitives;

  for (let i = 0; i < weights.length; i++) {
    if (weights[i] > 0) {
      props["Primitive " + primitiveNames[i]] = scoring(weights[i], [
        "Low",
        0.2,
        "Medium",
        0.8,
        "High",
      ]);
    }
  }

  props["Mountains Visibility"] = scoring(base_offset, [
    "Full",
    -1,
    "High",
    0,
    "Low",
    1,
    "Empty",
  ]);

  props["Mountains 2-Colors"] = hasWaveSplit ? "Yes" : "No";

  let f1 = mix(0.002, 0.05, random());
  let f2 = mix(0.01, 0.1, random() * random());
  let f3 = mix(0.0, 0.5, random() * random());
  let a1 = mix(0.0, 1.6, 1.0 - random() * random());
  let a2 = random();
  let a3 = random();
  props["Base Noise"] =
    a1 < 0.05
      ? "None"
      : scoring(f1, [
          "Very Low",
          0.005,
          "Low",
          0.01,
          "Medium",
          0.02,
          "High",
          0.04,
          "Very High",
        ]);
  props["Perturbating Noise"] =
    a1 < 0.05 || (a2 < 0.05 && a3 < 0.05)
      ? "None"
      : scoring(Math.max(f2, f3), [
          "Very Low",
          0.03,
          "Low",
          0.05,
          "Medium",
          0.16,
          "High",
          0.25,
          "Very High",
          0.4,
          "Extreme",
        ]);
  props["Perturbation Weight"] = scoring(a2 * a3, [
    "Low",
    0.05,
    "Medium",
    0.3,
    "High",
    0.6,
    "Very High",
    0.8,
    "Extreme",
  ]);
  const base_pad = mix(2, 10, random());
  props["Padding"] = scoring(base_pad, ["Low", 4, "Medium", 8, "High"]);
  const max_scale = mix(10, 160, random());
  props["Entities Max Radius"] = scoring(max_scale, [
    "Small",
    30,
    "Medium",
    90,
    "Big",
    140,
    "Very Big",
  ]);
  const base_min_scale = mix(3, 10, random());
  props["Entities Min Radius"] = scoring(base_min_scale, [
    "High",
    5,
    "Medium",
    8,
    "Low",
  ]);
  const xfactor = mix(0.15, 0.85, 0.5 + (random() - 0.5) * random() * random());

  props["Rolling"] = scoring(xfactor, [
    "Horizontal",
    0.3,
    "Slightly Horizontal",
    0.4,
    "Normal",
    0.6,
    "Slighly Vertical",
    0.7,
    "Vertical",
  ]);

  const ribbons = 10 * Math.max(0, random() * random() * random() - 0.3);
  const ribbons_freq = mix(0.02, 0.1, random() * random());
  const ribbonsTwoColors = !hasWaveSplit && ribbons > 0 && random() < 0.3;
  props["Ribons"] =
    ribbons < 0.001 ? "None" : ribbonsTwoColors ? "Bicolored" : "Active";

  const desired_count =
    base_offset > 0.9
      ? 100
      : Math.max(0, Math.ceil(80 * (1 - 0.5 * base_offset) * random()));
  props["Elements Count"] = scoring(desired_count, [
    "None",
    0.1,
    "Very Low",
    5,
    "Low",
    10,
    "Normal",
  ]);

  const diversity = primitives > 1 ? random() : 0;
  props["Diversity"] = scoring(diversity, [
    "None",
    0.05,
    "Low",
    0.2,
    "Medium",
    0.5,
    "High",
    0.9,
    "Full",
  ]);

  const seed = 1000 * random();
  const opts = {
    seed,
    max_scale,
    desired_count,
    a1,
    a2,
    a3,
    f1,
    f2,
    f3,
    base_pad,
    base_min_scale,
    wave_split_color,
    base_offset,
    xfactor,
    primary_name: primary.name,
    secondary_name: secondary.name,
    weights,
    diversity,
    ribbons,
    ribbons_freq,
    ribbons_two_colors: ribbonsTwoColors,
  };

  // eslint-disable-next-line no-undef
  if (process.env.NODE_ENV !== "production" && typeof window !== "undefined") {
    console.log(window.fxhash);
    //      Object.keys(opts).forEach((key) => console.log(key + " =", opts[key]));
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

function smoothstep(min, max, value) {
  var x = Math.max(0, Math.min(1, (value - min) / (max - min)));
  return x * x * (3 - 2 * x);
}

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
