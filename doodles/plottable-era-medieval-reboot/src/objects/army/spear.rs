use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polylines::{grow_as_rectangle, route_translate_rotate, Polylines},
};

pub struct Spear {
  pub routes: Polylines,
  pub polys: Vec<Vec<(f32, f32)>>,
}

impl Spear {
  pub fn init(clr: usize, origin: (f32, f32), size: f32, angle: f32) -> Self {
    let mut routes = vec![];
    let mut polys = vec![];

    let spear_len = size;
    let spear_w = 0.03 * size;
    let blade_w = 0.07 * size;
    let blade_len = 0.15 * size;
    let stick = grow_as_rectangle(
      (-spear_len / 2.0, 0.0),
      (spear_len / 2.0, 0.0),
      spear_w / 2.0,
    );
    let stick = route_translate_rotate(&stick, origin, -angle);
    polys.push(stick.clone());
    routes.push((clr, stick));

    let mut head = Vec::new();
    head.push((spear_len / 2.0, -blade_w / 2.0));
    head.push((spear_len / 2.0 + blade_len, 0.0));
    head.push((spear_len / 2.0, blade_w / 2.0));
    head.push(head[0]);
    let head = route_translate_rotate(&head, origin, -angle);
    polys.push(head.clone());
    routes.push((clr, head));

    Self { routes, polys }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let routes = regular_clip(&self.routes, paint);
    for poly in &self.polys {
      paint.paint_polygon(poly);
    }
    routes
  }
}
