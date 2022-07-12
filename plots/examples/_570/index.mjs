/**
 * Mountains Reflection – @greweb – 2022 – CC BY-NC-ND 4.0
 *
 * This generator was developed from scratch in plain JavaScript
 * for @sableraph's week theme "reflection"
 *
 * inspired from my past work on https://greweb.me/plots
 *
 * A SVG is generated in the page and can be plotted with pens
 */

import noise from "./perlin.mjs";

// uncomment this to auto refresh the art =)
// setTimeout("location.href = location.href", 5000);

// feel free to play with these
let PRECISION = 0.1; // can set to 0.1 for best precision, but it's slower
let NOISE_AMP = 0.6;
let MAX_MOUNTAIN_LAYERS = 7;
let MOUNTAIN_DENSITY = 5;
let MAX_SUN_RADIUS = 40;
let SUN_DENSITY = 14;
let MAX_GROUP_OF_BIRDS = 3;
let MAX_BOATS = 10;
let REFLECTION_PROBABILITY = 0.04;

///////////////////////////////

const seed = Uint32Array.from(
  [0, 0, 0, 0].map(() => (Math.random() * 0xffffffff) | 0)
);
const a = art(seed);
const svg = makeSVG(a);
document.body.innerHTML = svg;
console.log(svg); // quick way, in the console, to copy the SVG code

