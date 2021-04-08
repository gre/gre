use gre::smoothstep;
use noise::{NoiseFn, Perlin};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn layer(id: &str) -> Group {
    return Group::new()
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", id);
}

struct Config {
    color: String,
    seed: f64,
    length: f64,
    lines: usize,
    psize: f64,
    pmul: f64,
    div: f64,
}

fn main() {
    let mut groups = Vec::new();

    let configs = vec![
        Config {
            color: String::from("orange"),
            seed: 0.0,
            length: 65.0,
            lines: 200,
            psize: 30.0,
            pmul: 0.25,
            div: 0.9,
        },
        Config {
            color: String::from("red"),
            seed: 3.0,
            length: 72.0,
            lines: 150,
            psize: 60.0,
            pmul: 0.2,
            div: 0.8,
        },
        Config {
            color: String::from("turquoise"),
            seed: 1.0,
            length: 40.0,
            lines: 150,
            psize: 30.0,
            pmul: 0.5,
            div: 0.3,
        },
    ];

    for c in configs {
        let color = c.color.as_str();

        let perlin = Perlin::new();

        // give the field angle (not the length)
        let field = |(x, y): (f64, f64)| {
            let a = (y - 0.5).atan2(x - 0.5);
            c.div * PI + a + c.pmul * PI * perlin.get([c.psize * x, c.psize * y, c.seed])
        };
        let mut data = Data::new();

        let boundaries = (10.0, 10.0, 190.0, 190.0);
        let lines = c.lines;
        let precision = 1.0;
        let iterations = (c.length / precision) as usize;
        for l in 0..lines {
            let angle = (l as f64) * 2.0 * PI / (lines as f64);
            let length = 60.0;

            let mut p = (
                boundaries.0 + (boundaries.2 - boundaries.0) / 2.0 + length * angle.cos(),
                boundaries.1 + (boundaries.3 - boundaries.1) / 2.0 + length * angle.sin(),
            );
            data = data.move_to(p);
            let mut t: f64 = 0.0;
            for _i in 0..iterations {
                let normalized = (
                    (p.0 - boundaries.0) / (boundaries.2 - boundaries.0),
                    (p.1 - boundaries.1) / (boundaries.3 - boundaries.1),
                );
                let angle = field(normalized);
                let (px, py) = p;
                p = (p.0 + precision * angle.cos(), p.1 + precision * angle.sin());
                if p.0 < boundaries.0
                    || p.0 > boundaries.2
                    || p.1 < boundaries.1
                    || p.1 > boundaries.3
                {
                    break;
                }
                data = data.line_to(p);
            }
        }

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", 0.5)
            .set("d", data);

        groups.push(layer(color).add(path));
    }

    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 210, 210))
        .set("width", "210mm")
        .set("height", "210mm")
        .add(gre::signature(1.0, (175.0, 195.0), "black"));
    for g in groups {
        document = document.add(g);
    }

    svg::save("image.svg", &document).unwrap();
}
