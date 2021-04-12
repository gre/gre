import React, { useEffect, useState, useMemo } from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import MersenneTwister from "mersenne-twister";

// Notes
// - i have to use WebGL2 otherwise v1 is just not performant enough. it means this blockstyle don't work in Safari. hopefully safari support WebGL2 later this year (it's experimental right now)
// - for mobile, the shader is pretty challenging so i decided to downscale to 128px. The mobile is essentially a different experience (pixelated one) which still looks pretty cool imo.
// - you can inject "highQuality" props to true only when generating the snapshot to get a very good quality one (anti aliasing). doing it on real time controls is not recommended because perf.

export const styleMetadata = {
  name: "CryptoAliens: Genesis",
  description:
    "CryptoAliens: Genesis establishes the first embryonic species. Which CryptoAliens are you going to chose? Each creature gets born on an Ethereum block that nourishes its shape: transactions, ETH value transferred, gas,... their unique skin comes from Mandelglitch's BlockStyle with the same rarity scheme. See greweb.me/cryptoaliens",
  image: "",
  creator_name: "greweb",

  // debug_noRefresh: 1, // debug
  options: {
    // comment seed when going production!
    // seed: -4, // used for internal debug
    // highQuality: 0, // used for debug
    mod1: 0.5,
    mod2: 0.5,
    mod3: 0.5,
    mod4: 0.5,
  },
};

//// MAIN COMPOSITION PART ////

const CustomStyle = (props) => {
  // prettier-ignore
  const { block, attributesRef, mod1, mod2, mod3, mod4, highQuality, width, height } = props;
  // prettier-ignore
  const { kg, bones, theme, background, s1, s2, s3, s4, s5, s6, s7, s8, heavy, head, bonesK, armsLen, armsSpread, armsCenter, armsEndW, dateText, blockNumber } =
    useBlockDerivedData(block, mod1, mod2, mod3, mod4);

  useAttributesSync(attributesRef, kg, bones, theme);

  const resolutionCap = 128;
  const maxDim = Math.max(width, height);
  const max = Math.min(resolutionCap, maxDim);
  const w = Math.round((max * width) / maxDim);
  const h = Math.round((max * height) / maxDim);

  return (
    <LiveTV
      text={
        <FrameTextCached
          kg={kg}
          bones={bones}
          blockNumber={blockNumber}
          dateText={dateText}
          width={width}
          height={height}
        />
      }
      background={background}
    >
      <Scene
        width={w}
        height={h}
        t={
          <MandelglitchCached
            block={block}
            mod1={mod1}
            mod2={mod2}
            mod3={mod3}
            dim={max}
          />
        }
        mod1={mod1}
        mod2={mod2}
        mod3={mod3}
        mod4={mod4}
        background={background}
        s1={s1}
        s2={s2}
        s3={s3}
        s4={s4}
        s5={s5}
        s6={s6}
        s7={s7}
        s8={s8}
        heavy={heavy}
        head={head}
        bonesK={bonesK}
        armsLen={armsLen}
        armsSpread={armsSpread}
        armsCenter={armsCenter}
        armsEndW={armsEndW}
        highQuality={highQuality}
      />
    </LiveTV>
  );
};

function useAttributesSync(attributesRef, kg, bones, theme) {
  useEffect(() => {
    const attributes = [
      {
        trait_type: "Theme",
        value: theme,
      },
      {
        trait_type: "Weight",
        value: `${kg} kg`,
      },
      {
        trait_type: "Bones",
        value: bones,
      },
    ];
    attributesRef.current = () => ({
      attributes,
    });
  }, [attributesRef, kg, bones, theme]);
}

// LOGIC

var dayMs = 1000 * 60 * 60 * 24,
  J1970 = 2440588,
  J2000 = 2451545;
function toJulian(date) {
  return date.valueOf() / dayMs - 0.5 + J1970;
}
function toDays(date) {
  return toJulian(date) - J2000;
}

const TX_UPPER_BOUND = 500;
const TX_MIN_THRESHOLD = 100 * Math.pow(10, 18);
const TX_LIGHT_VALUE = 10 * Math.pow(10, 18);

const safeParseInt = (a) => {
  const v = parseInt(a);
  if (isNaN(v) || !isFinite(a)) return 0;
  return v;
};

