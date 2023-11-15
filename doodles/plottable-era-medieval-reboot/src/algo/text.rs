/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
use crate::algo::paintmask::*;
use crate::algo::wormsfilling::*;
use fontdue::layout::*;
use fontdue::*;
use rand::prelude::*;

pub fn load_font(fontdata: &Vec<u8>) -> Font {
  Font::from_bytes(fontdata.clone(), FontSettings::default()).unwrap()
}

pub fn draw_font_with_worms_filling<R: Rng>(
  rng: &mut R,
  font: &mut Font,
  paint: &mut PaintMask,
  fontsize: f32,
  pos: (f32, f32),
  text: &str,
  clr: usize,
  iterations: usize,
  density: f32,
  growpad: f32,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let filling = WormsFilling::rand(rng);

  let mut drawing = paint.clone_empty();
  let prec = drawing.precision;

  let fonts = &[font.clone()];

  let mut routes = Vec::new();

  let px = (fontsize / prec) as f32;

  let mut layout = Layout::new(CoordinateSystem::PositiveYDown);

  let mut settings = LayoutSettings::default();
  settings.x = (pos.0 / prec) as f32;
  settings.y = (pos.1 / prec) as f32;
  layout.reset(&settings);
  layout.append(fonts, &TextStyle::new(text, px, 0));

  let mut maxw = 0.0f32;
  for glyph in layout.glyphs() {
    let (metrics, bytes) = font.rasterize_config(glyph.key);
    if glyph.parent == '\n' {
      continue;
    }
    let o = (glyph.x as f32 * prec, glyph.y as f32 * prec);
    maxw = maxw.max(o.0 + metrics.width as f32 * prec);
    drawing.paint_pixels(o, &bytes, metrics.width);
  }

  routes.extend(filling.fill_in_paint(
    rng,
    &drawing,
    clr,
    density,
    (pos.0, pos.1, maxw, pos.1 + layout.height() as f32),
    iterations,
  ));

  drawing.grow(growpad);

  // we don't need collision
  // routes = regular_clip(&routes, paint);

  paint.paint(&drawing);

  routes
}
