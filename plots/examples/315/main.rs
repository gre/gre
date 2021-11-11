use std::f64::consts::PI;
use clap::Clap;
use gre::*;
use noise::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "27.0")]
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

fn art(opts: Opts) -> Vec<Group> {
    let seed = opts.seed;
    let width = 420.0;
    let height = 297.0;
    let pad = 10.0;
    let bounds = (pad, pad, width-pad, height-pad);
    let stroke_width = 0.35;
    let precision = 0.0025;
    let target_size = 30;
    let tries = 1000000;
    let search_res = 2000;
    let particles = 20000;
    let bounds_container = VCircle::new(width/2.0, height/2.0, width);
    let themes = vec![
        ("MediumPurple", 0, 70),
        ("LightSeaGreen", 65, 90),
        ("MidnightBlue", 90, 100),
    ];
    let mut layers = Vec::new();
    let mut rng = rng_from_seed(opts.seed);
    let primaries = packing(
        opts.seed,
        tries,
        search_res,
        1,
        rng.gen_range(0.0, 20.0),
        &bounds_container,
        2.0,
        rng.gen_range(20.0, 120.0),
    );
    let amp1 = rng.gen_range(0.0, 1.0);
    let amp2 = rng.gen_range(0.0, 1.0);
    let all = themes.par_iter().enumerate().map(|(i, &(color, from, to))| {
        let mut passage = Passage2DCounter::new(0.5, width, height);
        let max_passage = 6;

        let mut rng = rng_from_seed(7.1 * opts.seed + 1.6 * (i as f64));
        let perlin = Perlin::new();

        let mut subset: Vec<VCircle> = Vec::new();
        for p in primaries.iter() {
            let n = rng.gen_range(0, 100);
            if  from <= n && n < to {
                subset.push(p.clone());
            }
        }
        let parts = ((particles as f64) * ((to-from) as f64 / 100.0)) as usize;
        
        let samples = sample_2d_candidates_f64(&|p| {
            let g = project_in_boundaries(
                p,
                bounds
            );
            let mut d = 99f64; 
            for p in subset.iter() {
                d = d.min(euclidian_dist((p.x, p.y), g) - p.r);
            }
            smoothstep(40.0, -20.0, d)

        }, 800, parts, &mut rng);
        
        let routes: Vec<Vec<(f64, f64)>> = samples.iter().enumerate().map(|(si, &sample)| {
            let mut rng = rng_from_seed(77.1 * opts.seed + 17.6 * (i as f64) + (si as f64) * 9.28);
            let mut route = Vec::new();
            let mut p = sample;
            let mut ang = rng.gen_range(0.0, 2. * PI);
            loop {
                if out_of_boundaries(p, (0.0, 0.0, 1.0, 1.0)) {
                    break;
                }
                if route.len() >= target_size {
                    break;
                }
                let g = project_in_boundaries(
                    p,
                    (0.0, 0.0, width - 2.0 * pad, height - 2.0 * pad)
                );
                if passage.count(g) > max_passage {
                    break;
                }
                route.push(
                    g
                );

                let mut v = (0f64, 0f64);
                for p in primaries.iter() {
                    let dist = euclidian_dist((p.x, p.y), g) - p.r;
                    let a = (p.y - g.1).atan2(p.x - g.0) + (si as f64 - 0.5) * PI;
                    let r = smoothstep(30.0, -30.0, dist);
                    v.0 += r * a.cos();
                    v.1 += r * a.sin();
                }

                if v.0 != 0.0 || v.1 != 0.0 {
                    let mut a = (v.1.atan2(v.0) + 2.0 * PI) % (2. * PI);
                    if (a - ang).abs() > PI / 2.0 {
                        a += PI;
                    }
                    ang = a;
                }
                
                ang += 0.8 * perlin.get([
                    0.02 * g.0,
                    0.02 * g.1,
                    7.4 * seed + amp1 * perlin.get([
                        0.1 * g.0,
                        0.1 * g.0,
                        10. + seed + amp2 * perlin.get([
                            0.2 * g.0,
                            0.2 * g.0,
                            seed
                        ])
                    ])
                ]);
                
                let front = (
                    p.0 + precision * ang.cos(),
                    p.1 + precision * ang.sin(),
                );
                p = front;
            }
            route
        }).collect();

        (color, routes)
    }).collect::<Vec<_>>();

    let inside = |from, to| {
        strictly_in_boundaries(from, bounds) &&
        strictly_in_boundaries(to, bounds)
    };

    for (color, routes) in all {
        let mut l = layer(color);
        let mut data = Data::new();
        for route in routes.clone() {
            let r = route.iter().map(|&p| {
                (p.0 + pad, p.1 + pad)
            }).collect();
            data = render_route_when(data, r, inside);
        }
        l = l.add(base_path(color, stroke_width, data));
        layers.push(l);
    }

    layers
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
