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

    let mut groups: Vec<Group> = vec!["#030", "#003"]
        .iter()
        .enumerate()
        .map(|(g, clr)| {
            let cang = (g as f64 - 0.5) * 2. * PI / 4.;
            let camp = 1.5;
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
                let c = 1.
                    - grayscale(get_color(
                        preserve_ratio_outside(
                            pn,
                            (width, height),
                        ),
                    ))
                    .powf(0.8);
                let draw = c
                    > ((map[((spins as f64 * ii) as usize)
                        % map.len()]
                        as f64
                        + 0.5)
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
        .collect();

    let color = "red";
    let mut data = Data::new();
    for c in vec![(92.0, 99.0), (120.0, 99.0)] {
        let rays = 100;
        for i in 0..rays {
            let a = i as f64 * 2. * PI / (rays as f64);
            let dist_from =
                0.5 + 0.8 * (map[i % map.len()] as f64);
            let dist_to =
                24.0 + 2.0 * (map[i % map.len()] as f64);
            data = data.move_to((
                c.0 + dist_from * a.cos(),
                c.1 + 0.8 * dist_from * a.sin(),
            ));
            data = data.line_to((
                c.0 + dist_to * a.cos(),
                c.1 + 0.8 * dist_to * a.sin(),
            ));
        }
    }
    groups.push(
        layer(color).add(base_path(color, 0.2, data)),
    );

    return groups;
}

fn main() {
    let path = String::from("images/profile.jpg");
    let spins = 1000;
    let portrait = false;
    let square = true;
    let padsig = (10.0, 5.0);
    let dim = if square {
        (210., 210.)
    } else if portrait {
        (210., 297.)
    } else {
        (297., 210.)
    };
    let groups = art(&path, spins, dim, padsig);
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
