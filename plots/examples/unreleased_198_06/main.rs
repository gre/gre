use clap::Clap;
use gre::*;
use rand::prelude::*;
use svg::node::element::{*, path::Data};

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "5.0")]
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
    let width = 297.0;
    let height = 210.0;
    let pad = 10.0;
    let stroke_width = 0.35;
    let mut rng = rng_from_seed(opts.seed);

    let container = VCircle::new(width/2.0, height/2.0, height / 2.0 - pad);
    let circles = packing(
        opts.seed,
        1000000,
        1000,
        3,
        0.0,
        &container,
        0.5,
        40.0,
    );

    let mut routes = Vec::new();
    for c in circles.iter() {
        for i in 0..2 {
            let mut candidates =
                sample_2d_candidates_f64(
                    &|p| {
                        smoothstep(0.5, 0.48, euclidian_dist(p, (0.5, 0.5))) *
                        euclidian_dist(p, (0.5, 0.1))
                    },
                    400,
                    (c.r * 16.0) as usize,
                    &mut rng,
                );

            candidates = candidates
                .iter()
                .map(|&p| (c.x + 2. * c.r * (p.0-0.5), c.y + 2. * c.r * (p.1-0.5)))
                .collect();

            let mul = 4.0 * (i as f64 - 0.5);
            candidates.sort_by(|&a, &b| {
                (a.0 + mul * a.1)
                    .partial_cmp(
                        &(b.0 + mul * b.1),
                    )
                    .unwrap()
                    .then(
                        a.1.partial_cmp(
                            &b.1,
                        )
                        .unwrap(),
                    )
            });
            
            routes.push(candidates);
        }
    }

    let mut layers = Vec::new();

    let color = "black";
    let mut l = layer(color);
    l = l.add(signature(
        0.8,
        (180.0, 193.0),
        color,
    ));
    let mut data = Data::new();
    for route in routes {
        data = render_route_curve(data, route);
    }
    l = l.add(base_path(color, stroke_width, data));
    layers.push(l);

    layers
    
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
