import React, { useEffect, useRef, useState } from "react";
import regl from "regl";
import art from "./art";

let alphabet = "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
var fxhash =
  "oo" +
  Array(49)
    .fill(0)
    .map((_) => alphabet[(Math.random() * alphabet.length) | 0])
    .join("");
let b58dec = (str) =>
  str
    .split("")
    .reduce(
      (p, c, i) =>
        p + alphabet.indexOf(c) * Math.pow(alphabet.length, str.length - i - 1),
      0
    );
let fxhashTrunc = fxhash.slice(2);
let regex = new RegExp(".{" + ((fxhashTrunc.length / 4) | 0) + "}", "g");
let hashes = fxhashTrunc.match(regex).map((h) => b58dec(h));
let sfc32 = (a, b, c, d) => {
  return () => {
    a |= 0;
    b |= 0;
    c |= 0;
    d |= 0;
    var t = (((a + b) | 0) + d) | 0;
    d = (d + 1) | 0;
    a = b ^ (b >>> 9);
    b = (c + (c << 3)) | 0;
    c = (c << 21) | (c >>> 11);
    c = (c + t) | 0;
    return (t >>> 0) / 4294967296;
  };
};
global.fxrand = sfc32(...hashes);

function Main() {
  const bigCanvasRef = useRef();
  const [seed, setSeed] = useState(0);
  const [md, setMetadata] = useState({});
  useEffect(() => {
    if (typeof window !== "undefined" && bigCanvasRef.current) {
      const { destroy, metadata } = art(
        //regl(bigCanvasRef.current),
        regl({
          extensions: ["ANGLE_instanced_arrays"],
        }),
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
      <canvas width={1000} height={1000} ref={bigCanvasRef}></canvas>
    </div>
  );
}

export default Main;
