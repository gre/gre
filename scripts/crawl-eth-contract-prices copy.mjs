import fetch from "node-fetch";
import fs from "fs";
import _ from "lodash";

const get = async (url) => {
  let full = "https://api.coingecko.com/api/v3" + url;
  console.log("=> " + full);
  const r = await fetch(full);
  const json = await r.json();
  return json;
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
  const list = await get("/coins/list?include_platform=true");
  const tokens = list.filter((c) => Boolean(c.platforms?.ethereum));
  const all = [];

  for (const items of _.chunk(tokens, 100)) {
    const prices = await retry(() =>
      get(
        "/simple/price?vs_currencies=usd&ids=" +
          items.map((o) => o.id).join(",")
      )
    );
    for (const { id, symbol, platforms } of items) {
      const usd = prices[id].usd || 0;
      const contract = platforms?.ethereum;
      if (!contract) continue;
      const item = {
        id,
        symbol,
        contract,
        usd,
      };
      all.push(item);
    }
    await delay(1000);
  }

  fs.writeFileSync("coins.json", JSON.stringify(all), "utf-8");
}

main();
