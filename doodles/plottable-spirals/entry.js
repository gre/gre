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

const urlHash = new URLSearchParams(window.location.search).get("fxhash");

if (!urlHash) {
  window.location.search = `?fxhash=${$fx.hash}&debug=true`;
}

const debug = new URLSearchParams(window.location.search).get("debug");

if (debug) {
  // on click
  window.addEventListener("click", (e) => {
    let alphabet =
      "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
    window.location.search = `?fxhash=${"oo" +
      Array(49)
        .fill(0)
        .map((_) => alphabet[(Math.random() * alphabet.length) | 0])
        .join("")}&debug=` + debug;
  });

  let n = parseInt(debug, 10);
  if (!isNaN(n) && n > 0) {
    setTimeout(() => {
      let alphabet =
        "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
      window.location.search = `?fxhash=${"oo" +
        Array(49)
          .fill(0)
          .map((_) => alphabet[(Math.random() * alphabet.length) | 0])
          .join("")}&debug=` + debug;
    }, n)
  }
}

const setProperties = (attributes) => {
  $fx.features(attributes);
  console.table(attributes);
};

function Root() {
  const { width, height, observe } = useDimensions({});

  return (
    <div ref={observe} style={viewportStyle}>
      <Main
        width={width || window.innerWidth}
        height={height || window.innerHeight}
        hash={$fx.hash}
        debug={debug}
        random={$fx.rand}
        setProperties={setProperties}
      />
    </div>
  );
}

ReactDOM.render(<Root />, document.getElementById("main"));
