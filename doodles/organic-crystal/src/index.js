/**
 * Organic Crystal – 2023 – CC BY-SA 4.0 https://creativecommons.org/licenses/by-sa/4.0/
 * Author: @greweb
 */
import React, {
  useEffect,
  useMemo,
  useRef,
  useState,
  useCallback,
} from "react";
import {
  Object3D,
  Vector3,
  Texture,
  CanvasTexture,
  RepeatWrapping,
  NearestFilter,
} from "three";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
import { EffectComposer, Bloom } from "@react-three/postprocessing";
import { Canvas, useThree, useFrame, extend } from "@react-three/fiber";
import init, { render } from "../rust/pkg/main";
import wasm from "base64-inline-loader!../rust/pkg/main_bg.wasm";

Object3D.DefaultUp = new Vector3(0, 0, 1);

extend({ OrbitControls });

const keyCodeByLetters = {
  d: 68,
  s: 83,
};

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

const fromOffset = -40;
const toOffset = 30;
const offsetPhase1 = 2.8;
const transitionDuration = 54 * 1000;
const spreadInitial = 70;
const spreadEnd = 20;
const spreadDurationTransition = 131 * 1000;
const gradients = {
  BlackRedYellow: [
    [80, 66, 74],
    [231, 98, 59],
    [240, 225, 110],
  ],
  SilkBlueGreen: [
    //[50, 120, 240],
    //[50, 200, 100],
    [50, 180, 240],
    [70, 190, 120],
  ],
};
const doesAnimateMap = {
  BlackRedYellow: true,
};
const onlyOnXAxisMap = {
  SilkBlueGreen: true,
};

function onChangeOffsetSpread(doesAnimate, onChange) {
  const t = Date.now();
  function f() {
    const dt = Date.now() - t;
    const phase = Math.sin(offsetPhase1 + (Math.PI * dt) / transitionDuration);
    const offset = fromOffset + ((toOffset - fromOffset) * (phase + 1)) / 2;
    const phase2 = Math.sin((Math.PI * dt) / spreadDurationTransition);
    const spread =
      spreadInitial + ((spreadEnd - spreadInitial) * (phase2 + 1)) / 2;
    onChange({ offset, spread });
  }
  f();
  if (doesAnimate) {
    const interval = setInterval(f, 200);
    return () => clearInterval(interval);
  }
}

function calculateColors(style, geometry, colors, offset, spread) {
  const gradient = gradients[style];
  const onlyOnXAxis = onlyOnXAxisMap[style];
  const gradientColorAt = (percent) => {
    const last = gradient.length - 1;
    if (percent >= 1) return gradient[last];
    const v = Math.max(0, percent * last);
    const index = Math.floor(v);
    const [r1, g1, b1] = gradient[index];
    const [r2, g2, b2] = gradient[index + 1];
    const p = v - index;
    return [
      Math.floor(r1 * (1 - p) + r2 * p),
      Math.floor(g1 * (1 - p) + g2 * p),
      Math.floor(b1 * (1 - p) + b2 * p),
    ];
  };

  for (let i = 0; i < colors.length; i += 3) {
    let percent = 0;
    if (onlyOnXAxis) {
      const n = geometry.attributes.normal.array[i];
      percent = isNaN(n) ? 0 : (1 + n) / 2;
    } else {
      const z = geometry.attributes.position.array[i + 2];
      const nz = geometry.attributes.normal.array[i + 2];
      const normalPointingDownFactor = isNaN(nz) ? 0 : 0.3 * Math.max(0, -nz);
      percent = Math.max(
        0,
        Math.min(1, (offset - z) / spread + normalPointingDownFactor)
      );
    }
    let clr = gradientColorAt(percent);
    colors[i] = clr[0] / 255;
    colors[i + 1] = clr[1] / 255;
    colors[i + 2] = clr[2] / 255;
  }
}

