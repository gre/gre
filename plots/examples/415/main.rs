use std::f64::consts::PI;

use clap::Clap;
use gre::*;
use noise::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::{Group, path::Data};

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "580.0")]
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
    fn inside_bounds(self: &Self, (x1, y1, x2, y2): (f64, f64, f64, f64)) -> bool {
        x1 <= self.x - self.r && self.x + self.r <= x2 &&
        y1 <= self.y - self.r && self.y + self.r <= y2
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
    container_boundaries: (f64, f64, f64, f64),
    container_circle: &VCircle,
    circles: &Vec<VCircle>,
    x: f64,
    y: f64,
    min_scale: f64,
    max_scale: f64,
) -> Option<f64> {
    let overlaps = |size| {
        let c = VCircle::new(x, y, size);
        c.inside_bounds(container_boundaries) &&
        container_circle.contains(&c) &&
        !circles.iter().any(|other| {
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
    container_boundaries: (f64, f64, f64, f64),
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
        if let Some(size) = search_circle_radius(
            container_boundaries,
            &container,
            &circles,
            x, y, min_scale, max_scale) {
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

fn flower(seed: f64, center: (f64, f64), radius: f64) -> Vec<Vec<(f64, f64)>> {
    let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();
    let perlin = Perlin::new();
    let mut rng = rng_from_seed(seed);

    let count = (10. * radius) as usize;
    let golden_ratio = (1. + (5f64).sqrt()) / 2.;
    let d = radius / 40.0;
    for i in 0..count {
        let k = i as f64 / (count as f64);
        let a = 2. * PI * (i as f64) / (golden_ratio * golden_ratio);
        let r = 0.5 * radius * k.sqrt() - 0.5 * d;
        let ad = 0.4 + 0.4 * k;
        let x = center.0 + r * a.cos();
        let y = center.1 + r * a.sin();
        let x2 = x + d * (a + ad).cos();
        let y2 = y + d * (a + ad).sin();
        let x3 = x + d * (a - ad).cos();
        let y3 = y + d * (a - ad).sin();
        let x2h = x + d * (a + 0.5 * ad).cos();
        let y2h = y + d * (a + 0.5 * ad).sin();
        let x3h = x + d * (a - 0.5 * ad).cos();
        let y3h = y + d * (a - 0.5 * ad).sin();
        let route = vec![
            (x3, y3),
            (x, y),
            (x2, y2),
            (x3, y3),
            (x3h, y3h),
            (x, y),
            (x2h, y2h),
            (x3h, y3h),
        ];
        routes.push(route);
    }

    let count = rng.gen_range(40, 50);
    for i in 0..count {
        let k = i as f64 / (count as f64);
        let main_a = 2. * PI * k;
        let init_a = main_a + rng.gen_range(-0.5, 0.5);
        let max = rng.gen_range(0.4, 0.7);
        let aamp = 0.8 - 0.4 * k;
        let lines = (0.4 * aamp * radius) as usize;
        for l in 0..lines {
            let kl = l as f64 / (lines as f64);
            let a = main_a + aamp * (kl - 0.5);
            let mut route = Vec::new();
            let r = 0.5 * radius;
            let mut x = center.0 + r * a.cos();
            let mut y = center.1 + r * a.sin();
            route.push((x, y));
            let m = max * (1. - 2. * (kl - 0.5).abs());
            let iterations = (radius * m) as usize;
            let mut a = init_a;
            let mut acc_diff = 0.0;
            let amp = 0.05;
            for j in 0..iterations {
                let mut d = amp * perlin.get([
                    0.001 * x + seed - 70.7 * (k as f64),
                    0.001 * y + 1.7 * (k as f64),
                    0.1 * perlin.get([
                        0.002 * x,
                        0.002 * y,
                        seed
                    ])
                ]);
                acc_diff += d;
                if j > iterations / 2 {
                    if (acc_diff < 0.) == (d < 0.) {
                        d *= -1.0;
                    }
                }
                a += d;
                x += a.cos();
                y += a.sin();
                route.push((x, y));
            }
            routes.push(route);
        }
        
    }

    routes
}



fn art(opts: Opts) -> Vec<Group> {
    let width = 297.0;
    let height = 210.0;
    let pad = 20.0;
    let bounds = (pad, pad, width - pad, height - pad);
    let stroke_width = 0.35;
    let mut seed = opts.seed / 7.;
    let min_scale = 6.0;
    let max_scale = 120.0;
    let colors = vec!["#f80", "#0ac"];
    colors.iter().map(|color| {
        let primary = packing(
            seed,
            2000000,
            10000,
            4,
            0.0,
            bounds,
            &VCircle::new(width/2., height/2., width+height),
            min_scale,
            max_scale,
        );
        let circles = primary;
        seed += 12.34;
        /*
        
        let mut circles = Vec::new();
        for c in primary {
            if c.r > 2. * min_scale {
                for c2 in packing(
                    seed,
                    2000000,
                    10000,
                    2,
                    0.,
                    bounds,
                    &c,
                    min_scale,
                    max_scale,
                ) {
                    circles.push(c2);
                }
                seed = seed * 1.1 + 0.3;
            }
            else {
                circles.push(c);
            }
        }
        */

        let routes: Vec<Vec<(f64, f64)>> =
            circles.par_iter()
            .enumerate()
            .flat_map(|(i, c)| {
                flower(
                    opts.seed * 7.7 + i as f64 / 3.1,
                    (c.x, c.y),
                    (c.r * 1.3).min(
                        c.r + pad / 3.0
                    )
                )
            })
            .collect();

        let mut data = Data::new();
        for route in routes {
            data = render_route(data, route);
        }
        
        layer(color).add(base_path(color, stroke_width, data))
    }).collect()
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
