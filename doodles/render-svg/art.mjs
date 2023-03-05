
const gainShader = `
float gain = smoothstep(0.5, 1.0, abs(cos(${Math.PI}*(length(p-0.5)+time))));
`;

const vert = `precision mediump float;attribute vec2 p;varying vec2 uv;void main(){uv=p;gl_Position=vec4(2.*p-1.,0,1);}`

const shaders = {
  paper: {
    vert,
    frag: `precision highp float;
    varying vec2 uv;
    uniform vec2 resolution;
    uniform float grain, seed;

void pR(inout vec2 p, float a) {
  p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
float hash(float p) {
  p = fract(p * .1031);
  p *= p + 33.33;
  p *= p + p;
  return fract(p);
}
float hash(vec2 p) {
  vec3 p3  = fract(vec3(p.xyx) * .1031);
  p3 += dot(p3, p3.yzx + 33.33);
  return fract((p3.x + p3.y) * p3.z);
}
float noise(float x) {
  float i = floor(x);
  float f = fract(x);
  float u = f * f * (3.0 - 2.0 * f);
  return mix(hash(i), hash(i + 1.0), u);
}
float noise(vec2 x) {
  vec2 i = floor(x);
  vec2 f = fract(x);
  float a = hash(i);
  float b = hash(i + vec2(1.0, 0.0));
  float c = hash(i + vec2(0.0, 1.0));
  float d = hash(i + vec2(1.0, 1.0));
  vec2 u = f * f * (3.0 - 2.0 * f);
  return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}
const mat2 m2 = mat2( 0.4,  0.7, -0.7,  0.4 );
float fbm( in vec2 x ) {
  float f = 2.0;
  float s = 0.55;
  float a = 0.0;
  float b = 0.5;
  for( int i=0; i<3; i++ ) {
    float n = noise(x);
    a += b * n;
    b *= s;
    x = f * x;
  }
  return a;
}
float paper(vec2 p, float z, float seed) {
  pR(p, seed);
  p += seed;
  float n = 0.7 * smoothstep(-0.1, 0.2, abs(fbm(-3.0 * p * z - seed)-0.5)-0.01) +
  0.3 * smoothstep(0.0, 0.1, abs(fbm(-6.0 * p * z + seed)-0.5)-0.01);
  return n;
}

void main () {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 p = 0.5 + (uv - 0.5) * ratio;
  float t = paper(p, grain, seed);
  gl_FragColor = vec4(t, t, t, 1.0);
}`,
  },
  light: {
    vert,
    frag: `
    precision highp float;
    varying vec2 uv;
    uniform vec3 baseColor, background;
    uniform float grainAmp, lighting, time;
    uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight;
    uniform sampler2D t, paper;
    vec3 pal(float t, vec3 c1, vec3 c2){
      float m = smoothstep(0.3, 0.0, t);
      return mix(
        vec3(1.0, 1.0, 1.0),
        mix(c1, c2, m),
        smoothstep(1.0, 0.5, t)
      );
    } 
    void main() {
      vec2 p = uv;
      ${gainShader}
      vec4 g = texture2D(paper, p);
      float grain = g.r;
      vec4 v = texture2D(t, p);
      vec3 c1 = pal(v.r, primary, primaryHighlight);
      vec3 c2 = pal(v.g, secondary, secondaryHighlight);
      vec3 c =
        min(vec3(1.), c1 * c2 * (1. + lighting * gain)) +
        grainAmp * /*developed by @greweb*/
        mix(1.0, 0.5, step(0.5, grain)) *
        (0.5 - grain) +
        baseColor;
      gl_FragColor = vec4(c, 1.0);
    }
  `,
  },
  dark: {
    vert,
    frag: `
    precision highp float;
    varying vec2 uv;
    uniform vec3 baseColor, background;
    uniform float grainAmp, lighting, time;
    uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight;
    uniform sampler2D t, paper;
    vec3 pal(float t, vec3 c1, vec3 c2) {
      float m = smoothstep(0.3, 0.0, t);
      return mix(
        vec3(0.0),
        mix(c1, c2, m),
        smoothstep(1.0, 0.5, t)
      );
    } 
    void main() {
      vec2 p = uv;
      ${gainShader}
      vec4 g = texture2D(paper, p);
      float grain = g.r;
      vec4 v = texture2D(t, p);
      vec3 c1 = pal(v.r, primary, primaryHighlight);
      vec3 c2 = pal(v.g, secondary, secondaryHighlight);
      vec3 c =
      (c1 + c2) * (1. + lighting * gain) +
        grainAmp * grain +/*developed by @greweb*/
        baseColor +
        background * smoothstep(0.5, 1.0, v.r * v.g);
      gl_FragColor = vec4(c, 1.0);
    }
  `,
  },
};


const baseColors = {
  none: [0, 0, 0],
  standard: [-0.003, -0.006, -0.01]
}

const papers = {
  white: [1, 1, 1],
  black: [0.1, 0.1, 0.1],
  darkBlue: [0.1, 0.1, 0.2],
  grey: [0.69, 0.72, 0.71]
}

const defaultOpts = {
  grain: 100,
  seed: 1000 * Math.random()
}

