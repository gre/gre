use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "100.0")]
  width: f64,
  #[clap(short, long, default_value = "100.0")]
  height: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let pad = 8.0;
  let width = opts.width;
  let height = opts.height;
  let samples = 6;
  let stroke_width = 0.35;
  let precision = 0.3;
  let w = (width as f64 / precision) as u32;
  let h = (height as f64 / precision) as u32;
  let perlin = Perlin::new();
  fn length(l: (f64, f64)) -> f64 {
    (l.0 * l.0 + l.1 * l.1).sqrt()
  }
  fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
    (a.cos() * p.0 + a.sin() * p.1, a.cos() * p.1 - a.sin() * p.0)
  }
  fn op_union_round(a: f64, b: f64, r: f64) -> f64 {
    r.max(a.min(b)) - length(((r - a).max(0.), (r - b).max(0.)))
  }
  let sdf_box2 = |(x, y): (f64, f64), (w, h): (f64, f64)| {
    let dx = x.abs() - w;
    let dy = y.abs() - h;
    length((dx.max(0.), dy.max(0.))) + dx.min(0.).max(dy.min(0.))
  };
  let colors = vec!["#000", "#000", "#000", "#000", "#000"];
  let mut layers = Vec::new();
  for (ci, &color) in colors.iter().enumerate() {
    let f = |(x, y): (f64, f64)| {
      let mut rng = rng_from_seed(opts.seed);
      let mut c = ((x - 0.5) * width / height, y - 0.5);
      if rng.gen_bool(0.5) {
        c.0 = c.0.abs();
      }
      if rng.gen_bool(0.5) {
        c.1 = c.1.abs();
      }
      c = p_r(c, 0.25 * (ci as f64 - 2.0) * PI / 2.);
      let mut s = 100f64;
      let k = 0.05;
      for _i in 0..rng.gen_range(3, 8) {
        let mut p = (c.0, c.1);
        p.0 += rng.gen_range(-0.2, 0.2);
        p.1 += rng.gen_range(-0.2, 0.2);
        p = p_r(p, rng.gen_range(0.0, 10.0));
        let dim = (rng.gen_range(0.0, 0.1), rng.gen_range(0.0, 0.3));
        s = op_union_round(s, sdf_box2(p, dim), k);
      }
      let f1 = rng.gen_range(2.0, 6.0);
      let f3 = rng.gen_range(2.0, 9.0);
      let a1 = 0.4 + 0.05 * (ci as f64 - 2.0).abs();
      let a2 = rng.gen_range(3.0, 4.0) + 0.3 * (ci as f64 - 2.0);
      let n = a1
        * perlin.get([
          f1 * c.0,
          f1 * c.1,
          3.7 * opts.seed
            + ci as f64 * 0.2
            + a2
              * perlin.get([
                opts.seed / 3.,
                c.0 + 2.0 * perlin.get([f3 * c.0, f3 * c.1, 2.1 * opts.seed]),
                8.0 * c.1
                  + 2.0 * perlin.get([f3 * c.0, f3 * c.1, 2.7 * opts.seed]),
              ]),
        ]);
      lerp(-0.04, 0.04, s) + n
    };

    let thresholds: Vec<f64> = (0..samples)
      .map(|i| {
        ((ci + colors.len() * i) as f64) / ((colors.len() * samples) as f64)
      })
      .collect();
    let res = contour(w, h, f, &thresholds);
    let routes = features_to_routes(res, precision);
    let inside = |from, to| {
      strictly_in_boundaries(from, (pad, pad, width - pad, height - pad))
        && strictly_in_boundaries(to, (pad, pad, width - pad, height - pad))
    };
    let mut l = Group::new()
      .set("inkscape:groupmode", "layer")
      .set("inkscape:label", color)
      .set("fill", "none")
      .set("stroke", color)
      .set("stroke-width", stroke_width);

    for r in routes.clone() {
      if r.len() < 2 {
        continue;
      }
      let data = render_route_when(Data::new(), r, inside);
      l = l.add(Path::new().set("d", data));
    }
    layers.push(l);
  }

  layers
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