function useColors(geometry, colorsBufferAttributeRef, style) {
  const colors = useMemo(() => {
    if (!geometry || !gradients[style]) return null;
    const colors =
      (colorsBufferAttributeRef.current &&
        colorsBufferAttributeRef.current.array) ||
      new Float32Array(geometry.attributes.position.count * 3);
    return colors;
  }, [geometry, style]);

  useEffect(() => {
    if (!colors) return;
    const doesAnimate = doesAnimateMap[style];
    return onChangeOffsetSpread(doesAnimate, ({ offset, spread }) => {
      calculateColors(style, geometry, colors, offset, spread);
      if (colorsBufferAttributeRef.current) {
        colorsBufferAttributeRef.current.array = colors;
        colorsBufferAttributeRef.current.needsUpdate = true;
      }
    });
  }, [colors, geometry, style]);

  const color = style === "Gold" ? "#fc5" : "#fff";
  const roughness =
    style === "SilkGreenBlue" ? 0.1 : style === "Gold" ? 0.25 : 0.4;
  const metalness = style === "Gold" ? 0.05 : 0.0;

  return { colors, color, roughness, metalness };
}

function useRustRender(variables) {
  const [loaded, setLoaded] = useState(wasmLoaded);
  useEffect(() => {
    if (!loaded) promiseOfLoad.then(() => setLoaded(true));
  }, [loaded]);
  return useMemo(() => {
    if (!loaded) return {};
    let prev = Date.now();
    const result = render(variables.opts);
    const stl = result.stl();
    const features = JSON.parse(result.features());
    console.log(
      "calc time = " + (Date.now() - prev) + "ms – " + stl.length + " bytes"
    );
    const loader = new STLLoader();
    const geometry = loader.parse(new Uint8Array(stl).buffer);
    return { geometry, stl, features };
  }, [variables.opts, loaded]);
}

function useDarkMode() {
  const [isDark, setIsDark] = useState(
    (window.matchMedia &&
      window.matchMedia("(prefers-color-scheme: dark)").matches) ||
      false
  );
  const onSwitchDarkMode = useCallback(() => {
    setIsDark((isDark) => !isDark);
  }, []);
  return { isDark, onSwitchDarkMode };
}

function useFxhashScreenshot(shouldScreenshot, delay) {
  useEffect(() => {
    if (!shouldScreenshot) return;
    const t = setTimeout(fxpreview, delay);
    return () => clearTimeout(t);
  }, [shouldScreenshot, delay]);
}

function useFxhashFeatures(features) {
  useEffect(() => {
    if (!features) return;

    const fts = {};
    for (const key in features) {
      let keyInCamelCase = "";
      let shouldUppercase = true;
      for (let i = 0; i < key.length; i++) {
        const c = key[i];
        if (shouldUppercase) {
          keyInCamelCase += c.toUpperCase();
          shouldUppercase = false;
        } else if (c === "_") {
          shouldUppercase = true;
          keyInCamelCase += " ";
        } else {
          keyInCamelCase += c;
        }
      }
      fts[keyInCamelCase] = features[key];
    }

    window.$fxhashFeatures = fts;
    if (console && console.table) {
      console.table(window.$fxhashFeatures);
    }
  }, [features]);
}

