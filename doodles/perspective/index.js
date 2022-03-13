const WIDTH = 297;
const HEIGHT = 210;

let collides_segment = ([p00, p01], [p10, p11], [p20, p21], [p30, p31]) => {
  let s10_x = p10 - p00,
    s10_y = p11 - p01,
    s32_x = p30 - p20,
    s32_y = p31 - p21;
  let d = s10_x * s32_y - s32_x * s10_y;
  if (d == 0) return;
  let s02_x = p00 - p20,
    s02_y = p01 - p21;
  let s_numer = s10_x * s02_y - s10_y * s02_x;
  if (s_numer < 0 == d > 0) return;
  let t_numer = s32_x * s02_y - s32_y * s02_x;
  if (t_numer < 0 == d > 0) return;
  if (s_numer > d == d > 0 || t_numer > d == d > 0) return;
  let t = t_numer / d;
  return [p00 + t * s10_x, p01 + t * s10_y];
};

function art(S) {
  let t, s;
  let rand = (a = 1) =>
    a *
    ((t = S[3]),
    (S[3] = S[2]),
    (S[2] = S[1]),
    (s = S[1] = S[0]),
    (t ^= t << 11),
    (S[0] ^= t ^ (t >>> 8) ^ (s >>> 19)),
    S[0] / 2 ** 32);

  let routes = [];
  let padding = 10;
  let center = [WIDTH / 2, 0.2 * HEIGHT];
  // horizon
  routes.push([
    [
      [padding, center[1]],
      [WIDTH - padding, center[1]],
    ],
    0,
  ]);

  let a = [padding, padding];
  let b = [WIDTH - padding, padding];
  let c = [WIDTH - padding, HEIGHT - padding];
  let d = [padding, HEIGHT - padding];
  let boundaries = [
    [a, b],
    [b, c],
    [c, d],
    [d, a],
  ];
  function collide_boundaries(a, b) {
    for (let [c, d] of boundaries) {
      let p = collides_segment(a, b, c, d);
      if (p) return p;
    }
  }

  const lines = [];
  for (let i = 0; i < 20; i++) {
    let angle = rand(Math.PI);
    let amp = 1000;
    let dx = Math.cos(angle) * amp;
    let dy = Math.sin(angle) * amp;
    let a = center; // TODO offset a bit center to avoid proximity of line & ink density
    let b = [a[0] + dx, a[1] + dy];
    let p = collide_boundaries(a, b);
    if (p) {
      routes.push([[a, p], 0]);
      lines.push([a, p]);
    }
  }

  function lerp(a, b, p) {
    return [a[0] + (b[0] - a[0]) * p, a[1] + (b[1] - a[1]) * p];
  }

  function buildVolume() {
    let l = rand(lines.length) | 0;
    let line = lines[l];
    let x = 0.3 + 0.4 * rand();
    let p = lerp(line[0], line[1], x);
    // project vertically
    let dystart = 20;
    let dystop = 80;
    for (let i = -1; i <= 1; i += 2) {
      let a = [p[0], p[1] + i * dystart];
      let b = [p[0], p[1] + i * dystop];

      // collides_segment(a, b, c, d)
      //
    }

    // todo: project horizontally
    // todo find 4rd point

    // todo: project on the perspective axis
  }

  return { routes };
}

function makeSVG(a) {
  let colors = ["#FC0", "#000"];
  let layers = colors
    .map((color, ci) => {
      let layer = a.routes
        .filter(([, clr]) => clr === ci)
        .map(
          ([route]) =>
            `<path d="${route
              .map(
                ([x, y], i) =>
                  `${i === 0 ? "M" : "L"}${x.toFixed(2)},${y.toFixed(2)}`
              )
              .join(
                " "
              )}" fill="none" stroke="${color}" stroke-width="0.35" style="mix-blend-mode: multiply;" />`
        )
        .join("\n");
      return `<g inkscape:groupmode="layer" inkscape:label="${color}">${layer}</g>`;
    })
    .join("\n");
  return `<svg height="210mm" style="background:white" viewBox="0 0 297 210" width="297mm" xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape">
${layers}
</svg>`;
}

const a = art(
  Uint32Array.from([0, 0, 0, 0].map(() => (Math.random() * 0xffffffff) | 0))
);
const svg = makeSVG(a);
document.body.innerHTML = svg;
