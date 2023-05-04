/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era (II) Medieval
 */

import { width, height, pad } from "./constants";

export function generateVariables(hash,) {
  const opts = {
    hash,
    width,
    height,
    pad,
  };
  return opts;
}

export function inferPalette(svg) {
  const m = svg.match("data-palette='([^']+)'");
  const props = JSON.parse(m[1]);
  return props;
}

export function inferProps(svg) {
  const m = svg.match("data-traits='([^']+)'");
  const props = JSON.parse(m[1]);
  const r = {};
  for (let k in props) {
    if (props[k]) {
      r[camelCaseFeature(k)] = props[k];
    }
  }
  return r;
}

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
