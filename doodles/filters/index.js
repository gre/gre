import React, { useState } from "react";
import ReactDOM from "react-dom";
import { Surface } from "gl-react-dom";
import GLImage from "gl-react-image";
import useDimensions from "react-cool-dimensions";
import { useControls } from "leva";
import Effects from "./Effects";

const fields = {
  blur: {
    name: "Blur",
    value: 0,
    min: 0,
    max: 6,
    step: 0.1,
  },
  contrast: {
    name: "Contrast",
    value: 1,
    min: 0,
    max: 4,
    step: 0.1,
  },
  brightness: {
    name: "Brightness",
    value: 1,
    min: 0,
    max: 4,
    step: 0.1,
  },
  saturation: {
    name: "Saturation",
    value: 1,
    min: 0,
    max: 10,
    step: 0.1,
  },
  hue: {
    name: "HueRotate",
    value: 0,
    min: 0,
    max: 2 * Math.PI,
    step: 0.1,
  },
  negative: {
    name: "Negative",
    value: 0,
    min: 0,
    max: 1,
    step: 0.05,
  },
  sepia: {
    name: "Sepia",
    value: 0,
    min: 0,
    max: 1,
    step: 0.05,
  },
  flyeye: {
    name: "FlyEye",
    value: 0,
    min: 0,
    max: 1,
    step: 0.05,
  },
};

const Scene = ({ imageSrc, width, height, values }) => {
  return (
    <Effects width={width} height={height} {...values}>
      <GLImage source={imageSrc} resizeMode="contain" />
    </Effects>
  );
};

const Rendering = ({ imageSrc, onDrop, onDragOver }) => {
  const { ref, width, height } = useDimensions({});
  const values = useControls(fields);

  return (
    <div
      onDrop={onDrop}
      onDragOver={onDragOver}
      ref={ref}
      style={{ width: "100vw", height: "100vh" }}
    >
      <Surface preload={[imageSrc]} width={width} height={height}>
        <Scene
          imageSrc={imageSrc}
          width={width}
          height={height}
          values={values}
        />
      </Surface>
      <footer>Right Click &gt; Save as Image</footer>
    </div>
  );
};

const Main = () => {
  const [imageSrc, setImageSrc] = useState(null);

  const onDragOver = (e) => {
    e.preventDefault();
  };

  const onDrop = (e) => {
    e.preventDefault();
    if (e.dataTransfer.items && e.dataTransfer.items.length > 0) {
      const [fileItem] = e.dataTransfer.items;
      const file = fileItem.getAsFile();
      if (!file.type.startsWith("image/")) {
        alert("mmh. this is not an image?");
        return;
      }
      const reader = new FileReader();
      reader.onload = function (e) {
        setImageSrc(e.target.result);
      };
      reader.readAsDataURL(file);
    }
  };

  if (imageSrc) {
    return (
      <Rendering onDrop={onDrop} onDragOver={onDragOver} imageSrc={imageSrc} />
    );
  }

  return (
    <div onDrop={onDrop} onDragOver={onDragOver} className="container">
      <h1>Filters</h1>
      <h2>Drop any image here</h2>
      <p>
        <em>This NFT will allow to apply effects on it</em>
      </p>
    </div>
  );
};

ReactDOM.render(<Main />, document.getElementById("main"));
