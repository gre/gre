use serde::Serialize;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub fn make_document(
  hash: &str,
  feature_json: String,
  palette_json: String,
  width: f32,
  height: f32,
  mask_mode: bool,
  paper_background: &str,
  layers: &Vec<String>,
  extra_attributes: &Vec<String>,
) -> String {
  let mut attributes = vec![
    "data-credits=\"@greweb - 2023 - Plottable Era: (II) Medieval\""
      .to_string(),
    format!("data-hash=\"{}\"", hash),
    format!("data-traits='{}'", feature_json),
    format!("data-palette='{}'", palette_json),
    format!("viewBox=\"0 0 {} {}\"", width, height),
    format!("width=\"{}mm\"", width),
    format!("height=\"{}mm\"", height),
    format!(
      "style=\"background:{}\"",
      if mask_mode { "white" } else { paper_background }
    ),
    "xmlns:inkscape=\"http://www.inkscape.org/namespaces/inkscape\""
      .to_string(),
    "xmlns=\"http://www.w3.org/2000/svg\"".to_string(),
  ];

  for extra in extra_attributes {
    attributes.push(extra.clone());
  }

  let mut svg = format!("<svg {}>", attributes.join(" "));
  for layer in layers {
    svg.push_str(&layer);
  }
  svg.push_str("</svg>");

  svg
}

pub fn render_route(route: &Vec<(f32, f32)>) -> String {
  let mut d = String::new();
  if route.is_empty() {
    return d;
  }
  let (first_x, first_y) = route[0];
  d.push('M');
  d.push_str(&significant_mm(first_x));
  d.push(',');
  d.push_str(&significant_mm(first_y));
  for &(x, y) in &route[1..] {
    d.push(' ');
    d.push('L');
    d.push_str(&significant_mm(x));
    d.push(',');
    d.push_str(&significant_mm(y));
  }
  d
}

fn significant_mm(f: f32) -> String {
  ((f * 100.0).floor() / 100.0).to_string()
}

pub fn make_layers(
  data: &Vec<(&str, &str, f32, Vec<Vec<(f32, f32)>>)>,
) -> Vec<String> {
  data
    .iter()
    .filter(|(_color, _label, _stroke_width, routes)| !routes.is_empty())
    .enumerate()
    .map(|(ci, (color, label, stroke_width, routes))| {
      let layer_attributes = vec![
        format!("inkscape:groupmode=\"layer\""),
        format!("inkscape:label=\"{} {}\"", ci, label),
        format!("fill=\"none\""),
        format!("stroke=\"{}\"", color),
        format!("stroke-linecap=\"round\""),
        format!("stroke-width=\"{}\"", stroke_width),
      ]
      .join(" ");

      let mut layer = format!("<g {}>", layer_attributes);

      let opacity: f32 = 0.7;
      let opdiff = 0.15 / (routes.len() as f32);
      let mut trace = 0f32;
      for route in routes {
        trace += 1f32;
        let path_data = render_route(route);
        layer.push_str(&format!(
          "<path opacity=\"{}\" d=\"{}\"/>",
          (1000. * (opacity - trace * opdiff)).floor() / 1000.0,
          path_data
        ));
      }

      layer.push_str("</g>");

      layer
    })
    .collect()
}

#[derive(Clone, Copy, Serialize, PartialEq)]
pub struct Ink(
  pub &'static str,
  pub &'static str,
  pub &'static str,
  pub f32,
);

#[derive(Clone, Copy, Serialize, PartialEq)]
pub struct Paper(pub &'static str, pub &'static str, pub bool);

impl Paper {
  pub fn is_dark(&self) -> bool {
    self.2
  }
}

pub fn make_layers_from_routes_colors(
  routes: &Vec<(usize, Vec<(f32, f32)>)>,
  colors: &Vec<Ink>,
  mask_mode: bool,
  manhattan_skip_threshold: f32,
) -> Vec<String> {
  let mask_colors = vec!["#0FF", "#F0F", "#FF0"];
  make_layers(
    &colors
      .iter()
      .enumerate()
      .map(|(i, c)| {
        (
          if mask_mode { mask_colors[i] } else { c.1 },
          c.0,
          c.3,
          routes
            .iter()
            .filter_map(|(ci, routes)| {
              if *ci != i || routes.len() < 2 {
                return None;
              }
              // if manhattan dist is below the threshold we skip the stroke.
              // we don't plot want to plot "dots" that are relicate from clipping.
              let mut last = routes[0];
              let mut d = 0.0;
              for &p in &routes[1..] {
                d += (p.0 - last.0).abs() + (p.1 - last.1).abs();
                if d > manhattan_skip_threshold {
                  return Some(routes.clone());
                }
                last = p;
              }
              None
            })
            .collect(),
        )
      })
      .collect(),
  )
}

pub fn inks_stats(
  routes: &Vec<(usize, Vec<(f32, f32)>)>,
  colors: &Vec<Ink>,
) -> Vec<&'static str> {
  let colors_count = colors.len();
  let mut color_presence = vec![false; colors_count];
  for (i, _) in routes.iter() {
    if *i < colors_count {
      color_presence[*i] = true;
    }
  }
  let mut inks = vec![];
  for (i, &present) in color_presence.iter().enumerate() {
    let clr = colors[i];
    if present && !inks.contains(&clr.0) {
      inks.push(clr.0);
    }
  }
  inks.sort();
  inks
}
