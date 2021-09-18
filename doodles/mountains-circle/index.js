import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import useDimensions from "react-cool-dimensions";
import Main from "./Main";

let viewer = new URLSearchParams(window.location.search).get("viewer");
if (viewer && !viewer.startsWith("tz")) {
  viewer = null;
}

if (!viewer) {
  document.getElementById("noauth").style.display = "block";
  document.getElementById("close").style.display = "none";
}

const Root = () => {
  const { observe, width, height } = useDimensions({});
  const [helpOn, setHelpOn] = useState(!viewer);
  window.setHelpOn = setHelpOn;
  useEffect(() => {
    document.getElementById("help").style.display = helpOn ? "block" : "none";
  }, [helpOn]);

  if (!viewer) return null;
  return (
    <div ref={observe} style={{ width: "100vw", height: "100vh" }}>
      <Main
        width={width}
        height={height}
        helpOn={helpOn}
        setHelpOn={setHelpOn}
        viewer={viewer}
      />
    </div>
  );
};

ReactDOM.render(<Root />, document.getElementById("main"));
