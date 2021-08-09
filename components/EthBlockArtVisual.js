import React from "react";

export function EthBlockArtVisual({
  BlockStyle,
  attributesRef,
  block,
  width,
  height,
  values,
}) {
  return (
    <BlockStyle
      attributesRef={attributesRef}
      block={block}
      width={width}
      height={height}
      {...values}
    />
  );
}
