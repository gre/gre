use std::collections::HashMap;
use svg::node::element::*;
use svg::parser::Event;
use svg::Document;

pub struct Letter {
  pub routes: Vec<Vec<(f64, f64)>>,
  pub width: f64,
  pub height: f64,
  pub can_attach: bool,
}
impl Letter {
  pub fn new(
    routes: Vec<Vec<(f64, f64)>>,
    width: f64,
    height: f64,
    can_attach: bool,
  ) -> Letter {
    Letter {
      routes,
      width,
      height,
      can_attach,
    }
  }

  pub fn width_for_size(&self, size: f64) -> f64 {
    self.width * size / self.height
  }

  pub fn render(
    &self,
    (x, y): (f64, f64),
    size: f64,
    // TODO deprecate. userland responsability to rotate something
    vertical: bool,
  ) -> (Vec<Vec<(f64, f64)>>, (f64, f64)) {
    let mut routes = self.routes.clone();
    let w = self.width;
    let h = self.height;
    let ratio = w / h;
    let scale = size / h;

    for route in routes.iter_mut() {
      for p in route.iter_mut() {
        p.0 *= scale;
        p.1 *= scale;
        if vertical {
          *p = (h * scale - p.1, p.0);
        }
        p.0 += x;
        p.1 += y;
      }
    }
    let delta = if vertical {
      (0.0, ratio * size)
    } else {
      (ratio * size, 0.0)
    };
    (routes, delta)
  }
}

pub struct LetterSvgReferential {
  letters: HashMap<String, Letter>,
}

impl LetterSvgReferential {
  pub fn new(
    svg_file: String,
    letter_precision: f64,
    non_attached_pad: f64,
  ) -> LetterSvgReferential {
    let mut content = String::new();

    let mut height = 0.0;
    let mut documents_per_char: HashMap<String, String> = HashMap::new();

    for event in svg::open(svg_file, &mut content).unwrap() {
      match event {
        Event::Tag(_, _, attributes) => {
          if let Some(c) = attributes.get("inkscape:label") {
            if let Some(d) = attributes.get("d") {
              let data: String = d.to_string();
              let document =
                Document::new().add(Path::new().set("d", data)).to_string();
              documents_per_char.insert(c.to_string(), document);
            }
          }

          if let Some(h) = attributes.get("height") {
            let mut hv = h.to_string();
            hv = hv.replace("mm", "");
            if let Some(h) = hv.parse::<f64>().ok() {
              height = h;
            }
          }
        }
        _ => {}
      }
    }

    let mut letters = HashMap::new();
    for (c, svg) in documents_per_char.iter() {
      let polylines =
        svg2polylines::parse(svg.as_str(), letter_precision, true).unwrap();
      let can_attach = !"1234567890".contains(c);

      let mut minx = std::f64::INFINITY;
      let mut maxx = -std::f64::INFINITY;
      for poly in polylines.iter() {
        for p in poly.iter() {
          if p.x < minx {
            minx = p.x;
          }
          if p.x > maxx {
            maxx = p.x;
          }
        }
      }

      let mut width = maxx - minx;

      let mut dx = minx;
      if !can_attach {
        dx -= non_attached_pad;
        width += 2.0 * non_attached_pad;
      }

      let routes = polylines
        .iter()
        .map(|l| l.iter().map(|p| (p.x - dx, p.y)).collect())
        .collect();

      letters.insert(c.clone(), Letter::new(routes, width, height, can_attach));
    }

    letters.insert(
      " ".to_string(),
      Letter::new(vec![], 0.5 * height, height, false),
    );

    LetterSvgReferential { letters }
  }

  pub fn get_letter(&self, c: &String) -> Option<&Letter> {
    self.letters.get(c)
  }
}
