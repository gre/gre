// TODO GL rendering
// TODO regroup layers & remove empty ones
// TODO features script
// TODO optims
// TODO test more perf cases

let { PI, cos, sin, max, min, pow, ceil, sqrt, floor, abs } = Math;
let { assign } = Object;

let push = (arr, el) => arr.push(el);
let mapjoin = (arr, f, c) => arr.map(f).join(c);
let concat = (arr, el) => arr.concat(el);

let WIDTH = 297;
let HEIGHT = 420;

let PAD = 10;
let LINEAR = "linear";
let PX = (px) => px + "px";
let MM = (mm) => mm + "mm";
let TRIANGLE = [-2, 0, 0, -2, 2, 2];
let twopi = 2 * PI;
let WINDOW = this;
let MAX = 4096;
let ratio = WIDTH / HEIGHT;
let MASKS = ["#0FF", "#F0F"];
//let gold = ["Gold Gel", [0.85, 0.7, 0.25], [1, 0.89, 0.55]];
let gold = ["Gold Gel", [0.75, 0.65, 0.2], [0.9, 0.9, 0.4]];
let COLORS_LIGHT = [
  ["Black", [0.1, 0.1, 0.1], [0, 0, 0]],
  gold,
];
let COLORS_DARK = [
  ["Silver Gel", [0.8, 0.8, 0.8], [1, 1, 1]],
  gold,
];
let FILL_EMPTY = 0;
let FILL_CONCENTRIC = 1;
let FILL_FULL = 2;
let FILL_VSLIDE = 3;
let FILL_HSLIDE = 4;

let S = Uint32Array.from([0, 0, 0, 0].map((i) => parseInt(tokenData.hash.substr(i * 8 + 5, 8), 16)));

