mod utils;
use byteorder::*;
use noise::*;
use rand::prelude::*;
use rand::Rng;
use serde::Deserialize;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use svg::Document;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
pub struct Opts {
    pub seed: f64,
    pub primary_name: String,
    pub secondary_name: String,
}

pub struct Passage2DCounter {
    granularity: f64,
    width: f64,
    height: f64,
    counters: Vec<usize>,
}
impl Passage2DCounter {
    pub fn new(granularity: f64, width: f64, height: f64) -> Self {
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
        let wi = (self.width / self.granularity).ceil() as usize;
        let hi = (self.height / self.granularity).ceil() as usize;
        let xi = ((x / self.granularity).round() as usize).max(0).min(wi - 1);
        let yi = ((y / self.granularity).round() as usize).max(0).min(hi - 1);
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

trait FloatIterExt {
    fn float_min(&mut self) -> f64;
    fn float_max(&mut self) -> f64;
}

impl<T> FloatIterExt for T
where
    T: Iterator<Item = f64>,
{
    fn float_max(&mut self) -> f64 {
        self.fold(f64::NAN, f64::max)
    }
    fn float_min(&mut self) -> f64 {
        self.fold(f64::NAN, f64::min)
    }
}

fn art(opts: &Opts) -> Document {
    let seed = opts.seed;
    let pad = 10.0;
    let width = 210.;
    let height = 297.;
    let perlin = Perlin::new();
    let mut rng = rng_from_seed(seed);
    let amp: f64 =
        rng.gen_range(0.15, 0.5) * rng.gen_range(0.2, 1.1) + rng.gen_range(0f64, 1.0).powf(16.0);
    let f1 = 1.0
        + 5.0 * (1.0 - rng.gen_range(0f64, 1.0).powf(3.0))
        + rng.gen_range(0.0, 20.0) * rng.gen_range(0f64, 1.0).powf(2.0)
        + 100.0 * rng.gen_range(0f64, 1.0).powf(8.0);
    let f2 = f1 * rng.gen_range(1.0, 3.0);
    let f3 = f2 * rng.gen_range(1.0, 3.0);
    let angle_velocity_field = |x, y, _l, _length| {
        (1.0 - 0.8 * y * y)
            * amp
            * (0.5 * perlin.get([f1 * x, f1 * y, seed + 1.1])
                + 0.3 * perlin.get([f2 * x, f2 * y, seed * 7.7])
                + 0.2 * perlin.get([f3 * x, f3 * y, seed / 3.0]))
    };
    let dashstyle = rng.gen_range(6, 16);
    let w = 10.0 + (rng.gen_range(0., 160.) * rng.gen_range(0f64, 1.0).powf(4.0)).min(width * 0.6);
    let h = (0.6 + 0.6 * rng.gen_range(0f64, 1.0).powf(1.2)).min(1.0);
    let lines = 600 + (h * w * 60.0) as usize;
    let wf = 1.0;
    let basey = rng.gen_range(1.05, 1.3);
    let line_length = rng.gen_range(30.0, 60.0);
    let dash_length = (rng.gen_range(-40., 100.) + rng.gen_range(130.0, 300.0) * (1. - h)).max(0.0);
    let color_div = (2.0 + rng.gen_range(0.0, 8.0) * rng.gen_range(0.0, 1.0)) as usize;
    let color_hsplit = rng.gen_bool(0.4);
    let color_alt = if color_hsplit {
        lines / color_div
    } else {
        color_div
    };
    let colors = vec!["#0FF", "#F0F"];
    let disp = rng.gen_range(-40f64, 80.0).max(0.0);
    let origin_dx =
        |p| disp * (0.8 * perlin.get([seed, 0.4 * p]) + 0.2 * perlin.get([seed, 1. * p]));
    let origin_samples: Vec<f64> = (0..9).map(|i| origin_dx(i as f64 / 8.0)).collect();
    let origin_min = origin_samples.iter().cloned().float_min();
    let origin_max = origin_samples.iter().cloned().float_max();
    let center_delta = width / 2.0 - (origin_max - origin_min) / 2.0;
    let origin = |l| {
        let p = (l as f64 + 0.5) / (lines as f64);
        let dx = origin_dx(p);
        (
            dx + center_delta + w * (l as f64 * wf).sin(),
            (height - pad) * (basey - h * p.powf(0.8)).min(1.0),
        )
    };
    let initial_angle = |_l| -PI / 2.0;
    let mut passage = Passage2DCounter::new(0.45, width, height);
    let layers: Vec<Group> = colors
        .iter()
        .enumerate()
        .map(|(ci, &color)| {
            let step = 1.0;
            let mut routes = Vec::new();
            for l in 0..lines {
                if (l / color_alt) % colors.len() != ci {
                    continue;
                }
                let max_l = line_length + dash_length + rng.gen_range(0.0, 20.0);
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
                    if length > max_l {
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
                for &p in route.iter().rev() {
                    if p.0 < pad || p.1 < pad || p.0 > width - pad || p.1 > height - pad {
                        // out of bounds. reset
                        path = Vec::new();
                        last_p = (0.0, 0.0);
                    } else {
                        if passage.count(p) > 7 {
                            break;
                        }
                        if length < dash_length {
                            if i % dashstyle == 0 {
                                last_p = p;
                            } else if i % dashstyle == 1 && last_p.0 > 0.0 {
                                paths.push(vec![last_p, p]);
                            }
                        } else {
                            path.push(p);
                        }
                    }
                    length += step;
                    i += 1;
                }
                paths.push(path);
            }

            let label = if ci == 0 {
                opts.primary_name.clone()
            } else {
                opts.secondary_name.clone()
            };
            let mut l = Group::new()
                .set("inkscape:groupmode", "layer")
                .set("inkscape:label", label)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 0.35);

            let opacity: f64 = 0.6;
            let opdiff = 0.15 / (paths.len() as f64);
            let mut trace = 0f64;
            for path in paths {
                trace += 1f64;
                let data = render_route(Data::new(), path);
                l = l.add(
                    Path::new()
                        .set(
                            "opacity",
                            (1000. * (opacity - trace * opdiff)).floor() / 1000.0,
                        )
                        .set("d", data),
                );
            }

            l
        })
        .collect();

    let trait_width = if w < 16.0 {
        "Thin"
    } else if w > width * 0.5 {
        "Full"
    } else if w < width * 0.3 {
        "Normal"
    } else {
        "Large"
    };
    let trait_elevation = if h < 0.7 {
        "Small"
    } else if h > 0.99 {
        "Full"
    } else {
        "Normal"
    };
    let stippling_ratio = dash_length / (line_length + dash_length);
    let trait_stippling = if stippling_ratio < 0.001 {
        "None"
    } else if stippling_ratio < 0.3 {
        "Low"
    } else if stippling_ratio < 0.75 {
        "Normal"
    } else {
        "High"
    };
    let trait_noise = if amp < 0.05 {
        "Very Low"
    } else if amp < 0.1 {
        "Low"
    } else if amp < 0.5 {
        "Normal"
    } else if amp > 1.0 {
        "Extreme"
    } else {
        "High"
    };
    let trait_noise_freq = if f1 < 5.0 {
        "Small"
    } else if f1 < 12.0 {
        "Normal"
    } else if f1 > 24.0 {
        "Extreme"
    } else {
        "High"
    };
    let trait_ink_distribution = if color_hsplit {
        format!("Split by {}", color_div)
    } else {
        format!("Alternate % {}", color_div)
    };

    let traits = format!("{{\"Width\":\"{}\",\"Elevation\":\"{}\",\"Sippling\":\"{}\",\"Noise\":\"{}\",\"Noise Frequency\":\"{}\",\"Ink Distribution\":\"{}\"}}", trait_width,trait_elevation,trait_stippling,trait_noise,trait_noise_freq,trait_ink_distribution);

    let mut document = svg::Document::new()
        .set("viewBox", (0, 0, 210, 297))
        .set("data-traits", traits)
        .set("width", "210mm")
        .set("height", "297mm")
        .set("style", "background:white")
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("xmlns", "http://www.w3.org/2000/svg");
    for l in layers {
        document = document.add(l);
    }
    document
}

#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
    let opts = val.into_serde().unwrap();
    let doc = art(&opts);
    let str = doc.to_string();
    return str;
}

fn render_route(data: Data, route: Vec<(f64, f64)>) -> Data {
    if route.len() == 0 {
        return data;
    }
    let first_p = route[0];
    let mut d = data.move_to((significant_str(first_p.0), significant_str(first_p.1)));
    for p in route {
        d = d.line_to((significant_str(p.0), significant_str(p.1)));
    }
    return d;
}

#[inline]
fn significant_str(f: f64) -> f64 {
    (f * 100.0).floor() / 100.0
}

fn rng_from_seed(s: f64) -> impl Rng {
    let mut bs = [0; 16];
    bs.as_mut().write_f64::<BigEndian>(s).unwrap();
    let mut rng = SmallRng::from_seed(bs);
    // run it a while to have better randomness
    for _i in 0..50 {
        rng.gen::<f64>();
    }
    return rng;
}
