import React, { useState, useEffect, useRef } from "react";
import { GIFEncoder, quantize, applyPalette } from "gifenc";
import ReactDOM from "react-dom";
import { NearestCopy } from "gl-react";
import { Surface } from "gl-react-dom";
import useDimensions from "react-cool-dimensions";
import { useControls, buttonGroup } from "leva";
import * as faceLandmarksDetection from "./face-landmarks-detection";
// import * as faceLandmarksDetection from "@tensorflow-models/face-landmarks-detection";
import "@tensorflow/tfjs-backend-cpu";
import { FaceEyesEffect } from "./FaceEyesEffect";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

import blazerface from "./blazerface.raw";
import facemesh from "./facemesh.raw";
import iris from "./iris.raw";

window.HACKURL = {
  blazerface: "./" + blazerface,
  facemesh: "./" + facemesh,
  iris: "./" + iris,
};

function useTime() {
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
  return time;
}

function loadImage(src) {
  const img = new Image();
  img.src = src;
  return new Promise((onload, onerror) => {
    img.onload = () => onload(img);
    img.onerror = (e) => (console.error(e), onerror(e));
  });
}

async function detectEyes(imageSrc) {
  const image = await loadImage(imageSrc);
  const model = await faceLandmarksDetection.load(
    faceLandmarksDetection.SupportedPackages.mediapipeFacemesh
  );
  const predictions = await model.estimateFaces({ input: image });
  if (predictions.length > 0) {
    const { annotations } = predictions[0];
    const [x1, y1] = annotations.leftEyeIris[0];
    const [x2, y2] = annotations.rightEyeIris[0];
    return [
      x1 / image.width,
      1 - y1 / image.height,
      x2 / image.width,
      1 - y2 / image.height,
    ];
  }
}

function useEyes(imageSrc) {
  const [state, setEyesState] = useState({
    eyes: [0, 0, 0, 0],
    found: false,
  });
  useEffect(() => {
    setEyesState({
      eyes: [0, 0, 0, 0],
      found: false,
    });
    detectEyes(imageSrc).then(
      (eyes) => {
        if (eyes) {
          setEyesState({ eyes, found: true });
        }
      },
      (e) => {
        alert("Sorry, an unexpected problem occurred. " + e);
      }
    );
  }, [imageSrc]);
  return state;
}

const Scene = (props) => {
  const time = useTime();
  return <FaceEyesEffect time={time} {...props} />;
};

/*
const DebugScene = (p) => {
  const time = useTime();
  const eyesRes = {
    found: true,
    eyes: [
      0.5896685028076172,
      0.5442092895507813,
      0.4213985824584961,
      0.5467979049682616,
    ],
  };
  return (
    <FaceEyesEffect time={time} eyesRes={eyesRes} {...p}>
      {debugSrc}
    </FaceEyesEffect>
  );
};
*/

const viewportStyle = {
  position: "absolute",
  width: "100vw",
  height: "100vh",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
};
const Rendering = ({ image }) => {
  const {
    amount: mod1,
    size: mod2,
    tezos_maxi: mod3,
  } = useControls("Parameters", {
    amount: {
      value: 0.5,
      step: 0.01,
      min: 0,
      max: 1,
    },
    size: {
      step: 0.01,
      value: 0.5,
      min: 0,
      max: 1,
    },
    tezos_maxi: {
      step: 0.01,
      value: 0,
      min: 0,
      max: 1,
    },
  });
  const [capturing, setCapturing] = useState();

  const { exportSize, gif_fps, gif_duration } = useControls("Actions", {
    exportSize: 400,
    gif_fps: 16,
    gif_duration: 4,
    Export: buttonGroup({
      "as PNG": () => {
        setCapturing("png");
      },
      "as GIF": () => {
        setCapturing("gif");
      },
    }),
  });

  function download(blob, filename) {
    console.log("DL", filename);
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    toast(
      <a href={url} rel="noreferrer">
        Open Result (URL)
      </a>
    );
    anchor.href = url;
    anchor.download = filename;
    anchor.click();
  }

  function onFinished(blob) {
    download(blob, "lasereyes." + capturing);
    setCapturing(null);
  }

  const start = 0;
  const end = gif_duration;
  const framePerSecond = gif_fps;
  const speed = 1;

  const eyesRes = useEyes(image.src);

  const viewport = useDimensions({});
  if (!viewport.height) {
    return <div ref={viewport.observe} style={viewportStyle} />;
  }
  const imageRatio = image.width / image.height;
  const viewportRatio = viewport.width / viewport.height;
  const maxRatio = Math.max(viewportRatio, imageRatio);
  let width = 0.9 * (imageRatio / maxRatio) * viewport.width;
  let height = 0.9 * (viewportRatio / maxRatio) * viewport.height;
  width = Math.round(width);
  height = Math.round(height);

  const captureW =
    image.width > image.height
      ? exportSize
      : Math.round(exportSize * imageRatio);
  const captureH =
    image.width < image.height
      ? exportSize
      : Math.round(exportSize / imageRatio);

  return (
    <div ref={viewport.observe} style={viewportStyle}>
      <ToastContainer position="top-left" autoClose={false} />

      {capturing ? (
        <CaptureMemo
          preload={[image.src]}
          format={capturing}
          start={start}
          end={end}
          framePerSecond={framePerSecond}
          speed={speed}
          width={captureW}
          height={captureH}
          onFinished={onFinished}
          render={(time) => (
            <Scene
              eyesRes={eyesRes}
              mod1={mod1}
              mod2={mod2}
              mod3={mod3}
              time={time}
            >
              {image.src}
            </Scene>
          )}
        />
      ) : (
        <Surface width={width} height={height}>
          <Scene eyesRes={eyesRes} mod1={mod1} mod2={mod2} mod3={mod3}>
            {image.src}
          </Scene>
        </Surface>
      )}
    </div>
  );
};

