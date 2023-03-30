use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
  let document = make_svg();
  svg::save("image.svg", &document).unwrap();
}

// This generate the SVG and put together the coloring function and the vectorizer
fn make_svg() -> Document {
  let map_color = move |clr| grayscale(clr);
  let group = vectorize_as_fwave_rows(
    (160.0, 160.0),
    get_color,
    map_color,
    60,
    800.0,
    "white",
  )
  .set("transform", "translate(70,20)");

  let document = Document::new()
    .set(
      "xmlns:inkscape",
      "http://www.inkscape.org/namespaces/inkscape",
    )
    .set("viewBox", (0, 0, 297, 210))
    .set("width", "297mm")
    .set("height", "210mm")
    .set("style", "background:black")
    .add(group);

  return document;
}

///// raymarching a "Signed Distance Function" ///// (see http://jamie-wong.com/2016/07/15/ray-marching-signed-distance-functions/)
// This implements a raymarcher, similar to the one used at https://greweb.me/shaderday/56

// this is the "main" coloring function. for a given uv, returns a color.
fn get_color(uv: vec2) -> vec3 {
  let (x, y) = uv;
  // raymarching
  let origin = (0.0, 0.0, -3.0);
  let dir = normalize3((x - 0.5, y - 0.5, 1.0));
  let mut t = 0.0;
  let mut hit = 99.0;
  for _i in 0..100 {
    let h = map(add3(origin, mul3f(dir, t)));
    t += h;
    if h.abs() < 0.001 {
      hit = h;
      break;
    }
  }
  let p = add3(origin, mul3f(dir, t));
  let n = normal(p);
  return lighting(hit, p, n, dir);
}

// this is our "3D scene" distance function:
// for a given point in space, tells the distance to closest object
fn map(mut p: vec3) -> f32 {
  // x axis rotation
  let r = rot2((p.1, p.2), 0.8);
  p = (p.0, r.0, r.1);
  // y axis rotation
  let r = rot2((p.0, p.2), 0.8);
  p = (r.0, p.1, r.1);
  return fBox(p, (0.5, 0.5, 0.5))
    .min(fSphere(add3(p, (0.0, 0.5, 0.0)), 0.3))
    .min(fSphere(add3(p, (0.0, 0.0, 0.5)), 0.3))
    .min(fSphere(add3(p, (0.5, 0.0, 0.0)), 0.3));
}

// distance to a sphere
fn fSphere(p: vec3, r: f32) -> f32 {
  length3(p) - r
}

// distance to a box
fn fBox(p: vec3, b: vec3) -> f32 {
  let d = add3(abs3(p), neg3(b));
  return length3(max3(d, (0.0, 0.0, 0.0))) + vmax3(min3(d, (0.0, 0.0, 0.0)));
}

// apply a rotation on 2d
fn rot2(p: vec2, a: f32) -> vec2 {
  add2(mul2f(p, (a).cos()), mul2f((p.1, -p.0), (a).sin()))
}

// this implements lighting of the 3D scene. 2 lights here.
fn lighting(_hit: f32, p: vec3, n: vec3, _dir: vec3) -> vec3 {
  let mut c = 0.0;
  let ldir = (-1.0, 1.0, -2.0);
  c += 0.1 + diffuse(p, n, ldir);
  let ldir = (1.0, 0.0, -1.0);
  c += 0.5 * (0.1 + diffuse(p, n, ldir));
  c = clamp(c, 0.0, 1.0);
  return (c, c, c);
}

////// vectorize function :)
// the idea is we take a coloring function and we implement a way to display it with SVG paths
// with this implementation, we implement using different frequency of waves
fn vectorize_as_fwave_rows(
  (width, height): (f32, f32),
  get_color: impl Fn((f32, f32)) -> (f32, f32, f32),
  map_color: impl Fn((f32, f32, f32)) -> f32,
  rows: u32,
  wave_freq: f32,
  color: &str,
) -> Group {
  let mut group = Group::new();
  for yi in 0..rows {
    let yp = (0.5 + yi as f32) / (rows as f32);
    let y = height * yp;
    let mut data = Data::new().move_to((0, y));
    let nb = (3.0 * wave_freq) as u32;
    let mut t = 0.0;
    // TODO: this could be optimized to have less datapoints
    for i in 1..nb {
      let xp = (i as f32) / (nb as f32);
      let x = width * xp;
      let clr = get_color((xp, yp));
      let value = map_color(clr);
      let amp = 0.4 * (height as f32) / (rows as f32);
      t += wave_freq * (value).powf(2.0) / (nb as f32);
      let dy = amp * (t).cos();
      data = data.line_to((x, y + dy));
    }
    let path = Path::new()
      .set("fill", "none")
      .set("stroke", color)
      .set("stroke-width", 0.2)
      .set("d", data);
    group = group.add(path)
  }

  return group;
}

// a bunch of vectors helpers (in future, I need a library =D)
type vec2 = (f32, f32);
type vec3 = (f32, f32, f32);
fn length3((x, y, z): vec3) -> f32 {
  (x * x + y * y + z * z).sqrt()
}
fn normalize3(p: vec3) -> vec3 {
  let l = length3(p);
  return (p.0 / l, p.1 / l, p.2 / l);
}
fn add2(a: vec2, b: vec2) -> vec2 {
  (a.0 + b.0, a.1 + b.1)
}
fn add3(a: vec3, b: vec3) -> vec3 {
  (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}
fn neg3(a: vec3) -> vec3 {
  (-a.0, -a.1, -a.2)
}
fn mul3f(a: vec3, f: f32) -> vec3 {
  (a.0 * f, a.1 * f, a.2 * f)
}
fn mul2f(a: vec2, f: f32) -> vec2 {
  (a.0 * f, a.1 * f)
}
fn normal(p: vec3) -> vec3 {
  return normalize3((
    map(add3(p, (0.0005, 0.0, 0.0))) - map(add3(p, (-0.0005, 0.0, 0.0))),
    map(add3(p, (0.0, 0.0005, 0.0))) - map(add3(p, (0.0, -0.0005, 0.0))),
    map(add3(p, (0.0, 0.0, 0.0005))) - map(add3(p, (0.0, 0.0, -0.0005))),
  ));
}
fn clamp(a: f32, from: f32, to: f32) -> f32 {
  (a).max(from).min(to)
}
fn dot3(a: vec3, b: vec3) -> f32 {
  a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}
fn abs3(a: vec3) -> vec3 {
  (a.0.abs(), a.1.abs(), a.2.abs())
}
fn diffuse(p: vec3, n: vec3, lpos: vec3) -> f32 {
  let l = normalize3(add3(lpos, neg3(p)));
  let dif = clamp(dot3(n, l), 0.01, 1.);
  return dif;
}
fn vmax3(v: vec3) -> f32 {
  (v.0).max(v.1).max(v.2)
}
fn min3(a: vec3, b: vec3) -> vec3 {
  (a.0.min(b.0), a.1.min(b.1), a.2.min(b.2))
}
fn max3(a: vec3, b: vec3) -> vec3 {
  (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2))
}

fn grayscale((r, g, b): (f32, f32, f32)) -> f32 {
  return 0.299 * r + 0.587 * g + 0.114 * b;
}
