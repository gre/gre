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

function setFeatures(features) {
  window.$fxhashFeatures = features;
}

function Root() {
  const { width, height, observe } = useDimensions({});

  return (
    <div ref={observe} style={viewportStyle}>
      <Main
        width={width || window.innerWidth}
        height={height || window.innerHeight}
        hash={window.fxhash}
        setFeatures={setFeatures}
      />
    </div>
  );
}

ReactDOM.render(<Root />, document.getElementById("main"));
