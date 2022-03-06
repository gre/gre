let TD = tokenData;
let t, s;
let S = Uint32Array.from(
    [0, 0, 0, 0].map((i) => parseInt(TD.hash.substr(i * 8 + 5, 8), 16))
  ),
  R = (a = 1) =>
    a *
    ((t = S[3]),
    (S[3] = S[2]),
    (S[2] = S[1]),
    (s = S[1] = S[0]),
    (t ^= t << 11),
    (S[0] ^= t ^ (t >>> 8) ^ (s >>> 19)),
    S[0] / 2 ** 32);

// CONSTANTS //

let LINEAR = "linear";
let PX = (px) => px + "px";
let MM = (mm) => mm + "mm";
let TRIANGLE = [-2, 0, 0, -2, 2, 2];
let pi = Math.PI;
let twopi = 2 * pi;
let WINDOW = this;
let width = 297;
let height = 210;
let MAX = 4096;
let highNumber = 999;
let ratio = width / height;
let pad = 10;
let precision = 0.2;
let totalShape = 7;
let MASKS = ["#0FF", "#F0F", "#FF0"];
let COLORS = [
  ["Black", [0.2, 0.2, 0.2], [0, 0, 0]],
  ["Poppy Red", [0.98, 0.3, 0.3], [0.8, 0.2, 0.2]],
  ["Turquoise", [0, 0.72, 0.98], [0, 0.52, 0.8]],
  ["Amazing Amethyst", [0.67, 0.42, 0.75], [0.35, 0.15, 0.45]],
  ["Indigo", [0.45, 0.55, 0.7], [0.2, 0.3, 0.4]],
  ["Hope Pink", [1, 0.4, 0.75], [0.85, 0.22, 0.5]],
  ["Amber", [1, 0.7, 0.2], [1, 0.45, 0.0]],
  ["Pumpkin", [1, 0.42, 0.05], [0.9, 0.3, 0.0]],
  ["Aurora Borealis", [0.0, 0.6, 0.6], [0.0, 0.3, 0.3]],
];

// HELPERS //
let { abs, atan2, cos, sin, sqrt, pow, ceil, min, max } = Math;
let assign = Object.assign;
let push = (arr, el) => arr.push(el);
let mapjoin = (arr, f, c) => arr.map(f).join(c);
let mix = (a, b, x) => a * (1 - x) + b * x;

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

let collides_point_circle = ([x, y], [cx, cy, r]) => {
  let dx = cx - x;
  let dy = cy - y;
  return dx * dx + dy * dy <= r * r;
};
let collides_line_circle = (a, b, circle) => {
  if (collides_point_circle(a, circle) || collides_point_circle(b, circle)) {
    return true;
  }
  let [x1, y1] = a;
  let [x2, y2] = b;
  let [cx, cy] = circle;
  let dx = x2 - x1;
  let dy = y2 - y1;
  let lcx = cx - x1;
  let lcy = cy - y1;
  let d2 = dx * dx + dy * dy;
  let px = dx;
  let py = dy;
  if (d2 > 0) {
    let dp = (lcx * dx + lcy * dy) / d2;
    px *= dp;
    py *= dp;
  }
  let nearest = [x1 + px, y1 + py];
  let p2 = px * px + py * py;
  return (
    collides_point_circle(nearest, circle) && p2 <= d2 && px * dx + py * dy >= 0
  );
};

// in my case, a polygon is represented by all their points + one last point that is == to first.
let create_polygon = (pts) => pts.concat([pts[0]]);

let centroid = (polygon) => {
  let x = 0;
  let y = 0;
  let count = polygon.length - 1;
  for (let i = 0; i < count; i++) {
    let [px, py] = polygon[i];
    x += px;
    y += py;
  }
  return [x / count, y / count];
};

let bounding_rect = (polygon) => {
  let [x1, y1] = polygon[0];
  let x2 = x1;
  let y2 = y1;
  let length = polygon.length - 1;
  for (let i = 1; i < length; i++) {
    let [x, y] = polygon[i];
    if (x < x1) x1 = x;
    else if (x > x2) x2 = x;
    if (y < y1) y1 = y;
    else if (y > y2) y2 = y;
  }
  return [x1, y1, x2 - x1, y2 - y1];
};

