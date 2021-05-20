use clap::Clap;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let stroke_width = 0.5;
    let w = 297.;
    let h = 210.;
    let pad = 30.;
    let count = (
        (w / opts.size) as usize,
        (h / opts.size) as usize,
    );
    let project = |p: (f64, f64)| {
        (
            pad + p.0 * (w - 2. * pad),
            pad + p.1 * (h - 2. * pad),
        )
    };
    let colors =
        opts.colors.split(",").collect::<Vec<&str>>();
    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut data = Data::new();
            let psize = 0.5 * opts.size * opts.plus_size;
            let pattern = |data, p: (f64, f64)| {
                let mut d = data;
                d = render_route(
                    d,
                    vec![
                        (p.0 - psize, p.1),
                        (p.0 + psize, p.1),
                    ],
                );
                d = render_route(
                    d,
                    vec![
                        (p.0, p.1 - psize),
                        (p.0, p.1 + psize),
                    ],
                );
                d
            };
            for y in 0..count.1 {
                for x in 0..(if y % 2 == 1 {
                    count.0 - 1
                } else {
                    count.0
                }) {
                    let p = (
                        (0.5 + x as f64
                            + (y as f64 % 2.) * 0.5)
                            / (count.0 as f64),
                        (0.5 + y as f64) / (count.1 as f64),
                    );
                    if y % colors.len() == i {
                        data = pattern(data, project(p));
                    }
                }
            }
            let mut l = layer(color);
            l = l.add(base_path(color, stroke_width, data));
            if i == colors.len() - 1 {
                l = l.add(signature(
                    2.0,
                    project((0.7, 1.0)),
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
    #[clap(short, long, default_value = "5.0")]
    size: f64,
    #[clap(short, long, default_value = "0.7")]
    plus_size: f64, // in %
    #[clap(
        short,
        long,
        default_value = "darkblue,firebrick"
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
