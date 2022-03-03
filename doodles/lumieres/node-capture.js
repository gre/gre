const W = 8000;
const H = 8000;
const seed = process.argv[2] || 0;
const fps = 4;
const frames = fps * 8;

const createReglRecorder = require("regl-recorder");
const regl = require("regl")(
  require("gl")(W, H, { preserveDrawingBuffer: true })
);
var recorder = createReglRecorder(regl, frames);
art(regl, seedAsArray(seed));
function seedAsArray(s) {
  s.startsWith("0x") && (s = s.slice(2));
  const t = [];
  for (let e = 0; e < s.length; e += 2) t.push(parseInt(s.slice(e, e + 2), 16));
  return t;
}
function art(regl, hash, onImageRendered) {
  const random = RNG(hash);
  const colored = random() < 0.05;
  const numberColored = Math.ceil(3 - colored * (1 + random()));
  let baseColors = Array(numberColored).fill(0.5);
  const remain = 3 - numberColored;
  if (remain > 0) {
    for (let i = 0; i < remain; i++) baseColors.push(0);
    baseColors = baseColors
      .map((a) => [a, random()])
      .sort((a, b) => a[1] - b[1])
      .map((o) => o[0] * (1 + random()));
  }
  const colordelta = [
    0.3 + 0.5 * random() * (0.5 - random()),
    0.4 + 0.5 * random() * random() * random(),
    0.5 + 0.5 * random() * (0.5 - random()),
  ];
  let paletteVec3;
  let Palette = "";
  if (random() < 0.2) {
    paletteVec3 = "0.5";
    Palette = "Dark";
  } else if (random() < 0.4) {
    baseColors[2] = 0.75;
    paletteVec3 = "0.4,0.5,0.6";
    Palette = "Cold";
  } else {
    baseColors[0] = 0.6;
    paletteVec3 = "0.7,0.5,0.45";
    Palette = "Hot";
  }
  const baseColor = `vec3(${baseColors
    .map((n) => n.toFixed(1).toUpperCase())
    .join(",")})`;

  if (colored) {
    const baseColorHuman = `#${baseColors
      .map((n) =>
        Math.round(n * 15)
          .toString(16)
          .toUpperCase()
      )
      .join("")}`;
    Palette = baseColorHuman;
  }

  let noise = Math.max(0, 1.3 * random() * random() - 0.1);
  const noiseAmp = 0.01 + 6 * random() * random();
  const noiseIntensity = 0.5 + 8 * Math.pow(random() * random(), 2);
  const s1 = random();
  const s2 = random();
  const s3 = random();
  const weightR = random();
  const weight = (0.12 + 2 * weightR).toFixed(3);
  const k = 0.05 + 0.18 * random();
  const vsym = random() < 0.2;
  const hsym = random() < 0.8;
  const textureGranularity = (1 + 50 * random()).toFixed(3);
  const textureGranularityIntensity = (0.05 + 0.15 * random()).toFixed(3);
  const bubble = 0.03 * random();
  const sharpness = random();

  let polarMods = 0;
  let mirrorMods = 0;
  let stripePrimitives = 0;
  let rectPrimitives = 0;

  const ops = [];

  function rngtr() {
    const dx = (0.5 * random() * random()).toFixed(3);
    const dy = (0.5 * random() * random()).toFixed(3);
    const da = (6.3 * random()).toFixed(3);
    const x =
      random() < 0.8
        ? dx
        : `${dx}+${(0.2 * (random() - 0.5)).toFixed(3)}*sin(t${Math.ceil(
            3 * random()
          )})`;
    const y =
      random() < 0.8
        ? dy
        : `${dy}+${(0.2 * (random() - 0.5)).toFixed(3)}*cos(t${Math.ceil(
            3 * random()
          )})`;
    const a = random() < 0.9 ? da : `${da}+t${Math.ceil(3 * random())}`;
    return `tr(p,${x},${y},${a})`;
  }

  let count = Math.floor(20 + 30 * random());
  for (let i = 0; i < count; i++) {
    const r = random();
    if (r < 0.03) {
      ops.push({ tr: `o` });
    } else if (r < 0.1) {
      polarMods++;
      ops.push({
        op: `pmp(p,${Math.ceil(2 + 40 * random() * random() * random()).toFixed(
          1
        )})`,
      });
    } else if (r < 0.15) {
      mirrorMods++;
      ops.push({
        op: `pmm1(p.x,${(0.2 + random()).toFixed(3)})`,
      });
    } else if (r < 0.2) {
      mirrorMods++;
      ops.push({
        op: `pmm1(p.y,${(0.2 + random()).toFixed(3)})`,
      });
    } else if (r < 0.24) {
      stripePrimitives++;
      ops.push({
        tr: rngtr(),
        shape: `sf(p,${Math.ceil(2 + 100 * random() * random())}.,${(
          0.02 *
          random() *
          random()
        ).toFixed(3)},${random().toFixed(3)},${(2 * random()).toFixed(3)})`,
      });
    } else {
      rectPrimitives++;
      if (random() < 0.5) ops.push({ tr: `pres` });
      const sz = `vec2(${(0.02 + 0.3 * random() * random() * random()).toFixed(
        3
      )},${(0.02 + 0.05 * random() + 0.2 * random() * random()).toFixed(3)})`;
      ops.push({
        tr: rngtr(),
        shape: `box(p,${sz})`,
      });
    }
  }

  if (vsym && random() < 0.4) {
    ops.push({
      shape: `box(o,vec2(.02,${(3 * random()).toFixed(2)}))`,
    });
  }
  if (vsym && random() < 0.4) {
    ops.push({
      shape: `box(o,vec2(${(3 * random()).toFixed(2)},.02))`,
    });
  }

  if ((vsym || hsym) && rectPrimitives < 30) {
    noise = Math.max(0.4, 2 * noise);
  }

  ops.unshift({
    shape: `.3+length(p)-${noise.toFixed(3)}*fbm9(${noiseAmp.toFixed(
      3
    )}*p+80.*s3+${noiseIntensity.toFixed(3)}*fbm(vec2(fbm(${noiseAmp.toFixed(
      3
    )}*p+80.*s1),fbm(2.*p+80.*s2))))+${bubble.toFixed(3)}*cos(3.*T)`,
  });

  const zoom =
    1 + 2 * random() * Math.min(1, (stripePrimitives + rectPrimitives) / 50);

  function humanRound(r) {
    return Math.floor(r * 100) / 100;
  }

  const frag = `
precision highp float;varying vec2 uv;uniform vec2 R;uniform float T;
#define PI ${Math.PI}
const float s1=${s1.toFixed(4)},s2=${s2.toFixed(4)},s3=${s3.toFixed(4)};
float vmax(vec2 v) {return max(v.x,v.y);}
float box(vec2 p,vec2 b){vec2 d=abs(p)-b;return length(max(d,vec2(0.)))+vmax(min(d,vec2(0.)));}
float U(float a,float b,float r){vec2 u=max(vec2(r-a,r-b),vec2(0.));return max(r,min(a,b))-length(u);}
float hash(float p){p=fract(p*.1031);p*=p+33.33;p*=p+p;return fract(p);}
float hash(vec2 p){vec3 p3=fract(vec3(p.xyx)*.1031);p3+=dot(p3,p3.yzx+33.33);return fract((p3.x+p3.y)*p3.z);}
float noise(float x){float i=floor(x);float f=fract(x);float u=f*f*(3.-2.*f);return mix(hash(i),hash(i+1.),u);}
float noise(vec2 x){vec2 i=floor(x);vec2 f=fract(x);float a=hash(i);float b=hash(i+vec2(1.,0.));float c=hash(i+vec2(0.,1.));float d=hash(i+vec2(1.));vec2 u=f*f*(3.-2.*f);return mix(a,b,u.x)+(c-a)*u.y*(1.-u.x)+(d-b)*u.x*u.y;}const mat2 m2=mat2(.6,.8,-.8,.6);
float fbm(vec2 x){float f=2.;float s=.55;float a=0.;float b=.5;for(int i=0;i<5;i++){float n=noise(x);a+=b*n;b*=s;x=f*x;}return a;}
float fbm9(vec2 x){float f=2.;float s=.55;float a=0.;float b=.5;for(int i=0;i<9;i++){float n=noise(x);a+=b*n;b*=s;x=f*x;}return a;}
vec3 palette(float t,vec3 a,vec3 b,vec3 c,vec3 d){return a+b*cos(6.28318*(c*t+d));}
vec3 pal(float t){return palette(t,${baseColor},vec3(${colordelta
    .map((c) => c.toFixed(2))
    .join(",")}),vec3(1.),vec3(${paletteVec3}));}
float pmi1(inout float p,float size,float start,float stop){float halfsize=size/2.;float c=floor((p+halfsize)/size);p=mod(p+halfsize,size)-halfsize;if(c>stop){p+=size*(c-stop);c=stop;}if(c<start){p+=size*(c - start);c=start;}return c;}
vec2 tr(vec2 p,float dx,float dy,float a){p.x+=dx;p.y+=dy;return cos(a)*p + sin(a)*vec2(p.y,-p.x);}
float sf(vec2 p,float s,float lw,float w,float h){s-=1.;p.y+=h/2.;pmi1(p.y,h/s,0.,s);return box(p,vec2(w/2.,lw/2.));}
float pmm1(inout float p,float s){float halfs=s/2.;float c=floor((p+halfs)/s);p=mod(p+halfs,s)-halfs;p*=mod(c,2.)*2.-1.;return c;}
float pmp(inout vec2 p,float rep){float an=2.*PI/rep;float a=atan(p.y,p.x)+an/2.;float r=length(p);float c=floor(a/an);a=mod(a,an)-an/2.;p=vec2(cos(a),sin(a))*r;if(abs(c)>=rep/2.)c=abs(c);return c;}
vec3 scene(vec2 p){
vec2 q=vec2(fbm(30.*p),fbm(8.*p+vec2(6.6,-.1*T)));
vec2 r=vec2(fbm(100.*q),fbm(88.*s1+p+${textureGranularity}*(q+vec2(0.04*T))));
float v=fbm(${(0.05 + random() * random()).toFixed(
    3
  )}*p+${textureGranularityIntensity}*r+s2);
float t1=.4*T;float t2=t1/2.;float t3=t2/2.;
p*=${zoom.toFixed(3)};
vec2 o=p;${vsym ? "p.y=abs(p.y);" : ""}${
    hsym ? "p.x=abs(p.x);" : ""
  }vec2 pres=p;
float k=${k.toFixed(3)};
float s=99.;${ops
    .flatMap((o) => {
      const code = [];
      if (o.op) {
        code.push(`${o.op};`);
      }
      if (o.tr) {
        code.push(`p=${o.tr};`);
      }
      if (o.shape) {
        code.push(`s=U(s,${o.shape},k);`);
      }
      return code;
    })
    .join("\n")}
s=max(s,-${weight}-s);
s+=${(0.1 * (1.0 - sharpness)).toFixed(3)};
float d=smoothstep(0.,.2,s);
v=mix(v,.5,smoothstep(.01,0.,s));
v-=.5*pow(d,.05+.5*abs(uv.y-.5));
return pal(v)+smoothstep(.0,-.2,s);}
void main(){gl_FragColor=vec4(scene((uv-0.5)*R/min(R.x,R.y)),1.);}`;

  const render = regl({
    frag,
    vert: `precision mediump float;attribute vec2 p;varying vec2 uv;void main(){uv=p;gl_Position=vec4(2.*p-1.,0,1);}`,
    attributes: {
      p: [-2, 0, 0, -2, 2, 2],
    },
    uniforms: {
      T: regl.prop("T"),
      R: ({ viewportWidth, viewportHeight }) => [viewportWidth, viewportHeight],
    },
    count: 3,
  });

  let firstCall = onImageRendered;
  let t = 0;
  regl.frame(({ viewportHeight, viewportWidth }) => {
    regl.clear({
      depth: 1,
      color: [0, 0, 0, 1],
    });
    render({ T: t++ / fps });
    if (firstCall) {
      firstCall();
      firstCall = null;
    }
    recorder.frame(viewportWidth, viewportHeight);
  });

  return {
    destroy: () => regl.destroy(),
    metadata: {
      Palette,
      Symmetry:
        vsym && hsym
          ? "Both"
          : vsym
          ? "Vertical"
          : hsym
          ? "Horizontal"
          : "None",
      "Polar Ops": polarMods,
      "Mirror Ops": mirrorMods,
      "Stripe Primitives": stripePrimitives,
      "Rectangle Primitives": rectPrimitives,
      "Blob Factor": humanRound(Math.max(0, noiseIntensity * (noise - 0.2))),
      "Laser Intensity":
        sharpness < 0.5
          ? "Low"
          : sharpness < 0.8
          ? "Medium"
          : sharpness < 0.9
          ? "High"
          : "Very High",
      Border: weightR < 0.2 ? (weightR < 0.1 ? "Small" : "Large") : "None",
    },
  };
}