function art() {
  // RNG 
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

  let paperSeed = rand(99);

  // THE ART RANDOMNESS IS HERE
  let precision = 0.25;

  let hhetching = 1.5 + rand(1);
  let vhetching = 1.5 + rand(1);

  let withMainObject = rand() < 0.5;
  let will_line_divide = rand() < 0.4;
  let dark = rand(7) | 0;
  let main_color = rand(2) | 0;
  let main_obj_count = 1 + (rand(6) * rand()) | 0;
  let hasMainRayon = withMainObject && rand() < 0.5;
  let main_poly_size = 3 + (rand(5) * rand() + rand(80) * (rand() < 0.1)) | 0;
  let mainHasHole = hasMainRayon && (rand() < 1 - 1 / (1 + main_obj_count));
  let make_a_cloud = rand() < 0.2;
  let make_a_moon = rand() < (will_line_divide ? 0.4 : 0.2);
  let symmetry_moon = make_a_moon && rand() < 0.5;
  let connected_moons = symmetry_moon && rand() < (withMainObject ? 0.9 : 0.1);

  let reverseColor = rand() < 0.5
  let waves = rand() < 0.5
  let monochrome = rand() < (will_line_divide && waves ? 0.6 : 0.2)
  if (monochrome) {
    reverseColor = rand() < 0.9
  }
  if (!will_line_divide) {
    reverseColor = false;
  }

  // establish the background pattern randomness
  let base = 2 + rand(3);
  let b = rand();
  let diff = base + rand(5) + rand(70) * rand() * b;
  let minv = base + rand(5);
  let maxv = minv * (1 + rand()) + rand(40) * b;
  let internal_pad = maxv + 3 + rand(8);
  let hf = rand(2); // factor of the zindex sorting randomness vs edge distance
  let f = rand(0.2) * rand(); // size variation
  let f2 = rand() * rand() * rand(); // color variation
  let f3 = rand(2) * rand(); // fill technique
  let rotAmp = rand(40) * max(0, rand() - 0.3);

  if (FEATURE_MODE) {

    function scoring(value, sizes) {
      let i = 0;
      for (; i < sizes.length - 1; i += 2) {
        if (value < sizes[i + 1]) return sizes[i];
      }
      return sizes[i];
    }

    let composed = (words) =>
      words.filter(o => o).join(" ")

    // as early as we can, we yield the features, for uglify to optimize the code.
    let features = {
      Paper: dark ? "Black" : "White",
      Hole: mainHasHole ? "Yes" : "No",
      Rayon: hasMainRayon ? "Yes" : "No",
    };



    /*
    let golden = withMainObject && main_color && !monochrome && !reverseColor;
    let o = composed([
      golden && "Golden",
      mainHasHole && "Holed",
      hasMainRayon && "Radiant",
      withMainObject && (main_obj_count === 1 ?
        scoring(
          main_poly_size
          , [
            "Triangle",
            4,
            "Square",
            5,
            "Pentagon",
            6,
            "Hexagon",
            7,
            "Heptagon",
            8,
            "Octagon",
            9,
            "Polygon",
            16,
            "Circle"
          ]
        )
        : "Polygons"),
    ]);
    if (will_line_divide) {
      if (o) {
        o += " with ";
      }
      o += waves ? "Waves" : "Mountains"
    }

    features["Object"] = o || "None"
    */

    let sky = "";
    if (make_a_moon) {
      sky = composed([
        connected_moons && "Connected",
        "Moon" + (symmetry_moon ? "s" : ""),
      ]);
    }
    if (make_a_cloud) {
      if (sky) {
        sky += " and ";
      }
      sky += "Cloud"
    }
    features.Sky = sky || "None"


    features.Background =
      scoring(
        diff - (minv + maxv) / 2,
        [
          "Packed",
          -5,
          "Dense",
          2,
          "Scattered",
          8,
          "Sparse"
        ]
      )

    features["Background Alignment"] =
      scoring(rotAmp, [
        "Aligned",
        0.2,
        "Tilted",
        2,
        "Twisted"
      ]);

    const effect = composed([
      reverseColor && "Reverse",
      monochrome && "Monochrome",
    ])

    features.Effect = effect || "None"

    return features;
  }

  // THE ART CODE IS HERE
  let routes = [];

  // main mask will implement the collision logic
  let mask = newPaintMask(precision, WIDTH, HEIGHT);

  // MAKE FOREGROUND OBJECTS

  if (withMainObject) {
    add_obj_polymerge(main_obj_count, hasMainRayon, mainHasHole, main_poly_size, main_color, [WIDTH / 2, HEIGHT / 2], WIDTH * (0.2 + rand(0.2)))
  }

  // MAKE SECOND LEVEL OBJECTS

  if (make_a_cloud) {
    // MAKE A CLOUD
    let cx = WIDTH * (0.4 + rand(0.2));
    let cy = HEIGHT * ((withMainObject || will_line_divide ? 0.2 : 0.4) + rand(0.2));
    let sx = WIDTH * (0.1 + rand(withMainObject ? 0.1 : 0.3));
    let sy = sx * (0.4 + rand(0.4));
    let clr = (rand(2) * rand()) | 0;
    let n = (20 + rand(200)) | 0;
    let fillbase = (rand() - 0.1) * rand(3);
    let fillfactor = (rand(2) - 1) * rand(3);
    let d = 1 + rand(2);
    let cutoff = 0.5 + rand();

    for (let i = 0; i < n; i++) {
      let ang = rand(PI * 2);
      let acos = cos(ang);
      let asin = sin(ang);
      let r = rand();
      let p = [cx + r * sx * acos, cy + r * sy * asin];
      let sz = sx * (0.2 + rand(0.5));
      if (rand() < cutoff) {
        let offset = [
          -d * rand(9) * acos,
          -d * rand(9) * asin,
        ]
        let poly = square([
          p[0] + offset[0],
          p[1] + offset[1],
        ], sz * (1 - rand(0.2)));
        paintPolygon(mask, poly);
      }
      if (p[0] + sz / 2 + PAD < 0 || p[0] - sz / 2 - PAD > WIDTH || p[1] + sz / 2 + PAD < 0 || p[1] - sz / 2 - PAD > HEIGHT) continue;
      add_square(clr, max(0, min(2, (fillbase + 2 * fillfactor * rand(1 - i / n)))) | 0, p, 0.0, sz);
    }
  }

  if (make_a_moon) {
    // MAKE A MOON
    let cordond = 0.5 + (rand(10) + 1) * rand();
    let clr = (rand(2) * rand()) | 0;
    let ax = WIDTH * (0.3 + rand(0.4));
    let ay = HEIGHT * (0.1 + rand(0.3));
    let ar = (0.05 + rand(0.1) * (1 + rand())) * WIDTH;
    let adr = (rand() > 0.2) * mix(cordond, 0.5, rand());
    let halo = max(rand() - 0.5, 0) * 4;
    add_spiral(clr, [ax, ay], ar, adr, halo);
    if (symmetry_moon) {
      // MAKE A MOON ON OTHER SIDE
      let bx = WIDTH - ax;
      let by = HEIGHT - ay;
      let br = (0.01 + rand(0.1) * (1 + rand())) * WIDTH;
      let bdr = adr * (0.7 + rand());
      add_spiral(clr, [bx, by], br, bdr, halo);
      if (connected_moons) {
        // CONNECT THE MOONS
        add_cordon(clr, [ax, ay], ar, [bx, by], br, cordond);
      }
    }
  }

  // MAKE THE SQUARES BACKGROUND

  let seed = rand(9999)

  let centers = []
  let remainx = (WIDTH - internal_pad * 2) % diff;
  let remainy = (HEIGHT - internal_pad * 2) % diff;
  let offsetx = remainx / 2;
  let offsety = remainy / 2;
  for (let x = internal_pad; x < WIDTH - internal_pad; x += diff) {
    for (let y = internal_pad; y < HEIGHT - internal_pad; y += diff) {
      push(centers, [x + offsetx, y + offsety])
    }
  }
  centers =
    centers.map((v, i) => [v, rand(hf * HEIGHT) - max(v[1], HEIGHT - v[1]) - max(v[0], WIDTH - v[0])])
      .sort((a, b) => a[1] - b[1])
      .map((v) => v[0])

  let fills = shuffle([
    FILL_EMPTY,
    FILL_FULL,
    FILL_CONCENTRIC,
    FILL_VSLIDE,
    FILL_HSLIDE,
  ]);

  fills.splice(0, 2 + rand(3) | 0);
  if (rand() < 0.5) fills.unshift(FILL_EMPTY);
  fills.unshift(FILL_EMPTY);
  let distribpow = 0.9 + rand();

  for (let c of centers) {
    let distToEdge = min(
      c[0] - internal_pad,
      c[1] - internal_pad,
      WIDTH - c[0] - internal_pad,
      HEIGHT - c[1] - internal_pad
    );
    let distToEdgeNorm = distToEdge / ((WIDTH - 2 * internal_pad) / 2.0);
    let size = mix(
      minv,
      maxv,
      0.5 + 0.5 * perlin3(3.2 + seed, f * c[0], f * c[1])
    );
    let clr = (mix(0, 2, 0.5 + 0.5 * perlin3(seed / 0.4, f2 * c[0], f2 * c[1])) | 0) % 2;
    let fillTechnique = fills[(fills.length * pow(0.5 + 0.5 * perlin3(seed, f3 * c[0], f3 * c[1]), distribpow)) | 0];
    let rotation = distToEdgeNorm * rotAmp * perlin3(0.7 * seed, 0.005 * c[0], 0.005 * c[1]);
    add_square(clr, fillTechnique, c, rotation, size);
  }

  // MAKE THE POST PROCESSING EFFECTS

  if (monochrome) {
    // monochrome
    routes = routes.map(([clr, rt]) => [0, rt])
  }

  if (will_line_divide) {
    line_divisor(reverseColor, waves)
  }

  // NOW WE JUST HAVE ALL THE UTILITIES THAT USE RNG OR MUTATE ROUTES

  function isOutside(p) {
    return !inCanvas(p) || isPainted(mask, p);
  }

  function line_divisor(reverseColor, waves) {
    let line_top;
    let line_bottom;
    let pad = 0.5;

    let incr = 1 + rand(WIDTH / 3) * pow(rand(), 8);
    let freq1 = rand(0.5) * rand();
    let freq2 = rand(0.2) * rand();
    let freq3 = rand(0.02) * rand();
    let amp1 = 2 + rand(10) * max(rand() - 0.3, 0);
    let amp2 = 2 + rand(20) * max(rand() - 0.3, 0);
    let amp3 = rand(200) * max(rand() - 0.3, 0);
    let offset_p = rand();
    let offset = mix(1, 20, offset_p);
    let divisions = mix(2, 20, offset_p * rand()) | 0;

    if (waves) {
      let line_divisor_poly = [];
      for (let x = 0; x < WIDTH + incr; x += incr) {
        let y = HEIGHT / 2 + amp1 * cos(x * freq1) + amp2 * sin(x * freq2) + amp3 * sin(x * freq3);
        push(line_divisor_poly, [x, y]);
      }

      let lines = []
      for (let i = 0; i < divisions; i++) {
        let t = i / (divisions - 1);
        let off = mix(-offset, offset, t);
        let line = line_divisor_poly.map(([x, y]) => [x, y + off]);
        push(lines, [main_color, line]);
      }

      line_top = line_divisor_poly.map(([x, y]) => [x, y - offset - pad]);
      line_bottom = line_divisor_poly.map(([x, y]) => [x, y + offset + pad]);

      let routes_up = clipRoutes(routes, p =>
        interpolate_y(line_top, p[0]) < p[1]
      );

      let routes_down = clipRoutes(routes, p =>
        interpolate_y(line_bottom, p[0]) > p[1]
      );

      if (reverseColor) {
        // reverse color
        routes_down = routes_down.map(([clr, route]) => [(clr + 1) % 2, route]);
      }

      routes = concat(concat(
        routes_up,
        routes_down),
        clipRoutes(lines, p => !inCanvas(p))
      );
    }
    else {
      // mountains
      offset = 1 + rand(10);
      incr = mix(3, incr, rand() * rand() * rand());
      freq1 *= 0.5 + rand();
      freq2 *= 0.5 + rand();
      freq3 *= 0.5 + rand();
      let maxy = HEIGHT * (0.35 + rand(0.1));
      let local_mask = newPaintMask(precision, WIDTH, HEIGHT);
      let lines = []
      let clipy = HEIGHT / 2 + pad;
      // raise mountain lines
      let should_stop = 0
      let base_y = HEIGHT / 2 + rand(20);
      let centereffect = rand(150);
      for (let i = 0; i < 999 && !should_stop; i++) {
        let line = [];
        for (let x = 0; x < WIDTH + incr; x += incr) {
          let dy = centereffect * pow(2 * abs(x / WIDTH - 0.5), 2);
          let y = base_y + dy;
          let p3 = perlin3(
            2 * x * freq1,
            y * freq1,
            seed
          );
          y -= amp3 * p3;
          let p2 = perlin3(
            x * freq2,
            y * freq2,
            seed + 7.7 + p3
          );
          y -= amp2 * p2;
          let p1 = perlin3(
            x * freq3,
            2 * y * freq3,
            seed / 0.3 + p2 + 0.5 * p3,
          );
          y -= amp1 * p1;
          push(line, [x, min(y, clipy)]);
          if (y < maxy) {
            should_stop = 1
          }
        }

        let poly = [[0, clipy], ...line, [WIDTH, clipy]];
        let rts = clipRoutes([[main_color, poly]], p => p[1] > HEIGHT / 2 || !inCanvas(p) || isPainted(local_mask, p));

        lines = concat(lines, rts);
        paintPolygon(local_mask, poly);
        base_y -= offset
      }

      // grow the paint mask
      yoffsetPaintMask(local_mask, pad);

      // mirror
      lines = concat(lines, lines.map(([clr, rt]) => [
        reverseColor ? (clr + 1) % 2 : clr, rt.map(([x, y]) => [x, HEIGHT - y])]));
      ymirrorPaintMask(local_mask);

      routes = clipRoutes(routes, p => !inCanvas(p) || isPainted(local_mask, p));
      routes = concat(routes, lines);

    }
  }

  function square(
    [cx, cy],
    size
  ) {
    let s = size / 2;
    let poly = [
      [cx - s, cy - s],
      [cx + s, cy - s],
      [cx + s, cy + s],
      [cx - s, cy + s],
      [cx - s, cy - s],
    ];
    return poly;
  }

  function add_cordon(clr, from, fromr, to, tor, dist = 5) {
    let l = euclidianDist(from, to);
    let dir = 0;
    for (let v = 0; v < l; v += dist) {
      let m = dir ? v / l : 1 - v / l;
      let r = mix(fromr, tor, m);
      let cx = mix(from[0], to[0], m);
      let cy = mix(from[1], to[1], m);
      let route = [];
      let count = 64;
      for (let i = 0; i <= count; i++) {
        push(route, [
          cx + r * cos(i * twopi / count),
          cy + r * sin(i * twopi / count)
        ])
      }
      routes = concat(routes,
        clipRoutes([[clr, route]], isOutside)
      );
      paintCircle(mask, [cx, cy], r);
      dir = !dir;
    }
  }

  function euclidianDist(a, b) {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    return sqrt(dx * dx + dy * dy);
  }

  function add_spiral(clr, [x, y], maxr, dr, halo) {
    let approx = 0.2;
    let route = [];
    let a = 0;
    let r = maxr + dr;
    while (r > 0.05) {
      let actualr = min(r, maxr);
      let p = [x + actualr * cos(a), y + actualr * sin(a)];
      let l = route.length;
      if (l === 0 || euclidianDist(route[l - 1], p) > approx) {
        push(route, p);
      }
      let da = 1.0 / (r + 8.0);
      a += da;
      if (!dr && a > twopi) {
        break;
      }
      a %= twopi;
      r -= dr * da / twopi;
    }

    routes = concat(routes,
      clipRoutes([[clr, route]], isOutside)
    );

    paintCircle(mask, [x, y], maxr + halo);
  }

  function add_square(
    clrindex,
    fillTechnique,
    [cx, cy],
    rotation,
    size,
  ) {

    let rts = [];
    let s = size / 2;

    push(rts, [
      clrindex,
      [
        [-s, -s],
        [s, -s],
        [s, s],
        [-s, s],
        [-s, -s],
      ]
    ]);

    if (fillTechnique === FILL_FULL || fillTechnique === FILL_HSLIDE) {
      let incr = fillTechnique === FILL_HSLIDE ? hhetching : 0.5;
      let pad = 0.2;
      let route = []
      let y = -s + pad;
      let reverse = false;
      while (y < s - pad / 2.0) {
        if (!reverse) {
          push(route, [-s + pad, y]);
          push(route, [s - pad, y]);
        } else {
          push(route, [s - pad, y]);
          push(route, [-s + pad, y]);
        }
        y += incr;
        reverse = !reverse;
      }
      push(rts, [clrindex, route]);
    }
    else if (fillTechnique === FILL_VSLIDE) {
      let incr = vhetching;
      let pad = 0.2;
      let route = []
      let x = -s + pad;
      let reverse = false;
      while (x < s - pad / 2.0) {
        if (!reverse) {
          push(route, [x, -s + pad]);
          push(route, [x, s - pad]);
        } else {
          push(route, [x, s - pad]);
          push(route, [x, -s + pad]);
        }
        x += incr;
        reverse = !reverse;
      }
      push(rts, [clrindex, route]);
    }
    else if (fillTechnique === FILL_CONCENTRIC) {
      let incr = 1;
      let v = s;
      while (v > 0) {
        push(rts, [clrindex, [
          [-v, -v],
          [v, -v],
          [v, v],
          [-v, v],
          [-v, -v],
        ]]);
        v -= incr;
      }
    }

    rts = rts.map(([clrindex, route]) =>
      [clrindex, route.map(([x, y]) => {
        let r = rotate([x, y], rotation);
        return [r[0] + cx, r[1] + cy]
      })])

    routes = concat(routes, clipRoutes(rts, isOutside));

    // first route is the polygon too
    paintPolygon(mask, rts[0][1]);
  }

  function add_obj_polymerge(
    count,
    has_rayon,
    has_hole,
    main_poly_size,
    clrindex,
    [cx, cy],
    radius,
  ) {
    let polys = [];
    let rayon_poly = null;
    let main_index = rand(count) | 0;
    let digging_polygon;
    let main_polygon;
    let main_is_solid;
    let shape_motion_count = rand() * rand(16) | 0;
    let shape_motion_offset = 2 + rand() * rand(10);

    for (let i = 0; i < count; i++) {
      let is_main = i === main_index;
      let poly_size =
        is_main ? main_poly_size : 3 + (rand(5) * rand() + rand(80) * (rand() < 0.02)) | 0;
      let solid = rand(2) | 0;
      let poly = [];
      let angbase = rand(twopi);
      let rad = radius * mix(rand() * rand(), 1.0, (i + 0.5) / count);
      let randomness = i == 0 ? 0 : max(rand() - 0.2, 0);
      for (let j = 0; j < poly_size; j++) {
        let angle = (j + angbase + rand(randomness)) * twopi / poly_size;
        let c = cos(angle);
        let s = sin(angle);
        let amp = rad * mix(1, 0.5 + 0.5 * rand(), randomness);
        let x = cx + amp * c;
        let y = cy + amp * s;
        push(poly, [x, y]);
      }

      let route = poly.slice(0);
      push(route, route[0]);

      routes = concat(routes, clipRoutes([[clrindex, route]], isOutside));


      if (solid) {
        let rts = routes_fill_for_polygon(clrindex, poly, 0.5 + rand(3) * (0.5 + 0.5 * rand()), rand(0.5), rand(twopi));
        routes = concat(routes, clipRoutes(rts, isOutside));
      }

      let painted = false;

      if (is_main) {
        painted = true;
        paintPolygon(mask, poly);

        if (shape_motion_count) {
          let first_p = poly[0];
          let dx = -(cx - first_p[0]);
          let dy = -(cy - first_p[1]);
          let l = sqrt(dx * dx + dy * dy);
          dx /= l;
          dy /= l;
          for (let j = 0; j < shape_motion_count; j++) {
            let offset = shape_motion_offset * (j + 1);
            let rt = route.map(([x, y]) => [x + dx * offset, y + dy * offset]);
            routes = concat(routes, clipRoutes([[clrindex, rt]], isOutside));
            paintPolygon(mask, rt);
          }
        }

        if (has_rayon) {
          rayon_poly = poly.slice(0);
          let rts = [];
          let stroke = 3 + rand(20) * rand();
          let segments = []
          let divisions = 2 + (rand(10) * rand() | 0);
          for (let p of rayon_poly) {
            let dx = p[0] - cx;
            let dy = p[1] - cy;
            let length = sqrt(dx * dx + dy * dy);
            let d = 1000;
            let segment = [
              [cx, cy],
              [p[0] + dx * d, p[1] + dy * d]
            ];
            push(segments, segment);
            let offsets = [];
            for (let i = 0; i < divisions; i++) {
              push(offsets, stroke * (0.5 - i / (divisions - 1)));
            }
            // parallel segments
            for (let offset of offsets) {
              let ndx = dx / length;
              let ndy = dy / length;
              let diffx = ndy * offset;
              let diffy = -ndx * offset;
              let segment2 = segment.map(([x, y]) => [x + diffx, y + diffy]);
              push(rts, [clrindex, segment2]);
            }
          }
          routes = concat(routes, clipRoutes(rts, isOutside));
          for (let segment of segments) {
            paintSegment(mask, segment, stroke);
          }
        }
      }

      if (rand() < 0.3 && !painted) {
        painted = true;
        paintPolygon(mask, poly);
      }

      if (is_main) {
        main_polygon = route;
        main_is_solid = solid;
      }

      if (has_hole && rand(2 * pow(4 * rad / radius, 2)) < 1 && count > 1) {
        digging_polygon = route;
        if (!painted && rand() < 0.5) {
          painted = true;
          paintPolygon(mask, poly);
        }
      }

      if (!painted) {
        push(polys, poly);
      }
    }

    for (let poly of polys) {
      paintPolygon(mask, poly)
    }

    if (has_hole) {
      if (!digging_polygon) {
        digging_polygon = main_polygon;
        if ((!has_rayon || count < 9) && rand() < (main_is_solid ? 0.5 : 0.9)) {
          // make an alternative dig INSIDE that is more elegant
          let mul = 0.2 + rand(0.6);
          let copy = digging_polygon.map(([x, y]) => {
            let dx = x - cx;
            let dy = y - cy;
            dx *= mul;
            dy *= mul;
            return [x - dx, y - dy]
          });
          digging_polygon = copy;
        }
      }
    }


    if (digging_polygon) {
      routes = concat(routes, clipRoutes(routes, p => polygonIncludesPoint(digging_polygon, p)));
      push(routes, [clrindex, digging_polygon]);
      paintPolygon(mask, digging_polygon, false);
    }
  }

  function shuffle(array) {
    return array.map((v, i) => [v, rand()])
      .sort((a, b) => a[1] - b[1])
      .map((v) => v[0])
  }

  return { paperSeed, routes, dark };
}

