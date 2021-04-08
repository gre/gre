use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

struct GameOfLife {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}
impl GameOfLife {
    fn size(&self) -> usize {
        return self.width * self.height;
    }
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

fn random_gol(width: usize, height: usize, seed0: u8) -> GameOfLife {
    let mut cells = vec![false; width * height];
    let mut rng = SmallRng::from_seed([seed0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    for i in 0..cells.len() {
        cells[i] = rng.next_u32() % 2 == 1;
    }
    return GameOfLife {
        width,
        height,
        cells,
    };
}

fn main() {
    let size = 80;
    let seed = 6;
    let runs = 500;

    let mut gol = random_gol(size, size, seed);
    let mut stats = vec![0.0; gol.cells.len()];
    for _i in 0..runs {
        gol = gol.next();
        for i in 0..gol.cells.len() {
            let v = if gol.cells[i] { 1.0 } else { 0.0 };
            stats[i] = stats[i] * 0.8 + v;
        }
    }

    let mut document = Document::new().set("viewBox", (0, 0, gol.width, gol.height));

    for i in 0..gol.size() {
        let (x, y) = gol.reverse(i);
        let score = stats[i];
        let mut s = score * 0.3;
        if s < 1.0 && s > 0.2 {
            let cx = (x as f64) + 0.5;
            let cy = (y as f64) + 0.5;
            let hr = s / 3.;
            let data = Data::new()
                .move_to((cx - hr, cy - hr))
                .line_to((cx + hr, cy + hr))
                .move_to((cx - hr, cy + hr))
                .line_to((cx + hr, cy - hr));
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.1)
                .set("d", data);
            document = document.add(path);
            s = 0.5 * s - 1.0;
        }
        s = f64::min(s, 0.5);
        loop {
            if s < 0.1 {
                break;
            }
            let circle = Circle::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.1)
                .set("cx", (x as f64) + 0.5)
                .set("cy", (y as f64) + 0.5)
                .set("r", s);
            document = document.add(circle);
            s -= 0.25;
        }
    }

    svg::save("image.svg", &document).unwrap();
}
