use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polylines::{route_translate_rotate, Polylines},
};

pub struct Arrow {
  pub routes: Polylines,
}

impl Arrow {
  pub fn init(clr: usize, origin: (f32, f32), size: f32, angle: f32) -> Self {
    let mut routes = vec![];

    let arrow_len = size;
    let blade_w = 0.1 * size;
    let blade_len = 0.2 * size;
    let stick = vec![(-arrow_len / 2.0, 0.0), (arrow_len / 2.0, 0.0)];
    let stick = route_translate_rotate(&stick, origin, angle);
    routes.push((clr, stick));

    let mut head = Vec::new();
    head.push((arrow_len / 2.0, -blade_w / 2.0));
    head.push((arrow_len / 2.0 + blade_len, 0.0));
    head.push((arrow_len / 2.0, blade_w / 2.0));
    head.push(head[0]);
    let head = route_translate_rotate(&head, origin, angle);
    routes.push((clr, head));

    let mut feather = Vec::new();
    let w = 0.1 * size;
    let w2 = 0.05 * size;
    let l = 0.2 * size;
    feather.push((-arrow_len / 2.0 - w, -w2));
    feather.push((-arrow_len / 2.0 + l, 0.0));
    feather.push((-arrow_len / 2.0 - w, w2));
    let feather = route_translate_rotate(&feather, origin, angle);
    routes.push((clr, feather));

    Self { routes }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let routes = regular_clip(&self.routes, paint);
    for (_, route) in self.routes.iter() {
      paint.paint_polyline(route, 1.2);
    }
    routes
  }
}
