// @flow
import React, { useEffect, useMemo, useRef } from "react";
import MersenneTwister from "mersenne-twister";
import CirclePacker from "circlepacker";

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

const DEBUG_CANVAS = false;

const COLORS = [
  {
    css: "midnightblue",
    name: "Bloody Brexit",
  },
  {
    css: "deepskyblue",
    name: "Turquoise",
  },
  {
    css: "firebrick",
    name: "Red Dragon",
  },
];

/*
const burnColorIndex = 0;
const ethColorIndex = 1;
const erc20ColorIndex = 2;
*/

const pickColor = (f) =>
  COLORS[Math.floor(f * (COLORS.length - 1)) % COLORS.length];

const CustomStyle = ({
  block,
  canvasRef,
  attributesRef,
  width,
  height,
  mod1,
  mod2,
  mod3,
  mod4,
  mod5,
}) => {
  const ethColor = pickColor(mod3);
  const erc20Color = pickColor(mod4);
  const burnColor = pickColor(mod5);
  const palette = [ethColor.css, erc20Color, burnColor.css];

  const svgRef = useRef();
  const variables = useVariables({ block, mod1, mod2, mod3, mod4 });
  useSyncSvgInCanvas(svgRef, canvasRef, width, height, [variables]);
  useAttributes(attributesRef, variables);

  const strokeWidth = 0.35;

  return (
    <svg
      ref={svgRef}
      width={width}
      height={height}
      style={{ background: "white" }}
      viewBox="0 0 200 200"
    >
      <g>
        {variables.items.map((item) => (
          <a
            key={item.key}
            href={item.link}
            style={{ cursor: "pointer" }}
            target="_blank"
            rel="noreferrer"
          >
            <circle
              cx={200 * item.x}
              cy={200 * item.y}
              r={200 * item.r}
              fill="#ffffff01"
              stroke={palette[item.color]}
              strokeWidth={strokeWidth}
              style={{ mixBlendMode: "multiply" }}
            />
          </a>
        ))}
      </g>
    </svg>
  );
};

let burnBegin = "000000000000000000000000000000000000";
function isBurn(to) {
  return to && to.slice(2, 2 + burnBegin.length) === burnBegin;
}
function getTokenByContract(contract) {
  return dataByContractLowerCase[contract.toLowerCase()];
}
function erc20value(value, token) {
  return value * (token?.priceMultiplier || 0);
}

function format(symbol, value, magnitude) {
  if (!value) return "";
  return (
    (symbol || "").toUpperCase() +
    " " +
    (value / Math.pow(10, magnitude)).toLocaleString("en-US", {
      maximumFractionDigits: magnitude,
    })
  );
}

