use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art() -> Vec<Group> {
    let pad = 10.;
    let width = 210. - 2. * pad;
    let height = 290. - 2. * pad;

    let project = |(x, y): (f64, f64)| {
        (x * width + pad, y * height + pad)
    };

    let res = 40;

    let mut rng = SmallRng::from_seed([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    let get_color =
        image_get_color("/Users/grenaudeau/Desktop/theo.jpg").unwrap();

    let mut data = Data::new();
    let scales = vec![
        (21, 28, 1.5, 16),
        (30, 40, 2., 24),
        (50, 73, 2.5, 32),
    ];
    for (w, h, p, max_samples) in scales {
        for x in 0..w {
            let xp = x as f64 / (w as f64);
            for y in 0..h {
                let yp = y as f64 / (h as f64);
                let g = (1.
                    - grayscale(get_color((xp, yp))))
                .powf(p);
                let map_p = |(lx, ly)| {
                    (
                        xp + lx / (w as f64),
                        yp + ly / (h as f64),
                    )
                };
                let mut candidates =
                    sample_2d_candidates_f64(
                        &|p| {
                            1. - grayscale(get_color(
                                map_p(p),
                            )).powf(2.0)
                        },
                        res,
                        (max_samples as f64 * g) as usize,
                        &mut rng,
                    );
                candidates = candidates
                    .iter()
                    .map(|&p| project(map_p(p)))
                    .collect();
                data = render_fill_spiral(data, candidates);
                /*
                data = render_tsp(
                    data,
                    candidates,
                    time::Duration::seconds(1),
                );
                */
            }
        }
    }

    vec![Group::new().add(
        layer("black").add(base_path("black", 0.2, data)),
    )]
}

fn main() {
    let groups = art();
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (175.0, 280.0),
        "black",
    ));
    svg::save("image.svg", &document).unwrap();
}
