use noise::{NoiseFn, Perlin};
use std::f32::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let mut paths = Vec::new();
    let perlin = Perlin::new();

    let sz = 10.0;
    let limits = (12, 8);
    let project = |(x, y): (i32, i32)| (175.0 + (x as f32) * sz, 135.0 + (y as f32) * sz);
    let next_p = |(x, y): (i32, i32), head: i32| -> (i32, i32) {
        let angle = (head as f32) * PI / 2.0;
        let dx = angle.cos() as i32;
        let dy = angle.sin() as i32;
        let next = (x + dx, y + dy);
        return next;
    };
    let next_p_safe = |(x, y): (i32, i32), mut head: i32| -> ((i32, i32), i32) {
        let mut next = next_p((x, y), head);
        while next.0 < -limits.0 || next.0 > limits.0 || next.1 < -limits.1 || next.1 > limits.1 {
            head = (head + 1) % 4;
            next = next_p((x, y), head);
        }
        return (next, head);
    };

    for a in 0..14 {
        let mut data = Data::new();

        let mut pos = (0, 0);
        let mut head = 0; // from 0 to 3 to know the head rotation
        for b in 0..100 {
            let f1 = (a as f64) / 60.0; // impact the divergeance
            let f2 = (b as f64) / 3.21; // impact "centers"
            let seed = 1.01;
            let turn_threshold = 0.1;
            let decision = perlin.get([f1, f2, seed]);
            if decision.abs() > turn_threshold {
                let incr = if decision > 0.0 { 1 } else { -1 };
                head = (head + incr + 4) % 4;
            }
            let (p, h) = next_p_safe(pos, head);
            pos = p;
            head = h;
            let mut p = project(pos);
            let d = (a as f32) - 0.5 * (b as f32);
            p.0 += d;
            p.1 += d;
            if b == 0 {
                data = data.move_to(p);
            } else {
                data = data.line_to(p);
            }
        }

        paths.push(
            Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.2)
                .set("d", data),
        );
    }

    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 297, 210))
        .set("height", "210mm")
        .set("width", "297mm");
    for path in paths {
        document = document.add(path);
    }

    svg::save("image.svg", &document).unwrap();
}