function routes_fill_for_polygon(clrindex, polygon, incr, pad, angle) {
  let polyrotate = polygon.map(([x, y]) => rotate([x, y], angle));
  let routes = []
  let minx = INF;
  let miny = INF;
  let maxx = -INF;
  let maxy = -INF;
  for (let i = 0; i < polyrotate.length; i++) {
    let [x, y] = polyrotate[i];
    minx = min(minx, x);
    miny = min(miny, y);
    maxx = max(maxx, x);
    maxy = max(maxy, y);
  }

  let y = miny + pad;
  while (y < maxy - pad / 2.0) {
    push(routes, [clrindex, [
      [minx + pad, y],
      [maxx - pad, y],
    ].map(p => rotate(p, -angle))]);
    y += incr;
  }


  return clipRoutes(routes, p => !polygonIncludesPoint(polygon, p));
}


function interpolate_y(polyline, x) {
  // find in polyline (many points) the y interp where x is between two points
  let p = polyline[0];
  let x1 = p[0];
  let y1 = p[1];
  for (let i = 1; i < polyline.length; i++) {
    let p = polyline[i];
    let x2 = p[0];
    let y2 = p[1];
    if (x1 <= x && x <= x2) {
      return mix(y1, y2, (x - x1) / (x2 - x1));
    }
    x1 = x2;
    y1 = y2;
  }
  return y1;
}

