use gre::*;
use clap::Clap;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

#[derive(Clap)]
#[clap()]
pub struct Opts {
    #[clap(short, long, default_value = "image.svg")]
    file: String,
    #[clap(short, long, default_value = "44.0")]
    pub seed: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed1: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed2: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed3: f64,
}

fn main() {
    let opts: Opts = Opts::parse();
    let height = 280.0;
    let width = 190.0;
    let lines = 1200;
    let perlin = Perlin::new();
    let seed = opts.seed;
    let mut rng = rng_from_seed(seed);
    let seed1 = opts.seed1 + rng.gen_range(-10.0, 10.0);
    let seed2 = opts.seed2 + rng.gen_range(-10.0, 10.0);
    let seed3 = opts.seed3 + rng.gen_range(-10.0, 10.0);
    let frequencymul = mix(0.5, 3.0, (seed1 - 1.0).abs() / 5.0);
    let line_length = mix(0.0, 200.0, (seed1.abs() * 0.111).fract());
    let dash_length = mix(0.0, 200.0, (seed2.abs() * 0.222).fract());
    let amp: f64 = mix(0.5, 2.0, (seed2 * 0.3).abs() / 5.0);
    let color_alt = ((seed3.abs() * 40.0) as usize) + 1;
    let angle_velocity_field = |x, y, l, _length| {
        (0.5-(l as f64 - 0.5).abs()) * amp * (
            0.5 * perlin.get([0.7 * seed, frequencymul * 4.0 * x, frequencymul * 4.0 * y])
            + 0.3 * perlin.get([frequencymul * 7.0 * x, seed / 3., frequencymul * 7.0 * y])
            + 0.2 * perlin.get([frequencymul * 11.0 * x, frequencymul * 11.0 * y, seed / 7.]))
    };
    let w = mix(12., 50., (seed3.abs() / 10.).fract());
    let wf = mix(0.5, 10.0, (seed1.abs() / 20.).fract());
    let offset = mix(0.4, 0.9, (seed2.abs() / 5.).fract());
    let origin = |l| {
        (
            width / 2.0 + w * (l as f64 * wf).sin(),
            height * (1.0 - offset * (l as f64 + 0.5) / (lines as f64)),
        )
    };
    let initial_angle = |_l| -PI / 2.0;
    let art = render_angle_velocity_field(
        width,
        height,
        lines,
        color_alt,
        line_length,
        dash_length,
        angle_velocity_field,
        origin,
        initial_angle,
    )
    .set("transform", "translate(10,10)");
    svg::save(opts.file, &make_svg(art)).unwrap();
}

fn render_angle_velocity_field(
    width: f64,
    height: f64,
    lines: usize,
    color_alt: usize,
    line_length: f64,
    dash_length: f64,
    angle_velocity_field: impl Fn(f64, f64, f64, f64) -> f64,
    origin: impl Fn(usize) -> (f64, f64),
    initial_angle: impl Fn(usize) -> f64,
) -> Group {
    let mut passage = Passage2DCounter::new(0.4, width, height);

    let colors = vec!["#F90", "#09F"];
    let mut layers = Vec::new();

    for (ci, &color) in colors.iter().enumerate() {
        let mut data = Data::new();
        let step = 1.0;
        let mut routes = Vec::new();

        for l in 0..lines {
            if (l / color_alt) % colors.len() != ci {
                continue;
            }
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
                route.push(p);
                length += step;
                if length > line_length + dash_length {
                    break;
                }
            }
            routes.push(route);
        }

        let mut paths = Vec::new();
        for route in routes {
            let mut path = Vec::new();
            let mut length = 0.0;
            let mut i = 0;
            let mut last_p = (0.0, 0.0);
            let dashstyle = 10;
            for &p in route.iter().rev() {
                if p.0 < 0.0 || p.1 < 0.0 || p.0 > width || p.1 > height {
                    // out of bounds. reset
                    path = Vec::new();
                    last_p = (0.0, 0.0);
                }
                else {
                    if passage.count(p) > 5 {
                        break;
                    }
                    if length < dash_length {
                        if i % dashstyle == 0 {
                            last_p = p;
                        }
                        else if i % dashstyle == 1 && last_p.0 > 0.0 {
                            paths.push(vec![last_p, p]);
                        }
                    }
                    else {
                        path.push(p);
                    }
                }
                length += step;
                i += 1;
            }
            paths.push(path);
        }

        for path in paths {
            data = render_route(data, path);
        }

        let mut l = layer(color);

        l = l.add(base_path(color, 0.34, data));
        
        layers.push(l);
    }

    let mut group = Group::new();
    for l in layers {
        group = group.add(l);
    }
    group
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