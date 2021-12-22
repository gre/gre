import React, { useEffect, useMemo, useRef, useState } from "react";
import regl from "regl";
import Link from "next/link";
import { art, generate } from "../doodles/generative-nano-s-plus/dist/main";

export const n = 110;
export const title = "GNSP";
export const moreinfoLink = "https://greweb.me/gnsp";

export function Render({ width, height }) {
  const [index, setIndex] = useState(() => Math.floor(2048 * Math.random()));
  const ref = useRef();
  const { opts, metadata } = useMemo(() => generate(index), [index]);
  useEffect(() => {
    const i = setInterval(() => {
      setIndex((i) => (i + 1) % 2048);
    }, 8000);
    return () => clearInterval(i);
  }, []);
  useEffect(() => {
    const c = regl(ref.current);
    const frameTime = (_, o) => o.time;
    const onFrame = () => {};
    const createCanvas = (w, h) => {
      const canvas = document.createElement("canvas");
      canvas.width = w;
      canvas.height = h;
      return canvas;
    };
    const antialias = false;
    art(
      c,
      opts,
      frameTime,
      onFrame,
      createCanvas,
      (ctx) =>
        (...args) =>
          ctx.fillText(...args),
      (canvas) => ({ data: canvas, flipY: true }),
      false,
      antialias,
      0.25
    );
    return () => {
      c.destroy();
    };
  }, [index, opts]);
  const dpr =
    (typeof window !== "undefined" ? window.devicePixelRatio : null) || 1;
  return (
    <div
      style={{
        position: "relative",
        width,
        height,
        boxSizing: "content-box",
      }}
    >
      <Link href={"/gnsp/" + index}>
        <a target="_blank">
          <p
            style={{
              position: "absolute",
              bottom: 12,
              left: 6,
              margin: 0,
              color: "#444",
              fontWeight: 300,
              fontSize: "10px",
            }}
          >
            {metadata.name} ({index + 1} / 2048)
          </p>
          <canvas
            ref={ref}
            width={Math.round(width * dpr)}
            height={Math.round(height * dpr)}
            style={{ width, height }}
          ></canvas>
        </a>
      </Link>
    </div>
  );
}
