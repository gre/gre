import Head from "next/head";
import Link from "next/link";
import { useRouter } from "next/router";
import { GIFEncoder, quantize, applyPalette } from "gifenc";
import React, { useState, useEffect, useRef, useCallback } from "react";
import { Surface } from "gl-react-dom";
import { NearestCopy } from "gl-react";
import { findDay, getDays } from "../../../shaderdays";
import { Visual } from "../../../components/Visual";
import { LiveFooter } from "../../../components/LiveFooter";
import { SubTitle } from "../../../components/ShaderdaySubTitle";
import { Title } from "../../../components/Title";
import { SourceCodeFooter } from "../../../components/SourceCodeFooter";
import { Container } from "../../../components/Container";
import { Global } from "../../../components/Global";
import { Main } from "../../../components/Main";
import { SubTitleExport } from "../../../components/SubTitleExport";
import { Header } from "../../../components/Header";

export function getStaticPaths() {
  return {
    paths: getDays().map((Day) => {
      return {
        params: {
          day: String(Day.n),
        },
      };
    }),
    fallback: false,
  };
}

export function getStaticProps({ params }) {
  const day = parseInt(params.day, 10);
  return {
    props: { day },
  };
}

async function readImage(url) {
  const img = await loadImage(url);
  const canvas = document.createElement("canvas");
  canvas.width = img.width;
  canvas.height = img.height;
  const context = canvas.getContext("2d");
  context.drawImage(img, 0, 0);
  return context.getImageData(0, 0, img.width, img.height);
}

async function loadImage(url) {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = () => resolve(img);
    img.onerror = () => reject(new Error(`Could not load image ${url}`));
    img.src = url;
  });
}

