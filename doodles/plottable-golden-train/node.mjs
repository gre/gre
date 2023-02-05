import generateVariables from "./variables.js";
import fs from "fs";
import path from "path";

const __dirname = path.resolve(
  path.dirname(decodeURI(new URL(import.meta.url).pathname))
);

function newRandom(hash) {
  let alphabet = "0123456789abcdef";
  var mainHash =
    hash ||
    Array(64)
      .fill(0)
      .map((_) => alphabet[(Math.random() * alphabet.length) | 0])
      .join("");
  let regex = new RegExp(".{" + ((mainHash.length / 4) | 0) + "}", "g");
  let hashes = mainHash.match(regex).map((h) => parseInt("0x" + h));
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
  var rand = sfc32(...hashes);

  return {
    random: () => {
      // hack a bit the provided fn which don't have enough entropy to me
      if (rand() < 0.5) return rand();
      if (rand() > 0.5) return rand();
      return rand();
    },
    hash: mainHash,
  };
}

const argv2 = process.argv[2];
const argv3 = process.argv[3];
if (!isNaN(argv2)) {
  const count = parseInt(argv2, 10);
  const all = [];
  const keys = {};

  const arg = argv3 || "true";
  const wasmBuffer = fs.readFileSync(
    path.join(__dirname, "./rust/pkg/main_bg.wasm")
  );
  const wasmLoad = import("./rust/pkg/main.mjs");

  wasmLoad.then(async (wasmModule) => {
    await wasmModule.default(wasmBuffer);
    let pred = arg;
    let oos = [];
    if (arg.startsWith("oo")) {
      pred = "true";
      oos = [arg];
    }
    const predicate = new Function("p,o", "return (" + pred + ")");
    let r, hash;
    for (let i = 0; oos.length ? i < oos.length : i < count; i++) {
      const nr = newRandom(oos[i]);
      hash = nr.hash;
      r = generateVariables(nr.random, hash);
      if (predicate(r.props, r.opts)) {
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
          )
          .replace(
            /#FF0/g,
            "rgb(" +
            r.secondary.main.map((n) => Math.round(n * 255)).join(",") +
            ")"
          );
        const props = generateVariables.inferProps(r, svg);
        for (let k in props) {
          keys[k] = 1;
        }
        all.push({ hash, props });

        console.log(
          hash,
          (svg.length / (1024 * 1024)).toFixed(2) + " Mb",
          props
        );
        fs.writeFileSync("results/" + hash + ".svg", svg, "utf-8");
      }
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
  });
}
