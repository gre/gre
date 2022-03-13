use clap::Clap;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let stroke_width = 0.35;
    let colors = vec!["#f90", "#09f"];
    let width = 420.0;
    let height = 297.0;
    let pad = 20.0;
    colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
        let mut data = Data::new();
        let perlin = Perlin::new();
        let mut rng = rng_from_seed(opts.seed);
        let cidiv = ci as f64 / 16.;
        let mut passage = Passage2DCounter::new(0.8, width, height);

        let a = rng.gen_range(1000.0, 4000.0);
        let b = rng.gen_range(1000.0, 4000.0);
        let c = rng.gen_range(1., 100.0) * rng.gen_range(0.1, 1.0);
        let d = rng.gen_range(1., 100.0) * rng.gen_range(0.1, 1.0);
        let e = rng.gen_range(0.3, 0.5);

        let f = |i, ampx, ampy| {
          let x = width/2.0 + width * 0.5 * perlin.get([
            0.334 + 70.7 * opts.seed / 3.,
            0.3 + i as f64 / a,
            ampx * e * perlin.get([
                -opts.seed,
                i as f64 * c
            ]) + cidiv
        ]);
        let y = height/2.0 + height * 0.5 * perlin.get([
            i as f64 / b,
            9.1 + 40.3 * opts.seed / 7.,
            ampy * e * perlin.get([
                60.1 + opts.seed,
                i as f64 * d
            ]) + cidiv
        ]);
          (x, y)
        };

        let mut points = Vec::new();
        let mut minx = width;
        let mut miny = height;
        let mut maxx = 0.;
        let mut maxy = 0.;
        for i in 0..100000 {
          let p = f(i, 1., 1.);
          let dist = euclidian_dist(p, (width / 2., height / 2.));
          let amp = 0.5 + 1.5 * (2. * dist / width).powf(2.0);
          let p = f(i, amp, 0.2 * amp);
          if passage.count(p) > 2 {
            continue;
          }
          points.push(p);
          if p.0 < minx {
            minx = p.0;
          }
          if p.1 < miny {
            miny = p.1;
          }
          if p.0 > maxx {
            maxx = p.0;
          }
          if p.1 > maxy {
            maxy = p.1;
          }
        }

        let w = maxx - minx;
        let h = maxy - miny;
        let dx = (width - w) / 2. - minx;
        let dy = (height - h) / 2. - miny;

        for p in points {
          let x = p.0 + dx;
          let y = p.1 + dy;
          if x < pad || y < pad || x > width - pad || y > height - pad {
            continue;
          }
          data = data.move_to((x, y));
          let angle = (x-width/2.).atan2(y-height/2.) + rng.gen_range(-0.5, 0.5);
          let amp = rng.gen_range(1.0, 2.0);
          data = data.line_to((
            x + amp * angle.cos(),
            y + amp * angle.sin()
          ));
        }
        let mut l = layer(color);
        l = l.add(base_path(color, stroke_width, data));
        l
    })
    .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "143.0")]
    seed: f64,
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a3_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