function Capture({
  format,
  n,
  Day,
  onFinished,
  size,
  start,
  end,
  framePerSecond,
  exportSkipFrame,
  speed,
  preload,
}) {
  const ref = useRef();
  const totalFrames = Math.floor(((end - start) * framePerSecond) / speed);
  const [frame, setFrame] = useState(0);
  const [ready, setReady] = useState(false);
  const time = ((end - start) * (frame / totalFrames)) / (1 + exportSkipFrame);
  const f = Math.floor(frame / (1 + exportSkipFrame) - start * framePerSecond);

  const [recorder] = useState(() => {
    function captureNDArray() {
      const nda = ref.current.capture();
      const {
        shape: [height, width],
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
    let gif;
    let frame = 0;
    let ready = false;
    let worker;
    let images;
    let lastF;

    let onDraw = () => {
      if (!ready) return;
      const f = Math.floor(
        frame / (1 + exportSkipFrame) - start * framePerSecond
      );
      if (f < 0 || f === lastF) {
        setFrame(++frame);
        return;
      }
      lastF = f;

      if (format === "gif") {
        const { width, height, data } = captureNDArray();
        if (!gif) {
          gif = GIFEncoder();
        }
        if (!palette || !Day.exportPaletteGenOnce) {
          palette = quantize(data, Day.exportPaletteSize || 256);
        }
        index = applyPalette(data, palette);
        gif.writeFrame(index, width, height, {
          palette,
          delay: 1000 / framePerSecond,
        });

        if (f >= totalFrames) {
          gif.finish();
          onFinished(new Blob([gif.bytes()], { type: "image/gif" }));
        } else {
          setFrame(++frame);
        }
      } else {
        if (!worker) {
          worker = new Worker("/ffmpeg-worker-mp4.js");
          images = [];
        }
        const imageStr = ref.current.captureAsDataURL("image/jpeg", 1);
        const data = convertDataURIToBinary(imageStr);
        images.push({
          name: "img" + frame.toString().padStart(4, 0) + ".jpeg",
          data,
        });

        if (f >= totalFrames) {
          let start_time = Date.now();
          worker.onmessage = function (e) {
            var msg = e.data;
            switch (msg.type) {
              case "stdout":
              case "stderr":
                console.log(msg.data);
                break;
              case "exit":
                console.log("Process exited with code " + msg.data);
                // worker.terminate();
                break;

              case "done":
                console.log("reached done");
                const blob = new Blob([msg.data.MEMFS[0].data], {
                  type: "video/mp4",
                });
                onFinished(blob);
                break;
            }
          };

          // https://trac.ffmpeg.org/wiki/Slideshow
          // https://semisignal.com/tag/ffmpeg-js/
          worker.postMessage({
            type: "run",
            TOTAL_MEMORY: 268435456,
            arguments: [
              "-r",
              String(framePerSecond),
              "-i",
              "img%04d.jpeg",
              "-c:v",
              "libx264",
              "-crf",
              "1",
              //"-vf",
              //"scale=150:150",
              "-pix_fmt",
              "yuv420p",
              "-vb",
              Day.exportMP4vb || "5M",
              "out.mp4",
            ],
            MEMFS: images,
          });
        } else {
          setFrame(++frame);
        }
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
      <Surface
        webglContextAttributes={{ preserveDrawingBuffer: true }}
        onLoad={recorder.onLoad}
        ref={ref}
        width={size}
        height={size}
        pixelRatio={1}
        preload={preload}
      >
        <NearestCopy onDraw={recorder.onDraw}>
          <Day.Shader exporting time={time} n={n} />
        </NearestCopy>
      </Surface>
      {f + " / " + totalFrames}
    </>
  );
}

const CaptureMemo = React.memo(Capture);

export function Previewing({ n, Day, start, end, framePerSecond, speed }) {
  const [time, setTime] = useState(0);
  useEffect(() => {
    let startT;
    let lastT;
    let h;
    function loop(t) {
      h = requestAnimationFrame(loop);
      if (!startT) {
        lastT = startT = t;
      }
      if (t - lastT >= 1000 / framePerSecond) {
        lastT = t;
        setTime((t - startT) / 1000);
      }
    }
    h = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(h);
  }, []);
  const t = start + ((speed * time) % (end - start));
  return (
    <>
      <Surface width={400} height={400}>
        <Day.Shader exporting time={t} n={n} />
      </Surface>
      {t.toFixed(3)}
    </>
  );
}

export default function Home({ day }) {
  const [n, setN] = useState(0);
  const [capturing, setCapturing] = useState();
  const Day = findDay(parseInt(day, 10));
  if (!Day) return null;

  function download(blob, filename) {
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = filename;
    anchor.click();
  }

  function onFinished(blob) {
    download(blob, "shaderday_" + day + "_" + n + "." + capturing);
    setCapturing(null);
  }

  const start = Day.exportStart || 0;
  const end = Day.exportEnd || 1;
  const framePerSecond = Day.exportFramePerSecond || 24;
  const exportSkipFrame = Day.exportSkipFrame || 0;
  const speed = Day.exportSpeed || 1;
  const size = Day.exportSize || 800;
  const preload = Day.preload || [];

  return (
    <Global>
      <Container>
        <Head>
          <title>
            One Day One Shader – Day {Day.n}. "{Day.title}"
          </title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <Main>
          <Header>Export tools</Header>
          <SubTitleExport Day={Day} />
          {capturing ? (
            <CaptureMemo
              format={capturing}
              n={n}
              start={start}
              end={end}
              framePerSecond={framePerSecond}
              exportSkipFrame={exportSkipFrame}
              speed={speed}
              size={size}
              Day={Day}
              onFinished={onFinished}
              preload={preload}
            />
          ) : (
            <Previewing
              n={n}
              start={start}
              end={end}
              framePerSecond={framePerSecond}
              speed={speed}
              Day={Day}
            />
          )}
          <input
            style={{ margin: 10 }}
            value={n}
            range={1}
            onChange={(e) => setN(parseInt(e.target.value, 10))}
            type="number"
          ></input>
          <button onClick={() => setCapturing("gif")}>gif</button>
          <button onClick={() => setCapturing("mp4")}>mp4</button>
        </Main>
      </Container>
    </Global>
  );
}

function convertDataURIToBinary(dataURI) {
  var base64 = dataURI.replace(/^data[^,]+,/, "");
  var raw = window.atob(base64);
  var rawLength = raw.length;

  var array = new Uint8Array(new ArrayBuffer(rawLength));
  for (let i = 0; i < rawLength; i++) {
    array[i] = raw.charCodeAt(i);
  }
  return array;
}
