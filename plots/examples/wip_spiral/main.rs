use gre::*;
use noise::{NoiseFn, Perlin};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let default_seed = 0.0;
    let default_samples = 1000;
    let default_color1 = &String::from("purple");
    let default_color2 = &String::from("pink");
    let bg = "white";

    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(default_seed);
    let samples = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(default_samples);
    let color1 = args.get(3).unwrap_or(default_color1).as_str();
    let color2 = args.get(4).unwrap_or(default_color2).as_str();

    let perlin = Perlin::new();

    let golden_angle = PI * (3.0 - (5.0 as f64).sqrt());

    let field = |_i: f64, _j: f64, (x, y): (f64, f64), _base: f64| {
        perlin.get([2. * x, 2. * y, 1. + seed])
            + perlin.get([5. * x, 5. * y, 2. + seed])
            + perlin.get([20. * x, 20. * y, 3. + seed])
    };

    let angle_velocity = |i: f64, j: f64, (x, y): (f64, f64)| {
        0.3 * (0.5 + j.powf(2.))
            * (perlin.get([2. * x, 2. * y, i + 10. + seed])
                + perlin.get([3. * x, 3. * y, 10. + seed]))
    };

    let mix_field = |a, f, m| {
        let mut delta = f - a;
        if delta < PI {
            delta += 2. * PI;
        }
        if delta > PI {
            delta -= 2. * PI;
        }
        return a + m * delta;
    };

    let mut layer1 = layer(color1);
    let mut layer2 = layer(color2);
    let boundaries = (0.0, 0.0, 420.0, 297.0);
    let precision = 1.0;
    let radius_from = 0.0;
    let radius_to = 60.0;
    let length_base = 200.0;
    let length_unify = 0.3;
    let mixing_vel = 0.3;
    let mut history: Vec<(f64, f64, f64)> = Vec::new();
    for i in 0..samples {
        let mut data = Data::new();
        let a = (golden_angle * (i as f64)) % (2.0 * PI);
        let amp =
            radius_from + (radius_to - radius_from) * ((i as f64) / (samples as f64)).powf(0.6);
        let mut p = (
            boundaries.0 + (boundaries.2 - boundaries.0) * 0.3 + a.cos() * amp,
            boundaries.1 + (boundaries.3 - boundaries.1) * 0.5 + a.sin() * amp,
        );
        let length = length_base - length_unify * amp;
        let iterations = (length / precision) as usize;
        let mut first = true;
        let mut angle = field((i as f64) / (samples as f64), 0.0, p, a);
        for j in 0..iterations {
            let normalized = (
                (p.0 - boundaries.0) / (boundaries.2 - boundaries.0),
                (p.1 - boundaries.1) / (boundaries.3 - boundaries.1),
            );
            let norm_i = (i as f64) / (samples as f64);
            let norm_j = (j as f64) / (iterations as f64);
            let f = field(norm_i, norm_j, normalized, a);
            let vel = precision * angle_velocity(norm_i, norm_j, normalized);
            angle = mix_field(angle, f, mixing_vel * precision) + vel;
            p = (p.0 + precision * angle.cos(), p.1 + precision * angle.sin());
            let mut found = false;
            for histo in &history {
                let dx = p.0 - histo.0;
                let dy = p.1 - histo.1;
                let da = angle - histo.2;
                let dist = (dx * dx + dy * dy + da * da).sqrt();
                if dist < 0.01 {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
            history.push((p.0, p.1, angle));

            if p.0 < boundaries.0 || p.0 > boundaries.2 || p.1 < boundaries.1 || p.1 > boundaries.3
            {
                break;
            }
            if first {
                first = false;
                data = data.move_to(p);
            } else {
                data = data.line_to(p);
            }
        }

        let path = Path::new()
            .set("fill", "none")
            .set("stroke-width", 0.1)
            .set("d", data);

        if i % 8 == 0 {
            layer2 = layer2.add(path.set("stroke", color2));
        } else {
            layer1 = layer1.add(path.set("stroke", color1));
        }
    }

    layer1 = layer1.add(signature(1.0, (380.0, 278.0), color1));

    let groups = Group::new().add(layer1).add(layer2);

    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", format!("background: {}", bg))
        .set("viewBox", (0, 0, 420, 297))
        .set("width", "420mm")
        .set("height", "297mm")
        .add(groups);

    svg::save("image.svg", &document).unwrap();
}
