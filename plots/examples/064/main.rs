use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn map_p((x, y): (f64, f64)) -> (f64, f64) {
    (10. + 277. * x, 10. + 190. * y)
}

fn art(seed: u8) -> Vec<Group> {
    let mut groups = Vec::new();
    let mut rng = SmallRng::from_seed([
        seed, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    let get_color =
        image_get_color("images/train1.jpg").unwrap();

    let colors = vec![
        // c, m, y, k
        (2, "yellow"),
        (1, "red"),
        (0, "blue"),
        (3, "black"),
    ];

    for (i, color) in colors {
        let points = sample_2d_candidates_f64(
            &|p| {
                let c = get_color(p);
                let cmyk = rgb_to_cmyk_vec(c);
                0.1 * smoothstep(0.1, 1.0, cmyk[i])
                    .powf(1.2)
            },
            800,
            3000,
            &mut rng,
        );

        let mut data = Data::new();
        for p in points {
            let p1 = map_p(p);
            let a = rng.gen_range(0.0, 2. * PI);
            let length = 3.0;
            let p2 = (
                p1.0 + length * a.cos(),
                p1.1 + length * a.sin(),
            );
            data = data.move_to(p1);
            data = data.line_to(p2);
        }

        groups.push(
            layer(color).add(base_path(color, 1.0, data)),
        );
    }

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(0);
    let groups = art(seed);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (260.0, 195.0),
        "black",
    ));
    svg::save("image.svg", &document).unwrap();
}
