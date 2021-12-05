mod utils;
use noise::*;
use std::f64::consts::PI;
use std::cmp::Ordering;
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
    pub a1: f64,
    pub a2: f64,
    pub a3: f64,
    pub f1: f64,
    pub f2: f64,
    pub f3: f64,
    pub base_pad: f64,
    pub base_min_scale: f64,
    pub wave_split_color: f64,
    pub base_offset: f64,
    pub xfactor: f64,
    pub primary_name: String,
    pub secondary_name: String,
    pub weights: Vec<f64>,
    pub diversity: f64,
    pub ribbons: f64,
    pub ribbons_freq: f64,
    pub ribbons_two_colors: bool
}

pub fn art(opts: &Opts) -> Document {

    let width = 210.0;
    let height = 210.0;
    let stroke_width = 0.32;
    
    let bounds_container = VCircle::new(width/2.0, height/2.0, height / 2.0 - 10.0);

    let (circles, routes) = rec(0, opts.seed, &opts, &bounds_container);

    let colors = vec!["#0FF", "#F0F"];
    let mut layers = Vec::new();
    

    for (ci, &color) in colors.iter().enumerate() {
        let label = if ci == 0 { opts.primary_name.clone() } else { opts.secondary_name.clone() };
        let mut l = Group::new()
            .set("inkscape:groupmode", "layer")
            .set("inkscape:label", label)
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", stroke_width);

        let opacity: f64 = 0.6;
            let opdiff = 0.15 / ((routes.len() as f64) + (circles.len() as f64));
        let mut trace = 0f64;
        for (group, c) in circles.clone() {
            if ci == 1 && group != 0 || group == 0 {
                continue;
            }
            trace += 1f64;
            l = l.add(
                Circle::new()
                .set("r", c.r)
                .set("cx", c.x)
                .set("cy", c.y)
                .set("opacity", (1000. * (opacity - trace * opdiff)).floor() / 1000.)
            );
        }
        for (group, r) in routes.clone() {
            if ci == 1 && group != 0 || ci == 0 && group == 0 {
                continue;
            }
            trace += 1f64;
            if r.len() < 2 {
                continue;
            }
            let data = render_route(Data::new(), r);
            l = l.add(
                Path::new()
                .set("opacity", (1000. * (opacity - trace * opdiff)).floor() / 1000.)
                .set("d", data)
            );
        }
        
        layers.push(l);
    }
        
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
    height_map: &Vec<f64>,
    x: f64,
    y: f64,
    min_scale: f64,
    max_scale: f64,
) -> Option<f64> {
    let f = |size| {
        let c = VCircle::new(x, y, size);
        let l = height_map.len();
        let mut collides_height_map = false;
        if l > 0 {
            let factor = 2. * container.r / (l as f64);
            collides_height_map = height_map.iter().enumerate().any(|(i, &y)| {
                let x = container.x - container.r + i as f64 * factor;
                (c.x - x).abs() < c.r && y < c.y || c.includes((x, y))
            });
        }
        !collides_height_map && container.contains(&c) && !circles.iter().any(|other| {
            c.collides(other)
        })
    };
    scaling_search(f, min_scale, max_scale)
}

