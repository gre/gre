use std::{f64::consts::PI, cmp::Ordering};
use clap::Clap;
use geo::{*, intersects::Intersects, prelude::BoundingRect};
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
    let width = 420.0;
    let height = 297.0;
    let pad = 10.0;
    let stroke_width = 0.35;

    let circles = packing(
        opts.seed,
        1000000,
        9999,
        1,
        0.3,
        (pad, pad, width-pad, height-pad),
        0.6,
        8.0,
    );

    println!("{}", circles.len());

    let routes = 
        circles
        .par_iter()
        .flat_map(|circle| {
            let s = opts.seed + circle.x * 9. + circle.y / 29.;
            let mut rng = rng_from_seed(s);
            let choice = (rng.gen_range(0.0, 1.0) *rng.gen_range(0.0, 1.0) * 7f32).floor() as u8;
            match choice {
                0 => shape_strokes_random(s, &opts, circle),
                1 => shape_strokes_spiral(s, &opts, circle),
                2 => shape_strokes_sorted(s, &opts, circle),
                3 => shape_cross_hatch(s, &opts, circle),
                4 => shape_cross_x(s, &opts, circle),
                5 => shape_cross_y(s, &opts, circle),
                _ => shape_spiral(s, &opts, circle),
            }
        })
        .collect::<Vec<_>>();

    let mut layers = Vec::new();

    let color = "black";
    let mut l = layer(color);
    let mut data = Data::new();
    for route in routes {
        data = render_route_curve(data, route);
    }
    l = l.add(base_path(color, stroke_width, data));
    layers.push(l);

    layers
    
}



fn shape_strokes_random(seed: f64, _opts: &Opts, c: &VCircle) -> Vec<Vec<(f64, f64)>> {
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
    vec![samples.iter().map(|(x, y)| {
        (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y)
    }).collect()]
}

fn shape_cross_hatch(seed: f64, _opts: &Opts, c: &VCircle) -> Vec<Vec<(f64, f64)>> {
    let mut rng = rng_from_seed(seed);
    let mut points: Vec<(f64, f64)> = (0..((20. + 1.2 * c.r * c.r) as usize)).map(|_i| {
        let a = rng.gen_range(0., 2. * PI);
        (c.x + c.r * a.cos(), c.y + c.r * a.sin())
    }).collect();
    let mut routes = Vec::new();
    points.sort_by(sort_points_x);
    routes.push(points.clone());
    points.sort_by(sort_points_y);
    routes.push(points.clone());
    return routes;
}

fn shape_cross_x(seed: f64, _opts: &Opts, c: &VCircle) -> Vec<Vec<(f64, f64)>> {
    let mut rng = rng_from_seed(seed);
    let mut points: Vec<(f64, f64)> = (0..((20. + 2. * c.r * c.r) as usize)).map(|_i| {
        let a = rng.gen_range(0., 2. * PI);
        (c.x + c.r * a.cos(), c.y + c.r * a.sin())
    }).collect();
    let mut routes = Vec::new();
    points.sort_by(sort_points_x);
    routes.push(points.clone());
    return routes;
}

fn shape_cross_y(seed: f64, _opts: &Opts, c: &VCircle) -> Vec<Vec<(f64, f64)>> {
    let mut rng = rng_from_seed(seed);
    let mut points: Vec<(f64, f64)> = (0..((20. + 2. * c.r * c.r) as usize)).map(|_i| {
        let a = rng.gen_range(0., 2. * PI);
        (c.x + c.r * a.cos(), c.y + c.r * a.sin())
    }).collect();
    let mut routes = Vec::new();
    points.sort_by(sort_points_y);
    routes.push(points.clone());
    return routes;
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

fn shape_strokes_spiral(seed: f64, _opts: &Opts, c: &VCircle) -> Vec<Vec<(f64, f64)>> {
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
    }, (8. * c.r) as usize, (40. + 2.5 * c.r * c.r) as usize, &mut rng);
    if samples.len() < 2 {
        return Vec::new();
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
    }
    vec![result.iter().map(|(x, y)| {
        (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y)
    }).collect()]
}

fn shape_strokes_sorted(seed: f64, _opts: &Opts, c: &VCircle) -> Vec<Vec<(f64, f64)>> {
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
    }, (4. * c.r) as usize, (20.0 + c.r * c.r) as usize, &mut rng);
    samples.sort_by(sort_points_diag);
    vec![samples.iter().map(|(x, y)| {
        (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y)
    }).collect()]
}

fn shape_spiral(_seed: f64, _opts: &Opts, c: &VCircle) -> Vec<Vec<(f64, f64)>> {
    let mut r = c.r;
    let mut a = 0f64;
    let mut points = Vec::new();
    let increment = 0.02;
    loop {
        if r < 0.2 {
            break;
        }
        points.push((c.x + r * a.cos(), c.y + r * a.sin()));
        a += 4. / (20. + r.powf(0.5));
        r -= increment;
    }
    vec![points]
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a3_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
