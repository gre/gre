import React, { useMemo } from "react";
import useDimensions from "react-cool-dimensions";
import ReactDOM from "react-dom";
import Main from "./index";
import { params } from "./constants";

const viewportStyle = {
  position: "absolute",
  width: "100vw",
  height: "100vh",
  display: "flex",
  flexDirection: "column",
};

function setFeatures(features) {
  $fx.features(features);
}

$fx.params(params);

function Root() {
  const { width, height, observe } = useDimensions({});
  const params = useMemo(() => $fx.getRawParams(), []);

  return (
    <div ref={observe} style={viewportStyle}>
      <Main
        width={width || window.innerWidth}
        height={height || window.innerHeight}
        hash={window.fxhash}
        setFeatures={setFeatures}
        params={params}
      />
    </div>
  );
}

ReactDOM.render(<Root />, document.getElementById("main"));
