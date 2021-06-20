import React, { useState,  useEffect } from "react";
import ReactDOM from "react-dom";
import { Surface } from "gl-react-dom";
import useDimensions from "react-cool-dimensions";
import Main from "./Main";

export function useTime() {
  const [time, setTime] = useState(0);
  useEffect(() => {
    let startT;
    let h;
    function loop(t) {
      h = requestAnimationFrame(loop);
      if (!startT) startT = t;
      setTime((t - startT) / 1000);
    }
    h = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(h);
  }, []);
  return time;
}

let startT = Date.now();
const Root = () => {
  const time = useTime();
  const [n, setN] = useState(() => Date.now()-startT);
  const { observe, width, height } = useDimensions({});
  function onClick() {
    setN(() => Date.now()-startT);
  }
  return (
    <div
      onClick={onClick}
      ref={observe}
      style={{ cursor: "pointer", width: "100vw", height: "100vh" }}
    >
      <Surface width={width} height={height}>
        <Main n={n} time={time} />
      </Surface>
    </div>
  );
};

ReactDOM.render(<Root />, document.getElementById("main"));
