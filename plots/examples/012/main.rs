use kiss3d::camera::*;
use kiss3d::nalgebra::*;
use noise::{NoiseFn, Perlin};
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let mut points = vec![];
    let w = 20;
    let h = 20;

    let perlin = Perlin::new();
    let y = |x: f32, z: f32| 4.0 * (perlin.get([(x / 5.0) as f64, 1.0, (z / 5.0) as f64]) as f32);

    for z in 0..h {
        let ltr = z % 2 == 0;
        let zf = z as f32;
        for xi in 0..(w + 1) {
            let x = if ltr { xi } else { w - xi };
            let xf = x as f32;
            // naive implementation of points of a subdivided plane (unoptimized paths)
            points.push(Point3::new(xf, y(xf, zf), zf));
            points.push(Point3::new(xf, y(xf, zf + 1.0), zf + 1.0));
            if x < w {
                points.push(Point3::new(xf + 1.0, y(xf + 1.0, zf + 1.0), zf + 1.0));
            }
            points.push(Point3::new(xf, y(xf, zf), zf));
        }
    }

    // projecting the points with a camera
    let camera = FirstPerson::new(Point3::new(-6.0, 10.0, -6.0), Point3::new(15.0, -8.0, 10.0));
    let mut data = Data::new();
    let dim = Vector2::new(200.0, 200.0);
    let offset = Vector2::new(10.0, 10.0);
    let mut prev: Option<(f32, f32)> = None;
    for p in points {
        let pr = camera.project(&p, &dim);
        let pos = (offset.x + pr.x, offset.y + dim.y - pr.y);
        // TODO: would need to "crop" with the bounds...
        if let Some(_from) = prev {
            data = data.line_to(pos);
            prev = Some(pos);
        } else {
            data = data.move_to(pos);
            prev = Some(pos);
        }
    }
    // make svg
    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("viewBox", (0, 0, 297, 210))
        .set("height", "210mm")
        .set("width", "297mm")
        .add(
            Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.4)
                .set("d", data),
        );

    svg::save("image.svg", &document).unwrap();
}
