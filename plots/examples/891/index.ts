// "A grid inside a grid inside a grid"
// by @agrimony and @greweb
// code change OR click will refresh the art.
import perlin from "./perlin.mjs";

// this is in millimeters (e.g. A4 paper)
const WIDTH = 210;
const HEIGHT = 297;
const PAD = 10; // min padding to apply

// define number of points in x and y dirs
const nX = 20;
const nY = 20;

type Point = [number, number]; // x, y
type Route = Array<Point>; // basically a path
type Art = {
  routes: Route[];
};
type Bound = [number, number, number, number];
type Line = [Point, Point];

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

  // nodeSet will hold an array of nodes
  // nodes are arrays of points
  // routes will define the edges (segments) of the grid
  let nodeSet: Array<Array<Point>> = [];
  let r0 = rand(Math.PI / 2);
  console.log(r0);

  // first idea: a 3 level recursion of grids

  // feel free to break everything!
  // i've put a perlin module, maybe we can use it to sub divide on better locations
  // const seed = rand(1000)
  // perlin.perlin3(x,y,seed)

  // define frame
  const frame: Bound = [PAD, PAD, WIDTH - PAD, HEIGHT - PAD];

  // helper function to convert Bound to Array<Point>
  function boundToPoints([x1, y1, x2, y2]: Bound) {
    let p1: Point = [x1, y1];
    let p2: Point = [x2, y1];
    let p3: Point = [x2, y2];
    let p4: Point = [x1, y2];

    let p: Array<Point> = [p1, p2, p3, p4];
    return p;
  }

  // return width of bound rotated by angle a that fully encompasses original frame: Bound
  // only works for a > PI / 2
  function calcDims([p1, p2, p3, p4]: Array<Point>, a: number) {
    let [x1, y1, x2, y2] = pointsToBound([p1, p2, p3, p4]);

    let h = y2 - y1;
    let w = x2 - x1;
    let w2 = Math.sin(a) * h + Math.cos(a) * w;
    let h2 = Math.cos(a) * h + Math.sin(a) * w;

    return [w2, h2];
  }

  // check if two line segments intersect
  // line intercept math by Paul Bourke http://paulbourke.net/geometry/pointlineplane/
  // Determine the intersection point of two line segments
  // Return FALSE if the lines don't intersect
  function intersect(l1: Line, l2: Line) {
    let x1 = l1[0][0];
    let y1 = l1[0][1];
    let x2 = l1[1][0];
    let y2 = l1[1][1];
    let x3 = l2[0][0];
    let y3 = l2[0][1];
    let x4 = l2[1][0];
    let y4 = l2[1][1];

    // Check if none of the lines are of length 0
    if ((x1 === x2 && y1 === y2) || (x3 === x4 && y3 === y4)) {
      return false;
    }

    let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);

    // Lines are parallel
    if (denominator === 0) {
      return false;
    }

    let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denominator;
    let ub = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denominator;

    // is the intersection along the segments
    // if ua = 0 || ua == 1 || ub = 0 || ub == 1 then they just touch
    if (ua < 0 || ua > 1 || ub < 0 || ub > 1) {
      return false;
    }

    // Return a object with the x and y coordinates of the intersection
    let x = x1 + ua * (x2 - x1);
    let y = y1 + ua * (y2 - y1);

    let p: Point = [x, y];
    return p;
  }

  // check if point (x,y) is within polygon p
  function isOutOfBounds(polygon: Array<Point>, [x, y]: Point) {
    let p = polygon;

    // p must have more than 3 points to define the polygon
    let n = p.length;
    if (n < 3) {
      throw "Error: Not a polygon";
    }

    // draw line l extending out to the right
    let l: Line = [
      [x, y],
      [99999, y]
    ];
    let c = 0;

    // loop through sides of polygon and count number of line intersects
    for (let i = 0; i < n; i++) {
      let side: Line = [p[i], p[(i + 1) % n]];
      if (intersect(side, l) !== false) {
        c++;
      }
    }

    // if c is even, point is outside polygon
    // if c is odd, point is inside polygon
    if (c % 2 === 0) {
      return true;
    } else {
      return false;
    }
  }

  // 1. define nodes of the grid
  // returns a grid of x * y nodes rotated by r radians within boundary b
  function makeNodes(
    bp: Array<Point>,
    x: number,
    y: number,
    r: number,
    perc: number,
    level: number
  ) {
    if (level === 3) {
      return;
    }

    let nodes: Array<Point> = [];

    // calculate space between nodes with given rotation r counterclockwise
    if (r > Math.PI / 2) {
      r = r % (Math.PI / 2);
    }

    // expand polygon to a bounding rect
    let b = pointsToBound(bp);
    // convert bound back to polygon
    let nbp = boundToPoints(b);

    // calculate dimensions of outer bounding box
    let [w, h] = calcDims(nbp, r);

    // calculate space between nodes
    let xincr = w / (x - 1);
    let yincr = h / (y - 1);

    // calculate point of origin
    let ox = nbp[0][0] - Math.cos(r) * Math.sin(r) * (nbp[2][1] - nbp[0][1]);
    let oy = nbp[0][1] + Math.sin(r) * Math.sin(r) * (nbp[2][1] - nbp[0][1]);

    // generate nodes
    for (let i = 0; i < x; i++) {
      for (let j = 0; j < y; j++) {
        let p: Point = [
          ox + Math.cos(r) * xincr * i + Math.sin(r) * yincr * j,
          oy - Math.sin(r) * xincr * i + Math.cos(r) * yincr * j
        ];

        nodes.push(p);
      }
    }

    // warp nodes
    nodes = warpNodes(nodes);

    // save nodes to nodeSet
    nodeSet.push(nodes);

    // get new boundary from current nodes
    var count = 0;
    var newTile: Array<Point>;
    var inFrame = false;
    while (!inFrame || count < perc * nX * nY) {
      // randomly select new bound
      newTile = getTile(
        Math.floor(rand(nX - 1)),
        Math.floor(rand(nY - 1)),
        nodes
      );

      // test if the bound is in the polygon
      // all 4 points of the bound must be within the polygon
      if (isAllInFrame(newTile, bp)) {
        makeNodes(newTile, nX, nY, r0, perc, level + 1);

        inFrame = true;
        count++;
      }
    }
    drawGrid(nodes, bp);
  }

  // 2. warp nodes
  // return an adjusted y position based on perlin noise at x, y
  // amplified by magnitude m
  function warp([x, y]: Point, row: number, col: number, m: number) {
    //col % 2 === 0 ? m *= -1 : m;
    row % 2 === 1 ? (m *= -1) : m;

    y += m * 0.5;
    x += perlin.perlin2(x, y) * m;
    let p: Point = [x, y];

    return p;
  }

  // warpNodes applies warp to every node in nodes
  function warpNodes(nodes) {
    for (let i = 0; i < nodes.length; i++) {
      let row = Math.floor(i / nX);
      let col = i % nX;

      // find width of nodes
      let m = ((nodes[nX - 1][0] - nodes[0][0]) / nX) * 0.5;

      nodes[i] = warp(nodes[i], row, col, m);
    }

    return nodes;
  }

  // 3. connect adjacent nodes
  // draws edges between adjacent nodes within a boundary b
  function drawHoriz(bp: Array<Point>, [p1, p2]: Line) {
    if (isOutOfBounds(bp, p1) && isOutOfBounds(bp, p2)) {
      return;
    } else if (isOutOfBounds(bp, p1) && !isOutOfBounds(bp, p2)) {
      // p1 is out
      let hl: Line = [p1, p2];
      for (let k = 0; k < bp.length; k++) {
        let side: Line = [bp[k], bp[(k + 1) % bp.length]];
        if (intersect(hl, side) !== false) {
          p1 = intersect(hl, side);
        }
      }
    } else if (!isOutOfBounds(bp, p1) && isOutOfBounds(bp, p2)) {
      // p2 is out
      let hl: Line = [p1, p2];
      for (let k = 0; k < bp.length; k++) {
        let side: Line = [bp[k], bp[(k + 1) % bp.length]];
        if (intersect(hl, side) !== false) {
          p2 = intersect(hl, side);
        }
      }
    }
    routes.push([p1, p2]);
  }

  function drawVert(bp: Array<Point>, [p1, p2]: Line) {
    if (isOutOfBounds(bp, p1) && isOutOfBounds(bp, p2)) {
      return;
    } else if (isOutOfBounds(bp, p1) && !isOutOfBounds(bp, p2)) {
      // p1 is out
      let hl: Line = [p1, p2];
      for (let k = 0; k < bp.length; k++) {
        let side: Line = [bp[k], bp[(k + 1) % bp.length]];
        if (intersect(hl, side) !== false) {
          p1 = intersect(hl, side);
        }
      }
    } else if (!isOutOfBounds(bp, p1) && isOutOfBounds(bp, p2)) {
      // p2 is out
      let hl: Line = [p1, p2];
      for (let k = 0; k < bp.length; k++) {
        let side: Line = [bp[k], bp[(k + 1) % bp.length]];
        if (intersect(hl, side) !== false) {
          p2 = intersect(hl, side);
        }
      }
    }
    routes.push([p1, p2]);
  }

  function drawGrid(nodes: Array<Point>, bp: Array<Point>) {
    for (let i = 0; i < nodes.length - nX; i += nX) {
      for (let j = 0; j < nX; j++) {
        // draw "horizontal lines" going from bottom left to top right
        // check if p1 and p2 are within the polygon
        // push as route if both within
        // if one is out then find intersect and draw line from intersect
        // if both out then continue
        let p1 = nodes[i + j];
        let p2 = nodes[i + nX + j];
        drawHoriz(bp, [p1, p2]);

        // draw "vertical lines" going from top left to bottom right
        // skip bottom most row (every nX indice)
        if ((i + j + 1) % nX !== 0) {
          let p3 = nodes[i + j];
          let p4 = nodes[i + j + 1];
          drawVert(bp, [p3, p4]);
        }
      }
    }
  }

  // 4. randomly select one tile to further subdivide
  // helper function which returns a bound from a polygon
  function pointsToBound(p: Array<Point>) {
    let maxX = p[0][0];
    let maxY = p[0][1];
    let minX = p[0][0];
    let minY = p[0][1];

    for (let i = 1; i < p.length; i++) {
      if (p[i][0] > maxX) {
        maxX = p[i][0];
      }
      if (p[i][1] > maxY) {
        maxY = p[i][1];
      }
      if (p[i][0] < minX) {
        minX = p[i][0];
      }
      if (p[i][1] < minY) {
        minY = p[i][1];
      }
    }

    let b: Bound = [minX, minY, maxX, maxY];
    return b;
  }

  // takes in a row and col and outputs a polygon
  function getTile(row: number, col: number, nodes: Array<Point>) {
    let idx = row * nX + col;

    let p1: Point = nodes[idx];
    let p2: Point = nodes[idx + 1];
    let p3: Point = nodes[idx + nX + 1];
    let p4: Point = nodes[idx + nX];

    let p: Array<Point> = [p1, p2, p3, p4];
    return p;
  }

  // helper function takes in a bound
  // return a boolean if all points in bound b is within the frame f
  function isAllInFrame(bp: Array<Point>, fp: Array<Point>) {
    let checker = [];

    for (let i = 0; i < bp.length; i++) {
      checker.push(!isOutOfBounds(fp, bp[i]));
    }

    return checker.every((x) => x === true);
  }

  makeNodes(boundToPoints(frame), nX, nY, r0, 0.1, 0);

  // Draw frame
  routes.push(
    [
      [PAD, PAD],
      [WIDTH - PAD, PAD]
    ],
    [
      [WIDTH - PAD, PAD],
      [WIDTH - PAD, HEIGHT - PAD]
    ],
    [
      [WIDTH - PAD, HEIGHT - PAD],
      [PAD, HEIGHT - PAD]
    ],
    [
      [PAD, HEIGHT - PAD],
      [PAD, PAD]
    ]
  );

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

  // download has a bug where it downloads every single generation rather than the last
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
