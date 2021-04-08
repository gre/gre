use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(path: &String, spins: usize) -> Vec<Group> {
    let width = 210.;
    let height = 210.;
    let get_color = image_get_color(path).unwrap();
    let pad = 20.0;
    let boundaries = (pad, pad, width - pad, height - pad);

    let map = vec![0, 3, 5, 1, 2, 4];

    vec!["cyan", "magenta", "yellow", "black"]
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
                let cmyk = rgb_to_cmyk_vec(get_color(pn));
                let c = cmyk[g];
                let draw = c > 0.03
                    + 0.9
                        * ((map[((spins as f64 * ii)
                            as usize)
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
            if g == 3 {
                l = l.add(signature(
                    1.0,
                    (165.0, 190.0),
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
    let groups = art(path, spins);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
