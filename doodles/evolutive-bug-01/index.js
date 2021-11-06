import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import useDimensions from "react-cool-dimensions";
import Main from "./Main";
import MersenneTwister from "mersenne-twister";

const params = new URLSearchParams(window.location.search);
let viewer = params.get("viewer");
let highQuality = params.get("highQuality");
let objkt = parseInt(params.get("objkt"), 10);
if (isNaN(objkt)) {
  objkt = null;
}

if (viewer && !viewer.startsWith("tz")) {
  viewer = null;
}

async function fetchHist() {
  if (!objkt) return [];
  let lastId = null;
  let list = [];
  do {
    const { transfers, last_id } = await fetch(
      `https://api.better-call.dev/v1/tokens/mainnet/transfers/KT1HbQepzV1nVGg8QVznG7z4RcHseD5kwqBn?sort=asc&token_id=${objkt}${
        lastId ? "&last_id=" + lastId : ""
      }`
    ).then((r) => r.json());
    if (transfers.length === 0) break;
    list = list.concat(transfers);
    lastId = last_id;
    await new Promise((s) => setTimeout(s, 100));
  } while (lastId);
  return list;
}

function hashStr(str, hash = 0) {
  if (str.length === 0) return hash;
  for (let i = 0; i < str.length; i++) {
    let chr = str.charCodeAt(i);
    hash = (hash << 5) - hash + chr;
    hash |= 0;
  }
  return hash;
}

function inferPicks(list) {
  const picks = [];
  let hash = 1946420313;
  let owner = "tz1cgQAQfECg5bPASYTMyJ9QJQjSUi8rfL67";
  let v1 = 0;
  let v2 = 0;
  let v3 = 0;
  function accumulate(hash) {
    const rng = new MersenneTwister(hash);
    v1 += rng.random() - 0.4;
    v2 += rng.random() - 0.4;
    v3 += rng.random() - 0.4;
  }
  picks.push({
    v1,
    v2,
    v3,
    buyer: owner,
    timestamp: "",
    optimistic: false,
    genesis: true,
  });
  list.forEach((t) => {
    if (t.status !== "applied") return;
    if (t.parent !== "collect") return;
    owner = t.to;
    // TODO only first buy
    if (t.from !== "KT1HbQepzV1nVGg8QVznG7z4RcHseD5kwqBn") return;
    console.log(t.to);
    let timestamp = t.timestamp.slice(0, 13);
    hash = hashStr(t.to, hash);
    hash = hashStr(timestamp, hash);
    accumulate(hash);
    picks.push({
      v1,
      v2,
      v3,
      buyer: t.to,
      timestamp,
      optimistic: false,
    });
  });
  if (viewer) {
    const currentTimestamp = new Date().toISOString().slice(0, 13);
    hash = hashStr(viewer, hash);
    hash = hashStr(currentTimestamp, hash);
    accumulate(hash);
    picks.push({
      v1,
      v2,
      v3,
      buyer: viewer,
      timestamp: currentTimestamp,
      optimistic: true,
    });
  }
  console.log(picks);
  return { picks, owner };
}

function useHisto() {
  const [state, setHistoryState] = useState(() => ({
    loaded: false,
    data: inferPicks([]),
    error: null,
  }));
  const [refresh, setRefresh] = useState(0);

  useEffect(() => {
    const t = setTimeout(() => setRefresh((t) => t + 1), 60 * 1000);
    fetchHist().then(
      (h) => {
        setHistoryState({
          error: null,
          loaded: true,
          data: inferPicks(h),
        });
      },
      (error) => {
        setHistoryState((s) => ({
          ...s,
          loaded: true,
          error,
        }));
      }
    );
    return () => clearTimeout(t);
  }, [refresh]);
  return state;
}

const Root = () => {
  const { observe, width, height } = useDimensions({});
  const [helpOn, setHelpOn] = useState(true);
  const histo = useHisto();
  window.setHelpOn = setHelpOn;
  useEffect(() => {
    document.getElementById("help").style.display = helpOn ? "block" : "none";
  }, [helpOn]);
  if (!objkt) return "Something is wrong (no objkt in params)";
  return (
    <div ref={observe} style={{ width: "100vw", height: "100vh" }}>
      <Main
        width={width}
        height={height}
        helpOn={helpOn}
        setHelpOn={setHelpOn}
        viewer={viewer}
        histo={histo}
        highQuality={!!highQuality}
      />
    </div>
  );
};

ReactDOM.render(<Root />, document.getElementById("main"));
