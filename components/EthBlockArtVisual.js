import { Surface } from "gl-react-dom";

export function EthBlockArtVisual({
  BlockStyle,
  attributesRef,
  block,
  width,
  height,
  values,
}) {
  return (
    <Surface width={width} height={height}>
      <BlockStyle
        attributesRef={attributesRef}
        block={block}
        width={width}
        height={height}
        {...values}
      />
    </Surface>
  );
}
