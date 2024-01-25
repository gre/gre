import React, { useMemo } from "react";
import useDimensions from "react-cool-dimensions";
import ReactDOM from "react-dom";
import Main from "./index";

const viewportStyle = {
  position: "absolute",
  width: "100vw",
  height: "100vh",
  display: "flex",
  flexDirection: "column",
};

const generateHash = () => {
  let alphabet = "0123456789ABCDEF";
  return Array(64)
    .fill(0)
    .map((_) => alphabet[(Math.random() * alphabet.length) | 0])
    .join("");
};

class PRNGRand {
  constructor(hash) {
    hash = hash.toUpperCase();
    const regex = new RegExp("[0-9A-F]{64}");
    if (!regex.test(hash)) {
      console.error("Bad hash", hash);
      throw Error("Bad hash: " + hash);
    }
    this.useA = false;
    let sfc32 = function (uint128Hex) {
      let a = parseInt(uint128Hex.substring(0, 8), 16);
      let b = parseInt(uint128Hex.substring(8, 8), 16);
      let c = parseInt(uint128Hex.substring(16, 8), 16);
      let d = parseInt(uint128Hex.substring(24, 8), 16);
      return function () {
        a |= 0;
        b |= 0;
        c |= 0;
        d |= 0;
        let t = (((a + b) | 0) + d) | 0;
        d = (d + 1) | 0;
        a = b ^ (b >>> 9);
        b = (c + (c << 3)) | 0;
        c = (c << 21) | (c >>> 11);
        c = (c + t) | 0;
        return (t >>> 0) / 4294967296;
      };
    };
    // seed prngA with first half hash
    this.prngA = new sfc32(hash.substring(2, 32));
    // seed prngB with second half of hash
    this.prngB = new sfc32(hash.substring(34, 32));
    for (let i = 0; i < 1e6; i += 2) {
      this.prngA();
      this.prngB();
    }
    this.random = () => {
      this.useA = !this.useA;
      return this.useA ? this.prngA() : this.prngB();
    };
  }
}

const urlHash = new URLSearchParams(window.location.search).get("hash");

const hash = urlHash || generateHash()

if (!urlHash) {
  window.location.search = `?hash=${hash}&debug=true`;
}

const debug = new URLSearchParams(window.location.search).get("debug");

if (debug) {
  // on click
  window.addEventListener("click", (e) => {
    window.location.search = `?hash=${generateHash()}&debug=true`;
  });
}

function Root() {
  const { width, height, observe } = useDimensions({});


  const setProperties = useMemo(() => (attributes, traits = {}) => {
    console.log(hash);
    console.table(attributes);
    setWindowProperties("attributes", attributes);
    setWindowProperties("traits", traits);
  }, [hash]);


  const random = useMemo(() => {
    const rng = new PRNGRand(hash);
    return () => rng.random();
  }, [hash]);

  return (
    <div ref={observe} style={viewportStyle}>
      <Main
        width={width || window.innerWidth}
        height={height || window.innerHeight}
        hash={hash}
        debug={debug}
        random={random}
        setProperties={setProperties}
      />
    </div>
  );
}

ReactDOM.render(<Root />, document.getElementById("main"));
