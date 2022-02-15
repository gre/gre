import React from "react";
import { useState, useEffect } from "react";
import { Surface } from "gl-react-dom";

const Refresh = ({ Day, ...rest }) => {
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
  return <Day.Shader time={time} {...rest} />;
};

export function Visual({ Day, width, height, style }) {
  const [n, setN] = useState(0);
  if (Day.Render) return <Day.Render width={width} height={height} />;
  return !Day.Shader ? null : (
    <>
      <Surface width={width || 400} height={height || 400} style={style}>
        <Refresh key={Day.n} Day={Day} n={n} />
      </Surface>
      {null && (
        <input
          style={{ margin: 10 }}
          value={n}
          range={1}
          onChange={(e) => setN(parseInt(e.target.value, 10))}
          type="number"
        ></input>
      )}
    </>
  );
}
