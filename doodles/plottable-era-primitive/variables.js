/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Plottable Era: (I) Primitive
 */

const blackPaper = [0.1, 0.1, 0.1];
const gelWhiteOnBlack = {
  name: "Gel White",
  main: [0.9, 0.9, 0.9],
  highlight: [1, 1, 1],
  weight: 32,
  blackPaper: true,
  bg: blackPaper,
  bgTag: "black",
};
const gelGoldOnBlack = {
  name: "Gel Gold",
  main: [0.85, 0.7, 0.25],
  highlight: [1, 0.9, 0.55],
  weight: 9,
  blackPaper: true,
  bg: blackPaper,
  bgTag: "black",
};
const gelGreenOnBlack = {
  name: "Gel Green",
  main: [0.0, 0.7, 0.65],
  highlight: [0.1, 0.8, 0.75],
  weight: 8,
  blackPaper: true,
  bg: blackPaper,
  bgTag: "black",
};
const gelBlueOnBlack = {
  name: "Gel Blue",
  main: [0.2, 0.55, 1],
  highlight: [0.3, 0.55, 1],
  weight: 4,
  blackPaper: true,
  bg: blackPaper,
  bgTag: "black",
};
const gelOrangeOnBlack = {
  name: "Gel Orange",
  main: [0.7, 0.45, 0.2],
  highlight: [0.9, 0.55, 0.3],
  weight: 3,
  blackPaper: true,
  bg: blackPaper,
  bgTag: "black",
};
const gelRedOnBlack = {
  name: "Gel Red",
  main: [0.75, 0.45, 0.55],
  highlight: [0.85, 0.5, 0.65],
  weight: 11,
  blackPaper: true,
  bg: blackPaper,
  bgTag: "black",
};
const BlackGelPrimaryChoices = [
  gelWhiteOnBlack,
  gelGoldOnBlack,
  gelGreenOnBlack,
  gelBlueOnBlack,
  gelOrangeOnBlack,
  gelRedOnBlack,
];
const BlackGelSecondaryChoices = [
  gelWhiteOnBlack,
  gelGoldOnBlack,
  gelOrangeOnBlack,
  gelRedOnBlack,
  gelGreenOnBlack,
  gelBlueOnBlack,
];

const bluePaper = [0.3, 0.73, 0.86];
const gelWhiteOnBlue = {
  name: "Gel White",
  main: [0.9, 0.9, 0.9],
  highlight: [1.1, 1.1, 1.1],
  weight: 4,
  blackPaper: true,
  bg: bluePaper,
  bgTag: "blue",
};
const blackOnBlue = {
  name: "Black",
  main: [0.1, 0.1, 0.1],
  highlight: [0, 0, 0],
  weight: 4,
  blackPaper: true,
  bg: bluePaper,
  bgTag: "blue",
};

const FOUNTAIN_PRIMARY_CHOICES = [
  {
    name: "Black",
    main: [0.2, 0.2, 0.2],
    highlight: [0, 0, 0],
    weight: 60,
  },
  {
    name: "Indigo",
    main: [0.4, 0.5, 0.65],
    highlight: [0.2, 0.3, 0.4],
    weight: 20,
  },
  {
    name: "Amber",
    main: [1.0, 0.78, 0.28],
    highlight: [1.0, 0.5, 0.0],
    weight: 15,
  },
  {
    name: "FireAndIce",
    main: [0 / 255, 190 / 255, 220 / 255],
    highlight: [0 / 255, 100 / 255, 120 / 255],
    weight: 8,
  },
  {
    name: "Bloody Brexit",
    main: [0.02, 0.12, 0.42],
    highlight: [0.18, 0.0, 0.2],
    weight: 4,
  },
  {
    name: "Amazing Amethyst",
    main: [0.6, 0.3, 0.7],
    highlight: [0.3, 0.1, 0.4],
    weight: 2,
  },
  {
    name: "Imperial Purple",
    main: [0.3, 0.0, 0.6],
    highlight: [0.15, 0.1, 0.2],
    weight: 1,
  },
  {
    name: "Evergreen",
    main: [0.3, 0.4, 0.2],
    highlight: [0.15, 0.2, 0.1],
    weight: 1,
  },
  {
    name: "Poppy Red",
    main: [0.9, 0.2, 0.1],
    highlight: [0.5, 0.0, 0.1],
    weight: 1,
  },
];

