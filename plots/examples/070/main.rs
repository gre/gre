use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(seed: f64) -> Vec<Group> {
    let w = 297.;
    let h = 210.;
    let r = 90.;
    let perlin = Perlin::new();

    let in_area =
        |p| euclidian_dist(p, (w / 2., h / 2.)) < r;

    let field = |(x, y): (f64, f64), i| {
        PI / 4.
            + 0.5
                * perlin.get([
                    20. * x / w,
                    20. * y / h,
                    seed + i as f64 * 0.1,
                ])
            + 0.5
                * perlin.get([
                    10. * x / w,
                    10. * y / h,
                    seed + 100. + i as f64 * 0.05,
                ])
            + 2.0 * perlin.get([x / w, y / h, seed + 10.])
    };

    let mut routes = Vec::new();

    let lines = 600;
    let length = 500;
    let width = 400.;

    for i in 0..lines {
        let x = -(width / 2.)
            + (w - width) / 2.
            + i as f64 * width / (lines as f64);
        let y = (width / 2.)
            - i as f64 * width / (lines as f64);
        let mut points = Vec::new();
        let mut p = (x, y);
        let mut entered = false;
        for _k in 0..length {
            let a = field(p, i);
            p = follow_angle(p, a, 1.0);
            if in_area(p) {
                entered = true;
                points.push(p);
            } else {
                if entered {
                    break;
                }
            }
        }
        if points.len() > 1 {
            routes.push(points);
        }
    }

    let mut groups = Vec::new();

    let mut data = Data::new();

    for route in routes {
        data = render_route(data, route);
    }

    let color = "white";

    let mut l = layer(color);
    l = l.add(base_path(color, 0.3, data));
    l = l.add(signature(1.0, (260.0, 190.0), color));
    groups.push(l);

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let groups = art(seed);
    let mut document = base_a4_landscape("black");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
