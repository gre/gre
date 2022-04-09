use clap::Clap;
use gre::*;
use rand::Rng;
use rayon::prelude::*;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.0")]
    seed: f64,
    #[clap(short, long, default_value = "100.0")]
    width: f64,
    #[clap(short, long, default_value = "100.0")]
    height: f64,
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
        euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
    }
    fn collides(self: &Self, c: &VCircle) -> bool {
        self.dist(c) <= 0.0
    }
    fn contains(self: &Self, c: &VCircle) -> bool {
        euclidian_dist((self.x, self.y), (c.x, c.y)) - self.r + c.r < 0.0
    }
    fn inside_bounds(self: &Self, (x1, y1, x2, y2): (f64, f64, f64, f64)) -> bool {
        x1 <= self.x - self.r
            && self.x + self.r <= x2
            && y1 <= self.y - self.r
            && self.y + self.r <= y2
    }
}

fn scaling_search<F: FnMut(f64) -> bool>(mut f: F, min_scale: f64, max_scale: f64) -> Option<f64> {
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
        } else {
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
        c.inside_bounds(container_boundaries)
            && container_circle.contains(&c)
            && !circles.iter().any(|other| c.collides(other))
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
            x,
            y,
            min_scale,
            max_scale,
        ) {
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

fn rec_packing(
    i: usize,
    seed: f64,
    container_boundaries: (f64, f64, f64, f64),
    container: &VCircle,
    retries: f64,
    retries_mult: f64,
) -> Vec<VCircle> {
    if container.r < 2. {
        return Vec::new();
    }
    let mut pad = 0.12 + 0.2 * ((i % 3) as f64);
    let min = 0.3 + 0.5 * pad;
    if i == 0 {
        pad += 1.0;
    }
    let primaries = packing(
        seed,
        2000000,
        1000,
        retries as usize,
        pad,
        container_boundaries,
        &container,
        min,
        container.r, // / 2.0,
    );
    let secondaries = primaries
        .par_iter()
        .enumerate()
        .filter(|&(_i, p)| p.r > pad)
        .map(|(_j, p)| {
            rec_packing(
                i + 1,
                7.7777 * p.x + 9.95731 * p.y + seed / 3.,
                container_boundaries,
                &p,
                retries * retries_mult,
                retries_mult,
            )
        })
        .collect::<Vec<_>>()
        .concat();

    vec![primaries, secondaries].concat()
}

fn art(opts: &Opts) -> Vec<Group> {
    let pad = 8.0;
    let width = opts.width;
    let height = opts.height;
    let stroke_width = 0.3;
    let mut rng = rng_from_seed(opts.seed);
    let retries = 1.0 + rng.gen_range(1.0, 80.0) * rng.gen_range(0.0, 1.0);
    let retries_mult = rng.gen_range(0.9, 1.8);
    let container_boundaries = (pad, pad, width - pad, height - pad);
    let bounds_container = VCircle::new(width / 2.0, height / 2.0, height + width);
    let circles = rec_packing(
        0,
        opts.seed,
        container_boundaries,
        &bounds_container,
        retries,
        retries_mult,
    );

    println!("{} circles", circles.len());
    println!("{} retries", retries);

    let colors = vec!["black"];

    colors
        .iter()
        .map(|&color| {
            let mut l = layer(color);
            for c in circles.iter() {
                l = l.add(
                    Circle::new()
                        .set("r", c.r)
                        .set("cx", c.x)
                        .set("cy", c.y)
                        .set("stroke", color)
                        .set("stroke-width", stroke_width)
                        .set("fill", "none")
                        .set("style", "mix-blend-mode: multiply;"),
                );
            }
            l
        })
        .collect()
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(&opts);
    let mut document = base_document("white", opts.width, opts.height);
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
