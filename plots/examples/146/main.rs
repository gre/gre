use clap::Clap;
use core::f64;
use gre::*;
use std::f64::consts::PI;
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
            let pattern =
                |(width, height): (f64, f64),
                 p: (f64, f64)| {
                    base_rect(color, stroke_width)
                        .set("x", p.0 - width / 2.)
                        .set("y", p.1 - height / 2.)
                        .set("width", width)
                        .set("height", height)
                };
            let mut l = layer(color);
            for y in 0..count_y {
                let yf = y as f64 / count_y as f64;
                for x in 0..count_x {
                    let xf = x as f64 / count_x as f64;
                    let p = (
                        pad + 0.5 * opts.wsize
                            + (w - 2. * pad) * xf,
                        pad + 0.5 * opts.hsize
                            + (h - 2. * pad) * yf,
                    );
                    let width = opts.wsize;
                    let height = opts.hsize
                        * (1.
                            + opts.amp
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
                        l = l.add(pattern(
                            (width, height),
                            p,
                        ));
                    }
                }
            }
            if i == colors.len() - 1 {
                l = l.add(signature(
                    1.,
                    (w - pad - 55., h - pad - 5.),
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
    #[clap(short, long, default_value = "48")]
    count_x: usize,
    #[clap(short, long, default_value = "8")]
    count_y: usize,
    #[clap(short, long, default_value = "4.")]
    wsize: f64,
    #[clap(short, long, default_value = "16.")]
    hsize: f64,
    #[clap(short, long, default_value = "20.")]
    pad: f64,
    #[clap(short, long, default_value = "0.5")]
    amp: f64,
    #[clap(short, long, default_value = "3.")]
    freq: f64,
    #[clap(short, long, default_value = "-0.5")]
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
