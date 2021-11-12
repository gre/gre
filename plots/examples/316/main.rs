use clap::Clap;
use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let w = 297.0;
    let h = 210.0;
    let c = (w/2.0, h/2.0);
    vec!["red", "blue"]
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut data = Data::new();
            let mut r = opts.radius;
            let mut a = i as f64 * PI;
            data = data.move_to((c.0 + r * a.cos(), c.1 + r * a.sin()));
            loop {
                if r < 0.5 {
                    break;
                }
                a += 4.0 / (20.0 + r);
                r -= opts.increment;
                data = data.line_to((c.0 + r * a.cos(), c.1 + r * a.sin()));
            }
            let mut l = layer(color);
            l = l.add(base_path(color, 0.5, data));
            l
        })
        .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "100.")]
    radius: f64,
    #[clap(short, long, default_value = "0.03")]
    increment: f64,
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