function mix(a, b, x,) {
  return a * (1 - x) + b * x;
}

function lerpPoint([ax, ay], [bx, by], m) {
  return [ax * (1 - m) + bx * m, ay * (1 - m) + by * m];
}

function clipRoutes(inputRoutes, isOutside, stepping = 0.5, dichotomicIterations = 4) {
  function search(inside, outside, n) {
    let a = inside;
    let b = outside;
    for (let i = 0; i < n; i++) {
      let middle = lerpPoint(a, b, 0.5);
      if (isOutside(middle)) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerpPoint(a, b, 0.5);
  }

  let routes = [];

  for (let [clr, input_route] of inputRoutes) {
    if (input_route.length < 2) {
      continue;
    }
    let prev = input_route[0];
    let prev_isOutside = isOutside(prev);
    let route = [];
    if (!prev_isOutside) {
      push(route, prev);
    }

    for (let i = 1; i < input_route.length; i++) {
      let p = input_route[i];
      let static_prev = prev;
      let dx = p[0] - prev[0];
      let dy = p[1] - prev[1];
      let d = sqrt(dx * dx + dy * dy);
      let vx = dx / d;
      let vy = dy / d;
      let iterations = ceil(d / stepping);
      let v = 0;
      for (let j = 0; j < iterations; j++) {
        v = min(v + stepping, d);
        let q = [static_prev[0] + vx * v, static_prev[1] + vy * v];
        let q_isOutside = isOutside(q);
        if (prev_isOutside !== q_isOutside) {
          let intersection =
            (prev_isOutside) ? search(q, prev, dichotomicIterations) : search(prev, q, dichotomicIterations);

          if (q_isOutside) {
            push(route, intersection);
            if (route.length > 1) {
              push(routes, [clr, route]);
            }
            route = [];
          } else {
            push(route, intersection);
          }
          prev_isOutside = q_isOutside;
        }
        prev = q;
      }

      if (!prev_isOutside) {
        push(route, p);
      }
    }

    if (route.length > 1) {
      push(routes, [clr, route]);
    }
  }

  return routes;
}

/**
 * 
 * @param {*} p precision
 * @param {*} w width
 * @param {*} h height
 * @returns 
 */
function newPaintMask(p, w, h) {
  let wi = floor(w / p);
  let hi = floor(h / p);
  let m = new Array(wi * hi).fill(false);
  return { m, p, w, h };
}

function isPainted({ m, w, h, p }, point) {
  let [x, y] = point;
  if (x <= 0 || x >= w || y <= 0 || y >= h) {
    return false;
  }
  let wi = floor(w / p);
  let xi = floor(x / p);
  let yi = floor(y / p);
  return m[xi + yi * wi];
}

function paintPolygon({ m, w, h, p }, polygon, value = true) {
  let minx = w;
  let miny = h;
  let maxx = 0;
  let maxy = 0;
  for (let i = 0; i < polygon.length; i++) {
    let [x, y] = polygon[i];
    minx = min(minx, x);
    miny = min(miny, y);
    maxx = max(maxx, x);
    maxy = max(maxy, y);
  }
  minx = max(0, minx)
  miny = max(0, miny)
  maxx = min(w, maxx)
  maxy = min(h, maxy)
  let minxi = floor(minx / p);
  let minyi = floor(miny / p);
  let maxxi = floor(maxx / p);
  let maxyi = floor(maxy / p);
  let wi = floor(w / p);
  for (let x = minxi; x < maxxi; x++) {
    for (let y = minyi; y < maxyi; y++) {
      let point = [x * p, y * p];
      if (polygonIncludesPoint(polygon, point)) {
        m[x + y * wi] = value;
      }
    }
  }
}


function ymirrorPaintMask({ m, w, h, p }) {
  let wi = floor(w / p);
  let hi = floor(h / p);
  for (let x = 0; x < wi; x++) {
    for (let y = 0; y < hi; y++) {
      if (m[x + (hi - y - 1) * wi]) {
        m[x + y * wi] = true;
      }
    }
  }
}

function yoffsetPaintMask({ m, w, h, p }, off) {
  let offy = floor(off / p);
  let wi = floor(w / p);
  let hi = floor(h / p);
  let copy = m.slice(0);
  for (let x = 0; x < wi; x++) {
    for (let y = 0; y < hi; y++) {
      if (copy[x + max(0, min(hi - 1, y + offy)) * wi]) {
        m[x + y * wi] = true;
      }
    }
  }
}


function paintSegment(mask, [from, to], strokewidth) {
  let dx = to[0] - from[0];
  let dy = to[1] - from[1];
  let d = sqrt(dx * dx + dy * dy);
  let vx = dx / d;
  let vy = dy / d;
  let nx = -vy;
  let ny = vx;
  let p1 = [from[0] + nx * strokewidth / 2, from[1] + ny * strokewidth / 2];
  let p2 = [from[0] - nx * strokewidth / 2, from[1] - ny * strokewidth / 2];
  let p3 = [to[0] - nx * strokewidth / 2, to[1] - ny * strokewidth / 2];
  let p4 = [to[0] + nx * strokewidth / 2, to[1] + ny * strokewidth / 2];
  paintPolygon(mask, [p1, p2, p3, p4]);
}

function paintCircle({ m, w, h, p }, center, r) {
  let [cx, cy] = center;
  let minx = max(0, cx - r);
  let miny = max(0, cy - r);
  let maxx = min(w, cx + r)
  let maxy = min(h, cy + r);
  let minxi = floor(minx / p);
  let minyi = floor(miny / p);
  let maxxi = floor(maxx / p);
  let maxyi = floor(maxy / p);
  let wi = floor(w / p);
  let r2 = r * r;
  for (let x = minxi; x < maxxi; x++) {
    for (let y = minyi; y < maxyi; y++) {
      let point = [x * p, y * p];
      let dist = ((point[0] - cx) * (point[0] - cx) + (point[1] - cy) * (point[1] - cy));
      if (dist < r2) {
        m[x + y * wi] = true;
      }
    }
  }
}


let INF = 1e9;

function polygonIncludesPoint(polygon, [x, y]) {
  let c = false;
  let l = polygon.length;
  for (let i = 0; i < l; i++) {
    let j = (i + 1) % l;
    let [ax, ay] = polygon[i];
    let [bx, by] = polygon[j];
    if ((ay > y) !== (by > y) && x < ((bx - ax) * (y - ay)) / (by - ay) + ax) {
      c = !c;
    }
  }
  return c;
}

function inCanvas([x, y]) {
  return x >= PAD && x < WIDTH - PAD && y >= PAD && y < HEIGHT - PAD
}

function rotate([x, y], angle) {
  let c = cos(angle);
  let s = sin(angle);
  return [c * x - s * y, s * x + c * y];
}

function makeSVG(a, rendererMode) {
  let style = rendererMode
    ? 'opacity="0.5"'
    : 'style="mix-blend-mode:multiply"';
  let colors = a.dark ? COLORS_DARK : COLORS_LIGHT;
  let svgW = rendererMode ? PX(rendererMode[0]) : MM(WIDTH);
  let svgH = rendererMode ? PX(rendererMode[1]) : MM(HEIGHT);
  return `<svg style="background:${rendererMode ? "#fff" : a.dark ? "#222" : "#eee"};${rendererMode ? "" : "width:100%;height:100%"
    }" viewBox="0 0 ${WIDTH} ${HEIGHT}" height="${svgH}" width="${svgW}" xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape">
  ${mapjoin(colors,
      ([name, main], i) => {
        let rts = a.routes
          .filter(([ci]) => ci === i)
        return !rts[0] ? "" : `<g inkscape:groupmode="layer" inkscape:label="${i} ${name}" stroke="${rendererMode
          ? MASKS[i]
          : "rgb(" + mapjoin(main, (n) => (n * 255) | 0, ",") + ")"
          }" fill="none" stroke-linejoin="round" stroke-width="0.5">${mapjoin(rts,
            ([ci, route]) =>
              `<path ${style} d="${mapjoin(
                route,
                ([x, y], i) =>
                  `${i === 0 ? "M" : "L"}${x.toFixed(2)},${y.toFixed(2)}`
                ,
                " "
              )}" />`
            , "\n")
          }</g>`
      }, "\n")
    }
</svg>`;
}

