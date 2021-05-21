use clap::Clap;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "1.0")]
    step: f64,
    #[clap(short, long, default_value = "220")]
    count: usize,
}

fn draw(opts: Opts) -> Data {
    let mut data = Data::new();
    let mut rng = rng_from_seed(4.0);
    let boundaries = (10., 10., 200., 200.);
    let mut y = boundaries.1;
    let mut ltr = true;
    let mut routes = Vec::new();
    loop {
        if y > boundaries.3 {
            break;
        }
        if ltr {
            routes.push(vec![
                (boundaries.0, y),
                (boundaries.2, y),
            ]);
        } else {
            routes.push(vec![
                (boundaries.2, y),
                (boundaries.0, y),
            ]);
        }
        ltr = !ltr;
        y += opts.step;
    }
    let mut x = boundaries.0;
    let mut ltr = true;
    loop {
        if x > boundaries.2 {
            break;
        }
        if ltr {
            routes.push(vec![
                (x, boundaries.1),
                (x, boundaries.3),
            ]);
        } else {
            routes.push(vec![
                (x, boundaries.3),
                (x, boundaries.1),
            ]);
        }
        ltr = !ltr;
        x += opts.step;
    }
    rng.shuffle(&mut routes);
    routes.truncate(opts.count);
    for route in routes {
        data = render_route(data, route);
    }
    data
}

fn art(opts: Opts) -> Vec<Group> {
    vec![
        layer("brush").add(base_path(
            "black",
            0.5,
            draw(opts),
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
