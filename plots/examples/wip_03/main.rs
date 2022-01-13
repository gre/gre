use clap::Clap;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let width = 297.;
    let height = 210.;
    let colors = vec!["#000"];
    let mut rng = rng_from_seed(opts.seed);

    colors
        .iter()
        .enumerate()
        .map(|(_ci, &color)| {
            let mut routes = Vec::new();
            let samples = opts.samples;
            let perlin = Perlin::new();
            let a = rng.gen_range(100.0, 10000.0) * rng.gen_range(0.0, 1.0);
            let b = rng.gen_range(100.0, 10000.0) * rng.gen_range(0.0, 1.0);
            let c = rng.gen_range(0.1, 100.0) * rng.gen_range(0.0, 1.0);
            let d = rng.gen_range(0.1, 100.0) * rng.gen_range(0.0, 1.0);
            let e = rng.gen_range(0.1, 1.0) * rng.gen_range(0.0, 1.0);
            let mut passage = Passage2DCounter::new(0.5, width, height);

            for i in 0..samples {
                let x = width/2.0 + width * 0.48 * perlin.get([
                    0.334 + 70.7 * opts.seed / 3.,
                    0.3 + i as f64 / a,
                    e * perlin.get([
                        -opts.seed,
                        i as f64 * c
                    ])
                ]);
                let y = height/2.0 + height * 0.48 * perlin.get([
                    i as f64 / b,
                    9.1 + 40.3 * opts.seed / 7.,
                    e * perlin.get([
                        60.1 + opts.seed,
                        i as f64 * d
                    ])
                ]);
                if passage.count((x, y)) < 3 {
                    let a = rng.gen_range(0f64, 7.);
                    let amp = 1.5f64;
                    let x2 = x + amp * a.cos();
                    let y2 = y + amp * a.sin();
                    routes.push(vec![(x, y), (x2, y2)]);
                }
            }
            println!("{}", routes.len());
            let mut l = layer(color);

            for all in routes.chunks(100) {
                let mut data = Data::new();
                for route in all.iter() {
                    data = render_route_curve(data, route.clone());
                }
              l = l.add(base_path(color, 0.35, data));
            }
            
            l
        })
        .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "644.0")]
    seed: f64,
    #[clap(short, long, default_value = "50000")]
    samples: usize,
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
