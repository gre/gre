/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – New Wave
 */

module.exports = function generateVariables(random, hash, debug = false) {
  const opts = {
    tpow: 2 + 2 * random(),
    scale: 2 + 8 * random() * random(),
    cloudy: 5 * random(),
    shaping: 2 * random() * random(),
    crazyness: 2 * random() * random() * random(),
    dysymmetry: 0.5 * Math.max(random() - 0.5, 0),
    dt: 0.2 * random(),
    s1: random(),
    s2: random(),
    s3: random(),
  };

  let n;
  n = opts.shaping + opts.crazyness;
  let Distortion =
    n < 0.2 ? "None" : n < 0.5 ? "Light" : n < 2.5 ? "Normal" : "Extreme";

  n = opts.cloudy;
  let Noise = n < 0.4 ? "None" : n < 1 ? "Light" : n < 4 ? "Normal" : "High";

  n = opts.scale;
  let Scale = n < 3 ? "Low" : n < 6 ? "Normal" : "High";

  const symmetry = random() < 0.8;
  const Symmetry = symmetry
    ? opts.dysymmetry > 0.1
      ? "Yes, with slight asymmetry"
      : "Yes"
    : "No";

  let Color;
  n = random();
  if (n < 0.02) {
    Color = "Harlequin";
  } else if (n < 0.05) {
    Color = "Green";
  } else if (n < 0.1) {
    Color = "Pink";
  } else if (n < 0.17) {
    Color = "Sand";
  } else if (n < 0.25) {
    Color = "Red";
  } else {
    Color = "Dark";
  }

  let Shape;
  n = random();
  if (n < 0.4) {
    Shape = "Square";
  } else if (n < 0.5) {
    Shape = "Circle";
  } else if (n < 0.7) {
    Shape = "X";
  } else if (n < 0.8) {
    Shape = "Y";
  } else {
    Shape = "Cross";
  }

  const props = {
    Color,
    Shape,
    Symmetry,
    Distortion,
    Noise,
    Scale,
  };
  console.table(props);

  // eslint-disable-next-line no-undef
  if (process.env.NODE_ENV !== "production" && typeof window !== "undefined") {
    console.log(window.fxhash);
    Object.keys(opts).forEach((key) => console.log(key + " =", opts[key]));
  }

  return {
    opts,
    props,
    symmetry,
  };
};

module.exports.inferProps = function inferProps(variables) {
  return variables.props;
};

module.exports.getPerf = function getPerf(svg) {
  const m = svg.match("data-perf='([^']+)'");
  if (!m) return;
  return JSON.parse(m[1]);
};