function useBlockDerivedData(block) {
  return useMemo(() => {
    let { hash, number, timestamp, transactions } = block;
    const blockNumber = parseInt(number);
    const txsCount = transactions.length;
    const days = toDays(timestamp * 1000);
    const remaining = days - Math.floor(days); // 0 at 13:00, 0.5 at 01:00, 1 at 12:59
    const isDay = remaining < 0.3 || remaining > 0.7; // ~ between 6am and 10pm UTC
    const isSpecialBlock = blockNumber % 100 === 0;
    const txCountHeavyFactor = Math.pow(
      Math.min(TX_UPPER_BOUND, txsCount) / TX_UPPER_BOUND,
      2.0
    );
    let expectionalTxAmountFactor = 0;
    let txCountLightFactor = 0;
    const allGas = transactions.map(
      (t) => safeParseInt(t.gas) * safeParseInt(t.gasPrice)
    );
    const allNonZeroValues = transactions
      .map((t) => safeParseInt(t.value))
      .filter(Boolean);
    allGas.sort((a, b) => a - b);
    allNonZeroValues.sort((a, b) => a - b);
    const gasRatio =
      allGas.length === 0
        ? 0
        : allNonZeroValues.length === 0
        ? 1
        : allGas.reduce((acc, n) => acc + n, 0) /
          allNonZeroValues.reduce((acc, n) => acc + n, 0);
    if (allNonZeroValues.length > 50) {
      let valueMax = allNonZeroValues[allNonZeroValues.length - 1];
      if (valueMax > TX_MIN_THRESHOLD) {
        expectionalTxAmountFactor = Math.pow(
          (valueMax - TX_MIN_THRESHOLD) / valueMax,
          8
        );
      }
      if (valueMax < TX_LIGHT_VALUE) {
        txCountLightFactor = Math.pow(1 - valueMax / TX_LIGHT_VALUE, 0.5);
      }
    }
    const rngSeed = parseInt(hash.slice(0, 16), 16); // when seed is not provided, it means we're in "production" and the seed is actually the block hash

    const rng = new MersenneTwister(rngSeed);
    let s1 = rng.random();
    let s2 = rng.random();
    let s3 = rng.random();
    let s4 = rng.random();
    let s5 = rng.random(); // general height of base
    let s6 = rng.random(); // height
    let s7 = rng.random();
    let s8 = rng.random();
    s4 -= txCountLightFactor;
    let r = 0.1 + 0.8 * rng.random(); // random balance between # of arms vs length
    let armsLen = Math.floor(2 + Math.min(0.1 * txsCount * r, 30));
    let armsSpread = Math.min(1, 0.01 + 0.005 * txsCount * (1 - r));
    let armsCenter = 0.1 + 0.8 * rng.random();
    // make more or less bigger bones joint
    let bonesK = Math.max(0.005, 0.005 + 0.2 * Math.pow(rng.random(), 3));
    let armsEndW = 0.2 * Math.pow(gasRatio, 0.5);

    let heavy = txCountHeavyFactor;
    let head =
      0.1 * (Math.pow(rng.random(), 2.0) - 0.2) +
      0.2 * Math.pow(rng.random(), 8.0) +
      0.8 * expectionalTxAmountFactor;

    let theme, background;
    if (isSpecialBlock) {
      background = [rng.random() + heavy, rng.random() + head, rng.random()];
      theme = "anomaly";
    } else if (isDay) {
      background = [0.92, 0.93, 0.96];
      theme = "day";
    } else {
      theme = "night";
      background = [0.1, 0.11, 0.13];
    }

    let bones = Math.floor(10 + 10 * armsLen * armsSpread);
    let kgN =
      0.1 +
      0.1 * bonesK +
      2 * s1 * s1 * s1 + // incr
      s5 + // initial base
      0.1 * armsLen * armsSpread +
      40 * armsEndW * armsSpread +
      10 * heavy +
      20 * head;
    kgN *= 1 - 0.8 * txCountLightFactor;
    kgN = Math.floor(kgN * 100) / 100;
    let kg = String(kgN);
    let i = kg.indexOf(".");
    if (i > 0) {
      // in case JS have rare imprecision, we make sure to chunk the unecessary bits
      kg = kg.slice(0, i + 3);
    }

    const dateText =
      new Date(timestamp * 1000).toISOString().slice(0, 16).replace("T", " ") +
      " UTC";

    return {
      dateText,
      kg,
      bones,
      theme,
      background,
      s1,
      s2,
      s3,
      s4,
      s5,
      s6,
      s7,
      s8,
      heavy,
      head,
      bonesK,
      armsLen,
      armsSpread,
      armsCenter,
      armsEndW,
      blockNumber,
    };
  }, [block]);
}

