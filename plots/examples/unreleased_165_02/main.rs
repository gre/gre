use clap::Clap;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "1.0")]
    seed: f64,
    #[clap(short, long, default_value = "75")]
    samples: usize,
}

fn art(opts: Opts) -> Vec<Group> {
    let (width, height) = (297., 210.);
    let precision = 0.2;
    let pad = 20.;
    let w = (width as f64 / precision) as u32;
    let h = (height as f64 / precision) as u32;
    let perlin = Perlin::new();
    let bounds = (
        pad,
        pad,
        width - pad,
        height - pad,
    );

    let colors = vec!["darkturquoise", "firebrick"];

    let samples = opts.samples;

    colors.iter().enumerate().map(|(ci, &color)| {  
        let f = |(x, y): (f64, f64)| {
            let c = ((x-0.5) * width / height, y-0.5);
            let f1 = 1.2;
            let f2 = 1.6;
            let f3 = 2.6;
            let f4 = 50.;
            let a1 = 0.3 * (2. * (0.65 - length(c)).max(0.).powf(1.3));
            let a2 = 1.8;
            let a3 = 1.3;
            let a4 = 0.005;
            let n1 = a1 * perlin.get([
                f1 * c.0,
                f1 * c.1,
                3.134 * opts.seed + a2 * perlin.get([
                    6.1 + f2 * c.0,
                    3.6 + f2 * c.1,
                    opts.seed * 7.62 + 10. + a3 * perlin.get([
                        1.23456 * opts.seed + ci as f64 * 0.05 + a4 * perlin.get([ f4 * c.1, f4 * c.0 ]),
                        f3 * c.0,
                        f3 * c.1,
                    ]),
                ])
                ]);
            let sq = (c.0+0.5).min(0.5-c.0).min((c.1+0.5).min(0.5-c.1));
            -0.15 + 3.0 * sq + n1
        };
        
        let thresholds: Vec<f64> = 
            (0..samples)
            .map(|i| {
                let k = (i as f64 + 0.3 * (ci as f64)) / (samples as f64);
                mix(smoothstep(0.0, 1.0, k), k, 0.6)
            })
            .collect();

        let res = contour(w, h, f, &thresholds);
        let mut routes = features_to_routes(res, precision);
        routes = crop_routes(&routes, bounds);
        let mut data = Data::new();
        for route in routes {
            data = render_route(
                data,
                route,
            );
        }
        let mut l = layer(color);
        l = l.add(base_path(color, 0.35, data));
        l = l.add(signature(1.0, (260., 195.), color));
        l
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
