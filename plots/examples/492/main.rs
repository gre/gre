use std::f64::consts::PI;

use clap::Clap;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;  
use svg::node::element::*;

// todo passage
// todo x,y displacement
// symmetry in the noise or other kind of glitch directly in the noise
  // use grid of hexa grid to also have diff noise. maybe
// localized noise that impact diff amplitudes 
// diff scale of x vs y

#[derive(Clap)]
#[clap()]
pub struct Opts {
    #[clap(short, long, default_value = "image.svg")]
    file: String,
    #[clap(short, long, default_value = "297.0")]
    pub width: f64,
    #[clap(short, long, default_value = "210.0")]
    pub height: f64,
    #[clap(short, long, default_value = "61.0")]
    pub seed: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed1: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed2: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed3: f64,
}

fn art(opts: &Opts) -> Vec<Group> {

    let cx = opts.width / 2.0;
    let cy = opts.height / 2.0;
    
    let mut rng = rng_from_seed(opts.seed);

    let amp1pow = rng.gen_range(0.8, 1.6);
    let amp_factor = rng.gen_range(0.0, 1.0);
    let freq1 = rng.gen_range(0.01, 0.08) * (1. - amp_factor);
    let amp1 = 0.1 + 0.4 * amp_factor;
    let freq2 = rng.gen_range(2.0, 4.0) * mix(freq1, rng.gen_range(0.01, 0.08), rng.gen_range(0.0, 1.0));
    let amp2 = rng.gen_range(0.0, 4.0);
    let freq3 = rng.gen_range(4.0, 16.0) * mix(freq1, rng.gen_range(0.01, 0.08), rng.gen_range(0.0, 1.0));
    let amp3 = rng.gen_range(0.0, 0.5) * rng.gen_range(0.0, 1.0);
    let r_increment = rng.gen_range(0.3, 0.5);
    let safe_h = rng.gen_range(-0.2, 0.5);

    let angle_map_count = rng.gen_range(2000, 20000);
    let angle_const: f64 = rng.gen_range(8.0, 600.0);
    let angle_mul = (rng.gen_range(-10f64, 60.0) * rng.gen_range(0.0, 1.0)).max(0f64);

    let amp_r_const = rng.gen_range(0.1, 0.4);

    let max_r = 80.0;
    
    // logic
    let perlin = Perlin::new();
    let mut routes = Vec::new();
    let mut highest_by_angle = vec![0f64; angle_map_count];
    let mut shape_bound = (opts.width, opts.height, 0.0, 0.0);

    let mut base_r = 0.2;
    loop {
        if base_r > max_r {
            break;
        }
    let mut route = Vec::new();
    let angle_delta = rng.gen_range(0.0, 2.0 * PI);
    let mut a = angle_delta;
    let angle_precision = 2. * PI / (angle_const + angle_mul * base_r).ceil();
    loop {
        if a - angle_delta > 2. * PI + 0.0001 {
            break;
        }
        let hba_index = (highest_by_angle.len() as f64 * (a / 2. * PI)).round() as usize % highest_by_angle.len();

        let mut r = base_r;
        let x = cx + r * a.cos();
        let y = cy + r * a.sin();
        r += amp1 * base_r * (amp_r_const + base_r/max_r).powf(amp1pow) * 
            perlin.get([
                -opts.seed + amp2 * perlin.get([
                  freq2 * x,
                  opts.seed * 7.7 - 4.,
                  freq2 * y
                  + amp3 * perlin.get([
                    freq3 * x,
                    opts.seed * 2.7 + 11.,
                    freq3 * y
                  ])
                ]),
                freq1 * x,
                freq1 * y
            ]);

        // IDEA we could add on top of it another noise

        let should_draw = r > highest_by_angle[hba_index] + safe_h;
        
        let x = cx + r * a.cos();
        let y = cy + r * a.sin();
        let p = (x, y);

        if x < shape_bound.0 {
          shape_bound.0 = x;
        }
        if y < shape_bound.1 {
          shape_bound.1 = y;
        }
        if x > shape_bound.2 {
          shape_bound.2 = x;
        }
        if y > shape_bound.3 {
          shape_bound.3 = y;
        }

        if should_draw {
          highest_by_angle[hba_index] = r;
            route.push(p);
        }
        else {
            if route.len() > 1 {
              let mut simplified = Vec::new();
              simplified.push(route[0]);
              let mut dist = 0.0;
              let l = route.len();
              for i in 1..l {
                dist += euclidian_dist(route[i-1], route[i]);
                if dist > 0.2 {
                  simplified.push(route[i]);
                  dist = 0.0;
                }
              }
              if dist > 0.0 {
                simplified.push(route[l-1]);
              }
              if route.len() > 2 {
                routes.push(route);
              }
            }
            route = Vec::new();
        }

        a += angle_precision;
    }

    routes.push(route);

    base_r += r_increment;
}

// center our shape
let shape_cx = (shape_bound.0 + shape_bound.2) / 2.;
let shape_cy = (shape_bound.1 + shape_bound.3) / 2.;
let dx = cx - shape_cx;
let dy = cy - shape_cy;
routes = routes.iter().map(|route|
  route.iter().map(|p|
    (p.0+dx, p.1+dy)
  ).collect()
).collect();


    println!("{}", routes.len());

    // render
    let colors = vec!["black"];
    colors
        .iter()
        .enumerate()
        .map(|(_i, color)| {
            let mut data = Data::new();
            for route in routes.clone() {
                data = render_route(data, route);
            }
            let mut l = layer(color);
            l = l.add(base_path(color, 0.35, data));
            l
        })
        .collect()
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(&opts);
    let mut document =
        base_document("white", opts.width, opts.height);
    for g in groups {
        document = document.add(g);
    }
    svg::save(opts.file, &document).unwrap();
}
