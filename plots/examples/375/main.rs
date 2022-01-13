use clap::Clap;
use gre::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let width = 297.;
    let height = 210.;
    let pad = 10.0;
    let colors = vec!["red", "darkcyan"];
    colors
        .iter()
        .enumerate()
        .map(|(ci, &color)| {
            let mut l = layer(color);
            let mut rng = rng_from_seed(opts.seed);
            let groups = 5;
            let amp = rng.gen_range(5, 40);
            let ampc = rng.gen_range(0, 10);
            let modulo = rng.gen_range(4, 40);
            let p: f64 = rng.gen_range(0.4, 2.0);
            for g in 0..groups {
                let mut data = Data::new();
                let mut route = Vec::new();
                let samples = (rng.gen_range(100., 160.) * (1.3f64).powf(g as f64)) as usize;
                let w = width - 2.0 * pad;
                let h = w / 10.0;
                for i in 0..samples {
                    let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
                    let v =
                    0.5 * h * 
                    (sign * (rng.gen_range(0.0, 1f64).powf(p)) +
                        (0.2 * (ci as f64 - 0.5)));
                    let xv = ((amp * i + ampc * ci) % modulo + i) as f64 / ((samples + modulo) as f64);
                    let x = pad + w * xv;
                    let y = height*(g as f64 + 1.0) / ((groups+1) as f64) + v;
                    route.push((x, y));
                }
                data = render_route_curve(data, route);
                l = l.add(base_path(color, 0.35, data));
            }
            
            l
        })
        .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.0")]
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
