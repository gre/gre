use clap::Clap;
use gre::*;
use noise::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: &Opts) -> Vec<Group> {
    let mut rng = rng_from_seed(opts.seed);
    let seed = opts.seed;
    let (w, h, sign_pos) = if opts.portrait {
        (210, 297, (160.0, 270.0))
    } else {
        (297, 210, (240.0, 180.))
    };
    let precision = 0.5;
    let width = (w as f64 / precision) as u32;
    let height = (h as f64 / precision) as u32;
    let perlin = Perlin::new();
    let ratio = width as f64 / height as f64;
    let offset = rng.gen_range(-1., 0.);
    let amp_dist = rng.gen_range(5., 6.) - offset;
    let freq1 = rng.gen_range(2., 4.);
    let amp2 = rng.gen_range(0.1, 1.0);
    let freq2 = rng.gen_range(6., 8.);
    let amp3 = rng.gen_range(0.0, 0.1);
    let freq3 = rng.gen_range(10., 40.);
    let f = |(x, y)| {
        offset
            + amp_dist * euclidian_dist((x, y), (0.5, 0.5))
            + perlin.get([
                ratio * freq1 * x
                    + amp2
                        * perlin.get([
                            ratio * freq2 * x,
                            freq2 * y,
                            50. + seed,
                        ]),
                freq1 * y
                    + amp2
                        * perlin.get([
                            ratio * freq2 * x,
                            freq2 * y,
                            100. + seed,
                        ]),
                seed + amp3
                    * perlin.get([
                        ratio * freq3 * x,
                        freq3 * y,
                        10. + seed,
                    ]),
            ])
    };
    let thresholds = (0..opts.count)
        .map(|i| i as f64 * opts.mult)
        .collect();
    let res = contour(width, height, f, &thresholds);
    let mut data = Data::new();
    for route in crop_routes(
        &features_to_routes(res, precision),
        (
            10.0,
            10.0,
            (width - 10) as f64,
            (height - 10) as f64,
        ),
    ) {
        data = render_route(data, route);
    }
    let color = "black";
    let mut l = layer(color);
    l = l.add(base_path(color, 0.2, data));
    l = l.add(signature(1.0, sign_pos, color));
    vec![l]
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long)]
    portrait: bool,
    #[clap(short, long, default_value = "0.0")]
    seed: f64,
    #[clap(short, long, default_value = "60")]
    count: usize,
    #[clap(short, long, default_value = "0.025")]
    mult: f64,
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(&opts);
    let mut document = if opts.portrait {
        base_a4_portrait("white")
    } else {
        base_a4_landscape("white")
    };
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
