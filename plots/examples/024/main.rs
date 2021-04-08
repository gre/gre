use noise::{NoiseFn, Perlin};
use std::f32::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

// for i in {0..100}; do cargo run --example 018 $i; cp image.svg results/$i.svg; done

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(70.0);
    let k1 = args
        .get(2)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(400.0);
    let k2 = args
        .get(3)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(2.5);
    let k3 = args
        .get(4)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.1);
    let k4 = args
        .get(5)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(70);
    let k5 = args
        .get(6)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(50);

    let mut paths = Vec::new();
    let perlin = Perlin::new();

    let m = 6;

    let sz = 4.0;
    let limits = (20.0, 16.0);
    let project = |(x, y): (f32, f32)| (105.0 + (x as f32) * sz, 100.0 + (y as f32) * sz);
    let next_p = |(x, y): (f32, f32), head: i32| -> (f32, f32) {
        let angle = (head as f32) * 2.0 * PI / (m as f32);
        let dx = angle.cos();
        let dy = angle.sin();
        let next = (x + dx, y + dy);
        return next;
    };
    let next_p_safe = |(x, y): (f32, f32), mut head: i32| -> ((f32, f32), i32) {
        let mut next = next_p((x, y), head);
        if next.0 < -limits.0 || next.0 > limits.0 || next.1 < -limits.1 || next.1 > limits.1 {
            head = (head + 1) % m;
            next = next_p((x, y), head);
        }
        if next.0 < -limits.0 || next.0 > limits.0 || next.1 < -limits.1 || next.1 > limits.1 {
            head = (head - 2) % m;
            next = next_p((x, y), head);
        }
        return (next, head);
    };

    for a in 0..k4 {
        let mut data = Data::new();

        let mut pos = (0.0, 0.0);
        let mut head = 0; // from 0 to M to know the head rotation
        for b in 0..k5 {
            let f1 = (a as f64) / k1;
            let f2 = (b as f64) / k2;
            let turn_threshold = k3;
            let decision = perlin.get([f1, f2, seed]);
            if decision.abs() > turn_threshold {
                let incr = if decision > 0.0 { 1 } else { -1 };
                head = (head + incr + m) % m;
            }
            let (p, h) = next_p_safe(pos, head);
            pos = p;
            head = h;
            let mut p = project(pos);
            p.1 += 0.6 * (a as f32).powf(1.2);
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
        .set("style", "background: #FF9")
        .set("viewBox", (0, 0, 210, 297))
        .set("width", "210mm")
        .set("height", "297mm");
    for path in paths {
        document = document.add(path);
    }
    document = document.add(gre::signature(1.0, (180.0, 285.0), "black"));

    svg::save("image.svg", &document).unwrap();
}
