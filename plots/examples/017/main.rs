use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

// https://docs.rs/svg/0.8.0/svg/

fn mandelbrot(init: (f32, f32)) -> f32 {
    let mut p = init;
    let it = 500;
    for i in 0..it {
        p = (init.0 + p.0 * p.0 - p.1 * p.1, init.1 + 2.0 * p.0 * p.1);
        if p.0 * p.0 + p.1 * p.1 >= 4.0 {
            return (i as f32) / (it as f32);
        }
    }
    return 1.0;
}

////// vectorize function :)
fn vectorize_as_hlines(
    get: impl Fn((f32, f32)) -> f32,
    size: (f32, f32),
    lines: i32,
    divs: i32,
    w: i32,
    color: &str,
) -> Group {
    let mut data = Data::new();

    let pos = |xp, yp, dp| (size.0 * xp, size.1 * (yp + dp / (lines as f32)));

    for d in 0..divs {
        let dp = (d as f32) / (divs as f32);
        for l in 0..lines {
            let yp = (l as f32) / (lines as f32);
            let mut down = false;
            for i in 0..w {
                let mut xp = (i as f32) / (w as f32);
                let value = get((xp, yp + dp / (lines as f32)));
                let draw = dp < value;
                if !down && draw {
                    // start a line
                    data = data.move_to(pos(xp, yp, dp));
                    down = true;
                } else if down && !draw {
                    // finish to make a line
                    data = data.line_to(pos(xp, yp, dp));
                    down = false;
                }
            }
            if down {
                data = data.line_to(pos(1.0, yp, dp));
            }
        }
    }

    return Group::new().add(
        Path::new()
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", 0.1)
            .set("d", data),
    );
}

fn main() {
    let get = |(x, y)| mandelbrot((3.0 * (x - 0.7), 3.0 * (y - 0.5)));
    let art = vectorize_as_hlines(get, (160.0, 160.0), 50, 6, 500, "black")
        .set("transform", "translate(20,20)");

    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background:white")
        .set("viewBox", (0, 0, 210, 210))
        .set("width", "210mm")
        .set("height", "210mm")
        .add(art)
        .add(gre::signature(1.0, (180.0, 195.0), "black"));

    svg::save("image.svg", &document).unwrap();
}
