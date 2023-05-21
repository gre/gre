function random_hash() {
  let chars = "0123456789abcdef";
  let result = '0x';
  for (let i = 64; i > 0; --i) result += chars[Math.floor(Math.random() * chars.length)];
  return result;
}

const possibleFeatures = {}

for (let i = 0; i < 100000; i++) {
  let tokenData = { "tokenId": 1000000, "hash": random_hash() }
  let features = calculateFeatures(tokenData);
  for (let key in features) {
    possibleFeatures[key] = possibleFeatures[key] || {}
    possibleFeatures[key][features[key]] = 1
  }
}

const obj = Object.keys(possibleFeatures).map(key => ({
  type: "enum",
  name: key,
  options: Object.keys(possibleFeatures[key]).sort()
}))

require("fs").writeFileSync("dist/features.json", JSON.stringify(obj, null, 2))