/// postprocessing layer ///

// inspired from https://www.shadertoy.com/view/llsfD8
const liveTVShaders = Shaders.create({
  tv: {
    frag: GLSL`
precision highp float;
varying vec2 uv;

uniform float time;
uniform vec2 resolution;
uniform vec3 background;
uniform sampler2D children, text;

float getCenterDistance(vec2 coord) {
  return distance(coord, vec2(0.5)) * 2.0;
}

vec2 videoRollCoords(vec2 coord, float speed, float amount, float period) {
  float scrolling = step(0.0, sin(time * 6.28 / period) * 0.5 + (-0.5 + amount));
  coord.y -= time * speed * scrolling;
	coord = fract(coord);
  return coord;
}

vec2 bulgeCoords(vec2 coord, vec2 sourceCoord, float bulgeAmount) {
    float centerDist = getCenterDistance(sourceCoord);
    coord.xy -= vec2(0.5);
    coord.xy *= 1.0 + centerDist * bulgeAmount;
    coord.xy *= 1.0 - bulgeAmount;
    coord.xy += vec2(0.5);
    return coord;
}

vec4 sampleRGBVignette(sampler2D source, vec2 coord, vec2 sourceCoord, float amount, float power) {
    float centerDist = getCenterDistance(sourceCoord);
    centerDist = pow(centerDist, power);
    vec2 sampleCoord = coord;
    vec4 outputColor = texture2D(source, sampleCoord);
    sampleCoord = bulgeCoords(coord, sourceCoord, amount * centerDist);
    outputColor.g = texture2D(source, sampleCoord).g;
    sampleCoord = bulgeCoords(coord, sourceCoord, amount * 2.0 * centerDist);
    outputColor.b = texture2D(source, sampleCoord).b;
    return outputColor;
}

vec4 applyScanLines(vec4 color, vec2 coord, float number, float amount, float power, float drift) {
    coord.y += time * drift;
    float d = 0.5 + 0.5 * cos(coord.y * 6.28 * number);
    d = amount * pow(d, power);
    color.rgb = mix(color.rgb, background, d);
    return color;
}

vec4 applyVignette(vec4 color, vec2 sourceCoord, float amount, float scale, float power) {
    float centerDist = getCenterDistance(sourceCoord);
    float d = centerDist / scale;
    d = pow(d, power);
    d = amount * min(1.0, d);
    color.rgb = mix(color.rgb, background, d);
    return color;
}

void main () {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  float borderDist = ratio.y * min(1. - uv.y, uv.y); // min(min(1. - uv.x, uv.x), ratio.y * min(1. - uv.y, uv.y));
  float div = 0.05 - 0.03 * cos(0.5 * time);
  float textC = texture2D(text, uv).r;
  vec2 sourceCoord = uv;
  vec2 sampleCoord = sourceCoord;
  sampleCoord = videoRollCoords(sampleCoord, 10., 0.004, 8.0);
  sampleCoord = bulgeCoords(sampleCoord, sourceCoord, 0.1);
  vec4 outputColor = sampleRGBVignette(children, sampleCoord, sourceCoord, div, 3.0);
  outputColor = mix(outputColor, vec4(background, 1.0), 0.8 * smoothstep(0.06, 0.03, borderDist));
  outputColor = vec4(mix(outputColor.rgb, 1. - background, textC), 1.0);
  float vignetteAmount = 0.9 + 0.1 * cos(24. * time);
  outputColor = applyVignette(outputColor, sourceCoord, vignetteAmount, 1.5, 3.0);
  vec2 scanLineCoord = bulgeCoords(sourceCoord, sourceCoord, 0.5);
  outputColor = applyScanLines(outputColor, scanLineCoord, 150.0, div, 1.0, 0.05);
  gl_FragColor = outputColor;
}
      `,
  },
});

