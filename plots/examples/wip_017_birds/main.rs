use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

// https://docs.rs/svg/0.8.0/svg/

#[derive(Copy, Clone)]
struct Bird {
    id: usize,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

fn add_vec2(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 + b.0, a.1 + b.1)
}

// inspiration https://cs.stanford.edu/people/eroberts/courses/soco/projects/2008-09/modeling-natural-systems/boids.html
impl Bird {
    fn new(id: usize, x: f64, y: f64, vx: f64, vy: f64) -> Bird {
        Bird { id, x, y, vx, vy }
    }

    fn rule1(&self, birds: &Vec<Bird>) -> (f64, f64) {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut count = 0.0;
        for bird in birds {
            if self.id == bird.id {
                continue;
            }
            sum_x += bird.x;
            sum_y += bird.y;
            count += 1.0;
        }
        let mut rx = 0.0;
        let mut ry = 0.0;
        if count > 0.0 {
            let avg_x = sum_x / count;
            let avg_y = sum_y / count;
            rx = (avg_x - self.x) / 100.0;
            ry = (avg_y - self.y) / 100.0;
        }
        return (rx, ry);
    }

    fn rule2(&self, birds: &Vec<Bird>) -> (f64, f64) {
        let mut rx = 0.0;
        let mut ry = 0.0;
        for bird in birds {
            let dx = self.x - bird.x;
            let dy = self.y - bird.y;
            if self.id == bird.id || dx * dx + dy * dy > 100.0 {
                continue;
            }
            rx += dx;
            ry += dy;
        }
        return (rx, ry);
    }

    fn rule3(&self, birds: &Vec<Bird>) -> (f64, f64) {
        let mut sum_vx = 0.0;
        let mut sum_vy = 0.0;
        let mut count = 0.0;
        for bird in birds {
            if self.id == bird.id {
                continue;
            }
            sum_vx += bird.vx;
            sum_vy += bird.vy;
            count += 1.0;
        }
        let mut rx = 0.0;
        let mut ry = 0.0;
        if count > 0.0 {
            let avg_vx = sum_vx / count;
            let avg_vy = sum_vy / count;
            rx = (avg_vx - self.vx) / 8.0;
            ry = (avg_vy - self.vy) / 8.0;
        }
        return (rx, ry);
    }

    fn tend_to_place(&self, x: f64, y: f64) -> (f64, f64) {
        ((x - self.x) / 100.0, (y - self.y) / 100.0)
    }

    fn update(&self, birds: &Vec<Bird>) -> Bird {
        let Bird {
            id,
            mut x,
            mut y,
            mut vx,
            mut vy,
        } = self;
        let v1 = self.rule1(birds);
        let v2 = self.rule2(birds);
        let v3 = self.rule3(birds);
        let v4 = (0.0, 0.0); //self.tend_to_place(100.0, 100.0);
        let (dx, dy) = add_vec2(add_vec2(add_vec2(v1, v2), v3), v4);
        vx += dx;
        vy += dy;
        // TODO limit velocity
        x += vx;
        y += vy;
        // todo bouncing position
        // todo anti flocking
        return Bird {
            id: *id,
            x,
            y,
            vx,
            vy,
        };
    }
}

fn main() {
    let mut rng = SmallRng::from_entropy();
    let mut birds = Vec::new();
    let mut birds_history = Vec::new();
    for id in 1..10 {
        let b = Bird::new(
            id,
            rng.gen_range(90.0, 110.0),
            rng.gen_range(90.0, 110.0),
            rng.gen_range(1.0, 2.0),
            rng.gen_range(-1.0, 1.0),
        );
        birds.push(b);
        birds_history.push(Vec::new());
    }
    for _run in 0..200 {
        let previous = birds.clone();
        let mut copy = Vec::new();
        for i in 0..birds.len() {
            let bird = birds[i];
            let next = bird.update(&previous);
            copy.push(next);
            birds_history[i].push((bird.x, bird.y));
        }
        birds = copy;
    }

    let mut data = Data::new();

    for h in birds_history {
        let mut first = true;
        for p in h {
            if first {
                data = data.move_to(p);
                first = false;
            } else {
                data = data.line_to(p);
            }
        }
    }

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0.4)
        .set("d", data);

    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(path)
        .add(gre::signature(1.0, (265.0, 195.0), "black"));

    svg::save("image.svg", &document).unwrap();
}
