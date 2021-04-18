import React from "react";
import ReactDOM from "react-dom";
import { Shaders, Node, GLSL } from "gl-react";
import { Surface } from "gl-react-dom";
import useDimensions from "react-cool-dimensions";
import { useControls, button } from "leva";
import "react-toastify/dist/ReactToastify.css";

function toFloatGLSL(n) {
  const s = String(n);
  if (!s.includes(".")) return s + ".";
  return s;
}
function toVec3(v) {
  if (v.every((c) => c === v[0])) return `vec3(${toFloatGLSL(v[0])})`;
  return `vec3(${v.map(toFloatGLSL).join(",")})`;
}
function glslSnippet({ a, b, c, d }) {
  return `vec3 palette(float t,vec3 a,vec3 b,vec3 c,vec3 d){return a+b*cos(6.28318*(c*t+d));}
vec3 pal(float t){return palette(t,${toVec3(a)},${toVec3(b)},${toVec3(
    c
  )},${toVec3(d)});}
// Usage: gl_FragColor = vec4(pal(uv.x), 1.0);`;
}
function jsSnippet({ a, b, c, d }) {
  return `function palette(t,a,b,c,d){return a.map((v,i)=>v+b[i]*Math.cos(6.28318*(c[i]*t+d[i])))}
function pal(t){return palette(t,[${a.join(",")}],[${b.join(",")}],[${c.join(
    ","
  )}],[${d.join(",")}])}
function palRGB(t){return "rgb("+pal(t).map(v=>Math.floor(v*255)).join(",")+")"}
// Usage: pal(0.0) => [...] | palRGB(0.0) => "rgb(...)"`;
}

const shaders = Shaders.create({
  node: {
    frag: GLSL`
  precision highp float;
  varying vec2 uv;
  uniform vec3 a, b, c, d;

  vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
    return a + b*cos( 6.28318*(c*t+d) );
  }
  vec3 color (float t) {
    return palette(t,a,b,c,d);
  }
  
  vec3 simple (vec2 p) {
    return color(p.x);
  }
  vec3 rounded (vec2 p) {
    float yf = -0.1 + p.y * 5.;
    float y = floor(yf);
    float m = pow(2.0, 1.0 + y);
    float xf = p.x * m;
    float fxf = fract(xf);
    float marg = 0.004 * m;
    return color(floor(xf) / m) * step(fract(yf), 0.8) * step(marg, fxf) * step(fxf, 1.0 - marg);
  }
  vec3 harmonies (vec2 p) {
    float xf = p.x * 16.;
    float xfloor = floor(xf);
    float xfract = fract(xf);
    float marg = 0.1;
    float t = xfloor + p.y;
    return color(t) * step(marg, xfract) * step(xfract, 1.0 - marg);
  }

  // layers
  float margin = 0.05;
  float layers = 3.2;
  float l;
  float yp () {
    return layers * uv.y - l;
  }
  float xp () {
    float m = margin / layers;
    return (1.0 + 2.0 * m) * uv.x - m;
  }
  float layer () {
    float y = yp();
    float x = xp();
    l += 1. + margin;
    return step(0.0, y) * step(y, 1.0) * step(0.0, x) * step(x, 1.0);
  }
  vec2 lp () {
    return vec2(xp(), yp());
  }
  void main () {
    l = margin;
    vec3 c = vec3(0.0);
    c += harmonies(lp()) * layer();
    c += rounded(lp()) * layer();
    c += simple(lp()) * layer();
    gl_FragColor = vec4(c, 1.0);
  }
    `,
  },
});

const Scene = ({ a, b, c, d }) => {
  return <Node shader={shaders.node} uniforms={{ a, b, c, d }} />;
};

const viewportStyle = {
  position: "absolute",
  width: "100vw",
  height: "100vh",
  display: "flex",
  flexDirection: "column",
};
const Code = ({ children }) => (
  <textarea onClick={(e) => e.target.select()} value={children} readOnly />
);
function fromColor(a) {
  return [a.r, a.g, a.b].map((v) => v / 255);
}
const Rendering = () => {
  const { width, height, observe } = useDimensions({});
  const [{ a, b, c, d0, d1, d2 }, set] = useControls(() => ({
    a: { r: 128, g: 128, b: 128 },
    b: { r: 128, g: 128, b: 128 },
    c: { value: [1, 1, 1], lock: true },
    d0: { value: 0.0, step: 0.01, min: 0.0, max: 1.0 },
    d1: { value: 0.33, step: 0.01, min: 0.0, max: 1.0 },
    d2: { value: 0.66, step: 0.01, min: 0.0, max: 1.0 },
    Randomize: button(() => {
      set({
        a,
        b,
        c,
        d0: Math.random(),
        d1: Math.random(),
        d2: Math.random(),
      });
    }),
  }));
  const props = {
    a: fromColor(a),
    b: fromColor(b),
    c,
    d: [d0, d1, d2],
  };
  if (!height) {
    return <div ref={observe} style={viewportStyle} />;
  }
  return (
    <div ref={observe} style={viewportStyle}>
      <div style={{ padding: "0 2vw", height: "50vh", overflow: "auto" }}>
        <h2>GLSL</h2>
        <Code>{glslSnippet(props)}</Code>
        <h2>JavaScript</h2>
        <Code>{jsSnippet(props)}</Code>
        <footer>
          Must read:
          https://www.iquilezles.org/www/articles/palettes/palettes.htm
        </footer>
      </div>
      <Surface width={width} height={height / 2.0}>
        <Scene {...props} />
      </Surface>
    </div>
  );
};

const Main = () => {
  return <Rendering />;
};

ReactDOM.render(<Main />, document.getElementById("main"));
