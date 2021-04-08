use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

struct VLine {
    route: Vec<(f64, f64)>,
}

impl VLine {
    fn new(start: (f64, f64)) -> Self {
        let mut route = Vec::new();
        route.push(start);
        VLine { route }
    }
    fn go(self: &Self, p: (f64, f64)) -> Self {
        let mut route = self.route.clone();
        route.push(p);
        VLine { route }
    }
}

fn euclidian_rgb_distance(
    a: (f64, f64, f64),
    b: (f64, f64, f64),
) -> f64 {
    let r = a.0 - b.0;
    let g = a.1 - b.1;
    let b = a.2 - b.2;
    (r * r + g * g + b * b).sqrt()
}

fn ray_pixels_lookup(
    lookup: &dyn Fn((f64, f64)) -> f64,
    origin: (f64, f64),
    angle: f64,
    incr: f64,
    count: usize,
) -> f64 {
    let mut sum = 0.0;
    for i in 0..count {
        let dist = (1 + i) as f64 * incr;
        let p = (
            origin.0 + dist * angle.cos(),
            origin.1 + dist * angle.sin(),
        );
        sum += lookup(p);
    }
    sum / (count as f64)
}

/*
fn ray_pixels_euclidian_dist(
    get_color: &dyn Fn((f64, f64)) -> (f64, f64, f64),
    origin: (f64, f64),
    angle: f64,
    incr: f64,
    count: usize,
) -> f64 {
    let origin_color = get_color(origin);
    let lookup = |p| {
        euclidian_rgb_distance(get_color(p), origin_color)
    };
    ray_pixels_lookup(&lookup, origin, angle, incr, count)
}
*/

fn ray_pixels_dist_to_color(
    get_color: &dyn Fn((f64, f64)) -> (f64, f64, f64),
    color: (f64, f64, f64),
    origin: (f64, f64),
    angle: f64,
    incr: f64,
    count: usize,
) -> f64 {
    let lookup =
        |p| euclidian_rgb_distance(get_color(p), color);
    ray_pixels_lookup(&lookup, origin, angle, incr, count)
}

fn ray_pixels_find_best_angle(
    get_color: &dyn Fn((f64, f64)) -> (f64, f64, f64),
    angles: usize,
    color: (f64, f64, f64),
    origin: (f64, f64),
    incr: f64,
    count: usize,
) -> f64 {
    let mut best_angle = 0.0;
    let mut best_dist = ray_pixels_dist_to_color(
        get_color, color, origin, 0.0, incr, count,
    );
    for a in 1..angles {
        let angle = 2. * PI * a as f64 / angles as f64;
        let dist = ray_pixels_dist_to_color(
            get_color, color, origin, angle, incr, count,
        );
        if dist < best_dist {
            best_angle = angle;
            best_dist = dist;
        }
    }
    best_angle
}

// TODO: bilinear pixel interpolation
// TODO: curves on a "route" instead of "line_to's"

fn art(seed0: u8) -> Vec<Group> {
    let sampling_resolution = 800;
    let sampling_base = 0.0;
    let sampling_mul = 0.005;
    let sampling_max = 1000;
    let ray_incr = 0.004;
    let ray_count = 8;
    let angle_diverge_threshold = 0.05;
    let angle_ray_diverge = 0.08;
    let angle_diverge = 0.12;
    let lines_precision = 1.0;
    let lines_dist = 30;

    // dimensions
    let padx = 10.;
    let pady = 10.;
    let width = 277.;
    let height = 190.;

    let mut groups = Vec::new();

    let mut data = Data::new();

    let mut rng = SmallRng::from_seed([
        seed0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    let get_color =
        image_get_color("images/train1.jpg").unwrap();

    let samples = sample_2d_candidates_f64(
        &|p| {
            let c = get_color(p);
            let g = grayscale(c);
            sampling_base
                + sampling_mul
                    * smoothstep(0.95, 0.0, g).powf(1.2)
                    * smoothstep(
                        0.8,
                        0.2,
                        euclidian_dist(p, (0.5, 0.5)),
                    )
        },
        sampling_resolution,
        sampling_max,
        &mut rng,
    );

    let bounds = (padx, pady, padx + width, pady + height);

    let search_color = (0.0, 0.0, 0.0);

    let mut vlines = Vec::new();
    for origin in samples {
        let mut p = project_in_boundaries(origin, bounds);
        let mut vline = VLine::new(p);
        let mut a = rng.gen_range(-0.5, 0.5)
            + ray_pixels_find_best_angle(
                &get_color,
                8,
                search_color,
                normalize_in_boundaries(p, bounds),
                ray_incr,
                ray_count,
            );
        for _s in 0..lines_dist {
            let pn = normalize_in_boundaries(p, bounds);
            let d1 = ray_pixels_dist_to_color(
                &get_color,
                search_color,
                pn,
                a + angle_ray_diverge,
                ray_incr,
                ray_count,
            );
            let d2 = ray_pixels_dist_to_color(
                &get_color,
                search_color,
                pn,
                a - angle_ray_diverge,
                ray_incr,
                ray_count,
            );

            if d1 == 0. && d2 == 0.
                || d1 != 0.
                    && d2 != 0.
                    && (((d2 - d1) / d1).abs()
                        < angle_diverge_threshold)
            {
                // continue straight line
            } else if d1 < d2 {
                a += angle_diverge;
            } else {
                a -= angle_diverge;
            }

            p = (
                p.0 + lines_precision * a.cos(),
                p.1 + lines_precision * a.sin(),
            );
            if out_of_boundaries(p, bounds) {
                break;
            }
            vline = vline.go(p);
        }
        vlines.push(vline);
    }

    for vline in vlines {
        data = render_route(data, vline.route);
    }

    let color = "black";
    groups.push(
        layer(color).add(base_path(color, 0.2, data)),
    );

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(19);
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
