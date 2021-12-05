mod utils;
use noise::*;
use std::f64::consts::PI;
use byteorder::*;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Opts {
    pub seed: f64,
    pub max_scale: f64,
    pub desired_count: usize,
    pub samples: usize,
    pub particle_size: usize,
    pub fading: f64,
    pub gravity_dist: f64,
    pub spiral_pad: f64,
    pub a1: f64,
    pub a2: f64,
    pub a3: f64,
    pub f1: f64,
    pub f2: f64,
    pub f3: f64,
    pub yfactor: f64,
    pub primary_name: String,
    pub secondary_name: String,
}

pub fn art(opts: &Opts) -> Document {
    let seed = opts.seed;
    let max_scale = opts.max_scale;
    let desired_count = opts.desired_count;
    let samples = opts.samples;
    let particle_size = opts.particle_size;
    let fading = opts.fading;
    let gravity_dist = opts.gravity_dist;
    let spiral_pad = opts.spiral_pad;
    let a1 = opts.a1;
    let a2 = opts.a2;
    let a3 = opts.a3;
    let f1 = opts.f1;
    let f2 = opts.f2;
    let f3 = opts.f3;
    let yfactor = opts.yfactor;

    let pad = 10.0;
    let width = 297.0;
    let height = 210.0;
    let bounds = (pad, pad, width-pad, height-pad);
    let stroke_width = 0.32;
    let normbound = (0.0, 0.0, 1.0, 1.0);
    let particle_precision = 0.004;
    let perlin = Perlin::new();
    
    let bounds_container = VCircle::new(width/2.0, height/2.0, (width + height) / 2.0);
    let mut rng = rng_from_seed(opts.seed);
    let mut passage = Passage2DCounter::new(0.4, width, height);
    let max_passage = 8;

    let primaries = packing(
        seed,
        200000,
        desired_count,
        6,
        spiral_pad,
        &bounds_container,
        2.0,
        max_scale,
    );

    let inside = |from, to| {
        strictly_in_boundaries(from, bounds) &&
        strictly_in_boundaries(to, bounds)
    };

    let colors = vec!["#0FF", "#F0F"];
    let mut layers = Vec::new();

    let mut ownership = Vec::new();
    for p in primaries.iter() {
        let g = if mix(rng.gen_range(0.0, 1.0), p.y / height, yfactor) < 0.5 { 0 } else { 1 };
        ownership.push(g);
    }

    for (ci, &color) in colors.iter().enumerate() {
        let mut routes = Vec::new();

        let mut prim = Vec::new();
        for (i, &p) in primaries.iter().enumerate() {
            if ownership[i] == ci { 
                prim.push(p);
            }
        }

        let samples = sample_2d_candidates_f64(&|p| {
            let g = project_in_boundaries(p, bounds);
            let mut d = 99f64; 
            for p in prim.iter() {
                d = d.min(euclidian_dist((p.x, p.y), g) - p.r);
            }
            smoothstep(fading, -20.0, d)
        }, 400, samples, &mut rng);

        for (si, &sample) in samples.iter().enumerate() {
            let mut route = Vec::new();
            let mut p = sample;
            let mut ang = rng.gen_range(0.0, 2. * PI);
            loop {
                if route.len() >= particle_size {
                    break;
                }
                if out_of_boundaries(p, normbound) {
                    break;
                }
                let g = project_in_boundaries(p, bounds);
                if passage.count(g) > max_passage {
                    break;
                }
                route.push(g);

                let mut v = (0f64, 0f64);
                for p in primaries.iter() {
                    let dist = euclidian_dist((p.x, p.y), g) - p.r;
                    if dist > gravity_dist {
                        continue;
                    }
                    let r = smoothstep(gravity_dist, -30.0, dist);
                    let a = (p.y - g.1).atan2(p.x - g.0) + (si as f64 - 0.5) * PI;
                    v.0 += r * a.cos();
                    v.1 += r * a.sin();
                }

                if v.0 != 0.0 || v.1 != 0.0 {
                    let mut a = (v.1.atan2(v.0) + 2.0 * PI) % (2. * PI);
                    if (a - ang).abs() > PI / 2.0 {
                        a += PI;
                    }
                    ang = a;
                }
                
                ang += a1 * perlin.get([
                    f1 * g.0,
                    f1 * g.1,
                    seed + a2 * rng.gen_range(0.0, 1.0) * perlin.get([
                        seed + a3 * rng.gen_range(0.0, 1.0) * perlin.get([
                            f3 * g.0,
                            f3 * g.1,
                            seed
                        ]),
                        f2 * g.0,
                        f2 * g.1,
                    ])
                ]);
                
                p = (
                    p.0 + particle_precision * ang.cos(),
                    p.1 + particle_precision * ang.sin(),
                );
            }
            routes.push(route);
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
            let data = render_route_when(Data::new(), r, inside);
            l = l.add(
                Path::new()
                .set("opacity", (1000. * (opacity - trace * opdiff)).floor() / 1000.)
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

fn scaling_search<F: FnMut(f64) -> bool>(
    mut f: F,
    min_scale: f64,
    max_scale: f64,
) -> Option<f64> {
    let mut from = min_scale;
    let mut to = max_scale;
    loop {
        if !f(from) {
            return None;
        }
        if to - from < 0.1 {
            return Some(from);
        }
        let middle = (to + from) / 2.0;
        if !f(middle) {
            to = middle;
        }
        else {
            from = middle;
        }
    }
}

fn search_circle_radius(
    container: &VCircle,
    circles: &Vec<VCircle>,
    x: f64,
    y: f64,
    min_scale: f64,
    max_scale: f64,
) -> Option<f64> {
    let overlaps = |size| {
        let c = VCircle::new(x, y, size);
        container.contains(&c) && !circles.iter().any(|other| {
            c.collides(other)
        })
    };
    scaling_search(overlaps, min_scale, max_scale)
}

fn packing(
    seed: f64,
    iterations: usize,
    desired_count: usize,
    optimize_size: usize,
    pad: f64,
    container: &VCircle,
    min_scale: f64,
    max_scale: f64,
) -> Vec<VCircle> {
    let mut circles = Vec::new();
    let mut tries = Vec::new();
    let mut rng = rng_from_seed(seed);
    let x1 = container.x - container.r;
    let y1 = container.y - container.r;
    let x2 = container.x + container.r;
    let y2 = container.y + container.r;
    let max_scale = max_scale.min(container.r);
    for _i in 0..iterations {
        let x: f64 = rng.gen_range(x1, x2);
        let y: f64 = rng.gen_range(y1, y2);
        if let Some(size) = search_circle_radius(&container, &circles, x, y, min_scale, max_scale) {
            let circle = VCircle::new(x, y, size - pad);
            tries.push(circle.clone());
            if tries.len() > optimize_size {
                tries.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
                let c = tries[0];
                circles.push(c);
                tries = Vec::new();
            }
        }
        if circles.len() > desired_count {
            break;
        }
    }

    circles
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

fn sample_2d_candidates_f64<R: Rng>(
    f: &dyn Fn((f64, f64)) -> f64,
    dim: usize,
    samples: usize,
    rng: &mut R,
) -> Vec<(f64, f64)> {
    let mut candidates = Vec::new();
    for x in 0..dim {
        for y in 0..dim {
            let p = (
                (x as f64) / (dim as f64),
                (y as f64) / (dim as f64),
            );
            if f(p) > rng.gen_range(0.0, 1.0) {
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
                d = d.move_to((significant_str(p.0), significant_str(p.1)));
            }
        } else {
            if should_draw_line(last, p) {
                if up {
                    up = false;
                    d = d.move_to((significant_str(last.0), significant_str(last.1)));
                }
                d = d.line_to((significant_str(p.0), significant_str(p.1)));
            } else {
                up = true;
            }
        }
        last = p;
    }
    return d;
}

struct Passage2DCounter {
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
}

#[inline]
fn rng_from_seed(s: f64) -> impl Rng {
    let mut bs = [0; 16];
    bs.as_mut().write_f64::<BigEndian>(s).unwrap();
    let mut rng = SmallRng::from_seed(bs);
    for _i in 0..10 {
        rng.gen::<f64>();
    }
    return rng;
}


#[derive(Clone, Copy)]
struct VCircle {
    x: f64,
    y: f64,
    r: f64,
}

impl VCircle {
    fn new(x: f64, y: f64, r: f64) -> Self {
        VCircle { x, y, r }
    }
    fn dist(self: &Self, c: &VCircle) -> f64 {
        euclidian_dist((self.x,self.y), (c.x, c.y)) - c.r - self.r
    }
    fn collides(self: &Self, c: &VCircle) -> bool {
        self.dist(c) <= 0.0
    }
    fn contains(self: &Self, c: &VCircle) -> bool {
        euclidian_dist((self.x,self.y), (c.x, c.y)) - self.r + c.r < 0.0
    }
}


#[inline]
fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let k = ((x - a) / (b - a)).max(0.0).min(1.0);
    return k * k * (3.0 - 2.0 * k);
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

#[inline]
fn project_in_boundaries(
    p: (f64, f64),
    boundaries: (f64, f64, f64, f64),
) -> (f64, f64) {
    (
        p.0 * (boundaries.2 - boundaries.0) + boundaries.0,
        p.1 * (boundaries.3 - boundaries.1) + boundaries.1,
    )
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

#[inline]
fn mix(a: f64, b: f64, x: f64) -> f64 {
    (1. - x) * a + x * b
}