import fetch from "node-fetch";
import fs from "fs";

const getDecimals = async (contract) => {
  let full = "https://etherscan.io/token/" + contract;
  const r = await fetch(full);
  const text = await r.text();
  const i = text.indexOf("Decimals:</div>");
  const q = text.slice(i + 14, i + 100).match(/>\s*(\d+)\s*<\/div>/);
  if (q) {
    return parseInt(q[1], 10);
  }
  return -1;
};

const delay = (ms) => new Promise((r) => setTimeout(r, ms));

function retry(f, options) {
  const { maxRetry, interval, intervalMultiplicator, context } = {
    maxRetry: 4,
    interval: 300,
    intervalMultiplicator: 1.5,
    context: "",
    ...options,
  };

  function rec(remainingTry, i) {
    const result = f();

    if (remainingTry <= 0) {
      return result;
    }

    // In case of failure, wait the interval, retry the action
    return result.catch((e) => {
      console.log(
        "promise-retry",
        context + " failed. " + remainingTry + " retry remain. " + String(e)
      );
      return delay(i).then(() =>
        rec(remainingTry - 1, i * intervalMultiplicator)
      );
    });
  }

  return rec(maxRetry, interval);
}

async function main() {
  const list = JSON.parse(await fs.readFileSync("coins.json"));
  const all = [];
  for (const item of list) {
    const decimals = await retry(() => getDecimals(item.contract));
    if (decimals !== -1) {
      console.log(item.id, decimals);
      all.push({ ...item, decimals });
    }
    await delay(1000);
  }

  fs.writeFileSync("coins-with-decimals.json", JSON.stringify(all), "utf-8");
}

main();
