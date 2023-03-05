import dotenv from "dotenv";
import { art } from "./art.mjs";
import createRegl from "regl";
import createReglRecorder from "regl-recorder";
import createGL from "gl";
import getPixels from "get-pixels";

dotenv.config({ path: "~/.livedraw/render-svg-envs" })

const scale = parseInt(process.env.UPSCALE || "1");
const W = scale * parseInt(process.env.WIDTH);
const H = scale * parseInt(process.env.HEIGHT);
const fps = parseInt(process.env.FRAMERATE);
const frames = parseInt(process.env.FRAMES);

const regl = createRegl(createGL(W, H, { preserveDrawingBuffer: true }));
const recorder = createReglRecorder(regl, frames);

const frame = (t) => t / fps;
const onFrame = () => recorder.frame(W, H);

getPixels(process.env.IMAGE_INPUT, function (err, image) {
  if (err) {
    console.error(err);
    process.exit(1);
  }
  art({
    regl,
    frameTime: frame,
    onFrame,
    image,
    width: W,
    height: H,
  });

})