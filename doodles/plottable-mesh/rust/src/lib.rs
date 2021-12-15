mod utils;
use noise::*;
use std::f64::consts::PI;
use byteorder::*;
use contour::ContourBuilder;
use geojson::Feature;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Opts {
    pub seed: f64,
    pub precision: f64,
    pub samples: usize,
    pub iterations: usize,
    pub f1: f64,
    pub f2x: f64,
    pub f2y: f64,
    pub f3: f64,
    pub a1: f64,
    pub a2: f64,
    pub a3: f64,
    pub k: f64,
    pub shapeamp: f64,
    pub offset: f64,
    pub overflowin: f64,
    pub overflowout: f64,
    pub vertical: bool,
    pub symmetry: bool,
    pub primary_name: String,
    pub secondary_name: String,
}

pub fn art(opts: &Opts) -> Document {
    let samples = opts.samples;
    let f1 = opts.f1;
    let f2x = opts.f2x;
    let f2y = opts.f2y;
    let f3 = opts.f3;
    let a1 = opts.a1;
    let a2 = opts.a2;
    let a3 = opts.a3;
    let iterations = opts.iterations;
    let symmetry = opts.symmetry;
    let vertical = opts.vertical;
    let offset = opts.offset;
    let shapeamp = opts.shapeamp;
    let k = opts.k;

    let width = 297.0;
    let height = 210.0;
    let stroke_width = 0.32;
    let precision = opts.precision;
    let w = (width as f64 / precision) as u32;
    let h = (height as f64 / precision) as u32;
    let pad = 10.0;
    let perlin = Perlin::new();
    
    fn length(l: (f64, f64)) -> f64 {
        (l.0 * l.0 + l.1 * l.1).sqrt()
    }
    fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
        (
            a.cos() * p.0 + a.sin() * p.1,
            a.cos() * p.1 - a.sin() * p.0,
        )
    }
    fn op_union_round(a: f64, b: f64, r: f64) -> f64 {
        r.max(a.min(b)) - length(((r - a).max(0.), (r - b).max(0.)))
    }
    let  sdf_box2 = |(x,y):(f64,f64), (w, h):(f64,f64)| {
        let dx = x.abs() - w;
        let dy = y.abs() - h;
        length((dx.max(0.), dy.max(0.))) + dx.min(0.).max(dy.min(0.))
    };
    
    let colors = vec!["#0FF", "#F0F"];
    let mut layers = Vec::new();
    let mut dist_min_map = vec![99f64; (w as usize) * (h as usize)];
    let mut dist_max_map = vec![-99f64; (w as usize) * (h as usize)];

    let index_dist_map = |x: f64, y: f64| {
        (h as usize) * ((((w - 1) as f64) * x) as usize) + (y * ((h - 1) as f64)) as usize
    };

    let pattern = (2., 3.);
    let thresholds: Vec<f64> = 
        (0..samples)
        .map(|i| 
            (i as f64 + pattern.1 * (i as f64 / pattern.0).floor()) / (samples as f64 * (pattern.0+pattern.1) / pattern.0).floor())
        .collect();

    let data: Vec<_> = colors.iter().enumerate().map(|(ci, &color)| {
        let f = |(x, y): (f64, f64)| {
            let mut rng = rng_from_seed(7000. + 3.738 * opts.seed);
            let mut c = ((x-0.5) * width / height, y-0.5);
            if vertical {
                c.1 = c.1 + offset * (ci as f64 - 0.5);
                if ci == 1 {
                    c.1 = -c.1;
                }
                if symmetry {
                    c.0 = c.0.abs();
                }
            }
            else {
                c.0 = c.0 + offset * (ci as f64 - 0.5);
                if ci == 1 {
                    c.0 = -c.0;
                }
                if symmetry {
                    c.1 = c.1.abs();
                }
            }
            let mut s = 100f64;
            for _i in 0..iterations {
              let mut p = (c.0, c.1);
              let ang = rng.gen_range(-PI, PI);
              p.0 += shapeamp * rng.gen_range(-0.8, 0.8);
              p.1 += shapeamp * rng.gen_range(-0.5, 0.5);
              p = p_r(p, ang);
              let dim = (
                  rng.gen_range(0.0, 0.1),
                  rng.gen_range(0.0, 0.1)
              );
              s = op_union_round(s, sdf_box2(p, dim), k);
            }
            let n = a1 * perlin.get([
                f1 * c.0,
                f1 * c.1,
                opts.seed
                + a2 * perlin.get([
                    7. + opts.seed,
                    f2x * c.0 + a3 * perlin.get([f3 * c.0, f3 * c.1, 1. + opts.seed]),
                    f2y * c.1 + a3 * perlin.get([f3 * c.0, f3 * c.1, 2. + opts.seed])
                  ])
                ]);
            let d = lerp(-0.3, 0.4, s) + n;
            let i = index_dist_map(x, y);
            dist_min_map[i] = dist_min_map[i].min(d);
            dist_max_map[i] = dist_max_map[i].max(d);
            d
        };
     
        let res = contour(w, h, f, &thresholds);
        let routes = features_to_routes(res, precision);
        
        let label = if ci == 0 { opts.primary_name.clone() } else { opts.secondary_name.clone() };
        (routes, color, label)
    }).collect();

    let oin = -opts.overflowin;
    let oout = 1.0 + opts.overflowout;

    let inside = |from: (f64, f64), to: (f64, f64)| {
        let i = index_dist_map(from.0 / width, from.1 / height);
        let j = index_dist_map(to.0 / width, to.1 / height);
        dist_min_map[i] >= oin &&
        dist_max_map[i] <= oout &&
        dist_min_map[j] >= oin &&
        dist_max_map[j] <= oout &&
        strictly_in_boundaries(from, (pad, pad, width-pad, height-pad)) &&
        strictly_in_boundaries(to, (pad, pad, width-pad, height-pad))
    };

    for (routes, color, label) in data {
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
            let data = render_route_when(Data::new(), r, inside);
            l = l.add(
                Path::new()
                .set("opacity", (1000. * (opacity - trace * opdiff)).floor()/1000.0)
                .set("d", data)
            );
        }
        layers.push(l);
    }
        
    let mut doc = svg::Document::new()
    .set("viewBox", (0, 0, 297, 210))
    .set("width", "297mm")
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

