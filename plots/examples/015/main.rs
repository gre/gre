extern crate gre;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

// https://en.wikipedia.org/wiki/Cistercian_numerals
// use 2x3mm base. use scale to scale that. centered on bottom-center
fn cistercian(n: u32, x: f32, y: f32, scale: f32) -> Data {
    let gen = |d: Data, n, flipx, flipy| {
        let m = |mut dx, mut dy| {
            if flipx {
                dx *= -1;
            };
            if flipy {
                dy = 3 - dy;
            }
            (x + scale * (dx as f32), y - (dy as f32) * scale)
        };
        match n {
            1 => d.move_to(m(0, 3)).line_to(m(1, 3)),
            2 => d.move_to(m(0, 2)).line_to(m(1, 2)),
            3 => d.move_to(m(0, 3)).line_to(m(1, 2)),
            4 => d.move_to(m(0, 2)).line_to(m(1, 3)),
            5 => d.move_to(m(0, 3)).line_to(m(1, 3)).line_to(m(0, 2)),
            6 => d.move_to(m(1, 3)).line_to(m(1, 2)),
            7 => d.move_to(m(1, 2)).line_to(m(1, 3)).line_to(m(0, 3)),
            8 => d.move_to(m(0, 2)).line_to(m(1, 2)).line_to(m(1, 3)),
            9 => d
                .move_to(m(0, 2))
                .line_to(m(1, 2))
                .line_to(m(1, 3))
                .line_to(m(0, 3)),
            _ => d,
        }
    };

    let mut d = Data::new().move_to((x, y)).line_to((x, y - 3.0 * scale));
    // it's made of a same pattern that repeats with some reflection
    d = gen(d, n % 10, false, false);
    d = gen(d, (n / 10) % 10, true, false);
    d = gen(d, (n / 100) % 10, false, true);
    d = gen(d, (n / 1000) % 10, true, true);
    return d;
}

fn main() {
    let mut paths = vec![];

    let line_w = 30;
    let offset = (15.0, 30.0);
    let cell = (9.0, 11.0);
    let size = 2.0;
    for i in 0..1000 {
        let x = i % line_w;
        let y = i / line_w;
        paths.push(
            Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.2)
                .set(
                    "d",
                    cistercian(
                        i,
                        offset.0 + (x as f32) * cell.0,
                        offset.1 + (y as f32) * cell.1,
                        size,
                    ),
                ),
        );
    }

    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background:white")
        .set("viewBox", (0, 0, 297, 420))
        .set("width", "297mm")
        .set("height", "420mm")
        .add(gre::signature(1.0, (265.0, 405.0), "black"));
    for path in paths {
        document = document.add(path);
    }

    svg::save("image.svg", &document).unwrap();
}
