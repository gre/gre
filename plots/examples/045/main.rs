use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art() -> Vec<Group> {
    let w = 10;
    let h = 15;
    let max_samples = 250;
    let res = 40;
    let pad = 10.;
    let width = 210. - 2. * pad;
    let height = 290. - 2. * pad;

    let project = |(x, y): (f64, f64)| {
        (x * width + pad, y * height + pad)
    };

    let mut rng = SmallRng::from_seed([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    let heart = |(x, y)| {
        heart_distance((10. * (x - 0.5), 14. * (0.5 - y)))
    };

    let get_color = |p| {
        let d = heart(p);
        smoothstep(0.0, -1.0, d)
            + smoothstep(8., 10., d)
                * smoothstep(12., 10., d)
    };

    let samples_factor = |p| {
        let d = heart(p);
        0.3 + 0.7 * smoothstep(0.0, -10.0, d)
    };

    let mut data = Data::new();

    for x in 0..w {
        let xp = x as f64 / (w as f64);
        for y in 0..h {
            let yp = y as f64 / (h as f64);
            let map_p = |(lx, ly)| {
                (xp + lx / (w as f64), yp + ly / (h as f64))
            };
            let mut candidates = sample_2d_candidates_f64(
                &|p| get_color(map_p(p)),
                res,
                (max_samples as f64
                    * samples_factor((
                        xp + 0.5 / (w as f64),
                        yp + 0.5 / (h as f64),
                    ))) as usize,
                &mut rng,
            );
            candidates = candidates
                .iter()
                .map(|&p| project(map_p(p)))
                .collect();
            data = render_fill_spiral(data, candidates);
        }
    }

    vec![Group::new().add(
        layer("hotpink")
            .add(base_path("hotpink", 0.4, data)),
    )]
}

fn main() {
    let groups = art();
    let mut document = base_a4_portrait("black");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (170.0, 280.0),
        "hotpink",
    ));
    svg::save("image.svg", &document).unwrap();
}
