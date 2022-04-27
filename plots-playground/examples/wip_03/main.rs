use std::f64::consts::PI;

use clap::Clap;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
pub struct Opts {
    #[clap(short, long, default_value = "image.svg")]
    file: String,
    #[clap(short, long, default_value = "297.0")]
    pub width: f64,
    #[clap(short, long, default_value = "210.0")]
    pub height: f64,
    #[clap(short, long, default_value = "0.0")]
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

    let amp1pow = rng.gen_range(1.0, 2.4);
    let freq1 = rng.gen_range(0.005, 0.015);
    let amp1 = rng.gen_range(0.1, 0.5);
    let freq2 = rng.gen_range(2.0, 4.0) * freq1;
    let amp2 = rng.gen_range(1.0, 3.0);
    let freq3 = rng.gen_range(4.0, 16.0) * freq1;
    let amp3 = rng.gen_range(0.05, 0.2);

    let max_r = 80.0;
    
    // logic
    let perlin = Perlin::new();
    let mut routes = Vec::new();
    let mut highest_by_angle = vec![0f64; 8000];
    let mut shape_bound = (opts.width, opts.height, 0.0, 0.0);

    let safe_h = 0.0;
    let r_increment = 0.5;
    let mut base_r = 0.2;
    loop {
        if base_r > max_r {
            break;
        }
    let mut route = Vec::new();
    let mut a = 0.0;
    // TODO activate to create "snow" + prevent small lines < 0.3mm
    let angle_precision = 2. * PI / 800.0;// 2. * PI / (8f64 + 40. * base_r).ceil();
    let mut last_hba_index = 0;
    loop {
        if a > 2. * PI + 0.0001 {
            break;
        }
        let hba_index = (highest_by_angle.len() as f64 * (a / 2. * PI)) as usize % highest_by_angle.len();

        let mut r = base_r;
        let x = cx + r * a.cos();
        let y = cy + r * a.sin();
        r += amp1 * base_r * (base_r/max_r).powf(amp1pow) * 
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
              // TODO also don't add if it's too small distance
                routes.push(route);
            }
            route = Vec::new();
        }

        last_hba_index = hba_index;
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
        .map(|(i, color)| {
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
