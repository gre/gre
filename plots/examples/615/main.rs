/**
 * honk honk
 * if you ever see this Raph, I hope you survive the Rust code =)
 * I'll drive you through the code...
 */
use clap::*;
use gre::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

/**
 * Here, we're defining the INPUTS of the generator
 * It's basically a seed. but also we can configure a size (and an output file)
 * because yes, we are generating a .svg file for plotting purpose
 */
#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

/**
 * that's the main entry function. We're building a SVG file and saving it
 */
fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document =
    base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

/**
 * And this is our art function
 */
fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = 10.0;

  let mut rng = rng_from_seed(opts.seed);
  let mut routes = Vec::new();

  let bolddy = 0.5; // distance between lines on ground and roof
  let groundy = 8.0; // height of the ground
  let initialfloory = height - pad - groundy;
  let dy = 0.2; // extra delta between the floors
  let stability = // this factor will make the tower offseting on X to make like a Pise tower
    rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let safe_pad = rng.gen_range(2.0, 5.0); // pad around the rectangles for the sky drawing
  let max_floors_proba = rng.gen_range(1, 20); // floor maximum allowed

  // Make the ground base until the padding limit
  for i in 0..100 {
    let y = initialfloory + dy + i as f64 * bolddy;
    if y > height - pad {
      break;
    }
    let x1 = pad;
    let x2 = width - pad;
    routes.push(vec![(x1, y), (x2, y)]);
  }

  // this stores all the rectangles so we can do collision to make the sky later
  let mut rects = Vec::new();

  // number of towers to make
  let buildings_count = 1
    + (rng.gen_range(0.2, 1.0) * rng.gen_range(0.0, 20.0))
      as usize;
  let towerswidthpad = rng.gen_range(pad, 60.0);
  let w = (width - 2. * towerswidthpad)
    / (buildings_count as f64);
  let sw = w * rng.gen_range(0.3, 0.8);

  // MAKE THE TOWERS =>
  for c in 0..buildings_count {
    if buildings_count > 2 && rng.gen_bool(0.1) {
      // randomly disappear
      continue;
    }

    let mut floory = initialfloory;
    let minx =
      towerswidthpad + c as f64 * w + (w - sw) / 2.0;
    let maxx = minx + sw;

    // maximum amount of floors
    let max_floors = 2 + rng.gen_range(0, max_floors_proba);
    let mut dx = 0.0;

    // MAKE THE FLOORS =>
    for _i in 0..max_floors {
      // make a floor!
      if floory < 40.0 {
        // we're high enough
        break;
      }
      let splits = 1
        + (rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 20.0)) as usize;
      let w = (maxx - minx) / (splits as f64);
      let h = rng.gen_range(10.0, 20.0);
      let sw = w * rng.gen_range(0.7, 1.0);
      let offsetmax = (w - sw) * 0.5;
      let r1 = rng.gen_range(-1.0, 1.0);
      let r2 = rng.gen_range(-1.0, 1.0);
      let r3 = rng.gen_range(-1.0, 1.0);
      let r4 = rng.gen_range(-1.0, 1.0);

      // INTERNAL SUBDIVISION OF THE FLOOR =>
      for j in 0..splits {
        // disclaimer: we're just pushing a quadrilateral^^
        let x1 = dx + minx + j as f64 * w + (w - sw) / 2.0;
        let x2 = x1 + sw;
        let a = (x1 - offsetmax * r1, floory);
        let b = (x2 + offsetmax * r2, floory);
        let c = (x2 + offsetmax * r3, floory - h);
        let d = (x1 - offsetmax * r4, floory - h);
        let route = vec![a, b, c, d, a];
        routes.push(route);
        let minx = a.0.min(b.0).min(c.0).min(d.0);
        let miny = a.1.min(b.1).min(c.1).min(d.1);
        let maxx = a.0.max(b.0).max(c.0).max(d.0);
        let maxy = a.1.max(b.1).max(c.1).max(d.1);
        rects.push((
          // approximative boundaries
          minx - safe_pad,
          miny - safe_pad,
          maxx + safe_pad,
          maxy + safe_pad,
        ));
      }

      // offset to the next floor!
      dx += rng.gen_range(-1.0, 1.0)
        * stability
        * rng.gen_range(0.0, 8.0);
      floory -= h + dy;
    }

    // Make the roof with 20 lines
    for i in 0..20 {
      let y = floory - i as f64 * bolddy;
      routes.push(vec![(maxx + dx, y), (minx + dx, y)]);
    }
    rects.push((
      minx + dx - safe_pad,
      floory - 20. * bolddy - safe_pad,
      maxx + dx + safe_pad,
      floory + safe_pad,
    ));
  }

  // MAKE THE SKY =>

  let mut y = pad;
  let skystop = height - rng.gen_range(20.0, 40.0);
  let yincr = rng.gen_range(4.0, 6.0);
  let splits = (rng.gen_range(0f64, 140.0)
    * rng.gen_range(0.0, 1.0))
  .max(1.0);
  let splits_part = rng.gen_range(-0.4f64, 0.6);
  let yfactor =
    rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let curvefactor =
    rng.gen_range(-10.0, 10.0) * rng.gen_range(0.0, 1.0);
  let splits_delta_base = rng.gen_range(0.0, 2.0);
  loop {
    if y > skystop {
      break;
    }
    let mut x = pad;
    let xincr = 0.5;
    let splits_delta =
      splits_delta_base + (curvefactor * y).cos();
    let mut route = Vec::new();
    loop {
      if x > width - pad {
        break;
      }

      // this is the rule that drives the stops in the sky
      let disabled =
        ((splits * (x / width + splits_delta)) % 1.0)
          < splits_part + yfactor * y / height;

      if disabled
        || rects
          .iter()
          .any(|&b| strictly_in_boundaries((x, y), b))
      {
        if route.len() > 0 {
          route.push((x - xincr, y));
          routes.push(route);
        }
        route = Vec::new();
      } else {
        if route.len() == 0 {
          route.push((x, y));
        }
      }

      x += xincr;
    }
    if route.len() > 0 {
      route.push((x - xincr, y));
    }
    if route.len() > 1 {
      routes.push(route);
    }
    y += yincr;
  }

  // this builds the data of the <path/>
  let mut data = Data::new();
  for route in routes {
    data = render_route(data, route);
  }

  // we're making the layer with a <path/>
  let color = "black";
  let l = layer(color).add(base_path(color, 0.5, data));
  vec![l]
}
