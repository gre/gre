/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – GREWEBPAJONCOLLAB
 */

import { width, height, pad } from "./constants";

function loadImage(src) {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = () => resolve(img);
    img.onerror = reject;
    img.crossOrigin = "anonymous";
    img.src = src;
  });
}

function loadImageData(src) {
  return loadImage(src).then((img) => {
    const { width, height } = img;
    const canvas = document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;
    const ctx = canvas.getContext("2d");
    ctx.drawImage(img, 0, 0);
    const imageData = ctx.getImageData(0, 0, width, height);
    const data = new Array(imageData.data.length);
    for (var i = 0; i < data.length; i++) {
      data[i] = imageData.data[i];
    }
    return {
      width,
      height,
      data,
    };
  });
}

export async function generateVariables(hash, random) {
  const imageCount = (1 + 2 * random()) | 0;
  const images = await Promise.all(Array(imageCount).fill(null).map(() => loadImageData(`./images/${(random() * 99 + 1) | 0}.jpg`)));
  const opts = {
    hash,
    width,
    height,
    images,
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
