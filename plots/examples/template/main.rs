use clap::Clap;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let colors = vec!["black"];
    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut data = Data::new();

            data = data.move_to((0., 0.));

            let mut l = layer(color);
            l = l.add(base_path(color, 0.2, data));
            if i == colors.len() - 1 {
                l = l.add(signature(
                    1.0,
                    (260.0, 190.0),
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
    #[clap(short, long, default_value = "0.0")]
    seed: f64,
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
