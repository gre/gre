let calculateFeatures = require("./dist/features.node");
function newRandom() {
  function random_hash() {
    let chars = "0123456789abcdef";
    let result = "0x";
    for (let i = 64; i > 0; --i)
      result += chars[Math.floor(Math.random() * chars.length)];
    return result;
  }
  let tokenData = { tokenId: 1000000, hash: random_hash() };
  return tokenData;
}

const argv2 = process.argv[2];
if (!isNaN(argv2)) {
  const count = parseInt(argv2, 10);
  const all = [];
  const keys = {};
  for (let i = 0; i < count; i++) {
    const tokenData = newRandom();
    const features = calculateFeatures(tokenData);
    for (let k in features) {
      keys[k] = 1;
    }
    all.push({ tokenData, features });
  }

  Object.keys(keys)
    .sort()
    .forEach((k) => {
      const counters = {};
      for (let i = 0; i < all.length; i++) {
        const v = all[i];
        counters[v.features[k]] = (counters[v.features[k]] || 0) + 1;
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
}
