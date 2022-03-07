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
    let width = 297.;
    let height = 210.;
    let pad = 20.0;

    let colors = vec!["black"];
    colors
        .iter()
        .enumerate()
        .map(|(_ci, color)| {
            
            let delta = 2.;
            let mut rng = rng_from_seed(opts.seed);
            let samples = rng.gen_range(8, 32);
            let xd = rng.gen_range(0.0, 80.0);
            let yd = rng.gen_range(0.0, 80.0);
            let mut curve1: Vec<(f64, f64)> = (0..samples).map(|i| {
                let x = pad + (i as f64) * (width - 2.0 * pad) / ((samples - 1) as f64) + rng.gen_range(-xd, xd) / 2.0;
                let y = rng.gen_range(0.0, yd);
                (x, y)
            }).collect();
            rng.shuffle(&mut curve1);

            let mut curve2: Vec<(f64, f64)> = (0..samples).map(|i| {
                let x = pad + (i as f64) * (width - 2.0 * pad) / ((samples - 1) as f64) + rng.gen_range(-xd, xd) / 2.0;
                let y = rng.gen_range(0.0, yd);
                (x, y)
            }).collect();
            rng.shuffle(&mut curve2);

            let mut routes = Vec::new();

            let mut y = pad;
            loop {
                if y > height - pad {
                    break;
                }
                let halfh = height / 2.;
                let amp = (1. - (y - halfh).abs() / (height / 2.)).max(0.0);
                let interp = (y - pad) / (width - 2. * pad);
                let curve: Vec<(f64, f64)> = (1..samples).map(|i| {
                    let a = curve1[i];
                    let b = curve2[i];
                    let x = mix(a.0, b.0, interp);
                    let y = mix(a.1, b.1, interp);
                    (x, y)
                }).collect();
                routes.push(curve.iter().map(|&(xp,yp)| (xp, yp*amp+y)).collect::<Vec<_>>());
                y += delta;            }
            
            let mut data = Data::new();

            for route in routes {
                data = render_route_curve(data, route);
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
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save(opts.file, &document).unwrap();
}
