use blockstyle::*;
use svg::node::element::*;

fn main() {
    let mut g = Group::new();
    let a = art(&Opts {
        seed: 100.0,
        opacity: 1.0,
        opacity_fade: 0.0,
        border: 6.0,
        padding: (10.0, 10.0),
        margin: (20f64, 20f64),
        sdivisions: 200,
        lines: 40,
        sublines: 8,
        osc_amp: (0.001, 0.001),
        off: (0.02, 0.0),
        osc_freq: 80.0,
        lines_axis: vec![true],
        mirror_axis: vec![false],
        line_dir: 0.0,
        mirror_axis_weight: 1.0,
        lower: -0.05,
        upper: 1.0,
        lowstep: -0.3,
        highstep: 0.5,
        rotation: 0.0,
        m: 4.0,
        k: 4.0,
        k1: 1.0,
        k2: 3.0,
        k3: 1.0,
        k4: 1.0,
        second_color_div: 0,
        border_cross: String::from("-|/\\"),
        radius_amp: 0.0,
        radius_freq: 0.0,
        radius_offset: 0.0,
    });
    for e in a {
        g = g.add(e);
    }
    let doc = svg::Document::new()
    .set("viewBox", (0, 0, 200, 200))
    .set("width", "200mm")
    .set("height", "200mm")
    .set("style", "background:white")
    .add(g);
    svg::save("image.svg", &doc).unwrap();
}