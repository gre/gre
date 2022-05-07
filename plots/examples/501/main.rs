use clap::Clap;
use gre::*;
use noise::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::{path::Data, *};

#[derive(Clap)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "100.0")]
  seed: f64,
  #[clap(short, long, default_value = "420.0")]
  width: f64,
  #[clap(short, long, default_value = "297.0")]
  height: f64,
}

fn head(
  seed: f64,
  cx: f64,
  cy: f64,
  r: f64,
  samples: usize,
  offset: f64,
  phase: f64,
) -> Vec<Vec<(f64, f64)>> {
  let precision = 0.3;
  let w = (4. * r / precision) as u32;
  let h = (4. * r / precision) as u32;
  let perlin = Perlin::new();
  let f = |(x, y): (f64, f64)| -> f64 {
    let dx: f64 = x - 0.5;
    let dy: f64 = y - 0.5;
    let mut res: f64 = (dx * dx + dy * dy).sqrt();
    let xabs = (x - 0.5).abs();
    let (px, py) = p_r((dx, dy), 2.0 * PI * phase);
    let mut rng = rng_from_seed(seed);
    let f1 = rng.gen_range(1.0, 3.0);
    let f2 = rng.gen_range(0.5, 3.0) * f1;
    let f3 = rng.gen_range(2.0, 5.0) * f1;
    res += 0.07
      * perlin.get([
        // first level
        f1 * xabs,
        f1 * y + 0.3 * x,
        seed
          + 3.
            * perlin.get([
              // 2nd level
              f2 * px,
              f2 * py,
              seed
                + 0.2 * x
                + 3.
                  * perlin.get([
                    // 3rd level
                    seed,
                    f3 * xabs,
                    f3 * y,
                  ]),
            ]),
      ]);
    res * 4.5
  };
  let thresholds: Vec<f64> = (0..samples)
    .map(|i| (i as f64 + offset) / (samples as f64))
    .collect();
  let res = contour(w, h, f, &thresholds);
  let mut routes = features_to_routes(res, precision);
  routes = crop_routes(
    &routes,
    (1.0, 1.0, 4. * r - 1., 4. * r - 1.),
  );
  routes =
    translate_routes(routes, (cx - 2. * r, cy - 2. * r));
  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let pad = 10.0;
  let width = opts.width;
  let height = opts.height;
  let clrs = vec!["#f90", "#f90", "#10b"];
  let layers: Vec<Group> = clrs
    .iter()
    .enumerate()
    .map(|(ci, &color)| {
      let mut routes = Vec::new();

      let splitw = 4;
      let splith = 2;
      let total_width = width - (splitw as f64 + 1.0) * pad;
      let total_height =
        height - (splith as f64 + 1.0) * pad;
      let w = total_width / (splitw as f64);
      let h = total_height / (splith as f64);
      let phaseincr = 1.0 / (splitw * splith) as f64;
      let mut phase = 0.0;
      for y in 0..splith {
        let ytop = pad + y as f64 * (pad + h);
        for x in 0..splitw {
          let xleft = pad + x as f64 * (pad + w);
          let r = head(
            opts.seed,
            xleft + 0.5 * w,
            ytop + 0.5 * h,
            w.min(h) / 2.0 - 2.0,
            25,
            ci as f64 / (clrs.len() as f64),
            phase,
          );
          if ci == 1 {
            let d = 0.5;
            routes.push(vec![
              (xleft, ytop + d),
              (xleft, ytop),
              (xleft + d, ytop),
            ]);
            routes.push(vec![
              (xleft + w - d, ytop),
              (xleft + w, ytop),
              (xleft + w, ytop + d),
            ]);
            routes.push(vec![
              (xleft + w, ytop + h - d),
              (xleft + w, ytop + h),
              (xleft + w - d, ytop + h),
            ]);
            routes.push(vec![
              (xleft + d, ytop + h),
              (xleft, ytop + h),
              (xleft, ytop + h - d),
            ]);
          }
          routes = vec![routes, r].concat();
          phase += phaseincr;
        }
      }

      let mut data = Data::new();
      for r in routes {
        data = render_route(data, r);
      }

      Group::new()
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", color)
        .add(
          Path::new()
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", 0.32)
            .set("d", data),
        )
    })
    .collect();

  let mut group = Group::new();
  for l in layers {
    group = group.add(l);
  }
  vec![group]
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document =
    base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}

fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
  (
    a.cos() * p.0 + a.sin() * p.1,
    a.cos() * p.1 - a.sin() * p.0,
  )
}

fn translate_routes(
  routes: Vec<Vec<(f64, f64)>>,
  (tx, ty): (f64, f64),
) -> Vec<Vec<(f64, f64)>> {
  routes
    .iter()
    .map(|route| {
      route.iter().map(|&(x, y)| (x + tx, y + ty)).collect()
    })
    .collect()
}
