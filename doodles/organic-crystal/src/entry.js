import React from "react";
import { createRoot } from "react-dom/client";
import Main from "./index";

const viewportStyle = {
  position: "absolute",
  width: "100vw",
  height: "100vh",
  display: "flex",
  flexDirection: "column",
};

function Root() {
  return (
    <div style={viewportStyle}>
      <Main random={window.fxrand} />
    </div>
  );
}

const container = document.getElementById("main");
const root = createRoot(container);
root.render(<Root />);
