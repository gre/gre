use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(
    path: &String,
    spins: usize,
    (width, height): (f64, f64),
    padsig: (f64, f64),
) -> Vec<Group> {
    let get_color = image_get_color(path).unwrap();
    let pad = 20.0;
    let boundaries = (pad, pad, width - pad, height - pad);

    let map = vec![0, 3, 5, 1, 2, 4];

    vec!["darkolivegreen", "midnightblue"]
        .iter()
        .enumerate()
        .map(|(g, clr)| {
            let cang = (g as f64 - 0.5) * 2. * PI / 4.;
            let camp = 1.0;
            let c = (
                0.5 + camp * cang.cos(),
                0.5 + camp * cang.sin(),
            );
            let samples = 2000 * spins;
            let radius = 2. * camp;
            let mut routes = Vec::new();
            let mut route = Vec::new();
            let mut pen_up = true;
            for i in 0..samples {
                let ii = (i as f64) / (samples as f64);
                let a = 2. * PI * (spins as f64) * ii;
                let r = radius
                    * (ii
                        + (g as f64
                            / (4. * (spins as f64))));
                let pn =
                    (c.0 + r * a.cos(), c.1 + r * a.sin());
                let p =
                    project_in_boundaries(pn, boundaries);
                let c = 1. - grayscale(get_color(pn));
                let draw = c
                    > ((map[((2
                        * (spins as f64 * ii) as usize)
                        + g)
                        % map.len()]
                        as f64)
                        / (map.len() as f64))
                    && !out_of_boundaries(p, boundaries);
                if draw {
                    if pen_up {
                        if route.len() > 2 {
                            routes.push(route);
                        }
                        route = Vec::new();
                        pen_up = false;
                    }
                    route.push(p);
                } else {
                    pen_up = true;
                }
            }
            if route.len() > 2 {
                routes.push(route);
            }

            let data = routes.iter().fold(
                Data::new(),
                |acc, route| {
                    render_route(acc, route.clone())
                },
            );

            let mut l =
                layer(clr).add(base_path(clr, 0.2, data));
            if g == 1 {
                l = l.add(signature(
                    1.0,
                    (
                        width - 35. - padsig.0,
                        height - 15. - padsig.1,
                    ),
                    clr,
                ))
            }
            l
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).unwrap();
    let spins = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1000);
    let portrait = args
        .get(3)
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
    let square = args
        .get(4)
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
    let padsig = args
        .get(5)
        .map(|s| {
            let all: Vec<f64> = s
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|str| str.parse::<f64>().unwrap())
                .collect();
            if all.len() == 0 {
                return (40., 40.);
            }
            if all.len() == 1 {
                return (all[0], all[0]);
            }
            return (all[0], all[1]);
        })
        .unwrap_or((40.0, 40.0));
    let dim = if square {
        (210., 210.)
    } else if portrait {
        (210., 297.)
    } else {
        (297., 210.)
    };
    let groups = art(path, spins, dim, padsig);
    let mut document = if !portrait {
        base_a4_landscape("white")
    } else {
        base_a4_portrait("white")
    };
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
