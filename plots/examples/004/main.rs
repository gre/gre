use std::f32::consts::PI;
use svg::node::element::*;
use svg::Document;

fn main() {
    let document = make_svg();
    svg::save("image.svg", &document).unwrap();
}

fn make_svg() -> Document {
    let get_color = |(x, y)| (x, y, 0.5); // gl-react logo =)

    let size = 90.0;
    let divs = 30;
    let offset = 1.2;
    let radius = 1.0;

    let map_magenta = |clr| radius * rgb_to_cmyk(clr).1;
    let map_yellow = |clr| radius * rgb_to_cmyk(clr).2;
    let map_cyan = |clr| radius * rgb_to_cmyk(clr).0;
    let map_black = |clr| radius * rgb_to_cmyk(clr).3;
    let translate = |angle: f32, radius: f32| {
        let x = 20.0 + radius * angle.cos();
        let y = 20.0 + radius * angle.sin();
        return format!("translate({},{})", x, y);
    };

    let magenta = vectorize_as_circles(&get_color, &map_magenta, size, divs, "magenta")
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", "magenta")
        .set("transform", translate(2.0 * PI / 3.0, offset));
    let yellow = vectorize_as_circles(&get_color, &map_yellow, size, divs, "yellow")
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", "yellow")
        .set("transform", translate(-2.0 * PI / 3.0, offset));
    let cyan = vectorize_as_circles(&get_color, &map_cyan, size, divs, "cyan")
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", "cyan")
        .set("transform", translate(0.0, offset));
    let black = vectorize_as_circles(&get_color, &map_black, size, divs, "black")
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", "black")
        .set("transform", translate(0.0, 0.0));

    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(magenta)
        .add(cyan)
        .add(yellow)
        .add(black);

    return document;
}

fn vectorize_as_circles(
    get_color: &impl Fn((f32, f32)) -> (f32, f32, f32),
    map_color: &impl Fn((f32, f32, f32)) -> f32,
    size: f32,
    divs: u32,
    color: &str,
) -> Group {
    let mut group = Group::new();
    let d = divs as f32;
    for yi in 0..divs {
        let yp = (0.5 + yi as f32) / d;
        let y = size * yp;
        for xi in 0..divs {
            let xp = (0.5 + xi as f32) / d;
            let x = size * xp;
            let clr = get_color((xp, yp));
            let r = 0.5 * map_color(clr) * size / d;
            if r > 0.05 {
                group = group.add(
                    Circle::new()
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", r)
                        .set("stroke-width", 0.5)
                        .set("stroke", color)
                        .set("fill", "none"),
                );
            }
        }
    }

    return group;
}

fn rgb_to_cmyk((r, g, b): (f32, f32, f32)) -> (f32, f32, f32, f32) {
    let k = 1.0 - r.max(g).max(b);
    let c = (1.0 - r - k) / (1.0 - k);
    let m = (1.0 - g - k) / (1.0 - k);
    let y = (1.0 - b - k) / (1.0 - k);
    return (c, m, y, k);
}
