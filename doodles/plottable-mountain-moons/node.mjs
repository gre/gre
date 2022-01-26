import generateVariables from "./variables.js";
import fs from "fs";
import path from "path";

const __dirname = path.resolve(
  path.dirname(decodeURI(new URL(import.meta.url).pathname))
);

function newRandom() {
  let alphabet = "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
  var fxhash =
    "oo" +
    Array(49)
      .fill(0)
      .map((_) => alphabet[(Math.random() * alphabet.length) | 0])
      .join("");
  let b58dec = (str) =>
    str
      .split("")
      .reduce(
        (p, c, i) =>
          p +
          alphabet.indexOf(c) * Math.pow(alphabet.length, str.length - i - 1),
        0
      );
  let fxhashTrunc = fxhash.slice(2);
  let regex = new RegExp(".{" + ((fxhashTrunc.length / 4) | 0) + "}", "g");
  let hashes = fxhashTrunc.match(regex).map((h) => b58dec(h));
  let sfc32 = (a, b, c, d) => {
    return () => {
      a |= 0;
      b |= 0;
      c |= 0;
      d |= 0;
      var t = (((a + b) | 0) + d) | 0;
      d = (d + 1) | 0;
      a = b ^ (b >>> 9);
      b = (c + (c << 3)) | 0;
      c = (c << 21) | (c >>> 11);
      c = (c + t) | 0;
      return (t >>> 0) / 4294967296;
    };
  };
  var fxrand = sfc32(...hashes);
  return {
    random: () => {
      // hack a bit the provided fn which don't have enough entropy to me
      if (fxrand() < 0.5) return fxrand();
      if (fxrand() > 0.5) return fxrand();
      return fxrand();
    },
    fxhash,
  };
}

const argv2 = process.argv[2];
if (!isNaN(argv2)) {
  const count = parseInt(argv2, 10);
  const all = [];
  const keys = {};
  for (let i = 0; i < count; i++) {
    const { random, fxhash } = newRandom();
    const res = generateVariables(random);
    for (let k in res.props) {
      keys[k] = 1;
    }
    all.push({ fxhash, props: res.props });
  }

  Object.keys(keys)
    .sort()
    .forEach((k) => {
      const counters = {};
      for (let i = 0; i < all.length; i++) {
        const v = all[i];
        counters[v.props[k]] = (counters[v.props[k]] || 0) + 1;
      }
      console.log(
        k +
          "\n" +
          Object.entries(counters)
            .sort((a, b) => b[1] - a[1])
            .map(
              ([k, v]) =>
                k.padStart(30) +
                ": " +
                ((100 * v) / all.length).toFixed(1) +
                "%"
            )
            .join("\n")
      );
      // fs.writeFileSync("study.json", JSON.stringify(all), "utf-8");
    });
} else {
  const wasmBuffer = fs.readFileSync(
    path.join(__dirname, "./rust/pkg/main_bg.wasm")
  );
  const wasmLoad = import("./rust/pkg/main.mjs");
  wasmLoad.then(async (wasmModule) => {
    await wasmModule.default(wasmBuffer);
    const predicate = new Function("p,o", "return (" + (argv2 || "true") + ")");
    let r, hash;
    let always = true;
    while (always) {
      const { fxhash, random } = newRandom();
      r = generateVariables(random);
      hash = fxhash;
      if (predicate(r.props, r.opts)) {
        console.log(hash);
        const svg = wasmModule
          .render(r.opts)
          .replace(/opacity="[^"]*"/g, 'style="mix-blend-mode: multiply"')
          .replace(
            /#0FF/g,
            "rgb(" +
              r.primary.main.map((n) => Math.round(n * 255)).join(",") +
              ")"
          )
          .replace(
            /#F0F/g,
            "rgb(" +
              r.secondary.main.map((n) => Math.round(n * 255)).join(",") +
              ")"
          );
        fs.writeFileSync("dist/" + hash + ".svg", svg, "utf-8");
      }
    }
  });
}