let signed_area = (poly) => {
  let length = poly.length - 1,
    total = 0,
    i = 0;
  while (i < length) {
    let [x1, y1] = poly[i];
    let [x2, y2] = poly[++i];
    total += (x1 * y2 - x2 * y1) / 2;
  }
  return abs(total);
};

let contains_poly = (poly, [x, y]) => {
  let inside = false;
  let length = poly.length - 1;
  for (let i = 0, j = length - 1; i < length; j = i++) {
    let [xi, yi] = poly[i];
    let [xj, yj] = poly[j];
    let intersect =
      yi > y != yj > y && x < ((xj - xi) * (y - yi)) / (yj - yi) + xi;
    if (intersect) inside = !inside;
  }
  return inside;
};

let translate_polygon = (polygon, x, y) =>
  polygon.map(([px, py]) => [px + x, py + y]);

let route_sorted = (candidates, [dx, dy]) =>
  [...candidates].sort(
    ([x1, y1], [x2, y2]) => x1 * dx + y1 * dy - x2 * dx - y2 * dy
  );

let route_spiral = (candidates) => {
  let result = [];
  let p = candidates[0];
  if (!p) return result;
  let pindex = 0;
  for (let i = 1; i < candidates.length; i++) {
    let c = candidates[i];
    if (c[1] < p[1]) {
      p = c;
      pindex = i;
    }
  }
  push(result, p);
  let a = 0;
  let list = [...candidates];
  while (list.length) {
    list.splice(pindex, 1);
    let m;
    let mv = twopi;
    for (let i = 0; i < list.length; i++) {
      let q = list[i];
      let qp_angle = atan2(p[1] - q[1], p[0] - q[0]);
      let v = (twopi + qp_angle - a) % twopi;
      if (v < mv) {
        mv = v;
        m = q;
        pindex = i;
      }
    }
    if (m) {
      a = atan2(p[1] - m[1], p[0] - m[0]);
      p = m;
      push(result, p);
    } else {
      break;
    }
  }
  return result;
};

let samples_polygon_edge = (poly, samples, borderdist) => {
  let length = 0.0;
  let l = poly.length - 1;
  let dists = [];
  let cx = 0;
  let cy = 0;
  for (let i = 0; i < l; i++) {
    let [x1, y1] = poly[i];
    cx += x1;
    cy += y1;
    let [x2, y2] = poly[i + 1];
    let dx = x1 - x2;
    let dy = y1 - y2;
    let d = sqrt(dx * dx + dy * dy);
    length += d;
    push(dists, d);
  }
  cx /= l;
  cy /= l;
  let incr = length / samples;
  let points = [];
  let groups = [];
  for (let i = 0; i < l; i++) {
    let [x1, y1] = poly[i];
    let [x2, y2] = poly[i + 1];
    let d = dists[i];
    let dx = x2 - x1;
    let dy = y2 - y1;
    let inc = incr / d;
    for (let v = 0; v < 1; v += inc) {
      let px = x1 + v * dx,
        py = y1 + v * dy;
      let ddx = px - cx;
      let ddy = py - cy;
      let dist = sqrt(ddx * ddx + ddy * ddy);
      let d = R(borderdist) / dist;
      push(points, [mix(px, cx, d), mix(py, cy, d)]);
      push(groups, i);
    }
  }
  return [points, groups];
};

let pingpong = (pts, n, groups) => {
  let up;
  let routes = [];
  let route = [];
  let down = () => {
    if (up) {
      route = [up];
      up = 0;
    }
  };
  let save = (p) => {
    if (route.length > 1) {
      push(routes, route);
    }
    route = [];
    up = p;
  };

  let max_passage = 4;
  let res = 2;
  let W = (width * res) | 0;
  let H = (height * res) | 0;
  let passage = Array(W * H).fill(0);
  let travel = ([x1, y1], [x2, y2]) => {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let d = sqrt(dx * dx + dy * dy);
    for (let v = 0; v < 1; v += 1 / d) {
      let x = x1 + dx * v;
      let y = y1 + dy * v;
      let p = [x, y];
      if (++passage[((x * res) | 0) + W * ((y * res) | 0)] < max_passage) {
        down();
        push(route, p);
      } else {
        save(p);
      }
    }
  };

  let total = pts.length;
  let k = (total / n) | 0;
  let lastGroup;
  let lastP;
  for (let j = 0; j < k; j++) {
    for (let g = 0; g < n; g++) {
      let i = (g * k + j) % total;
      let p = pts[i];
      let group = groups[i];
      if (group === lastGroup) {
        save(p);
      } else {
        down();
        if (lastP) {
          travel(lastP, p);
        } else {
          push(route, p);
        }
      }
      lastGroup = group;
      lastP = p;
    }
  }
  save();
  return routes;
};

