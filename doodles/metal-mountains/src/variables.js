/**
 * LICENSE: ...
 * Author: ...
 */

const width = 160;
const height = 96;
const pad = 6;

const metals = [
  {
    name: "Black",
    placeholder: "white",
    rgb: [43 / 255, 39 / 255, 44 / 255],
  },
  {
    name: "Blue",
    placeholder: "white",
    rgb: [0, 80 / 255, 163 / 255],
  },
  /*
  {
    name: "Orange",
    placeholder: "white",
    rgb: [212 / 255, 139 / 255, 78 / 255],
  },
  {
    name: "Red",
    placeholder: "white",
    rgb: [198 / 255, 61 / 255, 64 / 255],
  },
  {
    name: "Purple",
    placeholder: "white",
    rgb: [127 / 255, 40 / 255, 116 / 255],
  },
  */
];

module.exports = function generateVariables(random, hash, debug = false) {
  let background = metals[random() > 0.2 ? 0 : 1];
  let layers = [{ name: background.name, search: /#000/g, rgb: [1, 1, 1] }];

  const opts = {
    layer1_name: layers[0].name,
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
    layers,
    background,
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
