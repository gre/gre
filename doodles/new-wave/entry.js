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

function Root() {
  const { width, height, observe } = useDimensions({});
  return (
    <div ref={observe} style={viewportStyle}>
      <Main
        width={width || window.innerWidth}
        height={height || window.innerHeight}
        random={window.fxrand}
      />
    </div>
  );
}

ReactDOM.render(<Root />, document.getElementById("main"));