var grad3 = [
  [1, 1, 0],
  [-1, 1, 0],
  [1, -1, 0],
  [-1, -1, 0],
  [1, 0, 1],
  [-1, 0, 1],
  [1, 0, -1],
  [-1, 0, -1],
  [0, 1, 1],
  [0, -1, 1],
  [0, 1, -1],
  [0, -1, -1],
];

var p = [
  151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140,
  36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
  75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237,
  149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48,
  27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105,
  92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73,
  209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
  164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38,
  147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189,
  28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101,
  155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
  178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12,
  191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
  181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
  138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215,
  61, 156, 180,
];

var perm = new Array(512);
var gradP = new Array(512);
for (var i = 0; i < 256; i++) {
  var v = p[i];
  perm[i] = perm[i + 256] = v;
  gradP[i] = gradP[i + 256] = grad3[v % 12];
}

function fade(t) {
  return t * t * t * (t * (t * 6 - 15) + 10);
}

function lerp(a, b, t) {
  return (1 - t) * a + t * b;
}


function dot3([gx, gy, gz], x, y, z) {
  return gx * x + gy * y + gz * z;
}

function perlin3(x, y, z) {
  let X = x | 0,
    Y = y | 0,
    Z = z | 0;
  x = x - X;
  y = y - Y;
  z = z - Z;
  X = X & 255;
  Y = Y & 255;
  Z = Z & 255;
  let n000 = dot3(gradP[X + perm[Y + perm[Z]]], x, y, z);
  let n001 = dot3(gradP[X + perm[Y + perm[Z + 1]]], x, y, z - 1);
  let n010 = dot3(gradP[X + perm[Y + 1 + perm[Z]]], x, y - 1, z);
  let n011 = dot3(gradP[X + perm[Y + 1 + perm[Z + 1]]], x, y - 1, z - 1);
  let n100 = dot3(gradP[X + 1 + perm[Y + perm[Z]]], x - 1, y, z);
  let n101 = dot3(gradP[X + 1 + perm[Y + perm[Z + 1]]], x - 1, y, z - 1);
  let n110 = dot3(gradP[X + 1 + perm[Y + 1 + perm[Z]]], x - 1, y - 1, z);
  let n111 = dot3(gradP[X + 1 + perm[Y + 1 + perm[Z + 1]]], x - 1, y - 1, z - 1);
  let u = fade(x);
  let v = fade(y);
  let w = fade(z);
  return lerp(
    lerp(lerp(n000, n100, u), lerp(n001, n101, u), w),
    lerp(lerp(n010, n110, u), lerp(n011, n111, u), w),
    v
  );
};



