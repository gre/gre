import React, { useEffect, useState, useRef } from "react";
import { useControls } from "leva";
import useDimensions from "react-cool-dimensions";

function loadImage(src) {
  const img = new Image();
  img.src = src;
  return new Promise((onload, onerror) => {
    img.onload = () => onload(img);
    img.onerror = (e) => (console.error(e), onerror(e));
  });
}

const viewportStyle = {
  position: "absolute",
  width: "100vw",
  height: "100vh",
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  justifyContent: "center",
  cursor: "crosshair",
};

export default function Main() {
  const dpr1 = 2;
  let zoomMultiply = 1;
  const zoomW = 200;
  const zoomH = 200;

  const { image: imageSrc } = useControls("Select an Image", {
    image: { image: "" },
    grid: { value: [4, 2], step: 1 },
  });
  const [image, setImage] = useState(null);
  const [points, setPoints] = useState([]);
  const [zoomAt, setZoomAt] = useState(null);
  const canvasRef = useRef();
  const canvas2Ref = useRef();
  const viewport = useDimensions({});

  useEffect(() => {
    if (!imageSrc) return;
    setImage(null);
    setPoints([]);
    setZoomAt(null);
    loadImage(imageSrc).then(setImage);
  }, [imageSrc]);

  // canvas1
  useEffect(() => {
    if (!image) return;
    const c = canvasRef.current;
    const ctx = c.getContext("2d");
    ctx.drawImage(image, 0, 0, c.width, c.height);
    ctx.strokeStyle = "red";
    for (const p of points) {
      const x = Math.round((p[0] * c.width) / image.width);
      const y = Math.round((p[1] * c.height) / image.height);
      ctx.beginPath();
      ctx.arc(x, y, 4, 0, 2 * Math.PI);
      ctx.stroke();
    }
  }, [image, points, viewport.width, viewport.height]);

  // canvas2
  useEffect(() => {
    if (!image || !zoomAt) return;
    const c = canvas2Ref.current;
    const ctx = c.getContext("2d");
    const sWidth = zoomW * zoomMultiply;
    const sHeight = zoomH * zoomMultiply;
    ctx.drawImage(
      image,
      zoomAt[0],
      zoomAt[1],
      sWidth,
      sHeight,
      0,
      0,
      zoomW,
      zoomH
    );
  }, [image, zoomAt]);

  const onClickCanvas1 = (e) => {
    if (!image) return;
    if (zoomAt) {
      setZoomAt(null);
      return;
    }
    const r = e.target.getBoundingClientRect();
    const x = e.clientX - r.x;
    const y = e.clientY - r.y;
    const xp = x / r.width;
    const yp = y / r.height;
    const ax = Math.floor(xp * image.width);
    const ay = Math.floor(yp * image.height);
    setZoomAt([
      ax - zoomMultiply * zoomW * 0.5,
      ay - zoomMultiply * zoomH * 0.5,
    ]);
  };
  const onClickCanvas2 = (e) => {
    if (!image || !zoomAt) return;
    const r = e.target.getBoundingClientRect();
    const x = e.clientX - r.x;
    const y = e.clientY - r.y;
    const xp = x / r.width;
    const yp = y / r.height;
    const p = [
      Math.round(zoomAt[0] + xp * zoomW),
      Math.round(zoomAt[1] + yp * zoomH),
    ];
    setPoints((pts) => pts.concat([p]));
    setZoomAt(null);
  };

  if (!viewport.height || !image) {
    return <div ref={viewport.observe} style={viewportStyle} />;
  }

  const imageRatio = image.width / image.height;
  const viewportRatio = viewport.width / viewport.height;
  const maxRatio = Math.max(viewportRatio, imageRatio);
  let width = 0.9 * (imageRatio / maxRatio) * viewport.width;
  let height = 0.9 * (viewportRatio / maxRatio) * viewport.height;
  width = Math.round(width);
  height = Math.round(height);

  const canvas1style = {
    border: "1px solid black",
    width,
    height,
  };
  const canvas2style = {
    position: "absolute",
    right: 10,
    bottom: 10,
    border: "1px solid black",
    width: zoomW,
    height: zoomH,
  };

  return (
    <div ref={viewport.observe} style={viewportStyle}>
      <style jsx global>{`
        html,
        body {
          padding: 0;
          margin: 0;
          font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Roboto,
            Oxygen, Ubuntu, Cantarell, Fira Sans, Droid Sans, Helvetica Neue,
            sans-serif;
        }
      `}</style>
      <canvas
        style={canvas1style}
        ref={canvasRef}
        width={width * dpr1}
        height={height * dpr1}
        onClick={onClickCanvas1}
      ></canvas>
      <canvas
        hidden={!zoomAt}
        style={canvas2style}
        ref={canvas2Ref}
        width={zoomW}
        height={zoomH}
        onClick={onClickCanvas2}
      ></canvas>
      <footer style={{ padding: 8, fontSize: 10 }}>
        {points.map((p) => p.join(",")).join(" ")}
      </footer>
    </div>
  );
}
