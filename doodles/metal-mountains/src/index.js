// @flow
/**
 * LICENSE ...
 * Author: ...
 */
import React, { useEffect, useMemo, useState } from "react";
import init, { render } from "../rust/pkg/main";
import wasm from "base64-inline-loader!../rust/pkg/main_bg.wasm";
import generateVariables, { getPerf, height, width } from "./variables";

function decode(dataURI) {
  const binaryString = atob(dataURI.split(",")[1]);
  var bytes = new Uint8Array(binaryString.length);
  for (var i = 0; i < binaryString.length; i++) {
    bytes[i] = binaryString.charCodeAt(i);
  }
  return bytes.buffer;
}
let wasmLoaded = false;
const promiseOfLoad = init(decode(wasm)).then(() => {
  wasmLoaded = true;
});

const svgSize = [width, height];
const MAX = 4096;
const ratio = svgSize[0] / svgSize[1];
const svgMMSize = svgSize.map((s) => s + "mm");

let adaptiveSvgWidth = (width) => Math.max(64, Math.ceil(width / 64) * 64);

function RenderInCSS({ renderedSVG, svgW, svgH, variables }) {
  const rgb = variables.background.rgb
    .map((n) => Math.round(n * 255))
    .join(",");
  const style = {
    contentFit: "contain",
    height: "100%",
    width: "100%",
    background: `rgb(${rgb})`,
  };
  let stop1 = `rgba(${rgb},0.6)`;
  let stop2 = `rgba(255,255,255,0.1)`;
  return (
    <>
      <div
        className="layer"
        style={{
          backgroundImage: `linear-gradient(0deg, ${stop1}, ${stop2} 40%, ${stop2} 60%, ${stop1})`,
        }}
      />
      <img src={renderedSVG} style={style} />
    </>
  );
}

const Render = RenderInCSS;

const Debug = ({ features }) => {
  return (
    <pre
      style={{
        position: "absolute",
        zIndex: 1,
        top: 4,
        right: 16,
        fontSize: 12,
        color: "white",
        fontFamily: "monospace",
      }}
    >
      <code>
        {Object.keys(features)
          .map((key) => `${key}: ${features[key]}`)
          .join("\n")}
      </code>
    </pre>
  );
};

const Main = ({ width, height, random }) => {
  const dpr = window.devicePixelRatio || 1;
  let W = width;
  let H = height;
  H = Math.min(H, W / ratio);
  W = Math.min(W, H * ratio);
  W = Math.floor(W);
  H = Math.floor(H);
  let w = Math.min(MAX, dpr * W);
  let h = Math.min(MAX, dpr * H);
  h = Math.min(h, w / ratio);
  w = Math.min(w, h * ratio);
  w = Math.floor(w);
  h = Math.floor(h);
  const svgW = adaptiveSvgWidth(w);
  const svgH = Math.floor(svgW / ratio);
  const widthPx = svgW + "px";
  const heightPx = svgH + "px";

  const [loaded, setLoaded] = useState(wasmLoaded);
  const variables = useVariables({ random });

  useEffect(() => {
    if (!loaded) promiseOfLoad.then(() => setLoaded(true));
  }, [loaded]);

  const { svg, features } = useMemo(() => {
    if (!loaded) return { svg: "", features: {} };
    let prev = Date.now();
    const result = render(variables.opts);
    console.log(
      "svg calc time = " +
        (Date.now() - prev) +
        "ms – " +
        (result.length / (1024 * 1024)).toFixed(3) +
        " Mb"
    );
    const features = generateVariables.inferProps(variables, result);
    window.$fxhashFeatures = features;
    if (console && console.table) {
      console.table(window.$fxhashFeatures);
      const p = getPerf(result);
      if (p) {
        console.table(p.per_label);
      }
    }
    return { svg: result, features };
  }, [variables.opts, loaded]);

  const renderedSVG = useMemo(() => {
    const { background, layers } = variables;
    let svgOut = svg.replace(
      "background:" + background.placeholder,
      `background:transparent`
    );
    layers.forEach((l) => {
      svgOut = svgOut.replace(
        l.search,
        "rgb(" + l.rgb.map((n) => Math.round(n * 255)).join(",") + ")"
      );
    });
    return (
      "data:image/svg+xml;base64," +
      btoa(
        svgOut.replace(svgMMSize[1], heightPx).replace(svgMMSize[0], widthPx)
      )
    );
  }, [svg, widthPx, heightPx]);

  const props = { renderedSVG, svgW, svgH, variables };

  return (
    <>
      <Render {...props} />
      {variables.opts.debug && features ? <Debug features={features} /> : null}
      <Downloadable
        svg={svg}
        layers={variables.layers}
        background={variables.background}
      />
    </>
  );
};

/*
function useSvgTexture(src, width, height) {
  const [result, setResult] = useState(null);
  useMemo(() => {
    if (!src || !width || !height) return;
    const img = document.createElement("img");
    const canvas = document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;
    const ctx = canvas.getContext("2d");
    img.setAttribute("src", src);
    img.onload = function () {
      ctx.drawImage(img, 0, 0);
      const texture = new THREE.Texture(canvas);
      texture.needsUpdate = true;
      setResult(texture);
    };
  }, [src, width, height]);
  return result;
}
*/

const dlStyle = {
  opacity: 0,
  width: "100%",
  height: "100%",
  zIndex: 0,
  position: "absolute",
  top: 0,
  left: 0,
};
function Downloadable({ svg, layers, background }) {
  const [uri, setURI] = useState(null);
  useEffect(() => {
    const timeout = setTimeout(() => {
      let svgOut = svg
        .replace(
          "background:" + background.placeholder,
          `background:${
            "rgb(" +
            background.rgb.map((c) => Math.floor(c * 255)).join(",") +
            ")"
          }`
        )
        .replace(/opacity="[^"]*"/g, 'style="mix-blend-mode: multiply"');

      layers.forEach((l) => {
        svgOut = svgOut.replace(
          l.search,
          "rgb(" + l.rgb.map((n) => Math.round(n * 255)).join(",") + ")"
        );
      });

      setURI("data:image/svg+xml;base64," + btoa(svgOut));
    }, 500);
    return () => clearTimeout(timeout);
  }, [svg, layers, background]);

  return <img style={dlStyle} src={uri} />;
}

function useVariables({ random }) {
  return useMemo(
    () =>
      generateVariables(
        random,
        window.fxhash,
        new URLSearchParams(window.location.search).get("debug") === "1"
      ),
    []
  );
}

export default Main;
