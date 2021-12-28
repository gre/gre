mod utils;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::{Path, Group};
use svg::Document;
use wasm_bindgen::prelude::*;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Opts {
    pub primary_name: String,
    pub secondary_name: String,
    pub width: f64,
    pub height: f64,
    pub pad: f64,
    pub reverse_curve_x: bool,
    pub reverse_curve_y: bool,
    pub f1: f64,
    pub f2: f64,
    pub amp1: f64,
    pub amp2: f64,
    pub ricochets: usize,
    pub incr: f64,
    pub closing: bool,
    pub colordelta: f64,
    pub rad_start: f64,
    pub rad_incr: f64,
    pub precision: f64,
    pub max_passage: usize,
}

pub fn art(opts: &Opts) -> Document {
    let width = opts.width;
    let height = opts.height;
    let pad = opts.pad;
    let reverse_curve_x = opts.reverse_curve_x;
    let reverse_curve_y = opts.reverse_curve_y;
    let f1 = opts.f1;
    let f2 = opts.f2;
    let amp1 = opts.amp1;
    let amp2 = opts.amp2;
    let ricochets = opts.ricochets;
    let incr = opts.incr;
    let closing = opts.closing;
    let colordelta = opts.colordelta;
    let rad_start = opts.rad_start;
    let rad_incr = opts.rad_incr;
    let precision = opts.precision;
    let max_passage = opts.max_passage;

    let stroke_width = 0.35;
    let bounds = (pad, pad, width-pad, height-pad);

    let colors = vec!["#0FF", "#F0F"];
    let layers = colors.iter().enumerate().map(|(ci, &color)| {
        let label = if ci == 0 { opts.primary_name.clone() } else { opts.secondary_name.clone() };

        let mut passage = Passage2DCounter::new(precision, width, height);

        let mut should_draw_line = |a: (f64, f64), b: (f64, f64)| {
            passage.count(((a.0 + b.0) / 2., (a.1 + b.1) / 2.)) < max_passage
        };

        let mut routes: Vec<Vec<(f64,f64)>> = Vec::new();

        let parametric = |p: f64, rad: f64| -> (f64, f64) {
            let c1 = if reverse_curve_x { (f1 * PI * p).cos() } else { (f1 * PI * p).sin() };
            let c2 = if reverse_curve_y { (f2 * PI * p).sin() } else { (f2 * PI * p).cos() };
            (
                width/2. + rad * ((2. * PI * p).sin() * (1. - amp1) + c1 * amp1),
                height/2. + rad * ((2. * PI * p).cos() * (1. - amp2) + c2 * amp2)
            )
        };

        let ricochetsxincr = (ricochets as f64).max(3.);
        let rsize = if closing { ricochets + 1 } else { ricochets };

        let mut r = rad_start;
        let mut i_dt = ci as f64 * colordelta;
        loop {
            let mut route = Vec::new();
            let mut x = i_dt;
            let mut last_p = parametric(x, r);
            if out_of_boundaries(last_p, bounds) {
                break;
            }
            route.push((significant_str(last_p.0), significant_str(last_p.1)));
            let mut finished = false;
            for _j in 0..rsize {
                x += 1. / ricochetsxincr;
                let next_p = parametric(x, r);
                if out_of_boundaries(next_p, bounds) {
                    finished = true;
                    break;
                }
                let dx = next_p.0 - last_p.0;
                let dy = next_p.1 - last_p.1;
                let l = (dx * dx + dy * dy).sqrt();
                if l <= 0. { break; }
                let mut v = precision;
                loop {
                    if v > l { break; }
                    let p = (
                        significant_str(last_p.0 + dx * v / l),
                        significant_str(last_p.1 + dy * v / l),
                    );
                    route.push(p);
                    v += precision;
                }
                route.push((significant_str(next_p.0), significant_str(next_p.1)));
                last_p = next_p;
            }
            if finished {
                break;
            }
            routes.push(route);
            i_dt += incr;
            r += rad_incr;
        }

        let mut l = Group::new()
            .set("inkscape:groupmode", "layer")
            .set("inkscape:label", label)
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", stroke_width);

        let opacity: f64 = 0.6;
        
        let opdiff = 0.15 / (routes.len() as f64);
        let mut trace = 0f64;
        for r in routes.clone() {
            trace += 1f64;
            if r.len() < 2 {
                continue;
            }
            let data = render_route_when(Data::new(), r, &mut should_draw_line);
            l = l.add(
                Path::new()
                .set("opacity", (1000. * (opacity - trace * opdiff)).floor()/1000.0)
                .set("d", data)
            );
        }
        l
    });
        
    let mut doc = svg::Document::new()
    .set("viewBox", (0, 0, width, height))
    .set("width", format!("{}mm", width))
    .set("height", format!("{}mm", height))
    .set("style", "background:white")
    .set("xmlns:inkscape", "http://www.inkscape.org/namespaces/inkscape")
    .set("xmlns", "http://www.w3.org/2000/svg" );
    for g in layers {
        doc = doc.add(g);
    }
    return doc;
}

#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
    let opts = val.into_serde().unwrap();
    let doc = art(&opts);
    let str = doc.to_string();
    return str;
}

#[inline]
fn significant_str (f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}

fn render_route_when<F: FnMut(
    (f64, f64),
    (f64, f64),
) -> bool>(
    data: Data,
    route: Vec<(f64, f64)>,
    mut should_draw_line: F,
) -> Data {
    let mut first = true;
    let mut up = false;
    let mut last = (0.0, 0.0);
    let mut d = data;
    for p in route {
        if first {
            if should_draw_line(p, p) {
                first = false;
                d = d.move_to(p);
            }
        } else {
            if should_draw_line(last, p) {
                if up {
                    up = false;
                    d = d.move_to(last);
                }
                d = d.line_to(p);
            } else {
                up = true;
            }
        }
        last = p;
    }
    return d;
}

#[inline]
fn out_of_boundaries(
    p: (f64, f64),
    boundaries: (f64, f64, f64, f64),
) -> bool {
    p.0 < boundaries.0
        || p.0 > boundaries.2
        || p.1 < boundaries.1
        || p.1 > boundaries.3
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