function LiveTV({ children, text, background }) {
  const time = useTime();
  return (
    <Node
      shader={liveTVShaders.tv}
      uniformsOptions={{
        children: {
          interpolation: "nearest",
        },
      }}
      uniforms={{
        resolution: Uniform.Resolution,
        children,
        text,
        background,
        time: Math.max(0, time - 8),
      }}
    />
  );
}

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

/// canvas used to draw in postprocessing ///
function FrameText({ blockNumber, dateText, width, height, kg, bones }) {
  const onCanvasRef = (canvas) => {
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    const w = ctx.canvas.width;
    const h = ctx.canvas.height;
    const pad = Math.round(w * 0.01);
    const padX = Math.round(w * 0.02);
    const fontS = Math.floor(0.19 * w) / 10;
    const fontSize = `${fontS}px`;
    ctx.save();
    ctx.fillStyle = "#000";
    ctx.fillRect(0, 0, w, h);
    ctx.font = "bold " + fontSize + " monospace";
    ctx.textBaseline = "top";
    ctx.fillStyle = "#fff";
    ctx.fillText(`CryptoAliens specimen #${blockNumber}`, padX, pad);
    ctx.textBaseline = "bottom";
    ctx.textAlign = "right";
    ctx.font = fontSize + " monospace";
    ctx.fillText(
      `born ${dateText}, ${kg} kg, ${bones} bones`,
      w - padX,
      h - pad
    );
    ctx.restore();
  };
  const dpr = window.devicePixelRatio || 1;
  return (
    <canvas
      ref={onCanvasRef}
      width={String(width * dpr)}
      height={String(height * dpr)}
    />
  );
}
const FrameTextCached = React.memo(FrameText);

/// SCENE PART ///

