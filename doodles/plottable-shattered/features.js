function scoring(value, sizes) {
  let i = 0;
  for (; i < sizes.length - 1; i += 2) {
    if (value < sizes[i + 1]) return sizes[i];
  }
  return sizes[i];
}

let features = {};

(function () {
  let [
    polygons,
    colors,
    ,
    ,
    maxpow,
    arearatio,
    sides,
    alignementlevel,
    diagAlignementlevel,
  ] = art(0);

  let FillingAlias = [
    "Empty",
    "Spiral",
    "Web",
    "Ping Pong",
    "Scratches",
    "Hatch",
    "Stippling",
    "Zigzag",
    "Empty",
  ];
  let areaPerShape = Array(FillingAlias.length).fill(0);
  let areaPerColor = Array(colors.length).fill(0);
  let areas = [];
  for (let [poly, clr, shape] of polygons) {
    let area = signed_area(poly);
    areaPerShape[shape] += area;
    areaPerColor[clr] += area;
    areas.push(area);
  }
  areas.sort((a, b) => b - a);

  let total = areas.reduce((sum, a) => sum + a, 0);
  if (areas.length > 2) {
    let first = areas[0] / total;
    let second = areas[1] / total;
    if (first > 0.7) {
      features.Distribution = "One Major";
    } else if ((first + second) * areas.length < 4) {
      features.Distribution = "Balanced";
    } else if (first > 0.5) {
      features.Distribution = "One Main";
    } else if (first + second > 0.6) {
      features.Distribution = "Two Main";
    } else if (first < 0.12) {
      features.Distribution = "Small Parts";
    }
  }

  let filtered = areaPerShape
    .map((area, shape) => [area, shape])
    .filter(([area]) => area > 400);
  features.Fill = filtered
    .sort((a, b) => b[0] - a[0])
    .map(([, i]) => FillingAlias[i])
    .join(", ");

  features.Parts =
    polygons.length < 2
      ? "Intact"
      : polygons.length < 10
      ? polygons.length
      : scoring(polygons.length, [
          "10-50",
          50,
          "50-100",
          100,
          "100-250",
          250,
          "250-500",
          500,
          "500-1000",
          1000,
          "Huge",
        ]);

  features.Palette = colors
    .map((c, i) => [c, i])
    .sort((a, b) => areaPerColor[b[1]] - areaPerColor[a[1]])
    .map(([[name]]) => name)
    .join(", ");

  const ShapeAlias = {
    3: "Triangle",
    4: "Square",
    6: "Hexagon",
    100: "Circle",
  };
  features.Shape = ShapeAlias[sides] || sides + "-Polygon";

  features.Pushback = scoring(maxpow, [
    "Very Small",
    1,
    "Small",
    2.8,
    "Normal",
    4.1,
    "High",
  ]);

  if (arearatio < 0.98) {
    features.Destructed = scoring(arearatio, ["Important", 0.8, "Partially"]);
  }

  if (alignementlevel) {
    features.Alignment =
      "Aligned level-" + (alignementlevel < 4 ? alignementlevel : "max");
  } else if (diagAlignementlevel > 2) {
    features.Alignment = "Diagonal";
  }
})();
