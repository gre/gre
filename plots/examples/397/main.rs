use gre::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(30.0);
    let height = 280.0;
    let width = 190.0;
    let lines = 800;
    let perlin = Perlin::new();
    let mut rng = rng_from_seed(seed);
    let line_length = rng.gen_range(60.0, 140.0);
    let amp: f64 = rng.gen_range(0.08, 0.4);
    let angle_velocity_field = |x, y, _l, _length| {
        amp * (0.5 * perlin.get([7.0 * x, 7.0 * y, seed])
            + 0.3 * perlin.get([11.0 * x, 11.0 * y, seed])
            + 0.2 * perlin.get([21.0 * x, 21.0 * y, seed]))
    };
    let w = rng.gen_range(8., 16.);
    let wf = rng.gen_range(2., 5.);
    let origin = |l| {
        (
            width / 2.0 + w * (l as f64 * wf).sin(),
            height * (1.0 - 0.7 * (l as f64 + 0.5) / (lines as f64)),
        )
    };
    let initial_angle = |_l| -PI / 2.0;
    let art = render_angle_velocity_field(
        width,
        height,
        lines,
        line_length,
        angle_velocity_field,
        origin,
        initial_angle,
    )
    .set("transform", "translate(10,10)");
    svg::save("image.svg", &make_svg(art)).unwrap();
}

fn render_angle_velocity_field(
    width: f64,
    height: f64,
    lines: usize,
    line_max_length: f64,
    angle_velocity_field: impl Fn(f64, f64, f64, f64) -> f64,
    origin: impl Fn(usize) -> (f64, f64),
    initial_angle: impl Fn(usize) -> f64,
) -> Group {
    let mut data = Data::new();
    let step = 1.0;
    let mut routes = Vec::new();

    for l in 0..lines {
        let mut angle = initial_angle(l);
        let mut length = 0.0;
        let mut p = origin(l);
        let mut route = Vec::new();
        route.push(p);
        loop {
            let a = angle_velocity_field(
                p.0 / width,
                p.1 / height,
                (l as f64) / (lines as f64),
                length / width.min(height),
            );
            angle += step * a;
            p.0 += step * angle.cos();
            p.1 += step * angle.sin();
            if p.0 < 0. || p.1 < 0. || p.0 > width || p.1 > height {
                break;
            }
            route.push(p);
            length += step;
            if length > line_max_length {
                break;
            }
        }
        routes.push(route);
    }

    let mut passage = Passage2DCounter::new(0.4, width, height);
    for route in routes {
        let mut path = Vec::new();
        for &p in route.iter().rev() {
            if passage.count(p) > 4 {
                break;
            }
            path.push(p);
        }
        data = render_route(data, path);
    }

    return Group::new().add(
        Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.32)
            .set("d", data),
    );
}

fn make_svg(art: Group) -> Document {
    Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 210, 297))
        .set("width", "210mm")
        .set("height", "297mm")
        .add(art)
}



pub struct Passage2DCounter {
    granularity: f64,
    width: f64,
    height: f64,
    counters: Vec<usize>,
}
impl Passage2DCounter {
    pub fn new(
        granularity: f64,
        width: f64,
        height: f64,
    ) -> Self {
        let wi = (width / granularity).ceil() as usize;
        let hi = (height / granularity).ceil() as usize;
        let counters = vec![0; wi * hi];
        Passage2DCounter {
            granularity,
            width,
            height,
            counters,
        }
    }
    fn index(self: &Self, (x, y): (f64, f64)) -> usize {
        let wi =
            (self.width / self.granularity).ceil() as usize;
        let hi = (self.height / self.granularity).ceil()
            as usize;
        let xi = ((x / self.granularity).round() as usize)
            .max(0)
            .min(wi - 1);
        let yi = ((y / self.granularity).round() as usize)
            .max(0)
            .min(hi - 1);
        yi * wi + xi
    }
    pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
        let i = self.index(p);
        let v = self.counters[i] + 1;
        self.counters[i] = v;
        v
    }
    pub fn get(self: &Self, p: (f64, f64)) -> usize {
        self.counters[self.index(p)]
    }
}