let randomize = (pts, groups, circles, circlesAvoidIterations) => {
  pts = pts
    .map((p, i) => [R(), p, groups[i]])
    .sort((a, b) => (a[2] === b[2] ? 8 : 1) * (b[0] - a[0]))
    .map((p) => p[1]);

  for (let j = 0; j < circlesAvoidIterations; j++) {
    for (let i = 1; i < pts.length - 1; i++) {
      for (let c of circles) {
        if (collides_line_circle(pts[i - 1], pts[i], c)) {
          let tmp = pts[i];
          pts[i] = pts[i + 1];
          pts[i + 1] = tmp;
          break;
        }
      }
    }
  }

  return pts;
};

let samples_polygon = (poly, samples, precision) => {
  let [x1, y1, w, h] = bounding_rect(poly);
  let x2 = x1 + w;
  let y2 = y1 + h;
  let list = [];
  for (let x = x1; x <= x2; x += precision) {
    for (let y = y1; y <= y2; y += precision) {
      let p = [x, y];
      if (contains_poly(poly, p)) {
        push(list, [R(), p]);
      }
    }
  }
  return list
    .sort((a, b) => b[0] - a[0])
    .slice(0, samples)
    .map((s) => s[1]);
};

let middle = ([x1, y1], [x2, y2]) => [(x1 + x2) / 2, (y1 + y2) / 2];

let hatching = (poly) => {
  let best = 0;
  let bestI;
  let l = poly.length - 1;
  for (let i = 0; i < l; i++) {
    let [x1, y1] = poly[i];
    let [x2, y2] = poly[i + 1];
    let dx = x2 - x1;
    let dy = y2 - y1;
    let d = dx * dx + dy * dy;
    if (d > best) {
      best = d;
      bestI = i;
    }
  }
  let p1 = poly[bestI];
  let p2 = poly[bestI + 1];
  let [x1, y1] = p1;
  let [x2, y2] = p2;
  let dx = x2 - x1;
  let dy = y2 - y1;
  let a = atan2(dy, dx);
  let ad = a + pi / 2;
  let ads = sin(ad);
  let adc = cos(ad);
  let route = [];
  let collide_edge = ([px, py], angle, excludeI) => {
    let ca = cos(angle);
    let sa = sin(angle);
    px += ca / highNumber;
    py += sa / highNumber;
    let p2 = [px + highNumber * ca, py + highNumber * sa];
    for (let i = 0; i < l; i++) {
      if (i === excludeI) continue;
      let end = collides_segment([px, py], p2, poly[i], poly[i + 1]);
      if (end) return end;
    }
  };
  let c = middle(p1, p2);
  let end = collide_edge(c, ad, bestI);
  if (!end) return route;
  let rev = 0;
  for (let i = 0; i < 999; i++) {
    let incr = 0.5 + R() * (R() - 0.5);
    c[0] += incr * adc;
    c[1] += incr * ads;
    let a1 = a + (rev ? pi : 0);
    let a2 = a + (rev ? 0 : pi);
    let p1 = collide_edge(c, a1);
    let p2 = collide_edge(c, a2);
    if (p1 && !p2) {
      p2 = collide_edge(p1, a1);
      rev = !rev;
    }
    if (p2 && !p1) {
      p1 = collide_edge(p2, a2);
      rev = !rev;
    }
    if (!p1 || !p2) break;
    push(route, p1);
    push(route, p2);
    rev = !rev;
    c = middle(p1, p2);
  }
  return route;
};

let approx = (v) => v.toFixed(2);

