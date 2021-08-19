use noise::*;
use clap::Clap;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
struct Opts {
}

fn art(_opts: Opts) -> Vec<Group> {
    let get_color = image_get_color("a.jpg").unwrap();
    let colors = vec!["black"];
    colors
        .iter()
        .enumerate()
        .map(|(_ci, color)| {
            let pad = 50.0;
            let width = 100.0;
            let height = 100.0;
            let boundaries = (pad, pad, width + pad, height + pad);
            let f = |point: (f64, f64)| {
                let p = preserve_ratio_outside(
                    point,
                    (boundaries.2 - boundaries.0, boundaries.3 - boundaries.1)
                );
                let rgb = get_color(p);
                let c = grayscale(rgb);
                smoothstep(0.8, 0.0, c).powf(2.)
            };
            let mut routes = Vec::new(); // all the lines
            let xdivisions = 120; // how much to split the width space
            let lines = 40; // how much to split the height space
            let sublines = 6; // for each line, how much do we make "sublines" to make it grow
            for i in 0..lines {
                let ypi = i as f64 / ((lines-1) as f64); // y=0..1
                for j in 0..sublines {
                    let yp = ypi + (j as f64) / ((lines * sublines) as f64); // y=0..1 of the resp subline
                    let mut route = Vec::new(); // one line (points to make a curve)
                    for k in 0..xdivisions {
                        let xp = (k as f64) / ((xdivisions - 1) as f64); // x=0..1
                        let v = f((xp, yp)); // lookup from a normalized function
                        let p = ( // our final point (normalized in 0..1)
                            xp,
                            mix(ypi, yp, v) // interp the position based on f value
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

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
