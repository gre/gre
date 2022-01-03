use std::cmp::Ordering;

use clap::Clap;
use gre::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::{Group, path::Data};

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.0")]
    seed: f64,
}

#[derive(Clone, Copy, Debug)]
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
        euclidian_dist((self.x,self.y), (p.0, p.1)) - self.r < 0.0
    }
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
    bound: (f64, f64, f64, f64),
    circles: &Vec<VCircle>,
    x: f64,
    y: f64,
    min_scale: f64,
    max_scale: f64,
) -> Option<f64> {
    let overlaps = |size| {
        let c = VCircle::new(x, y, size);
        bound.0 < c.x - c.r && c.x + c.r < bound.2 &&
        bound.1 < c.y - c.r && c.y + c.r < bound.3 &&
        !circles.iter().any(|other| { c.collides(other) })
    };
    scaling_search(overlaps, min_scale, max_scale)
}

fn packing(
    seed: f64,
    iterations: usize,
    desired_count: usize,
    optimize_size: usize,
    pad: f64,
    bound: (f64, f64, f64, f64),
    min_scale: f64,
    max_scale: f64,
) -> Vec<VCircle> {
    let mut circles = Vec::new();
    let mut tries = Vec::new();
    let mut rng = rng_from_seed(seed);
        for _i in 0..iterations {
        let x: f64 = rng.gen_range(bound.0, bound.2);
        let y: f64 = rng.gen_range(bound.1, bound.3);
        if let Some(size) = search_circle_radius(bound, &circles, x, y, min_scale, max_scale) {
            let circle = VCircle::new(x, y, size - pad);
            tries.push(circle);
            if tries.len() > optimize_size {
                tries.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
                let c = tries[0];
                circles.push(c.clone());
                tries = Vec::new();
            }
        }
        if circles.len() > desired_count {
            break;
        }
    }
    circles
}

fn art(opts: Opts) -> Vec<Group> {
    let height = 210.0;
    let width = 297.0;
    let pad = 10.0;
    let stroke_width = 0.35;

    let circles = packing(
        opts.seed,
        1000000,
        1000,
        10,
        0.0,
        (pad, pad, width-pad, height-pad),
        2.0,
        60.0,
    );

    println!("{}", circles.len());

    let items =
        circles
        .par_iter()
        .map(|circle| {
            let s = opts.seed + circle.x * 9. + circle.y / 29.;
            let mut rng = rng_from_seed(s);
            let choice = (rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0) * 4f32).floor() as u8;
            match choice {
                0 => shape_strokes_random(&mut rng, circle, &opts),
                1 => shape_lines(&mut rng, circle, &opts),
                2 => shape_panel(&mut rng, circle, &opts),
                _ => shape_dots_random(&mut rng, circle, &opts),
            }
        })
        .collect::<Vec<_>>();

    let mut layers = Vec::new();

    let color = "black";
    let mut l = layer(color);
    for (rendering, routes) in items {
        let mut data = Data::new();
        for route in routes {
            match rendering {
              0 => { data = render_route_curve(data, route); }
              _ => { data = render_route(data, route); }
            }
        }
        l = l.add(base_path(color, stroke_width, data));
    }
    layers.push(l);

    layers
    
}

fn shape_strokes_random<R: Rng>(rng: &mut R, c: &VCircle, _opts: &Opts) -> (usize, Vec<Vec<(f64, f64)>>) {
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
    }, (4. * c.r) as usize, (40. + 0.5 * c.r.powf(1.5)) as usize, rng);
    (0, vec![samples.iter().map(|(x, y)| {
        (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y)
    }).collect()])
}

fn shape_dots_random<R: Rng>(rng: &mut R, c: &VCircle, _opts: &Opts) -> (usize, Vec<Vec<(f64, f64)>>) {
    let samples = sample_2d_candidates_f64(&|p| {
        let dx = p.0 - 0.5;
        let dy = p.1 - 0.5;
        let d2 = dx * dx + dy * dy;
        if d2 > 0.25 {
            0.0
        }
        else {
            1.0
        }
    }, (4. * c.r) as usize, (2. + 0.4 * c.r * c.r) as usize, rng);
    (1, samples.iter().map(|(x, y)| {
        let p = (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y);
        let amp = 1.0;
        let a = rng.gen_range(0f64, 10.0);
        vec![p, (p.0+amp*a.cos(), p.1+amp*a.sin())]
    }).collect())
}

fn shape_lines<R: Rng>(rng: &mut R, c: &VCircle, _opts: &Opts) -> (usize, Vec<Vec<(f64, f64)>>) {
    let mut samples = sample_2d_candidates_f64(&|p| {
        let dx = p.0 - 0.5;
        let dy = p.1 - 0.5;
        let d2 = dx * dx + dy * dy;
        if d2 > 0.25 {
            0.0
        }
        else {
            1.0
        }
    }, (4. * c.r) as usize, (8. + 0.1 * c.r * c.r) as usize * 2, rng);
    let v1 = rng.gen_range(-1.0, 1.0);
    let v2 = rng.gen_range(-1.0, 1.0);
    samples.sort_by(|a, b| (v1 * a.0 + v2 * a.1).partial_cmp(&(v1 * b.0 + v2 * b.1)).unwrap());
    let a = rng.gen_range(0f64, 10.);
    let points: Vec<(f64, f64)> = samples.iter().map(|(x, y)| {
        (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y)
    }).collect();
    let routes = points.chunks(2).map(|v| v.to_vec()).collect();
    (1, routes)
}

fn shape_panel<R: Rng>(rng: &mut R, c: &VCircle, _opts: &Opts) -> (usize, Vec<Vec<(f64, f64)>>) {
    let w = rng.gen_range(4, 16);
    let h = rng.gen_range(1, 4);
    let ang = rng.gen_range(0.0, 10f64);
    let max = w.max(h);
    let unit = 1.8 * c.r / (max as f64);
    let mut routes = shape_dots_random(rng, c, _opts).1;
    let delta = (-(w as f64)*unit/2.0, -(h as f64)*unit/2.0);
    for x in 0..(w+1) {
        let px = x as f64 * unit;
        let mut a = (px, 0.0);
        let mut b = (px, h as f64 * unit);
        a.0 += delta.0; a.1 += delta.1;
        b.0 += delta.0; b.1 += delta.1;
        a = p_r(a, ang);
        b = p_r(b, ang);
        a.0 += c.x; a.1 += c.y;
        b.0 += c.x; b.1 += c.y;
        routes.push(vec![ a, b ]);
    }
    for y in 0..(h+1) {
        let py = y as f64 * unit;
        let mut a = (0.0, py);
        let mut b = (w as f64 * unit, py);
        a.0 += delta.0; a.1 += delta.1;
        b.0 += delta.0; b.1 += delta.1;
        a = p_r(a, ang);
        b = p_r(b, ang);
        a.0 += c.x; a.1 += c.y;
        b.0 += c.x; b.1 += c.y;
        routes.push(vec![ a, b ]);
    }
    (1, routes)
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