function Capture({
  format,
  render,
  onFinished,
  width: sw,
  height: sh,
  start,
  end,
  framePerSecond,
  speed,
  preload,
}) {
  const ref = useRef();
  const totalFrames = Math.floor(((end - start) * framePerSecond) / speed);
  const [frame, setFrame] = useState(0);
  const [, setReady] = useState(false);
  const time = start + (end - start) * (frame / totalFrames);

  const [recorder] = useState(() => {
    function captureNDArray() {
      const nda = ref.current.capture();
      const {
        shape: [width, height],
      } = nda;
      const data = new Uint8Array(height * width * 4);
      for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
          for (let i = 0; i < 4; i++) {
            data[(width * y + x) * 4 + i] =
              nda.data[(width * (height - y - 1) + x) * 4 + i];
          }
        }
      }
      return { width, height, data };
    }

    let palette;
    let index;
    let ready;
    let gif;
    let finished;
    let frame = 0;

    let onDraw = () => {
      if (finished) return;
      if (!ready) return;

      if (format === "gif") {
        const { width, height, data } = captureNDArray();
        if (!gif) {
          gif = GIFEncoder();
        }
        palette = quantize(data, 256);
        index = applyPalette(data, palette);
        gif.writeFrame(index, width, height, {
          palette,
          delay: 1000 / framePerSecond,
        });

        if (frame >= totalFrames) {
          gif.finish();
          finished = true;
          onFinished(new Blob([gif.bytes()], { type: "image/gif" }));
        } else {
          setFrame(++frame);
        }
      }
      if (format === "png") {
        finished = true;
        ref.current.captureAsBlob().then(onFinished);
      }
    };

    let onLoad = () => {
      ready = true;
      setReady(true);
    };

    return {
      onDraw,
      onLoad,
    };
  });

  return (
    <>
      {format === "png" ? null : frame + " / " + totalFrames}
      <div hidden>
        <Surface
          webglContextAttributes={{ preserveDrawingBuffer: true }}
          preload={preload}
          onLoad={recorder.onLoad}
          ref={ref}
          width={sw}
          height={sh}
          pixelRatio={1}
        >
          <NearestCopy onDraw={recorder.onDraw}>{render(time)}</NearestCopy>
        </Surface>
      </div>
    </>
  );
}

const CaptureMemo = React.memo(Capture);

const Main = () => {
  const { image: imageSrc } = useControls("Select an Image", {
    image: { image: "" },
  });

  const [image, setImage] = useState(null);

  useEffect(() => {
    if (!imageSrc) return;
    loadImage(imageSrc).then(setImage);
  }, [imageSrc]);

  if (image) {
    return <Rendering image={image} />;
  }

  return (
    <div className="container">
      <h1>Laser Eyes</h1>
      <h2>select a selfie</h2>
      <p>
        This NFT applies laser eyes effect with parameters and allow to export
        as GIF or PNG.
      </p>
    </div>
  );
};

ReactDOM.render(<Main />, document.getElementById("main"));
