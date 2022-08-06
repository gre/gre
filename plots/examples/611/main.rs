use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
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
    let colors = vec!["black"];
    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut data = Data::new();

            let width = opts.width;
            let height = opts.height;

            let mut rng = rng_from_seed(opts.seed);

            let half = 80.0;
            let cx = width / 2.0;
            let cy = height / 2.0;

            data = data.move_to((cx, cy));

            let count = 1500;
            let mut choice = 9;
            for i in 0..count {
                if rng.gen_bool(0.5) {
                    // horizontal
                    let x = cx + rng.gen_range(-half, half);
                    let y = if rng.gen_bool(0.5)
                        && choice != 0
                        || choice == 1
                    {
                        choice = 0;
                        cy - half
                    } else {
                        choice = 1;
                        cy + half
                    };
                    data = data.line_to((x, y));
                } else {
                    // horizontal
                    let x = if rng.gen_bool(0.5)
                        && choice != 2
                        || choice == 3
                    {
                        choice = 2;
                        cx - half
                    } else {
                        choice = 3;
                        cx + half
                    };
                    let y = cy + rng.gen_range(-half, half);
                    data = data.line_to((x, y));
                }
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
