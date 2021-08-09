import React, { useRef } from "react";

export function EthBlockArtVisual({
  BlockStyle,
  attributesRef,
  block,
  width,
  height,
  values,
}) {
  const canvasRef = useRef();
  return (
    <BlockStyle
      canvasRef={canvasRef}
      attributesRef={attributesRef}
      block={block}
      width={width}
      height={height}
      {...values}
    />
  );
}
