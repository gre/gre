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
    #[clap(short, long, default_value = "210.0")]
    pub width: f64,
    #[clap(short, long, default_value = "297.0")]
    pub height: f64,
    #[clap(short, long, default_value = "10.0")]
    pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
    let width = opts.width;
    let height = opts.height;
    let colors = vec!["black"];
    let perlin = Perlin::new();

    let mut routes = Vec::new();

    let mut rng = rng_from_seed(opts.seed);
    let frequency = rng.gen_range(0.5, 3.0);
    let min_spiral_angle_divider =
        rng.gen_range(30.0, 50.0);
    let spiral_amp = 0.0001 * rng.gen_range(1.0, 4.0);
    let spiral_noise_pow = 1.4;
    let black_factor =
        rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0);

    let freq =
        rng.gen_range(0.0, 3.0) * rng.gen_range(0.0, 1.0);
    let amp: f64 = (rng.gen_range(0f64, 20.0)
        * rng.gen_range(0.0, 1.0)
        - 1.0)
        .max(0.0);

    let aincr = 0.001;
    let mut r = 0.2;
    let mut a = 0f64;
    let center = (width / 2.0, height / 2.0);
    let mut route = Vec::new();
    let min_stroke = 0.2;
    route.push((
        center.0 + r * a.cos(),
        center.1 + r * a.sin(),
    ));
    loop {
        if r > 90.0 {
            break;
        }
        let mut p = (
            center.0 + r * a.cos(),
            center.1 + r * a.sin(),
        );
        let rmul = 0.1 + r / 100.0;
        p.0 += rmul
            * amp
            * perlin.get([
                freq * p.0 / width,
                freq * p.1 / height,
                7.7 * opts.seed,
            ]);
        p.1 += rmul
            * amp
            * perlin.get([
                freq * p.0 / width,
                freq * p.1 / height,
                -3.3 * opts.seed,
            ]);

        if euclidian_dist(route[route.len() - 1], p)
            > min_stroke
        {
            route.push(p);
        }

        let rincr = aincr / min_spiral_angle_divider
            + spiral_amp
                * ((perlin
                    .get([frequency * r, opts.seed])
                    .abs()
                    * 2.0)
                    .powf(spiral_noise_pow)
                    - black_factor)
                    .max(0.0);

        r += rincr;
        a += aincr;
    }
    if route.len() > 1 {
        routes.push(route);
    }

    colors
        .iter()
        .enumerate()
        .map(|(_i, color)| {
            let mut data = Data::new();
            let mut l = layer(color);
            for route in routes.clone() {
                data = render_route(data, route);
            }
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