function RNG(seed) {
  if (seed == undefined) {
    seed = new Date().getTime();
  }

  let N = 624;
  let M = 397;
  let MATRIX_A = 0x9908b0df; /* constant vector a */
  let UPPER_MASK = 0x80000000; /* most significant w-r bits */
  let LOWER_MASK = 0x7fffffff; /* least significant r bits */

  let mt = new Array(N); /* the array for the state vector */
  let mti = N + 1; /* mti==N+1 means mt[N] is not initialized */

  if (typeof seed === "object") {
    init_by_array(seed, seed.length);
  } else {
    init_seed(seed);
  }

  function init_seed(s) {
    mt[0] = s >>> 0;
    for (mti = 1; mti < N; mti++) {
      let s = mt[mti - 1] ^ (mt[mti - 1] >>> 30);
      mt[mti] =
        ((((s & 0xffff0000) >>> 16) * 1812433253) << 16) +
        (s & 0x0000ffff) * 1812433253 +
        mti;
      /* See Knuth TAOCP Vol2. 3rd Ed. P.106 for multiplier. */
      /* In the previous versions, MSBs of the seed affect   */
      /* only MSBs of the array mt[].                        */
      /* 2002/01/09 modified by Makoto Matsumoto             */
      mt[mti] >>>= 0;
      /* for >32 bit machines */
    }
  }

  /* initialize by an array with array-length */
  /* init_key is the array for initializing keys */
  /* key_length is its length */
  /* slight change for C++, 2004/2/26 */
  function init_by_array(init_key, key_length) {
    var i, j, k;
    init_seed(19650218);
    i = 1;
    j = 0;
    k = N > key_length ? N : key_length;
    for (; k; k--) {
      var s = mt[i - 1] ^ (mt[i - 1] >>> 30);
      mt[i] =
        (mt[i] ^
          (((((s & 0xffff0000) >>> 16) * 1664525) << 16) +
            (s & 0x0000ffff) * 1664525)) +
        init_key[j] +
        j; /* non linear */
      mt[i] >>>= 0; /* for WORDSIZE > 32 machines */
      i++;
      j++;
      if (i >= N) {
        mt[0] = mt[N - 1];
        i = 1;
      }
      if (j >= key_length) j = 0;
    }
    for (k = N - 1; k; k--) {
      var s = mt[i - 1] ^ (mt[i - 1] >>> 30);
      mt[i] =
        (mt[i] ^
          (((((s & 0xffff0000) >>> 16) * 1566083941) << 16) +
            (s & 0x0000ffff) * 1566083941)) -
        i; /* non linear */
      mt[i] >>>= 0; /* for WORDSIZE > 32 machines */
      i++;
      if (i >= N) {
        mt[0] = mt[N - 1];
        i = 1;
      }
    }

    mt[0] = 0x80000000; /* MSB is 1; assuring non-zero initial array */
  }

  /* generates a random number on [0,0xffffffff]-interval */
  /* origin name genrand_int32 */
  function random_int() {
    var y;
    var mag01 = new Array(0x0, MATRIX_A);
    /* mag01[x] = x * MATRIX_A  for x=0,1 */

    if (mti >= N) {
      /* generate N words at one time */
      var kk;

      if (mti == N + 1)
        /* if init_seed() has not been called, */
        init_seed(5489); /* a default initial seed is used */

      for (kk = 0; kk < N - M; kk++) {
        y = (mt[kk] & UPPER_MASK) | (mt[kk + 1] & LOWER_MASK);
        mt[kk] = mt[kk + M] ^ (y >>> 1) ^ mag01[y & 0x1];
      }
      for (; kk < N - 1; kk++) {
        y = (mt[kk] & UPPER_MASK) | (mt[kk + 1] & LOWER_MASK);
        mt[kk] = mt[kk + (M - N)] ^ (y >>> 1) ^ mag01[y & 0x1];
      }
      y = (mt[N - 1] & UPPER_MASK) | (mt[0] & LOWER_MASK);
      mt[N - 1] = mt[M - 1] ^ (y >>> 1) ^ mag01[y & 0x1];

      mti = 0;
    }

    y = mt[mti++];

    /* Tempering */
    y ^= y >>> 11;
    y ^= (y << 7) & 0x9d2c5680;
    y ^= (y << 15) & 0xefc60000;
    y ^= y >>> 18;

    return y >>> 0;
  }

  /* generates a random number on [0,1)-real-interval */
  return () => random_int() * (1.0 / 4294967296.0);
}
