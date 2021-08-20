// @flow
import React, { useCallback, useEffect, useMemo, useRef } from "react";
import MersenneTwister from "mersenne-twister";
import SimplexNoise from "simplex-noise";
import { mix, smoothstep, useStats, useTime } from "../utils";
import { Surface } from "gl-react-dom";
import { GLSL, Node, Shaders, Uniform } from "gl-react";

const DEBUG_SVG = false;

// IDEAS
// one line of diff color
// diff symmetry of noise
// diff orientation
// make orientation repeat in patterns

const COLORS = [
  {
    css: "black",
    name: "Black",
  },
  {
    css: "midnightblue",
    name: "Bloody Brexit",
  },
  {
    css: "deepskyblue",
    name: "Turquoise",
  },
  {
    css: "firebrick",
    name: "Red Dragon",
  },
];

const pickColor = (f) =>
  COLORS[Math.floor(f * (COLORS.length - 1)) % COLORS.length];

function svgPathDataRouteCurve(route) {
  if (!route.length) {
    return "";
  }
  let d = "";
  let last = route[0];
  for (let i = 0; i < route.length; i++) {
    const p = route[i];
    if (i == 0) {
      d += `M${p[0].toFixed(2)},${p[1].toFixed(2)} `;
    } else {
      d += `Q${last[0].toFixed(2)},${last[1].toFixed(2)},${(
        (p[0] + last[0]) /
        2
      ).toFixed(2)},${((p[1] + last[1]) / 2).toFixed(2)} `;
    }
    last = p;
  }
  return d;
}

function renderSVG({ variables }) {
  const strokeWidth = 0.35;
  const opacity = 0.5;

  let seed = variables.seed;

  const noise = new SimplexNoise(seed);

  let m = 1.2;
  let k = 1.4;
  let k1 = 0.3;
  let k2 = 0.3;
  let k3 = 0.3;
  let k4 = 0.3;
  let k5 = 0.3;
  let k6 = 0.3;

  const f = ([x, y]) => {
    let [pointx, pointy] = [0.5 + Math.abs(x - 0.5), y];
    let [px, py] = [pointx * m, pointy * m];
    let a1 = noise.noise3D(3 + 0.9 * seed, px, py);
    let a2 = noise.noise3D(px, py, 7.3 * seed);
    let b1 = noise.noise3D(
      px + 4 * k * a1 + 7.8,
      py + k * a2,
      100.2 * seed - 999
    );
    let b2 = noise.noise3D(
      px + k * a1 + 2.1,
      seed * 8.8 + 99,
      py + 2 * k * a2 - 1.7
    );
    return smoothstep(
      -0.3,
      0.5,
      1.5 * (0.33 - Math.abs(pointx - 0.5)) +
        noise.noise3D(
          -seed,
          px + 0.2 * k * a1 + 0.4 * k * b1,
          py + 0.2 * k * a2 + 0.4 * k * b2
        )
    );
  };
  const offset = ([x, y]) => {
    let a = 1.0 * noise.noise3D(k1 * x, k2 * y, 6.7 * seed);
    let b = 1.5 * noise.noise3D(k4 * x, k3 * y, 99 - 0.3 * seed);
    let c = 2.0 * noise.noise2D(k5 * x + a, k6 * y + b);
    return [
      x + 0.02 * noise.noise2D(a, 10 + c),
      y + 0.01 * noise.noise2D(b, -10 - c),
    ];
  };

  const pad = variables.pad;
  const boundaries = [pad, pad, 200 - pad, 200 - pad];
  const project = (p) => [
    p[0] * (boundaries[2] - boundaries[0]) + boundaries[0],
    p[1] * (boundaries[3] - boundaries[1]) + boundaries[1],
  ];

  const routes = [];
  const xdivisions = 120;
  const lines = 60;
  const sublines = 6;
  for (let i = 0; i < lines; i++) {
    const ypi = i / (lines - 1);
    for (let j = 0; j < sublines; j++) {
      const yp = ypi + j / (lines * sublines);
      const route = [];
      for (let k = 0; k < xdivisions; k++) {
        const xp = k / (xdivisions - 1);
        const origin = offset([xp, ypi]);
        const target = offset([xp, yp]);
        const v = f(target);
        const p = [origin[0], mix(origin[1], target[1], v)];
        route.push(project(p));
      }
      route.push(route[route.length - 1]);
      routes.push(route);
    }
  }

  return `
  <g stroke="black" stroke-width="${strokeWidth}" fill="none">
  ${routes
    .map((r) => {
      return `<path opacity="${opacity}" d="${svgPathDataRouteCurve(r)}" />`;
    })
    .join("\n")}
  </g>
  `;
}

