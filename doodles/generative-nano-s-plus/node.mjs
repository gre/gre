import { generate } from "./features.mjs";
import { art } from "./art.mjs";
import cv from "canvas";
import { fillTextWithTwemoji } from "node-canvas-with-twemoji";
import createRegl from "regl";
import createReglRecorder from "regl-recorder";
import createGL from "gl";
import fs from "fs";

const index = parseInt(process.argv[2] || 0, 10);
const scale = 1;
const W = 1920 * scale;
const H = 1920 * scale;
const duration = 4;
const speed = 1 / duration;
const fps = 30;
const frames = duration * fps;

const regl = createRegl(createGL(W, H, { preserveDrawingBuffer: true }));
const recorder = createReglRecorder(regl, frames);

const frame = (t) => t / fps;
const onFrame = () => recorder.frame(W, H);

const { opts, metadata } = generate(index);

const createImageData = (canvas) => {
  const ctx = canvas.getContext("2d");
  const width = canvas.width;
  const height = canvas.height;
  const imageData = ctx.getImageData(0, 0, width, height);
  return { data: imageData.data, width, height };
};

const makeFillText =
  (ctx) =>
  (...args) =>
    fillTextWithTwemoji(ctx, ...args);

fs.writeFileSync("metadata.json", JSON.stringify(metadata, null, 2), "utf-8");

art(
  regl,
  opts,
  frame,
  onFrame,
  cv.createCanvas,
  makeFillText,
  createImageData,
  true,
  true,
  speed
);
