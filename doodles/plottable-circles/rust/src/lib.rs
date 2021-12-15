mod utils;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;
use wasm_bindgen::prelude::*;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Opts {
    pub seed: f64,
    pub rings: usize,
    pub primary_name: String,
    pub secondary_name: String,
    pub ringcenter: f64,
    pub ring_resolution_multiplier: f64,
    pub ring_w_lower: f64,
    pub ring_w_upper: f64,
    pub ring_max_width: f64,
    pub line_gap_max: f64,
    pub ring_1x: f64,
    pub ring_1y: f64,
    pub ring_1xf2x: f64,
    pub ring_1xf2y: f64,
    pub ring_1yf2x: f64,
    pub ring_1yf2y: f64,
    pub ring_1y3: f64,
    pub ring_1yf3x: f64,
    pub ring_1yf3y: f64,
    pub size: f64,
    pub zigzag_count: usize,
    pub zigzag_rep: usize,
    pub zigzag_ring_resolution_multiplier: f64,

}

pub fn art(opts: &Opts) -> Document {
    let width = 210.0;
    let height = 210.0;
    let stroke_width = 0.33;
    let ringcenter = opts.ringcenter;
    let ring_resolution_multiplier = opts.ring_resolution_multiplier;
    let ring_w_lower = opts.ring_w_lower;
    let ring_w_upper = opts.ring_w_upper;
    let line_gap_max = opts.line_gap_max;
    let rings = opts.rings;
    let ring_1x = opts.ring_1x;
    let ring_1y = opts.ring_1y;
    let ring_1xf2x = opts.ring_1xf2x;
    let ring_1xf2y = opts.ring_1xf2y;
    let ring_1yf2x = opts.ring_1yf2x;
    let ring_1yf2y = opts.ring_1yf2y;
    let ring_1y3 = opts.ring_1y3;
    let ring_1yf3x = opts.ring_1yf3x;
    let ring_1yf3y = opts.ring_1yf3y;
    let size = opts.size;
    let zigzag_count = opts.zigzag_count;
    let zigzag_rep = opts.zigzag_rep;
    let zigzag_ring_resolution_multiplier = opts.zigzag_ring_resolution_multiplier;

    let ringsf = rings as f64;

    let colors = vec!["#0FF", "#F0F"];
    let layers = colors.iter().enumerate().map(|(ci, &color)| {
        let count = (size / line_gap_max) as usize;
        let mut routes = Vec::new();
        let noise = Perlin::new();
        let seed = opts.seed;
        let f = |p: (f64, f64)| {
            let x = p.0 / width;
            let y = p.1 / height;
            ring_w_lower + (ring_w_upper - ring_w_lower) * noise.get([
                x + ring_1x * noise.get([
                    ring_1xf2x * x,
                    ring_1xf2y * y,
                    -seed
                ]),
                y + ring_1y * noise.get([
                    ring_1yf2x * x,
                    ring_1yf2y * y,
                    seed +
                    ring_1y3 * noise.get([
                        ring_1yf3x * x,
                        ring_1yf3y * y,
                        66. + seed
                    ])
                ]),
                seed
            ]).abs()
        };

        
        for c in 0..rings {
            let r = 90.0 * (c as f64 + 0.5) / ringsf;
            let should_zigzag = ((c + 1) % zigzag_rep) == 0;
            let m = if should_zigzag { zigzag_ring_resolution_multiplier } else { ring_resolution_multiplier };
            let splits = ((8. + r) * m) as usize;
            let cnt = if should_zigzag { zigzag_count } else { count };
            let mut zigzag = if should_zigzag { vec![(0.,0.); cnt*splits] } else { vec![] };
            for i in 0..cnt {
                if c % colors.len() != ci {
                    continue;
                }
                let x = (i as f64) / (cnt as f64);
                let mut route = Vec::new();
                for s in 0..splits {
                    let a = 2. * PI * (s as f64 + x) / ((splits - 1) as f64);
                    let p = (
                        width / 2.0 + a.cos() * r,
                        height / 2.0 + a.sin() * r,
                    );
                    let v = f(p) * (x - ringcenter);
                    let rx = r + size * v;
                    let p = (
                        width / 2.0 + a.cos() * rx,
                        height / 2.0 + a.sin() * rx,
                    );
                    route.push(p);
                    if should_zigzag {
                        zigzag[s * cnt + i] = p;
                    }
                }
                routes.push(route);
            }
            if should_zigzag {
                routes.push(zigzag);
            }
        }

        let label = if ci == 0 { opts.primary_name.clone() } else { opts.secondary_name.clone() };

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
            let data = render_route(Data::new(), r);
            l = l.add(
                Path::new()
                .set("opacity", (1000. * (opacity - trace * opdiff)).floor()/1000.0)
                .set("d", data)
            );
        }
        l
    });
        
    let mut doc = svg::Document::new()
    .set("viewBox", (0, 0, 210, 210))
    .set("width", "210mm")
    .set("height", "210mm")
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

fn render_route(
    data: Data,
    route: Vec<(f64, f64)>
) -> Data {
    if route.len() == 0 {
        return data;
    }
    let first_p = route[0];
    let mut d = data.move_to((
        significant_str(first_p.0),
        significant_str(first_p.1)
    ));
    for p in route {
        d = d.line_to((
            significant_str(p.0),
            significant_str(p.1),
        ));
    }
    return d;
}

#[inline]
fn significant_str (f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}