const Main = ({
  innerCanvasRef,
  block,
  attributesRef,
  width,
  height,
  mod1,
  mod2,
  mod3,
  mod4,
}) => {
  const variables = useVariables({ block, mod1, mod2, mod3, mod4 });
  useAttributes(attributesRef, variables);

  const svgBody = useMemo(() => renderSVG({ variables }), [variables]);

  const imgSrc = useMemo(
    () =>
      "data:image/svg+xml;base64," +
      btoa(
        `
        <svg xmlns="http://www.w3.org/2000/svg" width="${
          width * 2
        }px" height="${
          height * 2
        }px" style="background:white" viewBox="0 0 200 200">` +
          svgBody +
          "</svg>"
      ),
    [svgBody, width, height]
  );

  if (DEBUG_SVG) {
    return <img src={imgSrc} />;
  }

  return (
    <Surface width={width} height={height} ref={innerCanvasRef}>
      <Post>{imgSrc}</Post>
    </Surface>
  );
};

const Post = ({ children }) => {
  const time = useTime();
  return (
    <Node
      shader={shaders.main}
      uniforms={{
        t: children,
        time,
        resolution: Uniform.Resolution,
      }}
    />
  );
};

function useVariables({ block, mod1, mod2 }) {
  const stats = useStats({ block });

  // then, algos that also needs the mods
  return useMemo(() => {
    const mainColor = pickColor(mod1);
    const rng = new MersenneTwister(parseInt(block.hash.slice(0, 16), 16));
    return {
      lines: 80,
      mainColor,
      pad: 50 * mod2,
      seed: rng.random(),
    };
  }, [stats, block, mod1, mod2]);
}

function useAttributes(attributesRef, variables) {
  useEffect(() => {
    attributesRef.current = () => {
      return {
        attributes: [],
      };
    };
  }, [variables]);
}

const shaders = Shaders.create({
  main: {
    frag: GLSL`
    precision highp float;
    varying vec2 uv;
    uniform float time;
    uniform vec2 resolution;
    uniform sampler2D t;

    vec3 palette(float t,vec3 a,vec3 b,vec3 c,vec3 d){
      return a+b*cos(6.28318*(c*t+d));
    }
    vec3 pal(float t){
      return mix(
        vec3(1.0, 1.0, 1.0),
        mix(
          vec3(0.1, 0.1, 0.3),
          vec3(0.6, 0.0, 0.0),
          smoothstep(0.13, 0.12, t)),
        smoothstep(0.99, 0.96, t) + 0.2 * cos(4. * time)
      );
      /*
      return palette(
        t,
        vec3(0.5),
        vec3(0.5),
        vec3(1.0),
        vec3(0.1, 0.01, 0.1)
      );
      */
    }
    
    void main() {
      vec2 ratio = resolution / min(resolution.x, resolution.y);
      vec2 p = 0.5 + (uv - 0.5) * ratio;
      vec3 c = pal(texture2D(t, p).r);
      // c = pal(uv.x);
      gl_FragColor = vec4(c, 1.0);
    }
  `,
  },
});

export default Main;
