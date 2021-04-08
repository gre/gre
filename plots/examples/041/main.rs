use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(seed: f64, ys: usize, factor: f64) -> Vec<Group> {
    let perlin = Perlin::new();
    let mut groups = Vec::new();

    let x_o = 2.0;
    let y_o = 60.0 * factor * factor;
    let width = 206.0;
    let height = 210.0 / factor;

    let mut data = Data::new();

    let mut vlines: Vec<VLine> = Vec::new();

    for y in 0..ys {
        let yp = ((ys - y) as f64 - 0.5) / (ys as f64);
        let mut vline = VLine::new((x_o, y_o + height * yp));
        loop {
            let x = vline.current().0;
            if x > width || x < x_o || vline.points.len() > 2000 {
                break;
            }
            let xp = (x - x_o) / width;
            let angle = (1.0 - yp)
                * ((20.0 + 15.0 * perlin.get([0.0, 9.0 * yp, seed])) * xp + 20.0 * yp).cos()
                + 0.4 * perlin.get([2. * xp, 2. * yp, seed + 1.])
                + 0.3 * perlin.get([4. * xp, 4. * yp, seed + 2.])
                + 0.2 * perlin.get([9. * xp, 9. * yp, seed + 3.]);
            vline.follow_angle(factor * angle, 0.5);
        }
        let should_draw = |p| !vlines.iter().any(|vl| !vl.gt(p));
        data = vline.draw(data, &should_draw);
        vlines.push(vline);
    }

    groups.push(layer("black").add(base_path("black", 0.2, data)));

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let lines = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(20);
    let factor = args
        .get(3)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(1.);
    println!("{} {} {}", seed, lines, factor);
    let groups = art(seed, lines, factor);
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(1.0, (180.0, 280.0), "black"));
    svg::save("image.svg", &document).unwrap();
}

struct VLine {
    points: Vec<(f64, f64)>,
}
impl VLine {
    fn new(initial: (f64, f64)) -> Self {
        let mut points = Vec::new();
        points.push(initial);
        VLine { points }
    }
    fn current(self: &Self) -> (f64, f64) {
        self.points[0]
    }
    fn follow_angle(self: &mut Self, a: f64, amp: f64) {
        let cur = self.points[0];
        let p = (cur.0 + amp * a.cos(), cur.1 + amp * a.sin());
        self.points.insert(0, p);
    }
    fn draw(self: &Self, data: Data, should_draw: &dyn Fn((f64, f64)) -> bool) -> Data {
        let mut d = data;
        let mut points = self.points.clone();
        points.reverse();
        let first = points.remove(0);
        d = d.move_to(first);
        let mut drawing = false;
        let mut move_to = first;
        for p in points {
            if should_draw(p) {
                if !drawing {
                    d = d.move_to(move_to);
                }
                d = d.line_to(p);
                drawing = true;
            } else {
                drawing = false;
                move_to = p;
            }
        }
        return d;
    }
    // check if line is above a point
    fn gt(self: &Self, point: (f64, f64)) -> bool {
        let origin = self.points[0];
        let mut last = origin;
        for p in self.points.clone() {
            if p.0 == last.0 {
                if p.0 == point.0 {
                    return p.1 > point.1;
                } else {
                    continue;
                }
            }
            let (a, b) = if last.0 < p.0 { (last, p) } else { (p, last) };
            if a.0 <= point.0 && point.0 <= b.0 {
                let lerp = (p.0 - a.0) / (b.0 - a.0);
                let y = a.1 + (b.1 - a.1) * lerp;
                return y > point.1;
            }
            last = p;
        }
        false
    }
}
