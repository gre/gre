use clap::*;
use core::f64;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::{path::Data, *};

fn art(opts: Opts) -> Vec<Group> {
  let stroke_width = 0.35;
  let w = 297.;
  let h = 210.;
  let count_x = opts.count_x;
  let count_y = opts.count_y;
  let colors = opts.colors.split(",").collect::<Vec<&str>>();
  let perlin = Perlin::new();
  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let pattern = |(width, height): (f64, f64), p: (f64, f64)| {
        base_path(
          color,
          stroke_width,
          render_route(
            Data::new(),
            vec![
              (p.0, p.1 - height / 2.),
              (p.0 + width / 2., p.1),
              (p.0, p.1 + height / 2.),
              (p.0 - width / 2., p.1),
              (p.0, p.1 - height / 2.),
            ],
          ),
        )
      };
      let mut l = layer(color);
      for y in 0..count_y {
        let yf = y as f64 / count_y as f64;
        let xc = if y % 2 == 1 { count_x - 1 } else { count_x };
        for x in 0..xc {
          let xf = x as f64 / count_x as f64;
          let p = (
            opts.offx
              + opts.fullw
                * (xf + (if y % 2 == 1 { 0.5 } else { 0. }) / (count_x as f64)),
            opts.offy + opts.fullh * yf,
          );
          let width = opts.wsize
            * (1.
              + (xf * PI).sin()
                * opts.amp
                * perlin.get([
                  1.5 * opts.freq * xf,
                  opts.freq * yf,
                  opts.seed,
                ]));
          let height = opts.hsize
            * (1.
              + (yf * PI).sin()
                * opts.amp
                * perlin.get([
                  1.5 * opts.freq * xf,
                  opts.freq * yf,
                  opts.seed + opts.seed_div,
                ]));
          if y % colors.len() == i {
            l = l.add(pattern((width, height), p));
          }
        }
      }
      if i == 0 {
        l = l.add(signature(1., (w - 44., h - 16.), color));
      }
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "84")]
  count_x: usize,
  #[clap(short, long, default_value = "79")]
  count_y: usize,
  #[clap(short, long, default_value = "6.")]
  wsize: f64,
  #[clap(short, long, default_value = "6.")]
  hsize: f64,
  #[clap(short, long, default_value = "20.")]
  offx: f64,
  #[clap(short, long, default_value = "10.")]
  offy: f64,
  #[clap(short, long, default_value = "262.")]
  fullw: f64,
  #[clap(short, long, default_value = "184.")]
  fullh: f64,
  #[clap(short, long, default_value = "1.")]
  amp: f64,
  #[clap(short, long, default_value = "12.")]
  freq: f64,
  #[clap(short, long, default_value = "3.")]
  seed: f64,
  #[clap(short, long, default_value = "0.3")]
  seed_div: f64,
  #[clap(short, long, default_value = "hotpink,deepskyblue")]
  colors: String,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
