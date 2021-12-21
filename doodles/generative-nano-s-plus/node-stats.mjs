import { generate } from "./features.mjs";

const all = [];
const keys = {};
for (let i = 0; i < 2048; i++) {
  const { metadata } = generate(i);
  const features = {};
  metadata.attributes.forEach(({ trait_type, value }) => {
    if (trait_type === "Word") return;
    features[trait_type] = value;
  });
  for (let k in features) {
    keys[k] = 1;
  }
  all.push({ i, features });
}

Object.keys(keys)
  .sort()
  .forEach((k) => {
    const counters = {};
    for (let i = 0; i < all.length; i++) {
      const v = all[i];
      counters[v.features[k]] = (counters[v.features[k]] || []).concat(i);
    }
    console.log(
      k +
        "\n" +
        Object.entries(counters)
          .sort((a, b) => b[1].length - a[1].length)
          .map(
            ([k, v]) =>
              k.padStart(30) +
              ": " +
              v.length +
              " = " +
              ((100 * v.length) / all.length).toFixed(1) +
              "%\n" +
              v.join(" ")
          )
          .join("\n")
    );
    // fs.writeFileSync("study.json", JSON.stringify(all), "utf-8");
  });
