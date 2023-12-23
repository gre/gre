/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – TEMPLATE
 */

import("./pkg").then((rust) => {
  const $koda = window.$koda;
  const debug = $koda.debug;
  const hash = $koda.hash;
  const width = 210;
  const height = 297;
  const pad = 10;
  const precision = 0.2;

  // Generate the SVG

  const prev = Date.now();
  const svg = rust.render(hash, width, height, pad, precision, true, debug);
  console.log("generated in " + (Date.now() - prev) + "ms");

  if (debug) {
    const perf = JSON.parse(svg.match("data-perf='([^']+)'")[1]);
    console.table(perf.per_label);
  }

  const palette = JSON.parse(svg.match("data-palette='([^']+)'")[1]);

  const props = {};
  const _props = JSON.parse(svg.match("data-traits='([^']+)'")[1]);
  for (let k in _props) {
    if (_props[k]) {
      props[camelCaseFeature(k)] = _props[k];
    }
  }
  $koda.features(props);

  if (debug) {
    console.table(props);
    console.log(hash);
  }

  // Display the SVG

  let DOC = document;
  let BODY = DOC.body;
  let assign = Object.assign;
  let CENTER = "center";
  let HUNDREDPC = "100%";

  let createElement = (e) => DOC.createElement(e);
  let append = (n, e) => n.appendChild(e);
  let makeSVGDataImage = (svg) => "data:image/svg+xml;base64," + btoa(svg);

  assign(BODY.style, {
    display: "flex",
    alignItems: CENTER,
    justifyContent: CENTER,
    backgroundColor: palette.paper[1],
    margin: 0,
    padding: 0,
    width: HUNDREDPC,
    height: "100vh",
    overflow: "hidden",
  });

  let bgImage = createElement("img");
  bgImage.src = makeSVGDataImage(
    svg
      .replace("background:white", `background:${palette.paper[1]}`)
      .replace(/opacity="[^"]*"/g, 'style="mix-blend-mode: multiply"')
      .replace(/#0FF/g, palette.primary[1])
      .replace(/#F0F/g, palette.secondary[1])
      .replace(/#FF0/g, palette.third[1]),
  );
  assign(bgImage.style, {
    top: 0,
    left: 0,
    width: HUNDREDPC,
    height: HUNDREDPC,
  });

  append(BODY, bgImage);

  // global helpers
  function camelCaseFeature(key) {
    let keyInCamelCase = "";
    let shouldUppercase = true;
    for (let i = 0; i < key.length; i++) {
      const c = key[i];
      if (shouldUppercase) {
        keyInCamelCase += c.toUpperCase();
        shouldUppercase = false;
      } else if (c === "_") {
        shouldUppercase = true;
        keyInCamelCase += " ";
      } else {
        keyInCamelCase += c;
      }
    }
    return keyInCamelCase;
  }

  function colorRgb(str) {
    let r, g, b;

    // Check for #rgb or #rrggbb format
    if (str[0] === "#") {
      if (str.length === 4) {
        // Convert #rgb to #rrggbb
        str = "#" + str[1] + str[1] + str[2] + str[2] + str[3] + str[3];
      }
      r = parseInt(str.substr(1, 2), 16);
      g = parseInt(str.substr(3, 2), 16);
      b = parseInt(str.substr(5, 2), 16);
    }
    // Check for rgb(r, g, b) format
    else if (str.startsWith("rgb(")) {
      const parts = str.match(/rgb\((\d+),\s*(\d+),\s*(\d+)\)/);
      [r, g, b] = parts.slice(1, 4).map(Number);
    }
    // Convert to normalized RGB values
    return [r, g, b].map((x) => x / 255);
  }
});
