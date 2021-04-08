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


fn art(seed: f64) -> Vec<Group> {
    let perlin = Perlin::new();
    let mut groups = Vec::new();

    // VLine is our struct define below that record vertex lines
    let mut vlines: Vec<VLine> = Vec::new();

    let pad = 5.0;
    let color = "black";
    let y_offset = 0.0;
    let width = 210.0;
    let height = 297.0;
    let length = 200.0;

    let get_angle = |i, i_f, l| {
        i_f +
        PI * 0.5 * (0.5 + (i % 3) as f64)
        + 30.
          * (0.2 + 0.8 * i_f)
          * (0.5 + 0.5 * l)
          * perlin.get([seed, 2. * i_f, l as f64 * 3.0])
        + 5. * perlin.get([seed, i_f, l as f64 * 20.0]) *
          perlin.get([i_f, 1.0 + i_f * 3.0]).max(0.0)
    };
    let golden_angle = PI * (3.0 - (5.0 as f64).sqrt());

    let radius_from = 0.0;
    let radius_to = 0.35;

    let samples = 1000;
    for i in 0..samples {
        let a = golden_angle * ((samples - i - 1) as f64);
        let amp =
            radius_from + (radius_to - radius_from) * ((i as f64) / (samples as f64)).powf(0.5);
        let x_0 = width * (0.5 + a.cos() * amp);
        let y_0 = y_offset + (height-width).max(0.0)/2.0 + height.min(width) * (0.5 + a.sin() * amp);
        let origin = round_point((x_0, y_0));
        let mut vline = VLine::new(0, origin);
        let granularity = 1.0;
        for l in 0..((length / granularity) as usize) {
            let cur = vline.current();
            if cur.0 < pad || cur.1 < pad || cur.0 > 210.-pad || cur.1 > 297.-pad {
                break;
            }
            let angle = get_angle(
                i,
                (i as f64) / (samples as f64),
                l as f64 * granularity / length
            );
            let next = round_point(vline.follow_angle(angle, granularity));
            let collision = vlines.iter().find_map(|vl| vl.collides(cur, next));
            if let Some(point) = collision {
                vline.go(point);
                break;
            }
            vline.go(next);
        }
        if (vline.points.len() as f64) < (length / 8.0) {
            continue;
        }
        vlines.push(vline.clone());
    }
    let data = vlines.iter().fold(Data::new(), |data, vl| vl.draw(data));
    groups.push(layer(color).add(base_path(color, 0.5, data)));

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);

    let groups = art(seed);
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(1.0, (180.0, 280.0), "black"));
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
