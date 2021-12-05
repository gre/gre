import React, { useEffect, useRef, useState } from "react";
import regl from "regl";
import art from "./art";

function Main() {
  const bigCanvasRef = useRef();
  const [seed, setSeed] = useState(0);
  const [md, setMetadata] = useState({});
  useEffect(() => {
    if (typeof window !== "undefined" && bigCanvasRef.current) {
      const { destroy, metadata } = art(
        //regl(bigCanvasRef.current),
        regl(),
        seed,
        () => {}
      );
      setMetadata(metadata);
      return destroy;
    }
  }, [seed]);
  return (
    <div
      style={{
        position: "fixed",
        zIndex: 99,
        color: "white",
        width: "100%",
        padding: 10,
        boxSizing: "border-box",
        wordWrap: "break-word",
      }}
    >
      <input
        type="number"
        value={seed}
        onChange={(e) => setSeed(parseInt(e.target.value))}
      />
      <code>{JSON.stringify(md)}</code>
      <canvas width={4096} height={4096} ref={bigCanvasRef}></canvas>
    </div>
  );
}

export default Main;
