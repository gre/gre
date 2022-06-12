/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Field
 */
const COLORS = [
  {
    name: "Black",
    main: [0.2, 0.2, 0.2],
    highlight: [0, 0, 0],
  },
  {
    name: "Poppy Red",
    main: [0.9, 0.2, 0.1],
    highlight: [0.5, 0.0, 0.1],
  },
];

const COLORS_DARK = [
  {
    name: "WhiteOnBlack",
    main: [0.8, 0.8, 0.8],
    highlight: [1, 1, 1],
    blackPaper: true,
    bg: [0, 0, 0],
  },
  {
    name: "GoldOnBlack",
    main: [0.85, 0.6, 0.2],
    highlight: [1, 0.85, 0.5],
    blackPaper: true,
    bg: [0, 0, 0],
  },
];

module.exports = function generateVariables(random, hash) {
  const randoms = Array(20)
    .fill(null)
    .map(() => random());
  const paperSeed = 10 * random();
  const dark_mode = random() < 0.2;
  const [primary, secondary] = dark_mode ? COLORS_DARK : COLORS;

  const seed = random() * 999;

  const opts = {
    seed,
    primary_name: primary.name,
    secondary_name: secondary.name,
    dark_mode,
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
