use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::Group;

fn square_spiral(
  data: Data,
  c: (f64, f64),
  r: f64,
  initial_a: f64,
  d_length: f64,
) -> Data {
  let mut d = data;
  let mut a: f64 = initial_a;
  let length = r * 2. / (2. as f64).sqrt();
  let delta = p_r((-length / 2., length / 2.), a);
  let mut p = (c.0 + delta.0, c.1 + delta.1);
  let mut l = length;
  let mut i = 0;
  d = d.move_to((p.0, p.1));
  loop {
    if l < 0.0 {
      break;
    }
    p = (p.0 + l * a.cos(), p.1 + l * a.sin());
    d = d.line_to(p);
    a -= PI / 2.;
    if i > 0 {
      l -= d_length;
    }
    i += 1;
  }
  d
}

fn art() -> Vec<Group> {
  let mut groups = Vec::new();
  let data = square_spiral(Data::new(), (105. / 2., 105. / 2.), 60.0, 0.0, 1.0);
  groups.push(layer("black").add(base_path("black", 0.4, data)));
  groups
}

fn main() {
  let groups = art();
  let mut document = base_document("white", 105., 105.);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
