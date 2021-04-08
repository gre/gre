use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

struct Plot {
    points: Vec<(f64, f64)>,
}
impl Plot {
    fn new(initial: (f64, f64)) -> Self {
        let mut points = Vec::new();
        points.push(initial);
        Plot { points }
    }
    fn follow_angle(self: &mut Self, a: f64, amp: f64) {
        let cur = self.points[0];
        let p =
            (cur.0 + amp * a.cos(), cur.1 + amp * a.sin());
        self.points.insert(0, p);
    }
    fn draw(self: &Self, data: Data) -> Data {
        let mut d = data;
        let mut points = self.points.clone();
        points.reverse();
        let first = points.remove(0);
        d = d.move_to(first);
        for p in points {
            d = d.line_to(p);
        }
        return d;
    }
}

fn art(_seed: f64) -> Vec<Group> {
    let mut groups = Vec::new();

    let config = vec![("red", 0.0), ("blue", 1.0)];

    let pad = 10.0;
    let width = 190.0;
    let height = 190.0;

    for (color, i) in config {
        let mut data = Data::new();

        let ys = 20;
        let xs = 20;
        for y in 0..ys {
            let mut p = Plot::new((
                pad,
                pad + height * (y as f64 + 0.5)
                    / (ys as f64),
            ));
            for _i in 0..100 {
                p.follow_angle(0.1, 1.0)
            }
            data = p.draw(data);
        }

        groups.push(
            layer(color).add(base_path(color, 0.2, data)),
        );
    }

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let groups = art(seed);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (260.0, 190.0),
        "black",
    ));
    svg::save("image.svg", &document).unwrap();
}
