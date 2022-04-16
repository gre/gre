/*
Name: Plottable Fibers
Tags: plottable, webgl, svg, rust, wasm, A4, physical, phygital
Description:

More info: https://greweb.me/plots/465

Spring season is here! Plottable Fibers explore the beauty of perlin noise simulating the behavior of sprouting in Nature. There are many combination of parameters at stake: different intensity of noises, different width and elevation, different inks,...

The digital NFT is the recipe to a plottable art: Owning this NFT confers the right to request a physical plot (A4) – this is an optional decoupling as you can already enjoy the digital version:
- The digital representation simulates fountain pen drawing on paper. art published via a digital NFT on Tezos.
- Utility token to the physical world: Right Click Save to get the SVG recipe. using @greweb's services at https://greweb.me/plots/nft but also for any artist to interprete it with their own materials.

@greweb – 2022 – tech: WebGL + Rust + WASM – CC BY-SA 4.0 https://creativecommons.org/licenses/by-sa/4.0/
 */
const COLORS = [
  {
    name: "Bloody Brexit",
    main: [0.02, 0.12, 0.42],
    highlight: [0.18, 0.0, 0.2],
    weight: 2,
  },
  {
    name: "Black",
    main: [0.2, 0.2, 0.2],
    highlight: [0, 0, 0],
    weight: 8,
  },
  {
    name: "Indigo",
    main: [0.4, 0.5, 0.65],
    highlight: [0.2, 0.3, 0.4],
    weight: 4,
  },
  {
    name: "Amazing Amethyst",
    main: [0.6, 0.3, 0.7],
    highlight: [0.3, 0.1, 0.4],
    weight: 3,
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
    weight: 6,
  },
  {
    name: "Pumpkin",
    main: [1, 0.5, 0.2],
    highlight: [0.9, 0.3, 0.0],
    weight: 4,
  },
  {
    name: "Pink",
    main: [1.0, 0.32, 0.46],
    highlight: [0.9, 0.38, 0.3],
    weight: 4,
  },
  {
    name: "Hope Pink",
    main: [1.0, 0.4, 0.75],
    highlight: [0.9, 0.2, 0.6],
    weight: 4,
  },
  {
    name: "Imperial Purple",
    main: [0.5, 0.1, 0.9],
    highlight: [0.2, 0.0, 0.4],
    weight: 2,
  },
  {
    name: "Amber",
    main: [1.0, 0.78, 0.28],
    highlight: [1.0, 0.5, 0.0],
    weight: 7,
  },
  {
    name: "Sherwood Green",
    main: [0.25, 0.5, 0.3],
    highlight: [0.1, 0.3, 0.1],
    weight: 3,
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

module.exports = function generateVariables(random) {
  const paperSeed = random() + random() + random() + random();
  let primary = pickColor(random());
  let secondary = random() < 0.35 ? pickColor(random()) : primary;
  const seed = random() * 999;

  const opts = {
    seed,
    primary_name: primary.name,
    secondary_name: secondary.name,
  };

  const props = {
    "Inks Count": primary === secondary ? 1 : 2,
    Inks:
      primary === secondary
        ? primary.name
        : [primary.name, secondary.name].sort().join(" + "),
    ["Ink " + secondary.name]: primary === secondary ? "Both" : "Mountain",
    ["Ink " + primary.name]: primary === secondary ? "Both" : "Stars",
  };

  // eslint-disable-next-line no-undef
  if (process.env.NODE_ENV !== "production" && typeof window !== "undefined") {
    console.log(window.fxhash);
    Object.keys(opts).forEach((key) => console.log(key + " =", opts[key]));
  }

  return {
    props,
    opts,
    primary,
    secondary,
    paperSeed,
  };
};

module.exports.inferProps = function inferProps(variables, svg) {
  const m = svg.match("data-traits='([^']+)'");
  const props = JSON.parse(m[1]);
  if (variables.props["Inks Count"] === 1) {
    delete props["Ink Distribution"];
  }
  return {
    ...variables.props,
    ...props,
  };
};
