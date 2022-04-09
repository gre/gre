use clap::Clap;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::Group;

#[derive(Clap)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "0")]
  index: usize,
  #[clap(short, long, default_value = "8")]
  frames: usize,
  #[clap(short, long, default_value = "100.0")]
  width: f64,
  #[clap(short, long, default_value = "100.0")]
  height: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let color = "#000";
  let width = opts.width;
  let height = opts.height;
  let pad = 10.0;
  let stroke_width = 0.35;
  let progress = opts.index as f64 / (opts.frames as f64);

  let w = 4;
  let h = 4;
  let mut cells = vec![false; w * h];
  cells[1] = true;
  cells[6] = true;
  cells[8] = true;
  cells[9] = true;
  cells[10] = true;
  let mut gol = GameOfLife {
    width: w,
    height: h,
    cells,
  };
  for _i in 0..((progress * 4.0) as usize) {
    gol = gol.next();
  }
  let lookup_game_of_life = |x: f64, y: f64| -> f64 {
    let xi = (w as f64 + 1.0) * (x - pad) / (width - 2.0 * pad) + progress - 1.0;
    let yi = (h as f64 + 1.0) * (y - pad) / (height - 2.0 * pad) + progress - 1.0;
    if xi >= 0.0 && yi >= 0.0 && gol.alive(xi as usize, yi as usize) {
      1.0
    } else {
      0.0
    }
  };

  let mut routes = Vec::new();
  let mut y = 0.0;
  loop {
    if y > height {
      break;
    }
    let mut x = 0.0;
    let mut start_p: Option<(f64, f64)> = None;
    loop {
      if x > width {
        break;
      }
      let is_down = lookup_game_of_life(x, y) > 0.5;
      if is_down {
        if !start_p.is_some() {
          start_p = Some((x, y));
        }
      } else {
        if start_p.is_some() {
          routes.push(vec![start_p.unwrap(), (x, y)]);
          start_p = None;
        }
      }
      x += 0.1;
    }
    y += stroke_width;
  }

  let mut data = Data::new();
  for route in routes {
    data = render_route(data, route);
  }
  let mut l = layer(color);
  l = l.add(base_path(color, stroke_width, data));
  vec![l]
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}

struct GameOfLife {
  width: usize,
  height: usize,
  cells: Vec<bool>,
}
impl GameOfLife {
  fn index(&self, x: usize, y: usize) -> usize {
    return x + y * self.width;
  }
  fn reverse(&self, i: usize) -> (usize, usize) {
    let y = i / self.width;
    let x = i - self.width * y;
    return (x, y);
  }
  fn alive(&self, x: usize, y: usize) -> bool {
    if x >= self.width || y >= self.height {
      return false;
    }
    let alive = self.cells[self.index(x, y)];
    return alive;
  }
  fn next(&self) -> GameOfLife {
    let width = self.width;
    let height = self.height;
    let mut cells = vec![false; width * height];
    for i in 0..self.cells.len() {
      let (x, y) = self.reverse(i);
      let sum: u8 = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
      ]
      .into_iter()
      .map(|(dx, dy)| {
        let xi = ((x as i32) + dx) as usize;
        let yi = ((y as i32) + dy) as usize;
        let v = self.alive(xi, yi) as u8;
        return v;
      })
      .sum();
      cells[i] = sum == 3 || sum == 2 && self.alive(x, y);
    }

    return GameOfLife {
      width,
      height,
      cells,
    };
  }
}