#[inline]
fn strictly_in_boundaries(
    p: (f64, f64),
    boundaries: (f64, f64, f64, f64),
) -> bool {
    p.0 > boundaries.0
        && p.0 < boundaries.2
        && p.1 > boundaries.1
        && p.1 < boundaries.3
}

#[inline]
fn significant_str (f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}

pub fn sample_2d_candidates(
    f: &dyn Fn((f64, f64)) -> bool,
    dim: usize,
    samples: usize,
    rng: &mut impl Rng,
) -> Vec<(f64, f64)> {
    let mut candidates = Vec::new();
    for x in 0..dim {
        for y in 0..dim {
            let p = (
                (x as f64) / (dim as f64),
                (y as f64) / (dim as f64),
            );
            if f(p) {
                candidates.push(p);
            }
        }
    }
    rng.shuffle(&mut candidates);
    candidates.truncate(samples);
    return candidates;
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
fn lerp(a: f64, b: f64, x: f64) -> f64 {
    (x - a) / (b - a)
}

pub fn rng_from_seed(s: f64) -> impl Rng {
    let mut bs = [0; 16];
    bs.as_mut().write_f64::<BigEndian>(s).unwrap();
    let mut rng = SmallRng::from_seed(bs);
    for _i in 0..10 {
        rng.gen::<f64>();
    }
    return rng;
}

fn contour<F: FnMut((f64, f64)) -> f64>(
    width: u32,
    height: u32,
    mut f: F,
    thresholds: &Vec<f64>,
) -> Vec<Feature> {
    let c = ContourBuilder::new(width, height, true);
    let values = rasterize_1d(width, height, &mut f);
    c.contours(&values, &thresholds).unwrap_or(Vec::new())
}

fn features_to_routes(
    features: Vec<Feature>,
    precision: f64,
) -> Vec<Vec<(f64, f64)>> {
    let mut routes = Vec::new();
    for f in features {
        for g in f.geometry {
            let value = g.value;
            match value {
                geojson::Value::MultiPolygon(all) => {
                    for poly in all {
                        for lines in poly {
                            let mut points = lines
                                .iter()
                                .map(|p| {
                                    (
                                        significant_str(precision * p[0]),
                                        significant_str(precision * p[1]),
                                    )
                                })
                                .collect::<Vec<(f64, f64)>>(
                                );
                            let len = points.len();
                            if len < 3 {
                                continue;
                            }
                            if euclidian_dist(
                                points[0],
                                points[len - 1],
                            ) <= precision
                            {
                                points.push(points[0]);
                            }
                            routes.push(points);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    routes
}

#[inline]
fn euclidian_dist(
    (x1, y1): (f64, f64),
    (x2, y2): (f64, f64),
) -> f64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    return (dx * dx + dy * dy).sqrt();
}

fn rasterize_1d<F: FnMut((f64, f64)) -> f64>(
    width: u32,
    height: u32,
    mut f: F,
) -> Vec<f64> {
    (0..height)
        .flat_map(|y| {
            (0..width)
                .map(|x| {
                    f((
                        x as f64 / width as f64,
                        y as f64 / height as f64,
                    ))
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<f64>>()
}