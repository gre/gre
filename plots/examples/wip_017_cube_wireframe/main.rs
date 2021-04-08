use kiss3d::camera::*;
use kiss3d::light::*;
use kiss3d::nalgebra::*;
use kiss3d::scene::*;
use noise::{NoiseFn, Perlin};
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let mut camera = FirstPerson::new(Point3::new(-6.0, 10.0, -6.0), Point3::new(15.0, -8.0, 10.0));

    // scene.read_faces(&mut |p| {});

    // projecting the points with a camera
    let mut data = Data::new();

    /*
    let dim = Vector2::new(200.0, 200.0);
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
    */
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
                .set("d", data)
                .add(gre::signature(1.0, (265.0, 195.0), "black")),
        );

    svg::save("image.svg", &document).unwrap();
}
