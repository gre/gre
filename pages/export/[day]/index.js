import Head from "next/head";
import Link from "next/link";
import { useRouter } from "next/router";
import { GIFEncoder, quantize, applyPalette } from "gifenc";
import { useState, useEffect, useRef, useCallback } from "react";
import { Surface } from "gl-react-dom";
import { NearestCopy } from "gl-react";
import { findDay, getDays } from "../../../day";
import { Visual } from "../../../components/Visual";
import { LiveFooter } from "../../../components/LiveFooter";
import { SubTitle } from "../../../components/SubTitle";
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

export function Capture({
  Day,
  onFinished,
  size,
  start,
  end,
  framePerSecond,
  speed,
}) {
  const ref = useRef();
  const totalFrames = Math.floor(((end - start) * framePerSecond) / speed);
  const [frame, setFrame] = useState(0);
  const time = start + (end - start) * (frame / totalFrames);

  const [recorder] = useState(() => {
    function capture() {
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

    function onDraw() {
      const { width, height, data } = capture();
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
        onFinished(gif.bytes());
      } else {
        setFrame(++frame);
      }
    }

    return {
      onDraw,
    };
  });

  return (
    <>
      <Surface ref={ref} width={size} height={size} pixelRatio={1}>
        <NearestCopy onDraw={recorder.onDraw}>
          <Day.Shader time={time} />
        </NearestCopy>
      </Surface>
      {frame + " / " + totalFrames}
    </>
  );
}

export function Previewing({ Day, start, end, framePerSecond, speed }) {
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
        <Day.Shader time={t} />
      </Surface>
      {t.toFixed(3)}
    </>
  );
}

export default function Home({ day }) {
  const [capturing, setCapturing] = useState();
  const Day = findDay(parseInt(day, 10));
  if (!Day) return null;

  function download(buf, filename, type) {
    const blob = buf instanceof Blob ? buf : new Blob([buf], { type });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = filename;
    anchor.click();
  }

  function onFinished(r) {
    setCapturing(false);
    download(r, "shaderday_" + day + ".gif", "image/gif");
  }

  const start = Day.gifStart || 0;
  const end = Day.gifEnd || 1;
  const framePerSecond = Day.gifFramePerSecond || 24;
  const speed = Day.gifSpeed || 1;
  const size = Day.gifSize || 800;

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
            <Capture
              start={start}
              end={end}
              framePerSecond={framePerSecond}
              speed={speed}
              size={size}
              Day={Day}
              onFinished={onFinished}
            />
          ) : (
            <Previewing
              start={start}
              end={end}
              framePerSecond={framePerSecond}
              speed={speed}
              Day={Day}
            />
          )}
          <button onClick={() => setCapturing(true)}>generate</button>
        </Main>
      </Container>
    </Global>
  );
}
