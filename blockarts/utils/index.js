// @flow
import { useEffect, useMemo, useState } from "react";

// snapshot on August 18, 2021
import prices from "./minimal.json";

const ethPrice = 3050 / 10e17;
// cat coins-with-decimals.json | jq '[.[] | [.contract, .symbol, .usd, .decimals]]' > minimal.json

const dataByContractLowerCase = {};
prices.forEach(([contract, symbol, usd, decimals]) => {
  dataByContractLowerCase[contract.toLowerCase()] = {
    priceMultiplier: (usd || 0) / Math.pow(10, decimals || 0),
    symbol,
    decimals,
    contract,
  };
});

let burnBegin = "000000000000000000000000000000000000";
function isBurn(to) {
  return to && to.slice(2, 2 + burnBegin.length) === burnBegin;
}
function getTokenByContract(contract) {
  return dataByContractLowerCase[(contract || "").toLowerCase()];
}
function erc20value(value, token) {
  return value * (token?.priceMultiplier || 0);
}

export function formatCurrency(symbol, value, magnitude) {
  if (!value) return "";
  return (
    (symbol || "").toUpperCase() +
    " " +
    (value / Math.pow(10, magnitude)).toLocaleString("en-US", {
      maximumFractionDigits: magnitude,
    })
  );
}

export function useStats({ block, withLegend = false }) {
  return useMemo(() => {
    // txs with their value
    const transactions = block.transactions.map((tx) => {
      const value = safeParseInt(tx.value);
      let burn = isBurn(tx.to);
      let erc20transfer = false;
      const legends = [];
      let usd = 0;
      let eth = 0;
      if (value > 0) {
        if (withLegend) {
          legends.push(formatCurrency("ETH", value, 18));
        }
        usd = value * ethPrice;
        eth += value;
      }
      let input = (tx.input || tx.data || "").slice(2);
      let dataLength = 0;
      if (input) {
        // erc20 transfer
        if (input.startsWith("a9059cbb")) {
          const to = input.slice(32, 72);
          const token = getTokenByContract(tx.to);
          if (token) {
            const value = parseInt(input.slice(72, 136), 16);
            if (withLegend) {
              legends.push(formatCurrency(token.symbol, value, token.decimals));
            }
            usd += erc20value(value, token);
          }
          erc20transfer = true;
          burn = isBurn(to);
        } else if (input.startsWith("42966c68") && tx.to) {
          // e.g. https://etherscan.io/tx/0xbf97244cf0c5709e0dd36e37bacb994f95c245aaf20a279595b5371ff592f4b0
          burn = true;
          const token = getTokenByContract(tx.to);
          if (token) {
            const value = parseInt(input.slice(8, 40), 16);
            if (withLegend) {
              legends.push(
                "BURN " + formatCurrency(token.symbol, value, token.decimals)
              );
            }
            usd += erc20value(value, token);
          }
        } else {
          let l = input.length / 2;
          if (l > 8) {
            dataLength = l;
            if (withLegend) {
              legends.push(dataLength + " bytes");
            }
          }
          // TODO data points
        }
      }
      const gasPaid = safeParseInt(tx.gas) * safeParseInt(tx.gasPrice);
      const gasPaidUsd = gasPaid * ethPrice;

      const legend = legends.join(", ");

      return {
        tx,
        eth,
        usd,
        gasPaid,
        gasPaidUsd,
        erc20transfer,
        burn,
        dataLength,
        legend,
      };
    });
    const totalUsd = transactions.reduce(
      (acc, tx) => acc + tx.usd + tx.gasPaidUsd,
      0
    );
    const totalEth = transactions.reduce(
      (acc, tx) => acc + tx.eth + tx.gasPaid,
      0
    );
    const totalEthUsd = totalEth * ethPrice;

    // data txs
    const dataTxs = transactions.filter((o) => o.dataLength);
    const totalDataBytes = dataTxs.reduce((acc, o) => acc + o.dataLength, 0);

    // burned txs
    const burnedTxs = transactions.filter((o) => o.burn);

    // burned fees
    const burnedEth =
      safeParseInt(block.baseFeePerGas) * safeParseInt(block.gasUsed);
    const burnedFees = burnedEth * ethPrice;

    const totalBurn =
      burnedFees + burnedTxs.reduce((acc, tx) => acc + tx.usd, 0);

    return {
      transactions,
      dataTxs,
      totalUsd,
      totalEth,
      totalEthUsd,
      totalDataBytes,
      burnedTxs,
      burnedEth,
      burnedFees,
      totalBurn,
    };
  }, [block]);
}

export function useTime() {
  const [time, setTime] = useState(0);
  useEffect(() => {
    let startT;
    let h;
    function loop(t) {
      h = requestAnimationFrame(loop);
      if (!startT) startT = t;
      setTime((t - startT) / 1000);
    }
    h = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(h);
  }, []);
  return time;
}

export const safeParseInt = (a) => {
  if (a && typeof a === "object" && a.hex) {
    a = a.hex;
  }
  const v = parseInt(a);
  if (isNaN(v) || !isFinite(a)) return 0;
  return v;
};

export function mix(a, b, x) {
  return a * (1 - x) + b * x;
}

export function smoothstep(min, max, value) {
  var x = Math.max(0, Math.min(1, (value - min) / (max - min)));
  return x * x * (3 - 2 * x);
}