let render_route = (route) =>
  mapjoin(
    route,
    ([x, y], i) => `${i ? "L" : "M"}${approx(x)},${approx(y)}`,
    " "
  );

let cut_polygon = (poly, a, b) => {
  let prev;
  let first = [];
  let second = [];
  let on_first = true;
  for (let p of poly) {
    if (prev) {
      let c = collides_segment(prev, p, a, b);
      if (c) {
        push(first, c);
        push(second, c);
        on_first = !on_first;
      }
    }
    push(on_first ? first : second, p);
    prev = p;
  }
  return second.length < 2
    ? [poly]
    : [create_polygon(first), create_polygon(second)];
};

// ART //

let find_main_vector = (poly) => {
  // find out the vector of 2 most distant points of polygon
  let dx = 0;
  let dy = 0;
  let l = poly.length - 1;
  for (let i = 0; i < l; i++) {
    let [x1, y1] = poly[i];
    for (let j = i + 1; j < l; j++) {
      let [x2, y2] = poly[j];
      let x = x2 - x1;
      let y = y2 - y1;
      let m = y < 0 ? -1 : 1;
      let d = m * (x * x + y * y);
      dx += x * d;
      dy += y * d;
    }
  }
  return [dx, dy];
};

let art = (arti) => {
  while (R() < R()) R();
  while (R() > R()) R();
  let paperSeed = R(99);

  let cutangle = R() < 0.2 ? 0 : R() < 0.2 ? twopi / 4 : R(twopi);

  let colors = [...COLORS]
    .map((c, i) => [R(i + 5), c])
    .sort((a, b) => a[0] - b[0])
    .slice(0, (1.5 + R(2) * R()) | 0)
    .map((o) => o[1]);

  let angle_align = 0.1;

  let clr = R() < 0.1 ? 1 : 0;
  let shape = 1 + ((totalShape * pow(R(), 1.5)) | 0);
  // FIXME regression on not seeing anything...
  let max_depth = (16 * (1 - R() * R())) | 0; // TODO 1-R()*R() could be better
  let maxpowp = 0.9 - R() * R();
  let maxpow = 5 * maxpowp;
  let rad = mix(50, 90, mix(1 - maxpowp, R(), R(0.3))) + R(30);
  let alignmentFactor = 0.1 + 0.4 * (R() + R());
  let squareFactor = (R() + R() + R()) / 3;
  let stability = R() * R();
  let alignementlevel =
    abs(((cutangle + pi / 4) % (pi / 2)) - pi / 4) < angle_align
      ? max_depth
      : 0;

  let diagAlignementlevel =
    abs((cutangle % (pi / 2)) - pi / 4) < angle_align ? max_depth : 0;

  let poly1;
  let sides = 4;
  if (R() < 0.2) {
    rad *= 1.1;
    sides = R() < 0.2 ? 6 : R() < 0.2 ? 3 + (R() < 0.2 ? R(12) | 0 : 0) : 100;
    let pts = [];
    let incr = twopi / sides;
    for (let a = 0; a < twopi; a += incr) {
      push(pts, [width / 2 + rad * cos(a), height / 2 + rad * sin(a)]);
    }
    poly1 = create_polygon(pts);
  } else {
    let x1 = width / 2 - rad;
    let x2 = width / 2 + rad;
    let y1 = height / 2 - rad;
    let y2 = height / 2 + rad;
    poly1 = create_polygon([
      [x1, y1],
      [x2, y1],
      [x2, y2],
      [x1, y2],
    ]);
  }

  let generalcutoff = R(0.3) - 0.05;

  let cutoffmap = (cx, cy, ratio) => {
    let cutoff = generalcutoff;
    let xp = cx / width;
    let yp = cy / height;
    let dx = 2 * min(xp, 1 - xp);
    let dy = 2 * min(yp, 1 - yp);
    let borderdist = min(dx, dy);
    cutoff += 0.4 * (borderdist - 0.4);
    cutoff += 0.15 * ratio;
    return cutoff;
  };

  let shapeChangeChance = R(0.1);

  let rec = (polygon, d, max_depth, clr, shape, cutangle, maxpow) => {
    let ratio = d / max_depth;
    let revratio = 1 - ratio;
    if (R() < shapeChangeChance * revratio * revratio) {
      shape = (1 + R(totalShape) * R()) | 0;
    }

    let [cx, cy] = centroid(polygon);
    let [, , w, h] = bounding_rect(polygon);
    cx += w * R(revratio) * (R() - 0.5);
    cy += h * R(revratio) * (R() - 0.5);
    let dx = cos(cutangle);
    let dy = sin(cutangle);
    let a = [cx + highNumber * dx, cy + highNumber * dy];
    let b = [cx - highNumber * dx, cy - highNumber * dy];
    let cut = cut_polygon(polygon, a, b);

    let cutout = cutoffmap(cx, cy, ratio);
    let area = signed_area(polygon);
    if (
      cut.length == 1 ||
      d >= max_depth ||
      (d > 0 && R() < cutout) ||
      area < 9
    ) {
      if (R(area) > 3000 && shape === 1) {
        shape = 2;
      }
      return [[polygon, clr, shape]];
    }
    cut = cut.map((p, i) => {
      let [newcx, newcy] = centroid(p);
      let dx = newcx - cx;
      let dy = newcy - cy;
      let dist = sqrt(dx * dx + dy * dy);
      let split =
        0.8 * revratio * (stability + R(1 - stability)) +
        R(0.2) * R(1 - 0.8 * stability);
      let amp = split * pow(2, maxpow * revratio);
      let tx = (amp * dx) / dist;
      let ty = (amp * dy) / dist;
      let poly = translate_polygon(p, tx, ty);
      let [, , w, h] = bounding_rect(poly);

      let newshape = shape;
      let mindim = min(w, h);
      let maxdim = max(w, h);
      if (mindim < 2) {
        newshape = 0;
      } else if (newshape === 7 || newshape === 1) {
        let area = signed_area(poly);
        let cutoff = newshape === 1 ? 0.2 : 0.3;
        if (area / (maxdim * maxdim) < cutoff) {
          newshape = R() < 0.1 ? 6 : 2;
        }
      }
      let newclr = clr;
      if (i === 1 && d < 4 && R() < 0.3 * revratio * revratio) {
        newclr = (newclr + (R() < 0.2 ? colors.length - 1 : 1)) % colors.length;
        if (R() < 0.5) newshape++;
      }

      return [poly, newclr, newshape];
    });

    let all = [];

    let anglediverge = R() < alignmentFactor ? max(0, 0.2 * (R() - 0.5)) : 1;
    let rotateSquare = R() < squareFactor;
    if (rotateSquare) {
      cutangle += twopi / 4;
    }

    for (let [poly, clr, shape] of cut) {
      let mul = (R() - 0.5) * anglediverge;
      let nextcutangle = cutangle + twopi * mul;
      let nextd = d + 1;
      if (nextd < alignementlevel && abs(mul) > angle_align) {
        alignementlevel = nextd;
        diagAlignementlevel = nextd;
      }
      let inside = rec(
        poly,
        nextd,
        max_depth,
        clr,
        shape,
        nextcutangle,
        maxpow
      );
      let total_area = 0;
      let total = 0;
      let areas = [];
      for (let i = 0; i < inside.length; i++) {
        let area = signed_area(inside[i][0]);
        total++;
        total_area += area;
        push(areas, area);
      }
      if (!total) return all;
      let ratio = total_area / total;
      let transformShape8 = ratio < R(60) * R() * R() * R() * R();
      for (let i = 0; i < inside.length; i++) {
        let o = inside[i];
        if (transformShape8) o[2] = 8;
        push(all, o);
      }
    }
    return all;
  };

  let polygons = rec(poly1, 0, max_depth, clr, shape, cutangle, maxpow);

  let minx = width;
  let miny = height;
  let maxx = 0;
  let maxy = 0;

  let polygonsFiltered = polygons.filter(([poly]) => {
    let [x, y, w, h] = bounding_rect(poly);
    minx = min(minx, x);
    miny = min(miny, y);
    maxx = max(maxx, x + w);
    maxy = max(maxy, y + h);
    return x > pad && y > pad && x + w < width - pad && y + h < height - pad;
  });

  if (maxx - minx < width - 2 * pad && maxy - miny < height - 2 * pad) {
    let dx = width / 2 - (minx + (maxx - minx) / 2);
    let dy = height / 2 - (miny + (maxy - miny) / 2);
    polygons = polygons.map(([poly, ...rest]) => [
      poly.map(([x, y]) => [x + dx, y + dy]),
      ...rest,
    ]);
  } else {
    polygons = polygonsFiltered;
  }

  // circle avoid iterations
  let circleAvoidIterations = (R() < 0.5 ? 0 : 8) | 0;

  let arearatio = 0;
  let clrMax = 0;
  for (let [p, clr] of polygons) {
    arearatio += signed_area(p);
    clrMax = max(clr, clrMax);
  }
  arearatio /= signed_area(poly1);
  if (
    polygons.length === 2 ||
    (arearatio < 1 - R(0.5) * R() * R() && arti < 99)
  ) {
    return art(arti + 1);
  }

  return [
    polygons,
    colors,
    paperSeed,
    circleAvoidIterations,
    maxpow,
    arearatio,
    sides,
    alignementlevel,
    diagAlignementlevel,
  ];
};