function useVariables({ block, mod1, mod2 }) {
  // precalculate many things from txs
  const data = useMemo(() => {
    // txs with their value
    const data = block.transactions.map((tx) => {
      const value = safeParseInt(tx.value);
      let burn = isBurn(tx.to);
      let erc20transfer = false;
      const legends = [];
      let usd = 0;
      if (value > 0) {
        legends.push(format("ETH", value, 18));
        usd = value * ethPrice;
      }
      let input = (tx.input || "").slice(2);
      let dataLength = 0;
      if (input) {
        // erc20 transfer
        if (input.startsWith("a9059cbb")) {
          const to = input.slice(32, 72);
          const token = getTokenByContract(tx.to);
          if (token) {
            const value = parseInt(input.slice(72, 136), 16);
            legends.push(format(token.symbol, value, token.decimals));
            usd += erc20value(value, tx.to);
          }
          erc20transfer = true;
          burn = isBurn(to);
        } else if (input.startsWith("42966c68")) {
          // e.g. https://etherscan.io/tx/0xbf97244cf0c5709e0dd36e37bacb994f95c245aaf20a279595b5371ff592f4b0
          burn = true;
          const token = getTokenByContract(tx.to);
          if (token) {
            const value = parseInt(input.slice(8, 40), 16);
            legends.push("BURN " + format(token.symbol, value, token.decimals));
            usd += erc20value(value, tx.to);
          }
        } else {
          let l = input.length / 2;
          if (l > 8) {
            dataLength = l;
            legends.push(dataLength + " bytes");
          }
          // TODO data points
        }
      }
      const gasPaidUsd =
        safeParseInt(tx.gas) * safeParseInt(tx.gasPrice) * ethPrice;

      const legend = legends.join(", ");

      return {
        tx,
        usd,
        gasPaidUsd,
        erc20transfer,
        burn,
        dataLength,
        legend,
      };
    });
    return data;
  }, [block]);

  // then, algos that also needs the mods
  return useMemo(() => {
    const totalUsd = data.reduce((acc, tx) => acc + tx.usd + tx.gasPaidUsd, 0);

    // data txs
    const dataTxs = data.filter((o) => o.dataLength);
    const totalDataBytes = dataTxs.reduce((acc, o) => acc + o.dataLength, 0);

    // burned txs
    const burnedTxs = data.filter((o) => o.burn);

    // burned fees
    const burnedEth =
      safeParseInt(block.baseFeePerGas) * safeParseInt(block.gasUsed);
    const burnedFees = burnedEth * ethPrice;

    const totalBurn =
      burnedFees + burnedTxs.reduce((acc, tx) => acc + tx.usd, 0);

    const dataUsdRatio = totalUsd ? totalDataBytes / totalUsd : 1;

    // all txs sorted with their weights
    const weighted = data
      .map((item) => {
        const weight = mix(
          (item.usd + item.gasPaidUsd) * dataUsdRatio,
          item.dataLength,
          mod1
        );
        return {
          ...item,
          weight,
        };
      })
      .sort((a, b) => b.weight - a.weight);
    const totalWeight = weighted.reduce((acc, w) => w.weight + acc, 0);

    // packing algo

    const rng = new MersenneTwister(parseInt(block.hash.slice(0, 16), 16));
    const items = weighted.map(
      ({ tx, weight, burn, erc20transfer, legend }) => ({
        x: 0.1 + 0.8 * rng.random(),
        y: 0.1 + 0.8 * rng.random(),
        r: 0.1 * mod2 + (weight / totalWeight) * rng.random(),
        color: burn ? 2 : erc20transfer ? 1 : 0,
        legend,
        link: `https://etherscan.io/tx/${tx.hash}`,
        key: tx.hash,
        group: burn ? "burn" : tx.to || "",
      })
    );
    // TODO when there are a lot of tx.to similar that are not erc20, we should add a 4rd color case or something special?. (with threshold)

    const groupsObj = {};
    const singletons = [];
    for (const item of items) {
      if (!item.group) {
        singletons.push(item);
        continue;
      }
      if (!groupsObj[item.group]) {
        groupsObj[item.group] = [];
      }
      groupsObj[item.group].push(item);
    }
    const groups = Object.values(groupsObj);
    for (const group of groups) {
      let x = 0;
      let y = 0;
      for (const item of group) {
        item.x = x;
        item.y = y;
      }
      const circles = group.map((item) => ({
        id: item.key,
        radius: item.r,
        position: { x: 0.5, y: 0.5 },
      }));
      const packer = new CirclePacker({
        target: { x: 0.5, y: 0.5 },
        bounds: { width: 1, height: 1 },
        circles,
        continuousMode: false,
        collisionPasses: 3,
        centeringPasses: 2,
      });
      packer.update();
      group.forEach((o, i) => {
        Object.assign(o, circles[i].position);
      });
    }

    const result = {
      totalUsd,
      totalBurn,
      burnedEth,
      totalDataBytes,
      items,
      block,
    };

    console.log(
      result,
      "https://etherscan.io/block/" + safeParseInt(block.number)
    );
    return result;
  }, [data, block, mod1, mod2]);
}

function useAttributes(attributesRef, variables) {
  useEffect(() => {
    attributesRef.current = () => {
      return {
        attributes: [
          {
            display_type: "number",
            trait_type: "Block Estimated USD",
            value: Math.round(variables.totalUsd),
          },
          {
            display_type: "number",
            trait_type: "Contract Bytes",
            value: Math.round(variables.totalDataBytes),
          },
          {
            trait_type: "Block Burned",
            value: format("ETH", variables.burnedEth, 18).slice(0, 10),
          },
        ],
      };
    };
  }, [variables]);
}

function useSyncSvgInCanvas(svgRef, canvasRef, width, height, deps) {
  // capturing logic here. for now we copy into a canvas.
  // TBD if this can instead be done by snapshotter for better perf (no need to sync canvas)
  useEffect(() => {
    const svg = svgRef.current;
    if (!svg) return;
    const canvas = document.createElement("canvas");
    if (DEBUG_CANVAS) document.body.appendChild(canvas);
    canvas.width = width;
    canvas.height = height;
    canvasRef.current = canvas;
    var xml = new XMLSerializer().serializeToString(svg);
    const img = new Image();
    img.onload = () => {
      const ctx = canvas.getContext("2d");
      ctx.clearRect(0, 0, width, height);
      ctx.drawImage(img, 0, 0, width, height);
    };
    img.width = width;
    img.height = height;
    img.src = "data:image/svg+xml;base64," + btoa(xml);
  }, [...deps, width, height]);
}

export default CustomStyle;

const safeParseInt = (a) => {
  const v = parseInt(a);
  if (isNaN(v) || !isFinite(a)) return 0;
  return v;
};

function mix(a, b, x) {
  return a * (1 - x) + b * x;
}
