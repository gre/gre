use clap::Clap;
use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let stroke_width = 0.5;
    let w = 297.;
    let h = 210.;
    let pad = opts.pad;
    let count_x = opts.count_x;
    let count_y = opts.count_y;
    let colors =
        opts.colors.split(",").collect::<Vec<&str>>();
    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut data = Data::new();
            let pattern = |data, psize, p: (f64, f64)| {
                let mut d = data;
                d = render_route(
                    d,
                    vec![
                        (p.0 - psize, p.1 - psize),
                        (p.0 + psize, p.1 + psize),
                    ],
                );
                d = render_route(
                    d,
                    vec![
                        (p.0 + psize, p.1 - psize),
                        (p.0 - psize, p.1 + psize),
                    ],
                );
                d
            };
            for y in 0..count_y {
                let yf = y as f64 / count_y as f64;
                for x in 0..count_x {
                    let xf = x as f64 / count_x as f64;
                    let p = (
                        pad + (w - 2. * pad) * xf,
                        pad + (h - 2. * pad) * yf,
                    );
                    let psize =
                        0.5 * opts.size
                            * (1. + opts.amp
                                * (2.
                                    * PI
                                    * (yf
                                        * opts.freq_off
                                        + xf * opts.freq
                                        + i as f64
                                            / colors.len()
                                                as f64))
                                    .cos());
                    if y % colors.len() == i {
                        data = pattern(data, psize, p);
                    }
                }
            }
            let mut l = layer(color);
            l = l.add(base_path(color, stroke_width, data));
            if i == colors.len() - 1 {
                l = l.add(signature(
                    1.5,
                    (w - pad - 44., h - pad - 4.),
                    color,
                ));
            }
            l
        })
        .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "140")]
    count_x: usize,
    #[clap(short, long, default_value = "6")]
    count_y: usize,
    #[clap(short, long, default_value = "18.0")]
    size: f64,
    #[clap(short, long, default_value = "30.0")]
    pad: f64,
    #[clap(short, long, default_value = "0.8")]
    amp: f64,
    #[clap(short, long, default_value = "6.")]
    freq: f64,
    #[clap(short, long, default_value = "0.0")]
    freq_off: f64,
    #[clap(
        short,
        long,
        default_value = "hotpink,deepskyblue"
    )]
    colors: String,
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
