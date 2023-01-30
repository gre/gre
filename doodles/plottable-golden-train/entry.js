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

function Root() {
  const { width, height, observe } = useDimensions({});
  const prng = useMemo(() => createPrng(), []);
  return (
    <div ref={observe} style={viewportStyle}>
      <Main
        width={width || window.innerWidth}
        height={height || window.innerHeight}
        random={(hi) => prng.random(0, hi || 1)}
      />
    </div>
  );
}

ReactDOM.render(<Root />, document.getElementById("main"));
