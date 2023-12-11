use std::io::Cursor;

use crate::algo::{math1d::smoothstep, paintmask::PaintMask};
use base64::{engine::general_purpose, Engine};
use image::{ImageBuffer, Luma};

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone)]
// Feature tells characteristics of a given art variant. It is returned in the .SVG file
pub struct Effects {
  pub hot: PaintMask,
  pub water: PaintMask,
}

impl Effects {
  pub fn new(paintref: &PaintMask) -> Self {
    Effects {
      hot: PaintMask::new(2.0, paintref.width, paintref.height),
      water: PaintMask::new(2.0, paintref.width, paintref.height),
    }
  }

  pub fn finalize(&mut self) {}

  pub fn to_svg_metafields(&self) -> String {
    let mut water = self.water.clone();
    water.reverse();
    let water_dist = water.manhattan_distance();

    let mut hot = self.hot.clone();
    hot.reverse();
    let hot_dist = hot.manhattan_distance();

    //    self.water.dilate_manhattan(10.0);
    //    self.water.reverse();

    format!(
      "data-effects-hot=\'{}\' data-effects-water=\'{}\'",
      convert_to_base64_png(&hot_dist, &self.hot, 0.0, 10.0),
      convert_to_base64_png(&water_dist, &water, 5.0, 14.0)
    )
  }
}

fn convert_to_base64_png(
  dist_values: &Vec<usize>,
  mask: &PaintMask,
  from: f32,
  to: f32,
) -> String {
  let wi = (mask.width / mask.precision) as u32;
  let hi = (mask.height / mask.precision) as u32;
  let img = ImageBuffer::from_fn(wi, hi, |x, y| {
    let v = dist_values[(x + y * wi) as usize];
    let s = smoothstep(from, to, v as f32);
    Luma([(255. * s) as u8])
    // Luma([(255. - mult * v as f32).max(0.).min(255.) as u8])

    // Luma([(255. - mult * v as f32).max(0.).min(255.) as u8])
    /*
    if mask.unsafe_get_at((x + y * wi) as usize) {
      Luma([255u8])
    } else {
      Luma([0u8])
    }
    */
  });
  let mut writer = Cursor::new(Vec::new());
  img
    .write_to(&mut writer, image::ImageOutputFormat::Png)
    .unwrap();
  let data = writer.into_inner();
  let base64_string = general_purpose::STANDARD.encode(&data);
  format!("data:image/png;base64,{}", base64_string)
}
