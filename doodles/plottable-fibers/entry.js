import React from "react";
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

const fxrand = window.fxrand;
function random() {
  // hack a bit the provided fn which don't have enough entropy to me
  if (fxrand() < 0.5) return fxrand();
  if (fxrand() > 0.5) return fxrand();
  return fxrand();
}

function Root() {
  const { width, height, observe } = useDimensions({});
  return (
    <div ref={observe} style={viewportStyle}>
      <Main
        width={width || window.innerWidth}
        height={height || window.innerHeight}
        random={random}
      />
    </div>
  );
}

ReactDOM.render(<Root />, document.getElementById("main"));
