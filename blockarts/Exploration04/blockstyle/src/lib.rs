mod utils;

use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;
use wasm_bindgen::prelude::*;

fn base_path(
    color: &str,
    stroke_width: f64,
    data: Data,
) -> Path {
    Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("stroke-width", stroke_width)
        .set("opacity", 0.3)
        .set("d", data)
}

fn project_in_boundaries(
    p: (f64, f64),
    boundaries: (f64, f64, f64, f64),
) -> (f64, f64) {
    (
        p.0 * (boundaries.2 - boundaries.0) + boundaries.0,
        p.1 * (boundaries.3 - boundaries.1) + boundaries.1,
    )
}

fn render_route_curve(
    data: Data,
    route: Vec<(f64, f64)>,
) -> Data {
    if route.len() == 0 {
        return data;
    }
    let mut first = true;
    let mut d = data;
    let mut last = route[0];
    for p in route {
        if first {
            first = false;
            d = d.move_to(p);
        } else {
            d = d.quadratic_curve_to((
                last.0,
                last.1,
                (p.0 + last.0) / 2.,
                (p.1 + last.1) / 2.,
            ));
        }
        last = p;
    }
    return d;
}

fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let k = ((x - a) / (b - a)).max(0.0).min(1.0);
    return k * k * (3.0 - 2.0 * k);
}

fn mix(a: f64, b: f64, x: f64) -> f64 {
    (1. - x) * a + x * b
}

fn layer(id: &str) -> Group {
    return Group::new();
}

struct Opts {
  seed: f64
}

fn art(opts: Opts) -> Vec<Group> {
    let xdivisions = 200; // how much to split the width space
    let lines = 80; // how much to split the height space
    let sublines = 8; // for each line, how much do we make "sublines" to make it grow

    let m = 4.0;
    let k = 4.0;
    let k1 = 1.0;
    let k2 = 1.0;
    let k3 = 1.0;
    let k4 = 1.0;
    let k5 = 2.0;
    let k6 = 2.0;
    let colors = vec!["black"];
    colors
        .iter()
        .enumerate()
        .map(|(_ci, color)| {
            let height = 200.0;
            let width = 200.0;
            let pad = (0.0, 0.0);
            let boundaries = (pad.0, pad.1, width - pad.0, height - pad.1);
            let ratio = (boundaries.2 - boundaries.0) / (boundaries.3 - boundaries.1);
            
            let noise = OpenSimplex::new();
            let f = |point: (f64, f64)| {
                let point = (0.5 + (point.0 - 0.5).abs(), point.1);
                let p = ( point.0 * m * ratio, point.1 * m );
                let a1 = noise.get([3. + 0.9 * opts.seed, p.0, p.1 ]);
                let a2 = noise.get([p.0, p.1, 7.3 * opts.seed]);
                let b1 = noise.get([
                    p.0 + 4. * k * a1 + 7.8 + opts.seed,
                    p.1 + k * a2 ]);
                let b2 = noise.get([
                    p.0 + k * a1 + 2.1 - opts.seed,
                    p.1 + 2. * k * a2 - 1.7 ]);
                smoothstep(
                    -0.3,
                    0.5,
                    1.5 * (0.33 - (point.0-0.5).abs()) +
                    noise.get([
                    -opts.seed,
                    p.0 + 0.2 * k * a1 + 0.4 * k * b1,
                    p.1 + 0.2 * k * a2 + 0.4 * k * b2 ])
                )
            };
            let offset = |p: (f64, f64)| -> (f64, f64) {
                let a = 1.0 * noise.get([k1 * p.0, k2 * p.1, 6.7 * opts.seed]);
                let b = 1.5 * noise.get([k4 * p.0, k3 * p.1, 99. - 0.3 * opts.seed]);
                let c = 2.0 * noise.get([k5 * p.0 + a, k6 * p.1 + b]);
                (
                    p.0 + 0.05 * noise.get([a, 10. + c]),
                    p.1 + 0.02 * noise.get([b, -10. - c]),
                )
            };
            let mut routes = Vec::new(); // all the lines
            for i in 0..lines {
                let ypi = i as f64 / ((lines-1) as f64); // y=0..1
                for j in 0..sublines {
                    let yp = ypi + (j as f64) / ((lines * sublines) as f64); // y=0..1 of the resp subline
                    let mut route = Vec::new(); // one line (points to make a curve)
                    for k in 0..xdivisions {
                        let xp = (k as f64) / ((xdivisions - 1) as f64); // x=0..1
                        let origin = offset((xp, ypi));
                        let target = offset((xp, yp));
                        let v = f(target); // lookup from a normalized function
                        let p = ( // our final point (normalized in 0..1)
                            origin.0,
                            mix(origin.1, target.1, v) // interp the position based on f value
                        );
                        route.push(project_in_boundaries(p, boundaries));
                    }
                    route.push(route[route.len()-1]); // as it's a curve, we need to add last point again
                    routes.push(route);
                }
            }

            let mut l = layer(color);
            for r in routes {
                let data = render_route_curve(Data::new(), r);
                l = l.add(base_path(color, 0.35, data));
            }
            l
        })
        .collect()
}


#[wasm_bindgen]
pub fn blockstyle() -> String {
    let mut g = Group::new();
    let a = art(Opts {
        seed: 0.0
    });
    for e in a {
        g = g.add(e);
    }
    let str = g.to_string();
    return str;
}
