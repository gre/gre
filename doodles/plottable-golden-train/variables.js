/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Thousands
 */

const blackPaper = [0.1, 0.1, 0.1];
const darkBluePaper = [0.1, 0.1, 0.2];

const gelGoldOnBlack = {
  name: "Gold",
  main: [0.85, 0.7, 0.25],
  highlight: [1, 0.9, 0.55],
  blackPaper: true,
  bg: blackPaper,
  bgTag: "Black",
};

const gelGoldOnDarkBlue = {
  name: "Gold",
  main: [0.85, 0.7, 0.25],
  highlight: [1, 0.9, 0.55],
  blackPaper: true,
  bg: darkBluePaper,
  bgTag: "Dark Blue",
};

const gelGoldOnWhite = {
  name: "Gold",
  main: [0.85, 0.7, 0.25],
  highlight: [1, 0.9, 0.55],
  blackPaper: false,
};

const gelWhiteOnBlack = {
  name: "White",
  main: [0.9, 0.9, 0.9],
  highlight: [1, 1, 1],
  blackPaper: true,
  bg: blackPaper,
  bgTag: "Black",
};

const gelWhiteOnDarkBlue = {
  name: "White",
  main: [0.9, 0.9, 0.9],
  highlight: [1, 1, 1],
  blackPaper: true,
  bg: darkBluePaper,
  bgTag: "Dark Blue",
};

const blackInk = {
  name: "Black",
  main: [0.2, 0.2, 0.2],
  highlight: [0, 0, 0],
};

const greyPaper = [0.69, 0.72, 0.71];

const blackInkOnGrey = {
  name: "Black",
  main: [0.1, 0.1, 0.1],
  highlight: [0, 0, 0],
  bg: greyPaper,
  blackPaper: true,
  bgTag: "Grey",
};

const gelInkOnGrey = {
  name: "White",
  main: [0.9, 0.9, 0.9],
  highlight: [1, 1, 1],
  bg: greyPaper,
  blackPaper: true,
  bgTag: "Grey",
};

const gelGoldOnGrey = {
  name: "Gold",
  main: [0.92, 0.75, 0.35],
  highlight: [1, 0.9, 0.55],
  bg: greyPaper,
  blackPaper: true,
  bgTag: "Grey",
};

module.exports = function generateVariables(random, hash, debug = false) {
  const paperSeed = 10 * random();
  let primary = blackInk;
  let secondary = gelGoldOnWhite;
  let gold_border = false;

  if (random() < 0.33) {
    primary = gelWhiteOnBlack;
    secondary = gelGoldOnBlack;
  } else if (random() < 0.1) {
    primary = gelWhiteOnDarkBlue;
    secondary = gelGoldOnDarkBlue;
  } else if (random() < 0.1) {
    primary = random() < 0.3 ? gelInkOnGrey : blackInkOnGrey;
    secondary = gelGoldOnGrey;
  }

  if (random() < 0.02) {
    primary = secondary;
  }
  if (random() < 0.05) {
    secondary = primary;
  }

  if (secondary === gelGoldOnGrey) {
    gold_border = random() < 0.5;
  } else if (secondary === gelGoldOnBlack || secondary === gelGoldOnDarkBlue) {
    gold_border = random() < 0.05;
  } else if (secondary === gelGoldOnWhite) {
    gold_border = random() < 0.01;
  }

  const opts = {
    primary_name: primary.name,
    secondary_name: secondary.name,
    gold_border,
    hash,
    debug,
  };

  // eslint-disable-next-line no-undef
  if (process.env.NODE_ENV !== "production" && typeof window !== "undefined") {
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
  props.Paper = variables.primary.bgTag || "White";
  for (let k in props) {
    if (!props[k]) {
      delete props[k];
    }
  }
  return props;
};
