// "Reflection of a reflection"
// by @metamere and @greweb
// code change OR click will refresh the art.
import perlin from "./perlin.mjs";

// this is in millimeters (e.g. A4 paper)
const WIDTH = 210;
const HEIGHT = 297;
const PAD = 30;

type Point = [number, number]; // x, y
type Route = Array<Point>; // basically a path
type Art = {
  routes: Route[];
};
type Bound = [number, number, number, number];

// helpers

function eq(a, b) {
  return a[0] === b[0] && a[1] === b[1];
}
function mix(a, b, x) {
  return (1 - x) * a + x * b;
}
function mix2(
  a: [number, number],
  b: [number, number],
  x: number
): [number, number] {
  return [mix(a[0], b[0], x), mix(a[1], b[1], x)];
}

// The main art function where we can do what we want
function art(S): Art {
  // very simple RNG function
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

  // Routes are array of segments.
  // a segment is an array of points.
  // a point is a [number,number] (for [x,y])
  let routes: Route[] = [];

  if (rand() < 0.5) {
    routes.push([
      [130, 130],
      [100, 80],
      [70, 130],
      [130, 130]
    ]);
  } else {
    let edges = 3 + rand(20) * rand() * rand();
    let polygon = [];
    let rounding = 5;
    for (let i = 0; i < edges; i++) {
      let x = mix(PAD, WIDTH - PAD, rand());
      let y = mix(PAD, HEIGHT * 0.5, rand());
      x = Math.floor(x / rounding) * rounding;
      y = Math.floor(y / rounding) * rounding;
      polygon.push([x, y]);
    }
    polygon.push(polygon[0]);

    routes.push(polygon);
  }

  // just a random idea. there can be a reflection along a X axis
  function reflect() {
    const ymirror = HEIGHT / 2.0;
    routes.forEach((route) => {
      routes.push(route.map(([x, y]) => [x, 2 * ymirror - y]));
    });
  }

  reflect();

  const cutMiddleProba = 0.5 + rand(1);

  function reflectDestruct(power) {
    const ymirror = HEIGHT / 2.0;
    routes.forEach((route) => {
      const first = route[0];
      const last = route[route.length - 1];
      let mapped: Array<[number, number]> = route.map(([x, y]) => [
        x + power * rand() * rand(),
        2 * ymirror - y + power * rand() * rand()
      ]);
      if (eq(first, last)) {
        mapped[mapped.length - 1] = mapped[0];
      }

      if (rand() > cutMiddleProba) {
        // cut away the middle part of each segment
        let r = [];
        let last = route[0];
        let div = rand(0.5) * rand();
        r.push(last);
        for (let i = 1; i < route.length; i++) {
          const p = route[i];
          routes.push([last, mix2(last, p, 0.5 - div)]);
          routes.push([mix2(last, p, 0.5 + div), p]);
          last = p;
        }
      } else {
        routes.push(mapped);
      }
    });
  }

  const rep = 6;
  const mul = 2 + rand(3.5);
  const extremeprob = 0.1;
  const maxExtreme = 10 + rand(20);
  for (let i = 0; i < rep; i++) {
    reflectDestruct(
      (i + 1) * mul + (rand() < extremeprob ? rand(maxExtreme) : 0)
    );
  }

  return { routes };
}

// Bake the SVG from scratch
function makeSVG(a: Art) {
  let layer = a.routes
    .map(
      (route) =>
        `<path d="${route
          .map(
            ([x, y], i) =>
              `${i === 0 ? "M" : "L"}${x.toFixed(2)},${y.toFixed(2)}`
          )
          .join(
            " "
          )}" fill="none" stroke="black" stroke-width="0.35" style="mix-blend-mode: multiply;" />`
    )
    .join("\n");
  return `<svg style="background:white" viewBox="0 0 210 297" width="210mm" height="297mm" xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape">
<g inkscape:groupmode="layer" inkscape:label="black">${layer}</g>
</svg>`;
}

function generate() {
  const seed = Uint32Array.from(
    [0, 0, 0, 0].map(() => (Math.random() * 0xffffffff) | 0)
  );
  const a = art(seed);
  const $content = document.getElementById("content");
  const svg = makeSVG(a);
  $content.innerHTML = svg;
  if (Math.random() < 0.5) {
    document.body.className = "dark";
  } else {
    document.body.className = "light";
  }

  $content.onclick = generate;

  document.getElementById("download").addEventListener("click", function () {
    const blob = new Blob([svg], { type: "image/svg+xml" });
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = "image.svg";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  });
}

generate();
