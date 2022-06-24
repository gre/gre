/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Thousands
 */

const GEL_COLORS = [
  {
    name: "WhiteOnBlack",
    main: [0.8, 0.8, 0.8],
    highlight: [1, 1, 1],
    weight: 10,
    soloChance: 0.5,
    blackPaper: true,
    bg: [0, 0, 0],
  },
  {
    name: "GoldOnBlack",
    main: [0.85, 0.6, 0.2],
    highlight: [1, 0.85, 0.5],
    weight: 5,
    soloChance: 0,
    blackPaper: true,
    bg: [0, 0, 0],
  },
  {
    name: "WhiteOnRed",
    main: [0.8, 0.8, 0.8],
    highlight: [1, 1, 1],
    weight: 2,
    soloChance: 1,
    blackPaper: true,
    bg: [0.3, 0, 0],
  },
  {
    name: "WhiteOnBlue",
    main: [0.8, 0.8, 0.8],
    highlight: [1, 1, 1],
    weight: 1,
    soloChance: 1,
    blackPaper: true,
    bg: [0.1, 0.32, 0.8],
  },
];

const COLORS = [
  {
    name: "Bloody Brexit",
    main: [0.02, 0.12, 0.42],
    highlight: [0.18, 0.0, 0.2],
    weight: 20,
    soloChance: 0.7,
  },
  {
    name: "Black",
    main: [0.2, 0.2, 0.2],
    highlight: [0, 0, 0],
    weight: 40,
    soloChance: 0.5,
  },
  {
    name: "Indigo",
    main: [0.4, 0.5, 0.65],
    highlight: [0.2, 0.3, 0.4],
    weight: 15,
    soloChance: 0.5,
  },
  {
    name: "FireAndIce",
    main: [0 / 255, 190 / 255, 220 / 255],
    highlight: [0 / 255, 100 / 255, 120 / 255],
    weight: 15,
    soloChance: 0.01,
  },
  {
    name: "Poppy Red",
    main: [0.9, 0.2, 0.1],
    highlight: [0.5, 0.0, 0.1],
    weight: 20,
    soloChance: 0.01,
  },
  {
    name: "Aurora Borealis",
    main: [0.0, 0.6, 0.6],
    highlight: [0.0, 0.3, 0.4],
    weight: 8,
    soloChance: 0.01,
  },
  {
    name: "Pumpkin",
    main: [1, 0.5, 0.2],
    highlight: [0.9, 0.3, 0.0],
    weight: 1,
    soloChance: 0.2,
  },
  {
    name: "Pink",
    main: [1.0, 0.32, 0.46],
    highlight: [0.9, 0.38, 0.3],
    weight: 7,
    soloChance: 0.01,
  },
  {
    name: "Hope Pink",
    main: [1.0, 0.4, 0.75],
    highlight: [0.9, 0.2, 0.6],
    weight: 7,
    soloChance: 0.01,
  },
  {
    name: "Amber",
    main: [1.0, 0.78, 0.28],
    highlight: [1.0, 0.5, 0.0],
    weight: 13,
  },
  {
    name: "Imperial Purple",
    main: [0.3, 0.0, 0.6],
    highlight: [0.15, 0.1, 0.2],
    weight: 1,
    soloChance: 0.8,
  },
  {
    name: "Sherwood Green",
    main: [0.25, 0.5, 0.3],
    highlight: [0.1, 0.3, 0.1],
    weight: 1,
    soloChance: 0.9,
  },
  {
    name: "Amazing Amethyst",
    main: [0.6, 0.3, 0.7],
    highlight: [0.3, 0.1, 0.4],
    weight: 4,
    soloChance: 0.5,
  },
  {
    name: "Evergreen",
    main: [0.3, 0.4, 0.2],
    highlight: [0.15, 0.2, 0.1],
    weight: 2,
    soloChance: 0.8,
  },
  {
    name: "Soft Mint",
    main: [0.2, 0.88, 0.8],
    highlight: [0.1, 0.7, 0.6],
    weight: 4,
    soloChance: 0.1,
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

const gelColorsWeighted = [];
for (let i = 0; i < GEL_COLORS.length; i++) {
  const c = GEL_COLORS[i];
  for (let j = 0; j < c.weight; j++) {
    gelColorsWeighted.push(c);
  }
}
const pickGelColor = (f) =>
  gelColorsWeighted[
    Math.floor(0.99999 * f * gelColorsWeighted.length) %
      gelColorsWeighted.length
  ];

module.exports = function generateVariables(random, hash) {
  const paperSeed = 10 * random();
  let primary, secondary;

  if (random() < 0.3) {
    primary = pickGelColor(random());
    secondary = pickGelColor(random());
    if (secondary.name === "GoldOnBlack" && random() < 0.8) {
      let tmp = secondary;
      secondary = primary;
      primary = tmp;
    }

    if (random() < (primary.soloChance || 0)) {
      secondary = primary;
    }
    if (random() < (secondary.soloChance || 0)) {
      primary = secondary;
    }
  } else {
    primary = pickColor(random());
    secondary = pickColor(random());

    if (random() < (primary.soloChance || 0)) {
      secondary = primary;
    }
    if (random() < (secondary.soloChance || 0)) {
      primary = secondary;
    }
  }

  /*
  if (random() < 0.2) {
    primary = COLORS[4];
    secondary = COLORS[7];
  }
  if (primary.name === "Poppy Red" && secondary.name === "FireAndIce") {
    const tmp = secondary;
    secondary = primary;
    primary = tmp;
  }  */

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