// art() function is where the art generator is coded
function art(S) {
  // plot dimensions and padding security
  const HEIGHT = 297;
  const WIDTH = 210;
  const PAD = 10;

  let YCENTER = HEIGHT / 2;
  let rand = makeRand(S);

  // store all the paths to plot
  let black_routes = [];
  let red_routes = [];

  // ~~~ STEP ~~~ build the mountains

  let x_increment = PRECISION; // precision of the strokes in the mountain
  let y_increment = 1 / MOUNTAIN_DENSITY; // the base distance between lines in the mountain
  // store the highest points reached by a mountain to implement collision
  let heights = Array(Math.ceil(WIDTH / x_increment)).fill(HEIGHT - PAD);

  // For each mountain layer...
  let mountains = (1 + rand(MAX_MOUNTAIN_LAYERS)) | 0; // number of mountains layers
  let mountains_delta = 30 + rand(30); // defines the "stops" of each mountains layer
  let ystoppow = 0.9 + rand(1.2);
  for (let i = 0; i < mountains; i++) {
    y_increment *= 1.3; // the distance between lines in mountain will fade away with distance
    // we pick random perlin noise frequencies (f*) and amplitudes (amp*)
    // the different level of noises are composed with domain warping
    let f1 = 0.002 + rand(0.002);
    let f2 = 0.004 + rand(0.004);
    let f3 = 0.03 + rand(0.02);
    let amp1 = NOISE_AMP / f1;
    let amp2 = 0.3 + rand(0.5);
    let amp3 = 0.5 + rand(0.4);
    // "s" is for the second perlin noise added to the first
    let sf1 = 0.005 + rand(0.01);
    let sf2 = 0.005 + rand(0.02);
    let ampnoise2 = rand(0.08);
    let ampnoise3 = rand(0.04);
    let samp2 = NOISE_AMP * (3 + rand(4));
    let ystop =
      0.5 * HEIGHT - Math.pow((i + 1) / mountains, ystoppow) * mountains_delta;
    let perlin_seed = rand(1000);
    // For each line of the mountain
    for (let ybase = YCENTER; ybase > ystop; ybase -= y_increment) {
      let amp1mul =
        0.7 * smoothstep(YCENTER - 2, YCENTER - 12, ybase) +
        0.3 * smoothstep(YCENTER, 0, ybase);
      let route = [];
      let freqmul = 0.6 - (ybase - YCENTER) / HEIGHT;
      let xi = 0;
      // we iterate on X to build up the mountain
      for (let x = PAD; x < WIDTH - PAD; x += x_increment) {
        let dx = x - WIDTH / 2;
        let dy = ybase - HEIGHT / 3;
        // most important part now, we build up the complex noise of the mountain
        let amp1mul2 =
          1 - 0.5 * smoothstep(0, 100, Math.sqrt(dx * dx + dy * dy));
        let y =
          ybase +
          noise.perlin2(x * 0.02, 7.7 * perlin_seed) -
          amp1mul *
            amp1mul2 *
            amp1 *
            (0.2 + Math.pow(3 * noise.perlin2(perlin_seed, 0.002 * x), 2)) *
            (0.7 *
              noise.perlin3(
                f1 * x * freqmul,
                f1 * ybase * freqmul,
                perlin_seed / 3.3 +
                  amp2 *
                    noise.perlin3(
                      -5.5 * perlin_seed,
                      f2 * x * freqmul,
                      f2 * ybase * freqmul
                    ) -
                  amp3 *
                    noise.perlin3(
                      f3 * x * freqmul,
                      perlin_seed,
                      f3 * ybase * freqmul
                    )
              ) -
              0.1 *
                Math.pow(
                  noise.perlin3(
                    sf1 * x * freqmul,
                    sf1 * ybase * freqmul,
                    -perlin_seed +
                      samp2 *
                        noise.perlin3(
                          sf2 * x * freqmul,
                          sf2 * ybase * freqmul,
                          8 * perlin_seed
                        )
                  ),
                  2
                ) -
              ampnoise2 *
                noise.perlin3(
                  0.3 * x * freqmul,
                  0.6 * ybase * freqmul,
                  perlin_seed / 1.7
                ) -
              ampnoise3 *
                noise.perlin3(
                  0.6 * x * freqmul,
                  1.2 * ybase * freqmul,
                  -perlin_seed * 3.3
                ));
        let h = heights[xi];
        // implement a simple collision of mountains
        if (y < h + 0.2 && y > PAD) {
          heights[xi] = y;
          route.push([x, y]);
        } else {
          if (route.length > 1) {
            black_routes.push(route);
          }
          route = [];
        }
        xi++;
      }

      if (route.length > 1) {
        black_routes.push(route);
      }
    }
  }

  // ~~~ STEP ~~~ chose a place for the possible sun.

  // find the lowest point of the mountain
  let lowxi = -1;
  let lowy = 0;
  let padend = 2 * Math.ceil(PAD / x_increment);
  let padxi = rand(100) | 0;
  for (let xi = padxi; xi < heights.length - padend - padxi; xi++) {
    let y = heights[xi];
    if (y > lowy) {
      lowy = y;
      lowxi = xi;
    }
  }
  let lowx = PAD + lowxi * x_increment;

  let center = [lowx, lowy * rand(1.2)];
  let radius =
    Math.min(
      MAX_SUN_RADIUS,
      WIDTH - PAD - center[0],
      center[0] - PAD,
      center[1] - PAD
    ) *
    (0.5 + rand(0.5));
  if (radius > 10) {
    let route = [];
    let spins = SUN_DENSITY;
    let rbase = radius + 0.5;
    let a = 0;
    while (rbase > 0) {
      let r = Math.min(rbase, radius);
      let aincr = PRECISION / (r + 1.0);
      let rincr = (0.9 * aincr) / spins;
      let p = [center[0] + r * Math.cos(a), center[1] + r * Math.sin(a)];
      let xi = Math.ceil((p[0] - PAD) / x_increment);
      let h = heights[xi];
      if (p[1] < h) {
        route.push(p);
      } else {
        if (route.length > 1) {
          red_routes.push(route);
        }
        route = [];
      }
      rbase -= rincr;
      a += aincr;
    }

    if (route.length > 1) {
      red_routes.push(route);
    }
  }

  // ~~~ STEP ~~~ place birbs <3

  let groups = rand(MAX_GROUP_OF_BIRDS) | 0;
  for (let i = 0; i < groups; i++) {
    lowx = WIDTH / 2 + (rand() - 0.5) * rand(WIDTH - 2 * PAD);
    let xi = Math.ceil((lowx - PAD) / x_increment);
    lowy = heights[xi];
    center = [lowx, lowy * (0.2 + rand(0.6))];
    radius = // radius of a circle in which we place some birds
      Math.min(
        rand(100),
        WIDTH - PAD - center[0],
        center[0] - PAD,
        center[1] - PAD
      ) *
      (0.5 + rand(0.5));
    let golden_angle = Math.PI * (3 - Math.sqrt(5));
    let count_birds = Math.floor(rand(2) * radius - rand(4));
    let radius_from = 2;

    for (let i = 0; i < count_birds; i++) {
      let a = golden_angle * i;
      let amp =
        radius_from + (radius - radius_from) * Math.pow(i / count_birds, 0.6);
      let x = center[0] + amp * Math.cos(a) + (rand() - 0.5) * rand(10);
      let y = center[1] + amp * Math.sin(a) + (rand() - 0.5) * rand(10);
      let size = 1 + rand(3);
      let dx = size * (0.3 + rand(0.1));
      let dy = size * 0.5;
      black_routes.push(
        path_subdivide_to_curve(
          [
            [x - dx, y - dy],
            [x, y + dy],
            [x + dx, y - dy],
          ],
          2,
          0.51 + rand(0.2)
        )
      );
    }
  }

  // ~~~ STEP ~~~ reflect random points of the drawn shapes
  [black_routes, red_routes].forEach((routes, i) => {
    let probability = REFLECTION_PROBABILITY;
    routes
      .reduce((acc, r) => acc.concat(r), [])
      .forEach(([cx, cy]) => {
        if (rand() > probability) return;
        let base_stroke = 0.4;
        let sx = base_stroke / 2 + rand(8) * rand();
        let sy = 0.5 * rand() * (rand(1) - 0.5);
        let x = cx + rand(50) * rand() * (rand() - 0.5);
        let y = 2 * YCENTER - cy + rand(150) * (rand() - 0.5);
        if (y > YCENTER && y < HEIGHT - PAD) {
          let x1 = Math.min(Math.max(PAD, x - sx), WIDTH - PAD);
          let x2 = Math.min(Math.max(PAD, x + sx), WIDTH - PAD);
          if (x2 - x1 > base_stroke) {
            routes.push([
              [x1, y - sy],
              [x2, y + sy],
            ]);
          }
        }
      });
  });

  // ~~~ STEP ~~~ boats
  let boats = rand(MAX_BOATS) | 0;
  for (let j = 0; j < boats; j++) {
    let curvy1dt = mix(-2, 1, rand());
    let curvy2dt = mix(-2.0, 1.0, rand());
    let w1base = mix(5, 7, rand());
    let w2base = w1base + mix(-1.0, 1.0, rand());
    let curvy1 = curvy1dt + mix(-1.0, 1.0, rand());
    let curvy2 = curvy2dt + mix(-1.0, 1.0, rand());
    let x = WIDTH / 2 + (0.5 - rand()) * rand(WIDTH * 0.8);
    let y = HEIGHT * 0.65 + rand(0.1 * HEIGHT) * rand();
    let poleh = rand(10);
    let polexoff = rand() * (rand() - 0.5) * rand(5);
    for (let i = 0; i < 10; i++) {
      let dy = i * 0.25 - 0.1;
      let w1 = w1base + mix(-0.6, 0.6, rand());
      let w2 = w2base + mix(-0.3, 0.3, rand());
      let h1 = 3.0 + 2.0 * mix(-1.0, 1.0, rand()) * rand();
      let h2 = 3.0 + 2.0 * mix(-1.0, 1.0, rand()) * rand();
      let base_route = [
        [x - w1 + curvy1, y + dy - h1],
        [x - w1, y + dy - h1],
        [x - w1, y + dy],
        [x + w2, y + dy],
        [x + w2, y + dy - h2],
        [x + w2 - curvy2, y + dy - h2],
      ];

      let route = path_subdivide_to_curve(
        base_route,
        2,
        mix(0.72, 0.78, rand())
      );
      black_routes.push(route);

      let xoff = 0.8 * (rand() - 0.5);
      let ymul = rand();
      black_routes.push([
        [x + xoff, y + dy],
        [x + polexoff + xoff, y + dy - poleh * ymul],
      ]);
    }
  }

  return { black_routes, red_routes };
}

