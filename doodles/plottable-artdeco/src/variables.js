/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – ArtDeco
 */

const width = 297;
const height = 210;
const pad = 20;

const darkBluePaper = [0.1, 0.1, 0.2];
const gelGoldOnBlack = {
  name: "Gel Gold On Black",
  main: [0.85, 0.7, 0.25],
  highlight: [1, 0.9, 0.55],
  blackPaper: true,
  bg: darkBluePaper,
};

const gelGoldOnWhite = {
  name: "Gel Gold On White",
  main: [0.85, 0.7, 0.25],
  highlight: [1, 0.9, 0.55],
  blackPaper: false,
};

module.exports = function generateVariables(random, hash, debug = false) {
  let color = random() < 0.1 ? gelGoldOnWhite: gelGoldOnBlack
  const opts = {
    layer1_name: color.name,
    width,
    height,
    pad,
    hash,
    debug,
  };

  // eslint-disable-next-line no-undef
  if (process.env.NODE_ENV !== "production" && typeof window !== "undefined") {
    console.log(window.fxhash);
    Object.keys(opts).forEach((key) => console.log(key + " =", opts[key]));
  }

  return {
    opts,
    primary: color,
    secondary: color
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

module.exports.getPerf = function getPerf(svg) {
  const m = svg.match("data-perf='([^']+)'");
  if (!m) return;
  return JSON.parse(m[1]);
};

module.exports.width = width;
module.exports.height = height;
module.exports.pad = pad;
