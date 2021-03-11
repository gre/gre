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
    return () => cancelAnimationFrame(loop);
  }, []);
  return <Day.Shader time={time} {...rest} />;
};

export function Visual({ Day }) {
  const [n, setN] = useState(0);
  return (
    <>
      <Surface width={400} height={400}>
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