const inks = {
  redGel: {
    main: [0.75, 0.45, 0.55],
    highlight: [0.85, 0.5, 0.65],
  },
  orangeGel: {
    main: [0.7, 0.45, 0.2],
    highlight: [0.9, 0.55, 0.3],
  },
  blueGel: {
    main: [0.2, 0.55, 1],
    highlight: [0.3, 0.55, 1],
  },
  greenGel: {
    main: [0.0, 0.7, 0.65],
    highlight: [0.1, 0.8, 0.75],
  },
  goldGel: {
    main: [0.85, 0.7, 0.25],
    highlight: [1, 0.9, 0.55],
  },
  whiteGel: {
    main: [0.9, 0.9, 0.9],
    highlight: [1, 1, 1],
  },
  black: {
    main: [0.1, 0.1, 0.1],
    highlight: [0, 0, 0],
  },
  imperialPurple: {
    main: [0.3, 0.0, 0.6],
    highlight: [0.15, 0.1, 0.2],
  },
  sherwoodGreen: {
    main: [0.2, 0.45, 0.25],
    highlight: [0.1, 0.3, 0.1],
  },
  evergreen: {
    main: [0.3, 0.4, 0.2],
    highlight: [0.15, 0.2, 0.1],
  },
  softMint: {
    main: [0.2, 0.88, 0.8],
    highlight: [0.1, 0.7, 0.6],
  },
  indigo: {
    main: [0.4, 0.5, 0.65],
    highlight: [0.2, 0.3, 0.4],
  },
  auroraBorealis: {
    main: [0.0, 0.6, 0.6],
    highlight: [0.0, 0.3, 0.4],
  },
  pumpkin: {
    main: [1, 0.5, 0.2],
    highlight: [0.9, 0.3, 0.0],
  },
  pink: {
    main: [1.0, 0.32, 0.46],
    highlight: [0.9, 0.38, 0.3],
  },
  hopePink: {
    main: [1.0, 0.4, 0.75],
    highlight: [0.9, 0.2, 0.6],
  },
  amber: {
    main: [1.0, 0.78, 0.28],
    highlight: [1.0, 0.5, 0.0],
  },
  poppyRed: {
    main: [0.9, 0.1, 0.1],
    highlight: [0.5, 0.0, 0.1],
  },
  fireAndIce: {
    main: [0 / 255, 190 / 255, 220 / 255],
    highlight: [0 / 255, 100 / 255, 120 / 255],

  },
  bloodyBrexit: {
    main: [0.02, 0.12, 0.42],
    highlight: [0.18, 0.0, 0.2],
  },

}

export async function art({
  regl,
  frameTime,
  onFrame,
  image,
  opts,
  width,
  height

}) {
  const { grain, seed } = { ...defaultOpts, ...opts }

  const paperFBO = regl.framebuffer(width, height);

  const renderPaper = regl({
    ...shaders.paper,
    framebuffer: paperFBO,
    attributes: {
      p: [-2, 0, 0, -2, 2, 2],
    },
    count: 3,
    uniforms: {
      seed: regl.prop("seed"),
      grain: regl.prop("grain"),
      resolution: ({ viewportWidth, viewportHeight }) => [
        viewportWidth,
        viewportHeight,
      ],
    },
  });

  const inputTexture = regl.texture({
    data: image,
    mag: "linear",
    min: "linear",
  })

  const paper = process.env.PAPER
  const shader = paper === "white" ? shaders.light : shaders.dark
  const background = papers[paper];
  const lighting = parseFloat(process.env.LIGHTING || "0")
  const grainAmp = parseFloat(process.env.GRAIN || "0")
  const baseColor = baseColors[process.env.BASE_COLOR || "none"]

  const primaryInk = inks[process.env.PRIMARY]
  if (!primaryInk) {
    throw new Error("Invalid primary ink " + process.env.PRIMARY)
  }
  const secondaryInk = inks[process.env.SECONDARY || process.env.PRIMARY]
  if (!secondaryInk) {
    throw new Error("Invalid secondary ink " + process.env.SECONDARY)
  }
  const primary = primaryInk.main
  const primaryHighlight = primaryInk.highlight
  const secondary = secondaryInk.main
  const secondaryHighlight = secondaryInk.highlight

  const renderMain = regl({
    ...shader,
    attributes: {
      p: [-2, 0, 0, -2, 2, 2],
    },
    count: 3,
    uniforms: {
      t: regl.prop("t"),
      paper: regl.prop("paper"),
      time: regl.prop("time"),
      grainAmp,
      lighting,
      background,
      baseColor,
      primary,
      primaryHighlight,
      secondary,
      secondaryHighlight,
      resolution: ({ viewportWidth, viewportHeight }) => [
        viewportWidth,
        viewportHeight,
      ],
    },
  });

  let t = 0;
  regl.frame((v) => {
    regl.clear({
      depth: 1,
      color: [0, 0, 0, 1],
    });
    const time = frameTime(t, v);

    renderPaper({ seed, grain, time });

    renderMain({
      t: inputTexture,
      paper: paperFBO,
      time,
    });

    // render({ time });
    onFrame(t, time);
    t++;
  });

  return () => regl.destroy();
}
