use geo::{Line};
use gre::line_intersection::*;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn round_point ((x,y):(f64,f64)) -> (f64, f64) {
    let precision = 0.01;
    ((x/precision).round()*precision, (y/precision).round()*precision)
}

fn art(seed: f64, dim: usize, force:f64, length: f64, squares: Vec<&str>, freq_mul: f64, rotate: f64, small: f64, big: f64, length_mul: f64) -> Vec<Group> {
    let perlin = Perlin::new();
    let mut groups = Vec::new();

    let field = |xp: f64, yp: f64| -> f64 {
(        yp-0.5).atan2(xp-0.5)+force *(
        4.0 * perlin.get([freq_mul * xp, freq_mul * yp, seed + 1.])
            + 2.0 * perlin.get([freq_mul * 3. * xp, freq_mul * 3. * yp, seed + 2.])
            + 0.5 * perlin.get([freq_mul * 15. * xp, freq_mul * 15. * yp, seed + 3.]))
    };

    // VLine is our struct define below that record vertex lines
    let mut vlines: Vec<VLine> = Vec::new();

    let pad = 5.0;
    for (group, color) in squares.iter().enumerate() {
        let mut group_vlines = Vec::new();
        let gf = (group as f64) / (squares.len() as f64);
        let sz = small + gf * (big-small);
        let o = 105. - sz;
        let x_o = (297.-210.)/2.0 + o;
        let y_o = o;
        let width = 2. * sz;
        let height = 2. * sz;

        for x in 0..dim {
            let xp = (x as f64 + 0.5) / (dim as f64);
            for y in 0..dim {
                let yp = (y as f64 + 0.5) / (dim as f64);
                if x.min(y)!=0 && x.max(y)!=dim-1 {
                    continue;
                }
                let mut xo = x_o + width * xp;
                let mut yo = y_o + height * yp;
                xo -= x_o + width / 2.0;
                yo -= y_o + height / 2.0;
                let a = rotate * (PI / 2.0) * (group as f64);
                let r = (
                    a.cos() * xo + -a.sin() * yo,
                    a.sin() * xo + a.cos() * yo,
                );
                xo = r.0;
                yo = r.1;
                xo += x_o + width / 2.0;
                yo += y_o + height / 2.0;

                let origin = round_point((xo, yo));
                let mut vline = VLine::new(group, origin);
                let granularity = 1.0;
                for _i in 0..((length_mul.powf(group as f64) * length / granularity) as usize) {
                    let cur = vline.current();
                    if cur.0 < pad || cur.1 < pad || cur.0 > 297.-pad || cur.1 > 210.-pad {
                        break;
                    }
                    let angle = field((cur.0 - x_o) / width, (cur.1 - y_o) / height);
                    let next = round_point(vline.follow_angle(angle, granularity));
                    let collision = vlines.iter().find_map(|vl| vl.collides(cur, next));
                    if let Some(point) = collision {
                        vline.go(point);
                        break;
                    }
                    vline.go(next);
                }
                if vline.points.len() > 5 {
                    vlines.push(vline.clone());
                    group_vlines.push(vline);
                }
            }
        }
        let data = group_vlines.iter().fold(Data::new(), |data, vl| vl.draw(data));
        groups.push(layer(color).add(base_path(color, 0.2, data)));
    }

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
        .unwrap_or(50);
    let force = args
        .get(3)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(1.0);
    let length = args
        .get(4)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(100.0);
    let colors =
        args.get(5)
        .map(|s| s.split(",").collect::<Vec<&str>>())
        .unwrap_or(vec!["white", "gold", "orange"]);
    let default_bg = String::from("black");
    let bg = args.get(6).unwrap_or(&default_bg);
    let freq_mul = args.get(7)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(1.0);
    let rotate = args.get(8)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.5);
    let small = args.get(9)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(20.0);
    let big = args.get(10)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(80.0);
    let length_mul = args.get(11)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(1.0);

    let groups = art(seed, lines, force, length, colors.clone(), freq_mul, rotate, small, big, length_mul);
    let mut document = base_a4_landscape(bg);
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(1.0, (260.0, 190.0), colors[0]));
    svg::save("image.svg", &document).unwrap();
}

#[derive(Clone)]
struct VLine {
    group: usize,
    points: Vec<(f64, f64)>,
    aabb: ((f64, f64), (f64, f64)),
}
impl VLine {
    fn new(group: usize, initial: (f64, f64)) -> Self {
        let mut points = Vec::new();
        points.push(initial);
        VLine {
            group,
            points,
            aabb: (initial, initial),
        }
    }

    fn current(self: &Self) -> (f64, f64) {
        self.points[0]
    }

    fn follow_angle(self: &Self, a: f64, amp: f64) -> (f64, f64) {
        let cur = self.points[0];
        let p = (cur.0 + amp * a.cos(), cur.1 + amp * a.sin());
        p
    }

    fn go(self: &mut Self, p: (f64, f64)) {
        self.points.insert(0, p);
        if p.0 < self.aabb.0.0 {
            self.aabb.0.0 = p.0;
        }
        if p.1 < self.aabb.0.1 {
            self.aabb.0.1 = p.1;
        }
        if p.0 > self.aabb.1.0 {
            self.aabb.1.0 = p.0;
        }
        if p.1 > self.aabb.1.1 {
            self.aabb.1.1 = p.1;
        }
    }

    fn draw(self: &Self, data: Data) -> Data {
        let mut d = data;
        let l = self.points.len();
        let first = self.points[l - 1];
        d = d.move_to(first);
        for i in 0..l - 1 {
            let p = self.points[l - i - 2];
            d = d.line_to(p);
        }
        return d;
    }

    fn collides(self: &Self, from: (f64, f64), to: (f64, f64)) -> Option<(f64, f64)> {
        if from.0.min(to.0) < self.aabb.0.0 {
            return None;
        }
        if from.1.min(to.1) < self.aabb.0.1 {
            return None;
        }
        if from.0.max(to.0) > self.aabb.1.0 {
            return None;
        }
        if from.1.max(to.1) > self.aabb.1.1 {
            return None;
        }
        let segment = LineInterval::line_segment(Line {
            start: to.into(),
            end: from.into(),
        });
        let mut last = self.points[0];
        for i in 1..self.points.len() {
            let p = self.points[i];
            let intersection = segment
                .relate(&LineInterval::line_segment(Line {
                    start: p.into(),
                    end: last.into(),
                }))
                .unique_intersection()
                .map(|p| p.x_y());
            if intersection.is_some() {
                return intersection;
            }
            last = p;
        }
        return None;
    }
}
