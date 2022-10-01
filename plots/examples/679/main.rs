use clap::*;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "600.0")]
  pub width: f64,
  #[clap(short, long, default_value = "400.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "10.0")]
  pub offset: f64,
  #[clap(short, long, default_value = "8.0")]
  pub stroke_width: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let mut route = Vec::new();
  let mut y = opts.pad;
  let mut reverse = false;
  loop {
    if y > opts.height - opts.pad {
      break;
    }
    let a = (opts.pad, y);
    let b = (opts.width - opts.pad, y);
    if reverse {
      route.push(b);
      route.push(a);
    } else {
      route.push(a);
      route.push(b);
    }
    y += opts.offset;
    reverse = !reverse;
  }

  let color = "black";
  let mut data = Data::new();
  data = render_route(data, route);
  let mut l = layer(color);
  l = l.add(base_path(color, opts.stroke_width, data));
  vec![l]
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
