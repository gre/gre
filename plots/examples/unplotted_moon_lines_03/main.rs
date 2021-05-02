use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn parametric(mut t: f64) -> (f64, f64) {
    t = t * 2.0;
    return (
        0.3 * (4. * PI * t).sin()
            + 0.3 * (3. * PI * t).sin(),
        0.3 * (3. * PI * t).cos()
            + 0.3 * (2. * PI * t).cos(),
    );
}

fn art(seed: f64) -> Vec<Group> {
    let w = 297.;
    let h = 210.;
    let r = 90.;
    let perlin = Perlin::new();

    let in_area =
        |p| euclidian_dist(p, (w / 2., h / 2.)) < r;

    let field = |(x, y): (f64, f64)| {
        0.04 * perlin.get([x / w, y / w, seed])
            + 0.02
                * perlin.get([3. * x / w, 3. * y / w, seed])
        /*
        PI / 4.
            + 0.5
                * perlin.get([
                    10. * x / w,
                    10. * y / h,
                    seed + i as f64 * 0.05,
                ])
            + 1.0
                * perlin.get([
                    5. * x / w,
                    5. * y / h,
                    seed + 100. + i as f64 * 0.01,
                ])
            + 2.0 * perlin.get([x / w, y / h, seed + 10.])
            */
    };

    let mut routes = Vec::new();

    let lines = 500;
    let length = 500;
    // let width = 400.;

    for i in 0..lines {
        let sp = 2. * (i as f64) / (lines as f64);
        let p1 = parametric(sp);
        let p2 = parametric(0.001 + sp);
        let x = p1.0 * r + w / 2.;
        let y = p1.1 * r + h / 2.;

        /*
        let x = -(width / 2.)
            + (w - width) / 2.
            + i as f64 * width / (lines as f64);
        let y = (width / 2.)
            - i as f64 * width / (lines as f64);
            */
        let mut points = Vec::new();
        let mut p = (x, y);
        let mut entered = in_area(p);
        if entered {
            points.push(p);
        }
        let mut a = (p1.1 - p2.1).atan2(p1.0 - p2.0) + 0.3;

        for _k in 0..length {
            a += field(p);
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

    let samples = 2000;
    let mut route = Vec::new();
    for s in 0..samples {
        let p =
            parametric(2. * (s as f64) / (samples as f64));
        let x = p.0 * r + w / 2.;
        let y = p.1 * r + h / 2.;
        route.push((x, y));
    }
    route.push(route[0]);
    routes.push(route);

    let samples = 2000;
    let mut route = Vec::new();
    for s in 0..samples {
        let p = 2. * PI * (s as f64) / (samples as f64);
        let x = p.cos() * r + w / 2.;
        let y = p.sin() * r + h / 2.;
        route.push((x, y));
    }
    route.push(route[0]);
    routes.push(route);

    let mut groups = Vec::new();

    let colors = vec!["lightblue", "white"];

    for (i, color) in colors.iter().enumerate() {
        let mut data = Data::new();
        for (j, route) in routes.iter().enumerate() {
            if (j as f64 / 6. % 1.4) as usize == i {
                data = render_route(data, route.clone());
            }
        }
        let mut l = layer(color);
        l = l.add(base_path(color, 0.3, data));
        if i == colors.len() - 1 {
            l = l.add(signature(
                1.0,
                (240.0, 180.0),
                color,
            ));
        }
        groups.push(l);
    }

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(1.0);
    let groups = art(seed);
    let mut document = base_a4_landscape("black");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