const sceneShaders = Shaders.create({
  scene: {
    frag: GLSL`
precision highp float;
varying vec2 uv;

uniform vec2 resolution;

uniform vec3 background;
uniform float s1,s2,s3,s4,s5,s6,s7,s8;
uniform float mod1,mod2,mod3,mod4;
uniform sampler2D t;
uniform float heavy;
uniform float head;
uniform float bonesK;
uniform float armsCenter,armsSpread,armsEndW;
uniform int armsLen;
uniform bool highQuality;

#define PI ${Math.PI}

#define HIT vec4
HIT map (vec3 p);
vec3 shade (HIT m, vec3 p);
vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir);

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
void pR(inout vec2 p, float a) {
  p = cos(a) * p + sin(a) * vec2(p.y, -p.x);
}

float fOpUnionSoft(float r, float a, float b) {
  float e = max(r - abs(a - b), 0.);
  return min(a, b) - e*e*0.25/r;
}
float sdRoundedCylinder( vec3 p, float ra, float rb, float h ) {
  vec2 d = vec2( length(p.xz)-2.0*ra+rb, abs(p.y) - h );
  return min(max(d.x,d.y),0.0) + length(max(d,0.0)) - rb;
}
float sdSegment (in vec3 p, in float L, in float R) {
  p.y -= min(L, max(0.0, p.y));
  return length(p) - R;
}

// https://www.iquilezles.org/www/articles/rmshadows/rmshadows.htm
float softshadow( in vec3 ro, in vec3 rd, float mint, float maxt, float k ) {
  float res = 1.0;
  float ph = 1e20;
  float t = mint;
  for (int i=0; i<30; i++) {
    float h = map(ro + rd*t).x;
    if (t>=maxt) break;
    if( h<0.001) return 0.0;
    float y = h*h/(2.0*ph);
    float d = sqrt(h*h-y*y);
    res = min( res, k*d/max(0.0,t-y) );
    ph = h;
    t += h;
  }
  return res;
}

vec3 normal (in vec3 p) {
  vec3 eps = vec3(0.001, 0.0, 0.0);
  return normalize(vec3(
    map(p+eps.xyy).x-map(p-eps.xyy).x,
    map(p+eps.yxy).x-map(p-eps.yxy).x,
    map(p+eps.yyx).x-map(p-eps.yyx).x
  ));
}

float diffuse(vec3 p, vec3 n, vec3 lpos) {
  vec3 l = normalize(lpos-p);
  float dif = clamp(dot(n, l), 0.01, 1.);
  return dif;
}

HIT marcher (inout vec3 p, vec3 dir) {
  // raymarching perf technique from https://www.shadertoy.com/view/XsyGWV
  HIT hit = HIT(0.);
  float precis = 0.0001;
  float t = 0.;
  for (int i=0; i<120; i++) {
    HIT h = map(p + t * dir);
    precis = t*0.0002;
    float rl = max(t*.02, 1.);
    t += h.x * rl;
    if (abs(h.x) < precis || p.z > 20.) {
      hit = h;
      break;
    }
  }
  p += t * dir;
  return hit;
}

HIT opU (HIT a, HIT b) {
  if (a.x < b.x) return a;
  return b;
}

float specularStrength (float m) {
  if (m<1.) return .0;
  return 0.8;
}

float specular (vec3 n, float m, vec3 ldir, vec3 dir, float p) {
  return specularStrength(m) * pow(max(dot(dir, reflect(ldir, n)), 0.0), p);
}

vec3 light (float id) {
  return 0.6 * palette(
    id + s1,
    vec3(0.8),
    vec3(0.5),
    vec3(1.0),
    vec3(0.6, 0.8, 0.)
  );
}

vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir) {
  vec3 l, ldir;
  vec3 c = vec3(0.);
  l = vec3(-4., 4., -2.);
  ldir = normalize(l - p);
  c +=
  light(0.0) * (
    // ambient
    0.1
    // diffuse
    + shade(hit, p)
      * (.5 + .5 * diffuse(p, n, l)) // half lambert
    + specular(n, hit.y, ldir, dir, 20.)
  );
  l = vec3(4., 3., -2.);
  ldir = normalize(l - p);
  c +=
  light(0.5) * (
  // ambient
  0.1
  // diffuse
  + shade(hit, p)
    * (.5 + .5 * diffuse(p, n, l)) // half lambert
    * (0.6 + 0.4 * softshadow(p, ldir, 0.1, 16., 50.))
  + specular(n, hit.y, ldir, dir, 40.)
  );
  // adding ambient
  l = vec3(0., 6., -5.);
  ldir = normalize(l - p);
  c += vec3(.2) * (0.05 + shade(hit, p) * diffuse(p, n, l));
  return c;
}

vec3 shade (HIT hit, vec3 g) {
  float m = hit.y;
  if (m < 1.) {
    return background;
  }
  vec2 p = hit.zw;
  vec2 tUV = fract(p);
  return palette(
    s6 + mod4 * s5 * texture2D(t, tUV).r,
    vec3(0.5),
    vec3(0.5),
    vec3(1.0),
    vec3(0.6, 0.4, 0.3)
  );
}

float arm (inout vec3 p, float index, float w, float h) {
  float s = sdSegment(p, h, w);
  float base1 = 305.53 * s1 + 77.21 * index;
  float base2 = 403.53 * s2 + 69.71 * index;
  for (int i = 0; i < 32; i++) {
    if (i >= armsLen) break;
    float fi = float(i);
    float ss1 = fract(base1 + 9.412 * fi);
    float ss2 = fract(base2 + 8.823 * fi);
    pR(p.xy, 8. * s4 * (ss2-.5));
    pR(p.xz, 6. * s5 * (ss1-.5));
    s = fOpUnionSoft(bonesK, s, sdSegment(p, h, w));
    h *= .9;
    w *= .9;
    p.y -= 1.2 * h;
  }
  s = fOpUnionSoft(bonesK + 0.2 * s5, s, length(p) - armsEndW);
  return s;
}

HIT obj (vec3 p) {
  vec2 xy = .5 + vec2(0.5, 1.0) * (p.xz + p.xy) / 2.0;
  // displacement
  p += 0.006 * s3 * s4 * vec3(cos(20. * p.y), cos(20. * p.x), cos(20. * p.x));
  p.y -= 0.1;
  float s = sdRoundedCylinder(p, (0.2 + 0.6 * s3) / 2.0, 0.02, 0.1);
  // random twist
  float twistAmp = 0.2 * pow(s8, 10.0);
  float twistFreq = s7 * 20.0 * p.y;
  p.x += 0.1 * twistAmp * cos(twistFreq);
  p.z += 0.1 * twistAmp * sin(twistFreq);

  float stepR = (s3 - 0.5) * pow(s4, 8.0) + (mod3 - .5);
  float stepR2 = s3 * 7.;
  float w = 0.04 + 0.05 * s3 * s4;
  float h = 0.3 + 0.2 * s5;
  float incr = 0.1 + 0.2 * pow(s1, 3.0);
  float initialL = incr + s5;
  float arms = sdSegment(p, initialL, 0.1);
  p.y -= initialL;
  vec3 q;
  for (float f = 0.0; f<1.0; f+=0.1) {
    pR(p.xy, stepR);
    pR(p.xz, stepR2);
    s = fOpUnionSoft(0.1, s, sdSegment(p, incr, 0.1));
    q = vec3(p.y, -p.x, p.z);
    if (abs(f - armsCenter) < armsSpread) {
      arms = fOpUnionSoft(0.1, arms, arm(q, f, w, h));
    }
    p.y -= incr;
  }
  s = fOpUnionSoft(0.01 + heavy, s, arms);

  if (head > 0.0) {
    pR(q.xy, 100.0 * s2);
    q.y -= head;
    q.y += 0.05 * s5 * cos(30. * s4 * q.x);
    q.z += 0.05 * s5 * cos(30. * s4 * q.x);
    s = fOpUnionSoft(0.2, s, length(q) - head);
  }
  return HIT(s, 2.0, xy);
}

HIT map (vec3 p) {
  HIT s = HIT(min(20. - length(p),p.y), 0.1, 0., 0.);
  return opU(s, obj(p));
}

mat3 lookAt (vec3 ro, vec3 ta) {
  float cr = 0.;
  vec3 ww = normalize( ta - ro );
  vec3 uu = normalize( cross(ww,vec3(sin(cr),cos(cr),0.0) ) );
  vec3 vv =          ( cross(uu,ww));
  return mat3(uu,vv,ww);
}

vec3 scene(vec2 uvP) {
  float amp = 4. + 3. * pow(mod2, 1.5);
  float a = 2. * PI * mod1;
  vec3 origin = vec3(amp * cos(a), 0.5 + 4. * mod2, amp * sin(a));
  vec3 poi = vec3(0.0, 1. + mod2, 0.0);
  vec3 c = vec3(0.);
  vec3 dir = normalize(vec3(uvP - .5, 1.5));
  dir = lookAt(origin, poi) * dir;
  vec3 p = origin;
  HIT hit = marcher(p, dir);
  vec3 n = normal(p);
  c += lighting(hit, p, n, dir);
  // mist
  c = mix(c, background, smoothstep(8.0, 16.0, length(origin - p)));
  return c;
}

void main() {
  vec3 c = vec3(0.);
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;
  c = scene(base);
  if (highQuality) {
    for (float x=-.5; x<=.5; x += 1.) {
      for (float y=-.5; y<=.5; y += 1.) {
        vec2 d = 0.5 * vec2(x,y) / resolution;
        vec2 p = base + d;
        c += scene(p);
      }
    }
    c /= 5.0;
  }
  gl_FragColor = vec4(c, 1.0);
}
  `,
  },
});

