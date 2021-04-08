use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(seed: f64) -> Vec<Group> {
    let mut groups = Vec::new();
    let perlin = Perlin::new();
    let bounds = (10., 20., 287., 190.);

    for g in 0..2 {
        let mut data = Data::new();

        let field = |(x, y): (f64, f64)| -> f64 {
            -PI / 2.
                + g as f64 * PI
                + 0.1
                    * perlin.get([0.03 * x, 0.03 * y, seed])
                + 0.2
                    * perlin.get([0.02 * x, 0.02 * y, seed])
                + 0.03
                    * (y - 105.).abs()
                    * perlin.get([0.01 * x, 0.01 * y, seed])
        };

        let count = 500;
        let f = 8.;
        let amp = 10.;
        for i in 0..count {
            let mut p = (
                10. + 277. * (g as f64 * 0.5 + i as f64)
                    / (count as f64),
                105. + amp
                    * ((g as f64
                        + i as f64 * f / (count as f64))
                        * PI)
                        .sin(),
            );

            data = data.move_to(p);
            let samples = 200;
            for _j in 0..samples {
                let a = field(p);
                p = (p.0 + a.cos(), p.1 + a.sin());
                if out_of_boundaries(p, bounds) {
                    break;
                }
                data = data.line_to(p);
            }
        }

        let color = "black";
        groups.push(
            layer(color).add(base_path(color, 0.2, data)),
        );
    }

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(36.0);
    let groups = art(seed);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (260.0, 190.0),
        "black",
    ));
    svg::save("image.svg", &document).unwrap();
}
