use clap::Clap;
use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "1.0")]
    step: f64,
}

fn draw(step: f64) -> Data {
    let mut data = Data::new();
    let boundaries = (10., 10., 200., 200.);
    let mut y = boundaries.1;
    let mut ltr = true;
    loop {
        if y > boundaries.3 {
            break;
        }
        if ltr {
            data = data
                .move_to((boundaries.0, y))
                .line_to((boundaries.2, y));
        } else {
            data = data
                .move_to((boundaries.2, y))
                .line_to((boundaries.0, y));
        }
        ltr = !ltr;
        y += step;
    }
    let mut x = boundaries.0;
    let mut ltr = true;
    loop {
        if x > boundaries.2 {
            break;
        }
        if ltr {
            data = data
                .move_to((x, boundaries.1))
                .line_to((x, boundaries.3));
        } else {
            data = data
                .move_to((x, boundaries.3))
                .line_to((x, boundaries.1));
        }
        ltr = !ltr;
        x += step;
    }
    data
}

fn art(opts: Opts) -> Vec<Group> {
    vec![
        layer("brush").add(base_path(
            "black",
            0.5,
            draw(opts.step),
        )),
        layer("signature").add(signature(
            2.0,
            (150.0, 192.0),
            "black",
        )),
    ]
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