function SceneRaw({
  width,
  height,
  t,
  highQuality,
  background,
  mod1,
  mod2,
  mod3,
  mod4,
  // from seed (block hash)
  s1,
  s2,
  s3,
  s4,
  s5,
  s6,
  s7,
  s8,
  // other controls
  heavy,
  head,
  bonesK,
  armsLen,
  armsSpread,
  armsCenter,
  armsEndW,
}) {
  return (
    <Node
      width={width}
      height={height}
      shader={sceneShaders.scene}
      uniforms={{
        resolution: Uniform.Resolution,
        t,
        highQuality: !!highQuality,
        background,
        mod1,
        mod2,
        mod3,
        mod4,
        // from seed (block hash)
        s1,
        s2,
        s3,
        s4,
        s5,
        s6,
        s7,
        s8,
        // other controls
        heavy,
        head,
        bonesK,
        armsLen,
        armsSpread,
        armsCenter,
        armsEndW,
      }}
    />
  );
}
const Scene = React.memo(SceneRaw);

////// MANDEL GLITCH PART //////

const shadersMandelglitch = Shaders.create({
  mandelglitch: {
    frag: GLSL`
  precision highp float;
  varying vec2 uv;

  uniform vec2 resolution;
  uniform float mod2, mod1, mod3;
  uniform float s1, s2, s3, s4, s5, s6, s7, s8, s9;

  const float PI = ${Math.PI};
  void pR(inout vec2 p, float a) {
  	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
  }
  vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
    return a + b*cos( 6.28318*(c*t+d) );
  }
  vec3 pal (float t) {
    return palette(
      t + 0.5 * mod3 * mod3,
      vec3(.85 - .5 * mod3),
      vec3(.5),
      vec3(1.),
      vec3(0.8 + 0.2 * s1, 0.2 * s2, .2)
    );
  }
  float run (vec2 init) {
    float iterations = 2. + 500. * pow(mod2, 2.0);
    vec2 p = init;
    for (float iter = 0.; iter < 502.; iter += 1.) {
      if (iter >= iterations) break;
      float x2 = p.x * p.x;
      float y2 = p.y * p.y;
      float xy = p.x * p.y;
      float a = 1. + .1 * (s1 - 0.5) * s2 * s2;
      float b = -1. + .1 * (s1 - 0.5) * s2 * s2;
      float c = 0.0 + 2. * (s2 - 0.5) * s3 * s3;
      float d = max(0., pow(s8, 5.) - 0.5) * cos(100. * s7 * s2 * s9 * p.y);
      float e = max(0., pow(s9, 5.) - 0.5) * sin(100. * s2 * s1 * p.x);
      float f = 2. + s6 - s6 * s6 * s6;
      vec2 offset = init + mix(vec2(0.0), vec2(s4, s5) - .5, s3 * s4 * s5);
      p = vec2(
        a * x2 + b * y2 + c * xy + d,
        f * xy + e
      ) + offset;
      if (length(p) >= 2.0) {
        return iter / iterations;
      }
    }
    return 0.;
  }
  vec3 shade (vec2 uv) {
    float zoom = (0.3 + 12. * s7 * s7 * s7) * (1. + 3. * mod1);
    float focusAngle = 4. * mod1;
    float focusAmp = 0.4 * s7;
    vec2 init = 2. * (uv - .5) / zoom;
    pR(init, PI * floor(0.5 + 8. * s3) / 4.);
    init -= vec2(.8, .0);
    init += focusAmp * vec2(cos(focusAngle), sin(focusAngle));
    return pal(pow(run(init), .5));
  }

  void main() {
    vec2 ratio = resolution / min(resolution.x, resolution.y);
    vec2 uvRatio = 0.5 + (uv - 0.5) * ratio;
    vec3 c = vec3(0.);
    float total = 0.0;
    for (float x=-.5; x<=.5; x += 1.) {
      for (float y=-.5; y<=.5; y += 1.) {
        vec2 uvP = uvRatio;
        uvP += 0.5 * vec2(x, y) / resolution;
        c += shade(uvP);
        total += 1.0;
      }
    }
    c /= total;
    gl_FragColor = vec4(c, 1.0);
  }
    `,
  },
});

const Mandelglitch = ({ block, mod2, mod1, mod3, dim }) => {
  const { hash } = block;

  const rng = new MersenneTwister(parseInt(hash.slice(0, 16), 16));
  const s1 = rng.random();
  const s2 = rng.random();
  const s3 = rng.random();
  const s4 = rng.random();
  const s5 = rng.random();
  const s6 = rng.random();
  const s7 = rng.random();
  const s8 = rng.random();
  const s9 = rng.random();

  return (
    <Node
      width={dim}
      height={dim}
      shader={shadersMandelglitch.mandelglitch}
      uniforms={{
        resolution: Uniform.Resolution,
        mod2,
        mod1,
        mod3,
        s1,
        s2,
        s3,
        s4,
        s5,
        s6,
        s7,
        s8,
        s9,
      }}
    />
  );
};

const MandelglitchCached = React.memo(Mandelglitch);

export default CustomStyle;
