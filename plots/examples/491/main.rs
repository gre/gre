use clap::Clap;
use gre::*;
use rand::Rng;
use rayon::prelude::*;
use svg::node::element::{path::Data, *};

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "585.0")]
    seed: f64,
    #[clap(short, long, default_value = "240.0")]
    width: f64,
    #[clap(short, long, default_value = "210.0")]
    height: f64,
    #[clap(short, long, default_value = "30")]
    seconds: i64,
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

fn search_circle_radius<F: Fn(&VCircle)-> bool>(
    bound: (f64, f64, f64, f64),
    inside: F,
    circles: &Vec<VCircle>,
    x: f64,
    y: f64,
    min_scale: f64,
    max_scale: f64,
) -> Option<f64> {
    let overlaps = |size| {
        let c = VCircle::new(x, y, size);
        bound.0 < c.x - c.r
            && c.x + c.r < bound.2
            && bound.1 < c.y - c.r
            && c.y + c.r < bound.3
            && inside(&c)
            && !circles.iter().any(|other| c.collides(other))
    };
    scaling_search(overlaps, min_scale, max_scale)
}

fn packing<F: Fn(&VCircle) -> bool>(
    seed: f64,
    iterations: usize,
    desired_count: usize,
    optimize_size: usize,
    pad: f64,
    bound: (f64, f64, f64, f64),
    crop: F,
    min_scale: f64,
    max_scale: f64,
    multiply_max: f64
) -> Vec<VCircle> {
    let mut circles = Vec::new();
    let mut tries = Vec::new();
    let mut rng = rng_from_seed(seed);
    let half = (max_scale - min_scale) / 2.0;
    let mut m = max_scale;
    for _i in 0..iterations {
        let x: f64 = rng.gen_range(bound.0, bound.2);
        let y: f64 = rng.gen_range(bound.1, bound.3);
        if let Some(size) = search_circle_radius(bound, &crop, &circles, x, y, min_scale, m) {
            let circle = VCircle::new(x, y, size - pad);
            tries.push(circle);
            if tries.len() > optimize_size {
                tries.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
                let c = tries[0];
                circles.push(c.clone());
                m = (m * multiply_max).max(half);
                tries = Vec::new();
            }
        }
        if circles.len() > desired_count {
            break;
        }
    }
    circles
}

fn shape_strokes_random<R: Rng>(rng: &mut R, c: &VCircle, _opts: &Opts) -> Vec<(f64, f64)> {
    let pow = rng.gen_range(1.4, 1.8);
    let samples = sample_2d_candidates_f64(
        &|p| {
            let dx = p.0 - 0.5;
            let dy = p.1 - 0.5;
            let d2 = dx * dx + dy * dy;
            if d2 > 0.25 {
                0.0
            } else {
                d2
            }
        },
        (6. * c.r) as usize,
        (40. + (0.8 * c.r).powf(pow)) as usize,
        rng,
    );
    samples
        .iter()
        .map(|(x, y)| (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y))
        .collect()
}

fn art(opts: &Opts) -> Vec<Group> {
    let pad = 10.0;
    let width = opts.width;
    let height = opts.height;
    let stroke_width = 0.35;
    let mut rng = rng_from_seed(opts.seed);
    let min_scale = 2.0;
    let max_scale = 60.0;

    let crop = |c: &VCircle| {
        let d1 = euclidian_dist(
            (c.x * 0.6, c.y),
            ((width - 110.0) * 0.6, height/2. - 10.0)
        );
        let d2 = euclidian_dist(
            (c.x * 1.4, c.y),
            (0.0, height/2. - 10.0)
        );
        d1 < 75.0 || d2 < 100.0
    };

    let mut circles = packing(
        3.3 * opts.seed,
        1000000,
        1000,
        1,
        0.0,
        (pad, pad, width - pad, height - pad),
        crop,
        min_scale,
        max_scale,
        0.7
    );

    let points: Vec<(f64, f64)> = circles.iter().map(|c| (c.x, c.y)).collect();

    let tour = travelling_salesman::simulated_annealing::solve(&points, time::Duration::seconds(opts.seconds));

    circles = tour.route.iter().map(|&i| circles[i]).collect();

    let route: Vec<(f64, f64)> = circles
        .par_iter()
        .flat_map(|circle| {
            let s = opts.seed + circle.x * 3.1 + circle.y / 9.8;
            let mut rng = rng_from_seed(s);
            shape_strokes_random(&mut rng, circle, &opts)
        })
        .collect();

    let color = "black";
    let data = render_route_curve(Data::new(), route);
    vec![layer(color).add(base_path(color, stroke_width, data))]
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
