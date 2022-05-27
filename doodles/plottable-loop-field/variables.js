/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Field
 */
const COLORS = [
  {
    name: "WhiteOnBlack",
    main: [0.8, 0.8, 0.8],
    highlight: [1, 1, 1],
    weight: 5,
    soloChance: 1,
    blackPaper: true,
    bg: [0, 0, 0],
    group: 1,
  },
  {
    name: "WhiteOnRed",
    main: [0.8, 0.8, 0.8],
    highlight: [1, 1, 1],
    weight: 2,
    soloChance: 1,
    blackPaper: true,
    bg: [0.3, 0, 0],
    group: 1,
  },
  {
    name: "WhiteOnBlue",
    main: [0.8, 0.8, 0.8],
    highlight: [1, 1, 1],
    weight: 1,
    soloChance: 1,
    blackPaper: true,
    bg: [0.1, 0.32, 0.8],
    group: 1,
  },
  {
    name: "Bloody Brexit",
    main: [0.02, 0.12, 0.42],
    highlight: [0.18, 0.0, 0.2],
    weight: 11,
    soloChance: 0.4,
    avoidGroup: 0.5,
    group: 2,
  },
  {
    name: "Black",
    main: [0.2, 0.2, 0.2],
    highlight: [0, 0, 0],
    weight: 21,
    soloChance: 0.6,
    avoidGroup: 0.4,
    group: 2,
  },
  {
    name: "FireAndIce",
    main: [0 / 255, 190 / 255, 220 / 255],
    highlight: [0 / 255, 100 / 255, 120 / 255],
    weight: 16,
    soloChance: 0.1,
    avoidGroup: 0.1,
    group: 3,
  },
  {
    name: "Indigo",
    main: [0.4, 0.5, 0.65],
    highlight: [0.2, 0.3, 0.4],
    weight: 7,
    soloChance: 0.15,
    avoidGroup: 0.4,
    group: 2,
  },
  {
    name: "Poppy Red",
    main: [0.9, 0.2, 0.1],
    highlight: [0.5, 0.0, 0.1],
    weight: 12,
    soloChance: 0.1,
    group: 0,
  },
  {
    name: "Aurora Borealis",
    main: [0.0, 0.6, 0.6],
    highlight: [0.0, 0.3, 0.4],
    weight: 3,
    soloChance: 0.5,
    avoidGroup: 0.3,
    group: 3,
  },
  {
    name: "Pumpkin",
    main: [1, 0.5, 0.2],
    highlight: [0.9, 0.3, 0.0],
    weight: 6,
    soloChance: 0.2,
    avoidGroup: 0.4,
    group: 0,
  },
  {
    name: "Pink",
    main: [1.0, 0.32, 0.46],
    highlight: [0.9, 0.38, 0.3],
    weight: 21,
    soloChance: 0.1,
    avoidGroup: 0.4,
    group: 0,
  },
  {
    name: "Hope Pink",
    main: [1.0, 0.4, 0.75],
    highlight: [0.9, 0.2, 0.6],
    weight: 14,
    group: 0,
    soloChance: 0.1,
    avoidGroup: 0.4,
  },
  {
    name: "Amber",
    main: [1.0, 0.78, 0.28],
    highlight: [1.0, 0.5, 0.0],
    weight: 34,
    group: 4,
    soloChance: 0.05,
    avoidGroup: 0.1,
  },
  {
    name: "Imperial Purple",
    main: [0.3, 0.0, 0.6],
    highlight: [0.15, 0.1, 0.2],
    weight: 4,
    soloChance: 0.5,
    group: 5,
  },
  {
    name: "Sherwood Green",
    main: [0.2, 0.45, 0.25],
    highlight: [0.1, 0.3, 0.1],
    weight: 3,
    soloChance: 0.8,
    group: 6,
  },
  {
    name: "Evergreen",
    main: [0.3, 0.4, 0.2],
    highlight: [0.15, 0.2, 0.1],
    weight: 5,
    soloChance: 0.4,
    group: 6,
  },
  {
    name: "Soft Mint",
    main: [0.2, 0.88, 0.8],
    highlight: [0.1, 0.7, 0.6],
    weight: 14,
    group: 3,
    soloChance: 0.1,
    avoidGroup: 0.1,
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

module.exports = function generateVariables(random, hash) {
  const randoms = Array(20)
    .fill(null)
    .map(() => random());
  const paperSeed = 10 * random();
  let primary = pickColor(random());
  let secondary = pickColor(random());
  if (random() < (primary.soloChance || 0)) {
    secondary = primary;
  }
  if (random() < (secondary.soloChance || 0)) {
    primary = secondary;
  }
  if (
    primary.group === secondary.group &&
    random() < (primary.avoidGroup || 0) + (secondary.avoidGroup || 0)
  ) {
    secondary = primary;
  }

  const seed = random() * 999;

  const opts = {
    seed,
    primary_name: primary.name,
    secondary_name: secondary.name,
    hash,
  };

  // eslint-disable-next-line no-undef
  if (process.env.NODE_ENV !== "production" && typeof window !== "undefined") {
    console.log(window.fxhash);
    Object.keys(opts).forEach((key) => console.log(key + " =", opts[key]));
  }

  return {
    opts,
    primary,
    secondary,
    paperSeed,
    randoms,
  };
};

module.exports.inferProps = function inferProps(variables, svg) {
  const m = svg.match("data-traits='([^']+)'");
  const props = JSON.parse(m[1]);
  for (let k in props) {
    if (!props[k]) {
      delete props[k];
    }
  }
  return props;
};