function Scene({ variables, isDark }) {
  const bg = isDark ? "#222" : "#ddd";

  const { geometry, stl, features } = useRustRender(variables);
  const colorsBufferAttributeRef = useRef();
  const { colors, color, metalness, roughness } = useColors(
    geometry,
    colorsBufferAttributeRef,
    features.style
  );
  useFxhashFeatures(features);
  useFxhashScreenshot(!!stl, 1000);

  const ambLightsIntensity = 0.04;

  return (
    <>
      {geometry ? (
        <mesh receiveShadow castShadow>
          <primitive object={geometry} attach="geometry">
            {colors ? (
              <bufferAttribute
                attach="attributes-color"
                array={colors}
                itemSize={3}
                count={colors.length / 3}
                ref={colorsBufferAttributeRef}
              />
            ) : null}
          </primitive>
          <meshStandardMaterial
            vertexColors={!!colors}
            roughness={roughness}
            color={color}
            metalness={metalness}
          />
        </mesh>
      ) : null}

      {stl ? (
        <OnKeyUp
          onKeyUp={() => download(stl, "model.stl")}
          keyCode={keyCodeByLetters.s}
        />
      ) : null}

      <color attach="background" args={[bg]} />

      <fog
        attach="fog"
        color={bg}
        near={isDark ? 42 : 74}
        far={isDark ? 165 : 195}
      />

      <ambientLight intensity={0.28} />
      <pointLight
        shadow-mapSize={2048}
        intensity={0.1}
        castShadow
        position={[-200, 0, 200]}
      />
      {/* background lights */}
      <pointLight
        castShadow
        color="#6cF"
        intensity={ambLightsIntensity}
        position={[100, -100, -20]}
      />
      <pointLight
        castShadow
        color="#6cF"
        intensity={ambLightsIntensity}
        position={[-100, -100, -100]}
      />
      <pointLight
        castShadow
        color="#Fc6"
        intensity={ambLightsIntensity}
        position={[0, -100, 100]}
      />
      <pointLight
        castShadow
        color="#Fc6"
        intensity={ambLightsIntensity}
        position={[0, 0, 100]}
      />
      {/* foreground lights */}
      <pointLight
        castShadow
        color="#Fc6"
        intensity={ambLightsIntensity}
        position={[50, 100, -30]}
      />
      <pointLight
        castShadow
        color="#6cF"
        intensity={ambLightsIntensity}
        position={[-50, 100, -30]}
      />

      <mesh rotation={[0, 0, 0]} position={[0, 0, -30]} receiveShadow>
        <planeGeometry args={[100, 100]} />
        <shadowMaterial transparent opacity={isDark ? 0.1 : 0.05} />
      </mesh>

      <Controls />
    </>
  );
}

function download(data, filename) {
  const blob = new Blob([data], { type: "application/octet-stream" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

function OnKeyUp({ onKeyUp, keyCode }) {
  useEffect(() => {
    const handler = (e) => {
      if (e.keyCode === keyCode) onKeyUp();
    };
    window.addEventListener("keyup", handler);
    return () => window.removeEventListener("keyup", handler);
  }, [onKeyUp, keyCode]);
  return null;
}

function Main({ random }) {
  const { isDark, onSwitchDarkMode } = useDarkMode();
  const variables = useVariables({ random });

  return (
    <Canvas shadows camera={{ position: [0, 110, 25], fov: 54 }}>
      <Scene variables={variables} isDark={isDark} />

      <OnKeyUp onKeyUp={onSwitchDarkMode} keyCode={keyCodeByLetters.d} />

      <EffectComposer>
        <Bloom
          luminanceThreshold={isDark ? 0 : 0.1}
          luminanceSmoothing={isDark ? 0.2 : 0.8}
          intensity={0.8}
          width={1024}
        />
      </EffectComposer>
    </Canvas>
  );
}

function Controls() {
  const controls = useRef();
  const { camera, gl } = useThree();
  useFrame(() => controls.current.update());

  return (
    <orbitControls
      ref={controls}
      args={[camera, gl.domElement]}
      enableDamping
      dampingFactor={0.1}
      rotateSpeed={0.5}
      minDistance={40}
      maxDistance={140}
      autoRotate
      autoRotateSpeed={0.5}
    />
  );
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

function generateVariables(random, hash, debug = false) {
  const opts = {
    hash,
    scale: 60,
  };

  // eslint-disable-next-line no-undef
  if (process.env.NODE_ENV !== "production" && typeof window !== "undefined") {
    console.log(window.fxhash);
    Object.keys(opts).forEach((key) => console.log(key + " =", opts[key]));
  }

  return {
    opts,
  };
}

export default Main;
