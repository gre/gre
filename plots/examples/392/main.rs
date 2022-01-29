use std::f64::consts::PI;
use clap::Clap;
use gre::*;
use rand::Rng;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let colors = vec!["#006", "#006"];
    let width = 297.0;
    let height = 210.0;
    let pad = 10.;
    let padx = 20.;
    let maxtopy = 20.0;
    let mut rng = rng_from_seed(opts.seed);
    let count = 600;
    let wspread = width - padx * 2.;
    let divergence = 0.6;
    let escape_p = rng.gen_range(0.02, 0.03);
    let fruit_amp = rng.gen_range(2.0, 3.0);
    let fruit_size = rng.gen_range(2.0, 4.0);
    let unstability = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);

    let mut rng_v = 0f64;
    let mut custom_rng = move || {
        rng_v = rng_v + PI;
        rng_v % 1.0
    };

    let mut circles = Vec::new();
    let mut routes = Vec::new();
    for i in 0..count {
        let v = 2. * (i as f64 / (count as f64) - 0.5);
        let mut x = width / 2. + wspread * mix(v, v.powf(3.0), 0.6) / 2.0;
        let mut y = height - pad;
        let mut route = Vec::new();
        let mut a = -PI / 2.;
        let amp = 1.0;
        rng_v = (i as f64) / (count as f64);
        loop {
            let n = custom_rng();
            a += (n - 0.5) * divergence * (0.5 + 0.5 * y / height);
            x += amp * a.cos();
            y += amp * a.sin();
            if x < padx || y < maxtopy || x > width - padx || y > height - pad {
                break;
            }
            if rng.gen_bool(escape_p) {
                break;
            }
            route.push((x, y));
        }
        let ydt = (height - pad - y).abs();

        let p = (fruit_amp * (ydt - 40.0) / height).max(0.0).min(1.0);
        if rng.gen_bool(p) {
            let r = fruit_size * mix(0.1, 1.0, mix(p, rng.gen_range(0.0, 1.0), unstability));
            circles.push(VCircle::new(x, y, r));
        }
        
        routes.push(route);
    }

    colors
        .iter()
        .enumerate()
        .map(|(ci, color)| {
            let mut data = Data::new();

            if ci == 0 {
                for route in routes.iter() {
                    data = render_route(data, route.clone());
                }
            }
            else {
                let routes: Vec<Vec<(f64, f64)>> = circles
                    .par_iter()
                    .map(|c| {
                        let s = opts.seed + c.x * 3.1 + c.y / 9.8;
                        let mut rng = rng_from_seed(s);
                        let pow = 2.0;
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
                        }, 2 + (10. * c.r) as usize, (80. + (3.0 * c.r).powf(pow)) as usize, &mut rng);
                    let candidates = samples.iter().map(|(x, y)| {
                        (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y)
                    }).collect();
                    route_spiral(candidates)
                }).collect();
                for route in routes {
                    data = render_route_curve(data, route.clone());
                }
            }

            let mut l = layer(color);
            l = l.add(base_path(color, 0.35, data));
            
            l
        })
        .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "54.0")]
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
}