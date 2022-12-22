use clap::*;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: &Opts) -> Vec<Group> {
  let h = opts.h;
  let w = opts.w;

  vec!["black"]
    .iter()
    .enumerate()
    .map(|(_i, color)| {
      let pad = opts.pad;
      let width = w - 2.0 * pad;

      let mut data = Data::new();

      let mut route = vec![];
      let mut a: f64 = 0.0;
      let mut yc = pad + opts.starty / 2.0;

      loop {
        if yc > h - 1.5 * pad {
          break;
        }

        let r = width / 2.0;

        let x = w / 2.0 + r * a.cos();
        let y = yc + 0.2 * r * a.sin();

        route.push((x, y));

        yc += opts.dy;
        a += opts.da;
      }

      data = render_route(data, route);

      let mut l = layer(color);
      l = l.add(base_path(color, 0.5, data));
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "105.")]
  w: f64,
  #[clap(short, long, default_value = "148.5")]
  h: f64,
  #[clap(short, long, default_value = "10.")]
  pad: f64,
  #[clap(short, long, default_value = "10.")]
  starty: f64,
  #[clap(short, long, default_value = "0.0187")]
  dy: f64,
  #[clap(short, long, default_value = "0.1")]
  da: f64,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.w, opts.h);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