let main = () => {
  let [polygons, colors, paperSeed, circleAvoidIterations] = art(0);

  let routesData = polygons
    .map(([poly, c, shape]) => {
      let routes = [];
      if (shape === 1) {
        // spiral
        let volume = signed_area(poly);
        let samples = 5 + 6 * pow(volume, 0.75);
        push(routes, route_spiral(samples_polygon(poly, samples, precision)));
      } else if (shape === 2) {
        // web: random between edge points
        let volume = signed_area(poly);
        let samples = 8 + 2 * pow(volume, 0.65);
        let borderdist = 4;
        borderdist *= 1 - 1 / (1 + 0.1 * pow(volume, 0.5));
        let [pts, groups] = samples_polygon_edge(poly, samples, borderdist);
        let circles = [];
        let [x1, y1, w, h] = bounding_rect(poly);
        let maxR = min(w, h) * R(0.1);
        for (let c = 0; c < R(9); c++) {
          push(circles, [x1 + R(w), y1 + R(h), R(maxR)]);
        }
        push(routes, randomize(pts, groups, circles, circleAvoidIterations));
      } else if (shape === 3) {
        // pingpong connecting
        push(routes, poly);
        let volume = signed_area(poly);
        let samples = 5 + 1.7 * pow(volume, 0.5);
        let n = 2 + R() * R();
        let [pts, groups] = samples_polygon_edge(poly, samples, 0);
        routes = routes.concat(pingpong(pts, n, groups));
      } else if (shape === 4) {
        // scratches; random sorted cross
        let volume = signed_area(poly);
        let samples = 10 + 2.4 * pow(volume, 0.7);
        let v = find_main_vector(poly);
        let v2 = [-v[1], v[0]];
        let div = 0.4;
        let samples1 = samples_polygon(poly, samples * div, precision);
        let samples2 = samples_polygon(poly, samples * (1 - div), precision);
        push(routes, route_sorted(samples1, v));
        push(routes, route_sorted(samples2, v2));
      } else if (shape === 5) {
        // hatch
        push(routes, poly);
        push(routes, hatching(poly));
      } else if (shape === 6) {
        // stippling
        let volume = signed_area(poly);
        let [cx, cy] = centroid(poly);
        let samples = 5 + 2 * volume;
        let points = samples_polygon(poly, samples, precision);
        for (let p of points) {
          let a = atan2(cx - p[0], cy - p[1]);
          push(routes, [p, [p[0] + cos(a), p[1] + sin(a)]]);
        }
      } else if (shape === 7) {
        // zigzag
        let volume = signed_area(poly);
        let samples = 5 + 4 * pow(volume, 0.7);
        let edge_ratio = R(0.1) * R();
        push(
          routes,
          route_sorted(
            samples_polygon(poly, samples * (1 - edge_ratio), precision).concat(
              samples_polygon_edge(poly, samples * edge_ratio, 0.5)[0]
            ),
            find_main_vector(poly)
          )
        );
      } else {
        push(routes, poly);
      }
      return [routes, c];
    })
    .filter(Boolean);

  let makeSVG = (colors, rendererMode) => {
    let style = rendererMode
      ? 'opacity="0.5"'
      : 'style="mix-blend-mode:multiply"';
    let bodySvg = mapjoin(
      colors,
      ([name, main], i) => {
        let content = "";
        let addpath = (style, d) => {
          content += `<path ${style} d="${d}"/>`;
        };
        for (let [routes, c] of routesData) {
          if (c % colors.length == i) {
            for (let route of routes) {
              if (rendererMode) {
                for (let i = 1; i < route.length; i++) {
                  let d = render_route(route.slice(i - 1, i + 1));
                  addpath(style, d);
                }
              } else {
                let d = render_route(route);
                addpath(style, d);
              }
            }
          }
        }
        let stroke = rendererMode
          ? MASKS[i]
          : "rgb(" + mapjoin(main, (n) => (n * 255) | 0, ",") + ")";
        return `<g inkscape:groupmode="layer" inkscape:label="${name}" stroke="${stroke}" stroke-width="0.35" fill="none">${content}</g>`;
      },
      ""
    );

    let svgW = rendererMode ? PX(rendererMode[0]) : MM(width);
    let svgH = rendererMode ? PX(rendererMode[1]) : MM(height);

    let svg = `<svg width="${svgW}" height="${svgH}" style="background:#fff;${
      rendererMode ? "" : "width:100%;height:100%"
    }" viewBox="0 0 ${width} ${height}" xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape">${bodySvg}</svg>`;

    return svg;
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

  let DOC = document;
  let BODY = DOC.body;
  let createElement = (e) => DOC.createElement(e);
  let append = (n, e) => n.appendChild(e);
  let appendBody = (e) => append(BODY, e);

  let svg = makeSVG(colors);
  let container = createElement("div");
  if (TD.plot) {
    container.innerHTML = svg;
    return appendBody(container.children[0]);
  }

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

  let vert = `precision mediump float;attribute vec2 p;varying vec2 uv;void main(){uv=p;gl_Position=vec4(2.*p-1.,0,1);}`;

  let framebuffer = regl.framebuffer();

  let paper = regl({
    framebuffer,
    frag: "precision highp float;varying vec2 uv;uniform vec2 V;uniform float G,S;void a(inout vec2 b,float c){b=cos(c)*b+sin(c)*vec2(b.y,-b.x);}float d(float b){b=fract(b*.1031);b*=b+33.33;b*=b+b;return fract(b);}float d(vec2 b){vec3 e=fract(vec3(b.xyx)*.1031);e+=dot(e,e.yzx+33.33);return fract((e.x+e.y)*e.z);}float f(float g){float h=floor(g);float i=fract(g);float j=i*i*(3.-2.*i);return mix(d(h),d(h+1.),j);}float f(vec2 g){vec2 h=floor(g);vec2 i=fract(g);float c=d(h);float k=d(h+vec2(1.,0.));float l=d(h+vec2(0.,1.));float m=d(h+vec2(1.,1.));vec2 j=i*i*(3.-2.*i);return mix(c,k,j.g)+(l-c)*j.y*(1.-j.g)+(m-k)*j.g*j.y;}const mat2 n=mat2(.4,.7,-.7,.4);float o(in vec2 g){float i=2.;float p=.55;float c=0.;float k=.5;for(int h=0;h<3;h++){float q=f(g);c+=k*q;k*=p;g=i*g;}return c;}float r(in vec2 g){ivec2 b=ivec2(floor(g));vec2 i=fract(g);ivec2 s;vec2 t;float u=8.;for(int v=-1;v<=1;v++)for(int h=-1;h<=1;h++){ivec2 k=ivec2(h,v);vec2 w=vec2(k)+d(vec2(b+k))-i;float m=dot(w,w);if(m<u){u=m;t=w;s=k;}}u=8.;for(int v=-2;v<=2;v++)for(int h=-2;h<=2;h++){ivec2 k=s+ivec2(h,v);vec2 w=vec2(k)+d(vec2(b+k))-i;float m=dot(.5*(t+w),normalize(w-t));u=min(u,m);}return u;}float x(vec2 b,float y,float z){a(b,2.);float c=smoothstep(.02,.16,.13*o(z+.3*b*y)+r(.5*y*b));float k=smoothstep(0.,.15,abs(o(-2.*b*y)-.5)-.01);return .4*k+.6*c;}void main(){vec2 A=V/min(V.x,V.y);vec2 b=.5+(uv-.5)*A;float B=x(b,G,S);gl_FragColor=vec4(B,B,B,1.);}" /*`precision highp float;
varying vec2 uv;
uniform vec2 V;
uniform float G, S;
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
float vn(in vec2 x) {
ivec2 p = ivec2(floor(x));vec2  f = fract(x);
ivec2 mb;vec2 mr;
float res = 8.0;
for( int j=-1; j<=1; j++ )
for( int i=-1; i<=1; i++ ){
ivec2 b = ivec2(i, j);vec2  r = vec2(b) + hash(vec2(p+b))-f;float d = dot(r,r);
if( d < res ) { res = d; mr = r; mb = b; }
}
res = 8.0;
for( int j=-2; j<=2; j++ )
for( int i=-2; i<=2; i++ ) {
ivec2 b = mb + ivec2(i, j);
vec2  r = vec2(b) + hash(vec2(p+b)) - f;
float d = dot(0.5*(mr+r), normalize(r-mr));
res = min( res, d );
}
return res;
}
float pp(vec2 p, float z, float S) {
pR(p, 2.);
float a = smoothstep(0.02, 0.16, 0.13 * fbm(S + 0.3 * p * z) + vn(0.5 * z * p));
float b = smoothstep(0.0, 0.15, abs(fbm(-2.0 * p * z)-0.5)-0.01);
return 0.4 * b + 0.6 * a;
}

void main () {
vec2 ratio = V / min(V.x, V.y);
vec2 p = 0.5 + (uv - 0.5) * ratio;
float t = pp(p, G, S);
gl_FragColor = vec4(t, t, t, 1.0);
}`*/,
    vert,
    attributes: {
      p: TRIANGLE,
    },
    uniforms: {
      S: paperSeed,
      G: 100,
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
    frag: "precision highp float;varying vec2 uv;uniform vec3 B;uniform float G,L,T,S;uniform vec3 C1,C1H,C2,C2H,C3,C3H;uniform sampler2D t,P;vec3 a(float b,vec3 c,vec3 d){float e=smoothstep(0.3,.0,b);return mix(vec3(1.),mix(c,d,e),smoothstep(1.,.5,b));}void main(){vec2 f=uv;vec2 g=f.xy-.5;float h=length(g);float i=smoothstep(.2,.0,abs(fract(h-.5*T)-.2));vec4 j=texture2D(P,f);float k=j.r;vec4 l=mix(vec4(1.),texture2D(t,f),smoothstep(h,h+.01,.5*(T-1.)));vec3 c=a(l.r,C1,C1H);vec3 d=a(l.g,C2,C2H);vec3 m=a(l.b,C3,C3H);vec3 n=min(vec3(1.),c*d*m*(1.+L*i))+G*mix(1.,.5,step(.5,k))*(.5-k)+B;gl_FragColor=vec4(n,1.);}" /*`
  precision highp float;
  varying vec2 uv;
  uniform vec3 B;
  uniform float G, L, T, S;
  uniform vec3 C1, C1H, C2, C2H, C3, C3H;
  uniform sampler2D t, P;
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
    vec2 q = p.xy - .5;
    float d = length(q);
    float gain = smoothstep(0.2, 0.0, abs(fract(d - .5*T) - 0.2));
    vec4 g = texture2D(P, p);
    float grain = g.r;
    vec4 v = mix(vec4(1.0), texture2D(t, p), smoothstep(d, d + 0.01, 0.5 * (T - 0.5)));
    vec3 c1 = pal(v.r, C1, C1H);
    vec3 c2 = pal(v.g, C2, C2H);
    vec3 c3 = pal(v.b, C3, C3H);
    vec3 c =
      min(vec3(1.), c1 * c2 * c3 * (1. + L * gain)) +
      G *
      mix(1.0, 0.5, step(0.5, grain)) *
      (0.5 - grain) +
      B;
    gl_FragColor = vec4(c, 1.0);
  }
  `*/,
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
      L: 0.1,
      B: [0, -0.005, -0.01],
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
      img.src = makeSVGDataImage(makeSVG(colors, [svgWidth, svgHeight]));
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
};