// now lives all the utility to make the SVG and other helpers

function makeSVG(a) {
  return `<svg style="background:white" viewBox="0 0 210 297" width="210mm" height="297mm" xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape">
${make_svg_layer("#000", a.black_routes)}
${make_svg_layer("#F00", a.red_routes)}
</svg>`;
}

function make_svg_layer(color, routes) {
  let paths = routes
    .map(
      (route) =>
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
  return `<g inkscape:groupmode="layer" inkscape:label="${color}">${paths}</g>`;
}

function mix(a, b, x) {
  return a * (1 - x) + b * x;
}

function smoothstep(min, max, value) {
  var x = Math.max(0, Math.min(1, (value - min) / (max - min)));
  return x * x * (3 - 2 * x);
}

function euclidian_dist(a, b) {
  let dx = a[0] - b[0];
  let dy = a[1] - b[1];
  return Math.sqrt(dx * dx + dy * dy);
}

function makeRand(S) {
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
  return rand;
}

function lerp_point(a, b, m) {
  return [a[0] * (1 - m) + b[0] * m, a[1] * (1 - m) + b[1] * m];
}

function path_subdivide_to_curve_it(path, interpolation) {
  let l = path.length;
  if (l < 3) {
    return path;
  }
  let route = [];
  let first = path[0];
  let last = path[l - 1];
  let looped = euclidian_dist(first, last) < 0.1;
  if (looped) {
    first = lerp_point(path[1], first, interpolation);
  }
  route.push(first);
  for (let i = 1; i < l - 1; i++) {
    let p = path[i];
    let p1 = lerp_point(path[i - 1], p, interpolation);
    let p2 = lerp_point(path[i + 1], p, interpolation);
    route.push(p1);
    route.push(p2);
  }
  if (looped) {
    last = lerp_point(path[l - 2], last, interpolation);
  }
  route.push(last);
  if (looped) {
    route.push(first);
  }
  return route;
}

function path_subdivide_to_curve(path, n, interpolation) {
  let route = path;
  for (let i = 0; i < n; i++) {
    route = path_subdivide_to_curve_it(route, interpolation);
  }
  return route;
}