fn packing(
    seed: f64,
    iterations: usize,
    desired_count: usize,
    optimize_size: usize,
    pad: f64,
    container: &VCircle,
    height_map: &Vec<f64>,
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
        if let Some(size) = search_circle_radius(&container, &circles, &height_map, x, y, min_scale, max_scale) {
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


fn waves_in_circle(
    n: usize,
    seed: f64,
    opts: &Opts,
    circle: &VCircle,
    base_offset: f64,
    dy: f64
) -> (Vec<(usize, Vec<(f64, f64)>)>, Vec<f64>) {
    let xfactor = opts.xfactor;
    let ribbons = opts.ribbons;
    let ribbons_freq = opts.ribbons_freq;
    let ribbons_two_colors = opts.ribbons_two_colors;
    let f1 = opts.f1;
    let f2 = opts.f2;
    let f3 = opts.f3;
    let a1 = opts.a1;
    let a2 = opts.a2;
    let a3 = opts.a3;
    let mut routes = Vec::new();
    let mut base_y = circle.y + 2. * circle.r;
    let perlin = Perlin::new();
    let mut passage = Passage2DCounter::new(0.3, circle.r * 2.0, circle.r * 2.0);
    let passage_limit = 20;
    let mut height_map: Vec<f64> = Vec::new();
    loop {
        if base_y < circle.y + base_offset * circle.r {
            break;
        }
        let ribbons_y_base = (ribbons_freq * base_y).floor();
        let ribbons_y = ribbons * ribbons_y_base;
        let group = 
        if ribbons_two_colors {
            if (ribbons_y_base as usize) % 2 == 0 {
                n
            }
            else {
                n + 1
            }
        }
        else if base_y < circle.y + opts.wave_split_color * circle.r {
            n + 1
        }
        else {
            n
        };
        let precision = 0.2;
        let v = 0.0;
        if v < 0.6 {
            let mut route = Vec::new();
            let mut x = circle.x - circle.r;
            let mut was_outside = true;
            let mut i = 0;
            loop {
                if x > circle.x + circle.r {
                    break;
                }
                let y = base_y + (circle.r - 0.6 * euclidian_dist((circle.x, circle.y + 0.9 * circle.r), (x, base_y))) * (
                    a1 * perlin.get([
                        2.0 * xfactor * f1 * x +
                        ribbons_y,
                        2.0 * (1. - xfactor) * f1 * base_y,
                        seed +
                        a2 * perlin.get([
                            2.0 * (1. - xfactor) * f2 * base_y + a3 * perlin.get([
                                2.0 * (1. - xfactor) * f3 * base_y,
                                2.0 * xfactor * f3 * x,
                                -7.3 * seed
                            ]),
                            seed,
                            2.0 * xfactor * f2 * x,
                        ])
                    ])
                );
                let mut collides = false;
                if i >= height_map.len() {
                    height_map.push(y);
                }
                else {
                    if y > height_map[i] {
                        collides = true;
                    }
                    else {
                        height_map[i] = y;
                    }
                }
                let inside = !collides &&
                circle.includes((x, y)) &&
                passage.count(( x - circle.x + circle.r, y - circle.y + circle.r )) < passage_limit;
                if inside {
                    if was_outside {
                        if route.len() > 2 {
                            routes.push((group, route));
                        }
                        route = Vec::new();
                    }
                    was_outside = false;
                    route.push((x, y));
                }
                else {
                    was_outside = true;
                }
                x += precision;
                i += 1;
            }
            routes.push((group, route));
        }

        base_y -= dy;
    }
    (routes, height_map)
}

type WaveballRes = (Vec<(usize, VCircle)>, Vec<(usize, Vec<(f64, f64)>)>);

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


fn shape_strokes_random(n: usize, seed: f64, _opts: &Opts, c: &VCircle) -> WaveballRes {
    let mut rng = rng_from_seed(seed);
    let samples = sample_2d_candidates_f64(&|p| {
        let dx = p.0 - 0.5;
        let dy = p.1 - 0.5;
        let d2 = dx * dx + dy * dy;
        if d2 > 0.25 {
            0.0
        }
        else {
            d2
        }
    }, (4. * c.r) as usize, (40. + 0.5 * c.r * c.r) as usize, &mut rng);
    let routes = vec![(n, samples.iter().map(|(x, y)| {
        (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y)
    }).collect())];
    return (Vec::new(), routes);
}

fn shape_cross_hatch(n: usize, seed: f64, _opts: &Opts, c: &VCircle) -> WaveballRes {
    let mut rng = rng_from_seed(seed);
    let mut points: Vec<(f64, f64)> = (0..((20. + 2.5 * c.r.powf(1.5)) as usize)).map(|_i| {
        let a = rng.gen_range(0., 2. * PI);
        (c.x + c.r * a.cos(), c.y + c.r * a.sin())
    }).collect();
    let mut routes = Vec::new();
    points.sort_by(sort_points_x);
    routes.push((n, points.clone()));
    points.sort_by(sort_points_y);
    routes.push((n, points.clone()));
    return (Vec::new(), routes);
}

fn shape_cross_x(n: usize, seed: f64, _opts: &Opts, c: &VCircle) -> WaveballRes {
    let mut rng = rng_from_seed(seed);
    let mut points: Vec<(f64, f64)> = (0..((20. + 3. * c.r.powf(1.5)) as usize)).map(|_i| {
        let a = rng.gen_range(0., 2. * PI);
        (c.x + c.r * a.cos(), c.y + c.r * a.sin())
    }).collect();
    let mut routes = Vec::new();
    points.sort_by(sort_points_x);
    routes.push((n, points.clone()));
    return (Vec::new(), routes);
}

fn shape_cross_y(n: usize, seed: f64, _opts: &Opts, c: &VCircle) -> WaveballRes {
    let mut rng = rng_from_seed(seed);
    let mut points: Vec<(f64, f64)> = (0..((20. + 3. * c.r.powf(1.5)) as usize)).map(|_i| {
        let a = rng.gen_range(0., 2. * PI);
        (c.x + c.r * a.cos(), c.y + c.r * a.sin())
    }).collect();
    let mut routes = Vec::new();
    points.sort_by(sort_points_y);
    routes.push((n, points.clone()));
    return (Vec::new(), routes);
}

fn sort_points_diag (a: &(f64,f64), b: &(f64,f64)) -> Ordering {
    (a.0 - a.1)
        .partial_cmp(&(b.0 - b.1))
        .unwrap()
}
fn sort_points_x (a: &(f64,f64), b: &(f64,f64)) -> Ordering {
    (a.0)
        .partial_cmp(&(b.0))
        .unwrap()
}
fn sort_points_y (a: &(f64,f64), b: &(f64,f64)) -> Ordering {
    (a.1)
        .partial_cmp(&(b.1))
        .unwrap()
}

fn shape_strokes_spiral(n: usize, seed: f64, _opts: &Opts, c: &VCircle) -> WaveballRes {
    let mut rng = rng_from_seed(seed);
    let mut samples = sample_2d_candidates_f64(&|p| {
        let dx = p.0 - 0.5;
        let dy = p.1 - 0.5;
        let d2 = dx * dx + dy * dy;
        if d2 < 0.25 {
            1.0
        }
        else {
            0.0
        }
    }, (8. * c.r) as usize, (40. + 3. * c.r.powf(1.8)) as usize, &mut rng);
    if samples.len() < 2 {
        return (Vec::new(), Vec::new());
    }
    let mut result = Vec::new();
    let mut p = *(samples
        .iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap());
    let mut a = 0.0;
    result.push(p);
    loop {
        samples =
            samples.into_iter().filter(|&x| x != p).collect();
        let maybe_match = samples.iter().min_by_key(|q| {
            let qp_angle = (p.1 - q.1).atan2(p.0 - q.0);
            // HACK!!! no Ord for f64 :(
            return (1000000.0
                * ((2. * PI + qp_angle - a) % (2.0 * PI)))
                as i32;
        });
        if let Some(new_p) = maybe_match {
            a = (p.1 - new_p.1).atan2(p.0 - new_p.0);
            p = *new_p;
            result.push(p);
        } else {
            break;
        }
    }let routes = vec![(n, result.iter().map(|(x, y)| {
        (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y)
    }).collect())];
    return (Vec::new(), routes);
}

fn shape_strokes_sorted(n: usize, seed: f64, _opts: &Opts, c: &VCircle) -> WaveballRes {
    let mut rng = rng_from_seed(seed);
    let mut samples = sample_2d_candidates_f64(&|p| {
        let dx = p.0 - 0.5;
        let dy = p.1 - 0.5;
        let d2 = dx * dx + dy * dy;
        if d2 > 0.25 {
            0.0
        }
        else {
            d2
        }
    }, (4. * c.r) as usize, (20.0 + 0.8 * c.r.powf(1.8)) as usize, &mut rng);
    samples.sort_by(sort_points_diag);
    let routes = vec![(n, samples.iter().map(|(x, y)| {
        (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y)
    }).collect())];
    return (Vec::new(), routes);
}

fn shape_circle_packing(n: usize, seed: f64, _opts: &Opts, c: &VCircle) -> WaveballRes {
    let mut circles: Vec<(usize, VCircle)> = packing(
        seed,
        200000,
        (c.r * c.r) as usize,
        1,
        0.5,
        c,
        &Vec::new(),
        0.8,
        c.r / 3.0
    )
    .iter().map(|c| (n, c.clone())).collect();
    circles.push((n, c.clone()));
    return (circles, Vec::new());
}

fn shape_spiral(n: usize, _seed: f64, _opts: &Opts, c: &VCircle) -> WaveballRes {
    let mut r = c.r;
    let mut a = 0f64;
    let mut points = Vec::new();
    let increment = 0.015;
    loop {
        if r < 0.2 {
            break;
        }
        points.push((c.x + r * a.cos(), c.y + r * a.sin()));
        a += 4. / (20. + r.powf(0.5));
        r -= increment;
    }
    (vec![], vec![(n, points)])
}

fn random_choice<R: Rng>(weights: &Vec<f64>, rng: &mut R) -> usize {
    let weight_sum: f64 = weights.iter().sum();
    let mut acc = 0.0;
    let mut choice = 0;
    let r = rng.gen_range(0.0, 1f64);
    for w in weights.iter() {
        acc += w / weight_sum;
        if acc > r {
            break;
        }
        choice += 1;
    }
    return choice;
}

fn rec(n: usize, seed: f64, opts: &Opts, c: &VCircle) -> WaveballRes {
    if n > 4 || c.r < 1.0 {
        return (Vec::new(), Vec::new());
    }
    let mut rng = rng_from_seed(seed * 1.7 + 8.8);
    let base_offset = if n == 0 {
        opts.base_offset
    }
    else {
        rng.gen_range(-0.2, 0.4)
    };
    let (waves, height_map) = waves_in_circle(n, seed, &opts, c, base_offset, 0.32 + n as f64 * 0.2);
    
    let main_choice = random_choice(&opts.weights, &mut rng);

    let res = packing(seed, 100000, opts.desired_count, 5, opts.base_pad / (1. + n as f64), c, &height_map, opts.base_min_scale / (1. + 4. * n as f64), opts.max_scale)
        .iter()
        .filter(|circle| circle.r > 2.0)
        .map(|circle| {
            let s = seed + circle.x * 9. + circle.y / 29.;
            let second_choice = random_choice(&opts.weights, &mut rng);
            let choice = if rng.gen_range(0.0, 1.0) < opts.diversity {
                second_choice
            }
            else {
                main_choice
            };
            let group = if rng.gen_range(0.0, 1.0) < 0.02 { 0 } else { n + 1 };
            match choice {
                1 => shape_strokes_spiral(group, s, &opts, circle),
                2 => shape_strokes_random(group, s, &opts, circle),
                3 => shape_strokes_sorted(group, s, &opts, circle),
                4 => shape_circle_packing(group, s, &opts, circle),
                5 => shape_spiral(group, s, &opts, circle),
                6 => shape_cross_hatch(group, s, &opts, circle),
                7 => shape_cross_x(group, s, &opts, circle),
                8 => shape_cross_y(group, s, &opts, circle),
                _ => rec(n+1, s, &opts, circle)
            }
        })
        .collect::<Vec<_>>();

    let mut circles_acc = Vec::new();
    let mut routes_acc = Vec::new();
    circles_acc.push(vec![ (n , c.clone()) ]);
    routes_acc.push(waves);
    for (circles, routes) in res {
        circles_acc.push(circles);
        routes_acc.push(routes);
    }
    let circles = circles_acc.concat();
    let routes = routes_acc.concat();
    (circles, routes)
}

#[inline]
fn significant_str (f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
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
    for _i in 0..4 {
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
    fn includes(self: &Self, p: (f64, f64)) -> bool {
        euclidian_dist((self.x,self.y), p) < self.r
    }
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