let makeSVGDataImage = (svg) => "data:image/svg+xml;base64," + btoa(svg);

let adaptiveSvgWidth = (width) => max(64, ceil(width / 64) * 64);

let calcSizes = (width, height) => {
  let dpr = WINDOW.devicePixelRatio || 1;
  let W = width;
  let H = height;
  H = min(H, W / ratio) | 0;
  W = min(W, H * ratio) | 0;
  let w = min(MAX, dpr * W);
  let h = min(MAX, dpr * H);
  h = min(h, w / ratio) | 0;
  w = min(w, h * ratio) | 0;
  let svgW = adaptiveSvgWidth(w);
  let svgWidth = svgW;
  let svgHeight = (svgW / ratio) | 0;
  return [W, H, w, h, svgWidth, svgHeight];
};

let main = () => {
  let a = art();
  let colors = a.dark ? COLORS_DARK : COLORS_LIGHT;
  let { paperSeed } = a;

  let DOC = document;
  let BODY = DOC.body;
  BODY.style.background = a.dark ? "#000" : "#fff";
  let createElement = (e) => DOC.createElement(e);
  let append = (n, e) => n.appendChild(e);
  let appendBody = (e) => append(BODY, e);

  let svg = makeSVG(a);
  let container = createElement("div");

  let bgImage = createElement("img");
  bgImage.src = makeSVGDataImage(svg);
  let ABSOLUTE = "absolute";
  let CENTER = "center";
  let HUNDREDPC = "100%";
  let sharedStyle = { position: ABSOLUTE, opacity: 0 };
  assign(container.style, sharedStyle);
  assign(bgImage.style, {
    top: 0,
    left: 0,
    width: HUNDREDPC,
    height: HUNDREDPC,
    ...sharedStyle,
  });

  let V = ({ viewportWidth, viewportHeight }) => [
    viewportWidth,
    viewportHeight,
  ];

  let canvas = createElement("canvas");
  canvas.style.pointerEvents = "none";

  assign(BODY.style, {
    display: "flex",
    alignItems: CENTER,
    justifyContent: CENTER,
  });

  append(container, bgImage);
  appendBody(container);
  appendBody(canvas);

  let regl = createREGL(canvas);

  let vert = `precision mediump float;attribute vec2 p;varying vec2 uv; void main(){ uv = p; gl_Position = vec4(2. * p - 1., 0, 1); } `;

  let framebuffer = regl.framebuffer();

  let paper = regl({
    framebuffer,
    frag: "precision highp float;varying vec2 uv;uniform vec2 V;uniform float G,S;void a(inout vec2 b,float c){b=cos(c)*b+sin(c)*vec2(b.y,-b.x);}float d(float b){b=fract(b*.1031);b*=b+33.33;b*=b+b;return fract(b);}float d(vec2 b){vec3 e=fract(vec3(b.xyx)*.1031);e+=dot(e,e.yzx+33.33);return fract((e.x+e.y)*e.z);}float f(float g){float h=floor(g);float i=fract(g);float j=i*i*(3.-2.*i);return mix(d(h),d(h+1.),j);}float f(vec2 g){vec2 h=floor(g);vec2 i=fract(g);float c=d(h);float k=d(h+vec2(1.,0.));float l=d(h+vec2(0.,1.));float m=d(h+vec2(1.,1.));vec2 j=i*i*(3.-2.*i);return mix(c,k,j.g)+(l-c)*j.y*(1.-j.g)+(m-k)*j.g*j.y;}const mat2 n=mat2(.4,.7,-.7,.4);float o(in vec2 g){float i=2.;float p=.55;float c=0.;float k=.5;for(int h=0;h<3;h++){float q=f(g);c+=k*q;k*=p;g=i*g;}return c;}float r(in vec2 g){ivec2 b=ivec2(floor(g));vec2 i=fract(g);ivec2 s;vec2 t;float u=8.;for(int v=-1;v<=1;v++)for(int h=-1;h<=1;h++){ivec2 k=ivec2(h,v);vec2 w=vec2(k)+d(vec2(b+k))-i;float m=dot(w,w);if(m<u){u=m;t=w;s=k;}}u=8.;for(int v=-2;v<=2;v++)for(int h=-2;h<=2;h++){ivec2 k=s+ivec2(h,v);vec2 w=vec2(k)+d(vec2(b+k))-i;float m=dot(.5*(t+w),normalize(w-t));u=min(u,m);}return u;}float x(vec2 b,float y,float z){a(b,2.);float c=smoothstep(.02,.16,.13*o(z+.3*b*y)+r(.5*y*b));float k=smoothstep(0.,.15,abs(o(-2.*b*y)-.5)-.01);return .4*k+.6*c;}void main(){vec2 A=V/min(V.x,V.y);vec2 b=.5+(uv-.5)*A;float B=x(b,G,S);gl_FragColor=vec4(B,B,B,1.);}" /*`precision highp float;
varying vec2 uv;
uniform vec2 V;
uniform float G, S;
void pR(inout vec2 p, float a) {
  p = cos(a) * p + sin(a) * vec2(p.y, -p.x);
}
float hash(float p) {
  p = fract(p * .1031);
  p *= p + 33.33;
  p *= p + p;
  return fract(p);
}
float hash(vec2 p) {
vec3 p3 = fract(vec3(p.xyx) * .1031);
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
const mat2 m2 = mat2(0.4, 0.7, -0.7, 0.4);
float fbm( in vec2 x) {
float f = 2.0;
float s = 0.55;
float a = 0.0;
float b = 0.5;
  for (int i = 0; i < 3; i++ ) {
float n = noise(x);
    a += b * n;
    b *= s;
    x = f * x;
  }
  return a;
}
float vn(in vec2 x) {
ivec2 p = ivec2(floor(x));vec2  f = fract(x);
ivec2 mb;vec2 mr;
float res = 8.0;
  for (int j = -1; j <= 1; j++ )
  for (int i = -1; i <= 1; i++ ) {
ivec2 b = ivec2(i, j);vec2  r = vec2(b) + hash(vec2(p + b)) - f;float d = dot(r, r);
    if (d < res) { res = d; mr = r; mb = b; }
  }
  res = 8.0;
  for (int j = -2; j <= 2; j++ )
  for (int i = -2; i <= 2; i++ ) {
ivec2 b = mb + ivec2(i, j);
vec2  r = vec2(b) + hash(vec2(p + b)) - f;
float d = dot(0.5 * (mr + r), normalize(r - mr));
    res = min(res, d);
  }
  return res;
}
float pp(vec2 p, float z, float S) {
  pR(p, 2.);
float a = smoothstep(0.02, 0.16, 0.13 * fbm(S + 0.3 * p * z) + vn(0.5 * z * p));
float b = smoothstep(0.0, 0.15, abs(fbm(-2.0 * p * z) - 0.5) - 0.01);
  return 0.4 * b + 0.6 * a;
}

void main() {
vec2 ratio = V / min(V.x, V.y);
vec2 p = 0.5 + (uv - 0.5) * ratio;
float t = pp(p, G, S);
  gl_FragColor = vec4(t, t, t, 1.0);
} `*/,
    vert,
    attributes: {
      p: TRIANGLE,
    },
    uniforms: {
      S: paperSeed,
      G: 500,
      V,
    },
    count: 3,
  });

  let C1 = colors[0][1];
  let C1H = colors[0][2];
  let C2 = (colors[1] || colors[0])[1];
  let C2H = (colors[1] || colors[0])[2];
  let C3 = (colors[2] || colors[0])[1];
  let C3H = (colors[2] || colors[0])[2];
  let prop = regl.prop;

  let render = regl({
    frag: `precision highp float;varying vec2 uv;uniform vec3 B;uniform float G,L,T,S;uniform vec3 C1,C1H,C2,C2H,C3,C3H;uniform sampler2D t,P;vec3 a(float b,vec3 c,vec3 d){float e=smoothstep(0.3,.0,b);return mix(vec3(${a.dark ? "0." : "1."}),mix(c,d,e),smoothstep(1.,.5,b));}void main(){vec2 f=uv;vec2 g=f.xy-.5;float h=abs(g.y);float i=smoothstep(.3,.0,abs(fract(h-.5*T)-0.5));vec4 j=texture2D(P,f);float k=j.r;vec4 l=mix(vec4(1.),texture2D(t,f),smoothstep(h,h+.01,.5*(T-1.)));vec3 c=a(l.r,C1,C1H);vec3 d=a(l.g,C2,C2H);vec3 m=a(l.b,C3,C3H);vec3 n=${a.dark
      ? "(c+d+m)*(1.+L*i)+G*k"
      : "min(vec3(1.),c*d*m*(1.+L*i))+G*mix(1.,.5,step(.5,k))*(.5-k)"}+B+L*i/2.;gl_FragColor=vec4(n,1.);}`,
    vert,
    attributes: {
      p: TRIANGLE,
    },
    uniforms: {
      T: prop("T"),
      t: prop("t"),
      P: framebuffer,
      S: paperSeed,
      C1,
      C1H,
      C2,
      C2H,
      C3,
      C3H,
      G: 0.1,
      L: a.dark ? 0.1 : 0.05,
      B: a.dark ? [0.1, 0.1, 0.14] : [0, -0.005, -0.01],
      V,
    },
    count: 3,
  });

  let img = createElement("img");
  let lastPaperWidth;
  let lastSVGWidth;
  let txParam = (data) => ({ data, min: LINEAR, mag: LINEAR, flipY: true });

  let tex = regl.texture(txParam(img));
  let resize = (width, height) => {
    let [W, H, w, h, svgWidth, svgHeight] = calcSizes(width, height);
    canvas.width = w;
    canvas.height = h;
    container.style.width = canvas.style.width = PX(W);
    container.style.height = canvas.style.height = PX(H);
    if (lastPaperWidth !== w) {
      framebuffer.resize(w, h);
      paper();
    }
    if (lastSVGWidth !== svgWidth) {
      lastSVGWidth = svgWidth;
      img.onload = () => tex(txParam(img));
      img.src = makeSVGDataImage(makeSVG(a, [svgWidth, svgHeight]));
      img.width = svgWidth;
      img.height = svgHeight;
    }
  };

  let r = (onresize = () => resize(WINDOW.innerWidth, WINDOW.innerHeight));
  r();

  let startT;
  regl.frame(({ time }) => {
    if (!startT) startT = time;
    render({ T: time - startT, t: tex });
  });

}


