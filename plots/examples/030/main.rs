use gre::{layer, signature};
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;
use voronoi::{make_polygons, voronoi, Point};

fn main() {
    let count = 600;

    let args: Vec<String> = std::env::args().collect();
    let seed0 = args.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(1);
    let seed1 = args
        .get(2)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(37.0);
    let perlin = Perlin::new();
    let mut rng = SmallRng::from_seed([seed0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    let f = |(x, y)| {
        perlin.get([x * 3.0, y * 3.0, seed1]) + 0.5 * perlin.get([x * 7.0, y * 7.0, seed1 + 1.0])
    };

    let mut candidates = Vec::new();
    let dim = 400;
    for x in 0..dim {
        for y in 0..dim {
            let p = ((x as f64) / (dim as f64), (y as f64) / (dim as f64));
            if f(p) > 0.2 {
                candidates.push(p);
            }
        }
    }

    rng.shuffle(&mut candidates);
    candidates.truncate(count);

    let mut points = Vec::new();
    for c in candidates {
        points.push(Point::new(0.1 + 0.8 * c.0, 0.1 + 0.8 * c.1));
    }
    let dcel = voronoi(points, 1.0);
    let polys = make_polygons(&dcel);

    let mut data = Data::new();
    for poly in polys {
        let mut first = true;
        for point in poly {
            let p = (310.0 * point.x() - 10.0, 230.0 * point.y() - 10.0);
            if first {
                first = false;
                data = data.move_to(p);
            } else {
                data = data.line_to(p);
            }
        }
    }

    // Make svg
    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: black")
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(
            Path::new()
                .set("fill", "none")
                .set("stroke", "white")
                .set("stroke-width", 0.2)
                .set("d", data),
        )
        .add(signature(1.0, (10.0, 190.0), "white"));
    svg::save("image.svg", &document).unwrap();
}
