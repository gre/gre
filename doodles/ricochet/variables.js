/*
ooNei45wa4UD5Nhbf2WJpmJ1PbbgQgBCkGYMGv54wq1HFKoLjYW
ooji5FYMdYTuDtwKts4aoM2Z1Qnpk8WMEQsu2HNk9QR64dRkCMx
oohSjFoXkdjweU1GAEfvsQyhKustQwiXDqG1eG758ERTaF6yV6t
oo5EREkLfMyN1vwE4ULsWTwBMYqaYivBwpbffiCtGqXHakVsoWQ
ooeB5wzUCSaHW2XF1bjoVSKnT36Zxa3g8jeKNTajv2KBsSWbh19
ooxmfoyDSZjgsUZjvZNdhc6GMwb23Fbp3CFrTzXi9hVbykziQvm
oohJyXik3ZtyD97f8yTqdvpoAAcyGvDS2f17CYtQ6M6eERfsVRD

Name: Plottable Ricochets
Tags: plottable, webgl, svg, rust, wasm, A4, physical, phygital
Description:

Plottable Ricochets explores polygon shapes scaled and rotated inside a parametric curve. The curve you obtain can widely vary.

More info: https://greweb.me/plots/361

The digital NFT is the recipe to a plottable art: Owning this NFT confers the right to request a physical plot (A4 square, 21cm by 21cm) – this is an optional possibility as you can already enjoy the digital version.

Digital and Physical art, hybrid and decoupled:
- art published via a digital NFT on Tezos – its digital representation simulates fountain pen inks drawing on paper with animated effects.
- Token to the physical world: owning each NFT confer the power to request the related physical plot at https://greweb.me/plots/nft

Advanced plotting usecase: press "V" multiple times to increase lines collection when ink is too much for the paper.

@greweb – 2021 – tech: WebGL + Rust + WASM – CC BY-SA 4.0 https://creativecommons.org/licenses/by-sa/4.0/
 */
const COLORS = [
  {
    name: "Bloody Brexit",
    main: [0.02, 0.12, 0.42],
    highlight: [0.18, 0.0, 0.2],
    weight: 2,
    group: 1,
  },
  {
    name: "Black",
    main: [0.2, 0.2, 0.2],
    highlight: [0, 0, 0],
    weight: 5,
    group: 0,
  },
  {
    name: "Indigo",
    main: [0.4, 0.5, 0.65],
    highlight: [0.2, 0.3, 0.4],
    weight: 4,
    group: 1,
  },
  {
    name: "Amazing Amethyst",
    main: [0.6, 0.3, 0.7],
    highlight: [0.3, 0.1, 0.4],
    weight: 2,
    group: 1,
  },
  {
    name: "FireAndIce",
    main: [0 / 255, 190 / 255, 220 / 255],
    highlight: [0 / 255, 100 / 255, 120 / 255],
    weight: 5,
    group: 1,
  },
  {
    name: "Aurora Borealis",
    main: [0.0, 0.6, 0.6],
    highlight: [0.0, 0.3, 0.4],
    weight: 3,
    group: 1,
  },
  {
    name: "Poppy Red",
    main: [0.9, 0.2, 0.1],
    highlight: [0.5, 0.0, 0.1],
    weight: 4,
    group: 2,
  },
  {
    name: "Pumpkin",
    main: [1, 0.5, 0.2],
    highlight: [0.9, 0.3, 0.0],
    weight: 3,
    group: 2,
  },
  {
    name: "Pink",
    main: [1.0, 0.32, 0.46],
    highlight: [0.9, 0.38, 0.3],
    weight: 4,
    group: 2,
  },
  {
    name: "Hope Pink",
    main: [1.0, 0.4, 0.75],
    highlight: [0.9, 0.2, 0.6],
    weight: 3,
    group: 2,
  },
  {
    name: "Imperial Purple",
    main: [0.5, 0.1, 0.9],
    highlight: [0.2, 0.0, 0.4],
    weight: 1,
    group: 1,
  },
  {
    name: "Amber",
    main: [1.0, 0.78, 0.28],
    highlight: [1.0, 0.5, 0.0],
    weight: 4,
    group: 2,
  },
  {
    name: "Sherwood Green",
    main: [0.25, 0.5, 0.3],
    highlight: [0.1, 0.3, 0.1],
    weight: 0.5,
    group: 1,
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

const polygons = [
  "",
  "Monogon",
  "Bigon",
  "Triangle",
  "Quadrilateral",
  "Pentagon",
  "Hexagon",
  "Septagon",
  "Octagon",
  "Nonagon",
  "Decagon",
  "Undecagon",
  "Dodecagon",
  "Triskaidecagon",
  "Tetrakaidecagon",
  "Pentakaidecagon",
  "Hexakaidecagon",
  "Heptakaidecagon",
  "Octakaidecagon",
  "Enneakaidecagon",
];

module.exports = function generateVariables(random) {
  const paperSeed = random() + random() + random() + random();
  let primary = pickColor(random());
  let secondary = random() < 0.5 ? primary : pickColor(random());
  if (secondary.group !== primary.group && random() < 0.9) {
    secondary = primary;
  }

  const seed = random() * 999;

  let reverse_curve_x = random() < 0.16;
  let reverse_curve_y = random() < 0.16;
  let f1 = 2 * Math.floor(2 + 8 * random() * random());
  let f2 = random() < 0.85 ? f1 : 2 * Math.floor(2 + 8 * random() * random());
  let amp1 = 0.5 * random();
  let amp2 = 0.4 * random();
  let ricochets = Math.floor(
    3 -
      2 * random() * random() +
      (40 * random() * random() * random() * random() + 1) * random() * random()
  );
  let incr = mix(0.002, 0.01, random());
  let closing = ricochets > 2 && random() < 0.9;
  let Duality = scoring(random(), ["Disconnected", 0.1, "Alternance"]);
  const colordelta = Duality === "Disconnected" ? random() : incr / 2;
  const rad_start = 1 + 50 * random() * random();
  let rad_incr = 0.7 - 2 * incr + 0.06 * ricochets;
  let precision = 0.66;
  let max_passage = 7;

  const opts = {
    seed,
    primary_name: primary.name,
    secondary_name: secondary.name,
    width: 297.0,
    height: 210.0,
    pad: 10.0,
    reverse_curve_x,
    reverse_curve_y,
    f1,
    f2,
    amp1,
    amp2,
    ricochets,
    incr,
    closing,
    colordelta,
    rad_start,
    rad_incr,
    precision,
    max_passage,
  };

  const props = {};
  props["Inks Count"] = primary === secondary ? 1 : 2;
  props["Inks"] =
    primary === secondary
      ? primary.name
      : [primary.name, secondary.name].sort().join(" + ");

  props["Ink " + secondary.name] = primary === secondary ? "Both" : "Mountain";
  props["Ink " + primary.name] = primary === secondary ? "Both" : "Stars";
  props.Shape = closing
    ? polygons[ricochets] || ricochets + "-Polygon"
    : ricochets + "-Lines";
  props.Duality = Duality;
  props["Line Rotation"] = scoring(incr, [
    "Slow",
    0.004,
    "Medium",
    0.008,
    "Fast",
  ]);

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
