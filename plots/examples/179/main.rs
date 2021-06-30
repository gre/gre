use clap::Clap;
use geo::Point;
use geo::Polygon;
use geo::intersects::Intersects;
use geo::polygon;
use geo::*;
use geo::prelude::BoundingRect;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "14.0")]
    seed: f64,
}

fn rotated_square_as_polygon(x: f64, y: f64, size: f64, angle: f64) -> Polygon<f64> {
    polygon![
        p_r((x-size, y-size), angle).into(),
        p_r((x+size, y-size), angle).into(),
        p_r((x+size, y+size), angle).into(),
        p_r((x-size, y+size), angle).into(),
    ]
}

fn poly_collides_in_polys(polys: &Vec<Polygon<f64>>, poly: &Polygon<f64>) -> bool {
    polys.iter().any(|p| {
        poly.intersects(p)
    })
}

fn poly_square_scaling_search(
    boundaries: (f64, f64, f64, f64),
    polys: &Vec<Polygon<f64>>,
    x: f64,
    y: f64,
    angle: f64,
    min_threshold: f64
) -> Option<f64> {
    let mut size = 0.1;
    let dsize = 0.1;
    // TODO dichotomic search could help perf here...
    loop {
        let poly = rotated_square_as_polygon(x, y, size, angle);
        let bounds = poly.bounding_rect().unwrap();
        let topleft: Point<f64> = bounds.min().into();
        let bottomright: Point<f64> = topleft + point!(
            x: bounds.width(),
            y: bounds.height()
        );
        if out_of_boundaries(topleft.x_y(), boundaries) || out_of_boundaries(bottomright.x_y(), boundaries) {
            break;
        }
        if poly_collides_in_polys(polys, &poly) {
            break;
        }
        size += dsize;
    }
    if size < min_threshold {
        return None;
    }
    return Some(size);
}


fn art(opts: Opts) -> Vec<Group> {
    let colors = vec!["black"];
    let width = 297.0;
    let height = 210.0;
    let radius = 90.0;
    let bounds = (width / 2. - radius, height / 2. - radius, width / 2. + radius, height / 2. + radius);
    let stroke_width = 0.3;
    let upper_limit = 1000000;
    let desired_count = 6000;
    let pad = 0.4;
    let min_threshold = 1.0;

    let mut polys = Vec::new();
    let mut rng = rng_from_seed(opts.seed);
    for i in 0..upper_limit {
        let x: f64 = rng.gen_range(bounds.0, bounds.2);
        let y: f64 = rng.gen_range(bounds.1, bounds.3);
        let a: f64 = rng.gen_range(0.0, 8.0);
        if let Some(size) = poly_square_scaling_search(bounds, &polys, x, y, a, min_threshold) {
            let poly = rotated_square_as_polygon(x, y, size - pad, a);
            polys.push(poly);
        }
        if polys.len() > desired_count {
            break;
        }
    }

    colors
        .iter()
        .enumerate()
        .map(|(ci, &color)| {
            let mut l = layer(color);
            let mut data = Data::new();
            for (i, poly) in polys.iter().enumerate() {
                if ci == i % colors.len() {
                    data = render_polygon_stroke(data, poly.clone());
                }
            }


            if ci == colors.len() - 1 {
                data = render_route(data, boundaries_route(bounds));
                l = l.add(signature(
                    1.0,
                    (212.0, 194.0),
                    color,
                ));
            }
            l = l.add(base_path(color, stroke_width, data));
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
