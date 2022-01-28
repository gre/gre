use clap::Clap;
use gre::*;
use ndarray::Array2;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::{Group, path::Data};

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.0")]
    seed: f64,
    #[clap(short, long, default_value = "8")]
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
        euclidian_dist((self.x,self.y), (c.x, c.y)) - c.r - self.r
    }
    fn collides(self: &Self, c: &VCircle) -> bool {
        self.dist(c) <= 0.0
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
    downscaling: f64,
    downscaling_pow: f64,
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
                let mut c = tries[0];
                c.r *= rng.gen_range(downscaling, 1.0).powf(downscaling_pow);
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
    let mut rng = rng_from_seed(opts.seed);
    let min_scale = rng.gen_range(0.5, 5.) * rng.gen_range(0.0, 1.0);
    let max_scale = rng.gen_range(5., 80.) * rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
    let kmeans_clusters = rng.gen_range(8, 32);
    let downscaling = rng.gen_range(0.01, 1.);
    let downscaling_pow = rng.gen_range(0.8, 2.0);

    let circles = packing(
        3.3 * opts.seed,
        1000000,
        5000,
        1,
        0.0,
        (pad, pad, width-pad, height-pad),
        min_scale,
        max_scale,
        downscaling,
        downscaling_pow
    );

    let routes: Vec<(Vec<(f64, f64)>, usize)> =
    group_with_kmeans(circles, kmeans_clusters)
        .par_iter()
        .enumerate()
        .map(|(gi, cin)| {
            let points: Vec<(f64, f64)> = cin.iter().map(|c| (c.x, c.y)).collect();
            let tour =
                travelling_salesman::simulated_annealing::solve(&points, time::Duration::seconds(opts.seconds));
            let circles: Vec<VCircle> = tour
                .route
                .iter()
                .map(|&i| cin[i])
                .collect();
            let route: Vec<(f64, f64)> =
            circles
                .par_iter()
                .flat_map(|c| {
                    let s = opts.seed + c.x * 3.1 + c.y / 9.8;
                    let mut rng = rng_from_seed(s);
                    let pow = rng.gen_range(1.4, 2.2);
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
                    }, (6. * c.r) as usize, (8. + c.r.powf(pow)) as usize, &mut rng);
                let candidates = samples.iter().map(|(x, y)| {
                    (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y)
                }).collect();
                route_spiral(candidates)
            })
            .collect();
            (route, gi)
        })
        .collect();

    let colors = vec!["#FC0", "#F60", "#F3A"];
    colors.iter().enumerate().map(|(ci, color)| {
        let mut data = Data::new();
        for (route, gi) in routes.clone() {
            if gi % colors.len() == ci {
                data = render_route_curve(data, route);
            }
        }
        layer(color)
        .add(base_path(color, stroke_width, data))
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

fn group_with_kmeans(
    samples: Vec<VCircle>,
    n: usize,
) -> Vec<Vec<VCircle>> {
    let arr = Array2::from_shape_vec(
        (samples.len(), 2),
        samples
            .iter()
            .flat_map(|c| vec![c.x, c.y])
            .collect(),
    )
    .unwrap();

    let (means, clusters) =
        rkm::kmeans_lloyd(&arr.view(), n);

    let all: Vec<Vec<VCircle>> = means
        .outer_iter()
        .enumerate()
        .map(|(c, _coord)| {
            clusters
                .iter()
                .enumerate()
                .filter(|(_i, &cluster)| cluster == c)
                .map(|(i, _c)| samples[i])
                .collect()
        })
        .collect();

    all
}