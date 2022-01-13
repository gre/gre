use clap::Clap;
use gre::*;
use std::f64::consts::PI;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;


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
    let width = 297.;
    let height = 210.;
    let pad = 10.0;
    let bounds = (pad, pad, width - pad, height - pad);
    let colors = vec!["black"];
    colors
        .iter()
        .enumerate()
        .map(|(i, &color)| {

            let circles = packing(
                opts.seed,
                100000,
                2000,
                1,
                1.0,
                bounds,
                1.0,
                100.0
            );

            let mut l = layer(color);

            for c in circles.iter() {
                let circle = Circle::new()
                    .set("fill", "none")
                    .set("stroke", color)
                    .set("stroke-width", 0.35)
                    .set("cx", c.x)
                    .set("cy", c.y)
                    .set("r", c.r);
            }

            let mut rng = rng_from_seed(opts.seed);
            for c in circles.iter() {
                let mut data = Data::new();
                let mut route = Vec::new();
                let samples = (1.0 + 0.2 * c.r.powf(0.8)) as usize * 8;
                let mut angle = 0f64;
                for i in 0..samples {
                    let r = c.r;
                    angle += 0.1 + 2.0 * PI / 3.0;
                    let x = c.x + r * angle.cos();
                    let y = c.y + r * angle.sin();
                    route.push((x, y));
                }
                data = render_route(data, route);
                l = l.add(base_path(color, 0.35, data));
            }

            
            l
        })
        .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.0")]
    seed: f64,
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