const FOUNTAIN_SECONDARY_CHOICES = [
  {
    name: "Amber",
    main: [1.0, 0.78, 0.28],
    highlight: [1.0, 0.5, 0.0],
    weight: 40,
  },
  {
    name: "Pink",
    main: [1.0, 0.32, 0.46],
    highlight: [0.9, 0.38, 0.3],
    weight: 22,
  },
  {
    name: "Poppy Red",
    main: [0.9, 0.2, 0.1],
    highlight: [0.5, 0.0, 0.1],
    weight: 20,
  },
  {
    name: "Pumpkin",
    main: [1, 0.5, 0.2],
    highlight: [0.9, 0.3, 0.0],
    weight: 7,
  },
  {
    name: "Hope Pink",
    main: [1.0, 0.4, 0.75],
    highlight: [0.9, 0.2, 0.6],
    weight: 6,
  },
  {
    name: "Soft Mint",
    main: [0.2, 0.88, 0.8],
    highlight: [0.1, 0.7, 0.6],
    weight: 1,
  },
];

module.exports = function generateVariables(random, hash) {
  function pickColor(choices) {
    const colorsWeighted = [];
    for (let i = 0; i < choices.length; i++) {
      const c = choices[i];
      for (let j = 0; j < c.weight; j++) {
        colorsWeighted.push(c);
      }
    }
    return colorsWeighted[
      Math.floor(0.99999 * random() * colorsWeighted.length) %
        colorsWeighted.length
    ];
  }

  const paperSeed = 10 * random();
  let primary, secondary;

  if (random() < 0.1) {
    primary = blackOnBlue;
    secondary = gelWhiteOnBlue;
  } else {
    if (random() < 0.5) {
      primary = pickColor(BlackGelPrimaryChoices);
      if (random() < 0.7) {
        if (primary !== gelWhiteOnBlack) {
          secondary = gelWhiteOnBlack;
        } else if (primary !== gelGoldOnBlack) {
          secondary = gelGoldOnBlack;
        }
      } else {
        secondary = pickColor(BlackGelSecondaryChoices);
      }
      if (random() < 0.6 && secondary === primary) {
        secondary = pickColor(BlackGelSecondaryChoices);
      }
      if (
        random() < 0.6 &&
        secondary === gelWhiteOnBlack &&
        primary === gelGoldOnBlack
      ) {
        secondary = gelOrangeOnBlack;
        primary = gelWhiteOnBlack;
      }
      if (
        (primary === gelGoldOnBlack && secondary === gelOrangeOnBlack) ||
        (secondary === gelGoldOnBlack && primary === gelOrangeOnBlack)
      ) {
        primary = gelWhiteOnBlack;
      }
    } else {
      primary = pickColor(FOUNTAIN_PRIMARY_CHOICES);
      secondary = pickColor(FOUNTAIN_SECONDARY_CHOICES);
      if (random() < 0.5 && secondary === primary) {
        secondary = pickColor(FOUNTAIN_SECONDARY_CHOICES);
      }
    }
  }

  if (random() < 0.8 && primary.name === "Evergreen") {
    secondary = primary;
  }

  if (random() < 0.01) {
    secondary = primary;
  }
  if (random() < 0.02) {
    primary = secondary;
  }

  const seed = random() * 9999;

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
  props.Paper = variables.primary.bgTag || "white";
  for (let k in props) {
    if (!props[k]) {
      delete props[k];
    }
  }
  return props;
};
