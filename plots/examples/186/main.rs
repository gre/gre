use rayon::prelude::*;
use clap::Clap;
use gre::*;
use rand::prelude::*;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "2.0")]
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
    pad: f64,
    container: &VCircle,
    min_scale: f64,
    max_scale: f64,
    fill: f64,
    fill_stop: f64
) -> Vec<VCircle> {
    let mut circles = Vec::new();
    let mut shapes = Vec::new();
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
            circles.push(circle.clone());
            if fill <= 0.0 {
                shapes.push(circle);
            }
            else {
                shapes.push(circle);
                let mut s = size - pad - fill;
                loop {
                    if s < fill_stop {
                        break;
                    }
                    shapes.push(VCircle::new(x, y, s));
                    s -= fill;
                }
            }
        }
        if circles.len() > desired_count {
            break;
        }
    }

    shapes
}


fn art(opts: Opts) -> Vec<Group> {
    let width = 300.0;
    let height = 240.0;
    let pad = 10.0;
    let stroke_width = 0.3;

    let bounds_container = VCircle::new(width/2.0, height/2.0, height / 2.0 - pad);

    let primaries = packing(
        opts.seed,
        500000,
        2000,
        1.6,
        &bounds_container,
        0.8,
        14.0,
        0.0,
        0.0
    );

    let secondaries = primaries
        .par_iter()
        .filter(|p| p.r > 2.0)
        .map(|p| {
            packing(
                opts.seed,
                50000,
                1000,
                0.5,
                &p,
                0.6,
                10.0,
                0.2,
                1.6
            )
        })
        .collect::<Vec<_>>()
        .concat();

    let mut layers = Vec::new();

    let color = "black";
    let mut l = layer(color);
    for c in secondaries.iter() {
        l = l.add(
            Circle::new()
            .set("r", c.r)
            .set("cx", c.x)
            .set("cy", c.y)
            .set("stroke", color)
            .set(
                "stroke-width",
                stroke_width,
            )
            .set("fill", "none")
            .set("style", "mix-blend-mode: multiply;")
        );
    }
    l = l.add(signature(
        0.8,
        (133.0, 216.0),
        color,
    ));
    layers.push(l);

    layers
    
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_24x30_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
