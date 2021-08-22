use blockstyle::*;
use svg::node::element::*;

fn main() {
    let mut g = Group::new();
    let a = art(&Opts {
        seed: 0.0,
        opacity: 1.0,
        border: 2.0,
        padding: (4.0, 4.0),
        sdivisions: 200, // how much to split the width space
        lines: 60, // how much to split the height space
        sublines: 8, // for each line, how much do we make "sublines" to make it grow
        osc_amp: (0.001, 0.001),
        off: (0.02, 0.02),
        osc_freq: 80.0,
        margin: (0f64, 0f64),
        lines_axis: vec![],
        mirror_axis: vec![false],
        line_dir: 0.0, // 0 to 1
        mirror_axis_weight: 1.0,
        lower: -0.1,
        upper: 0.8,
        lowstep: -0.3,
        highstep: 0.5,
        rotation: 0.0,// PI / 4.,
        m: 4.0,
        k: 4.0,
        k1: 1.0,
        k2: 1.0,
        k3: 1.0,
        k4: 1.0,
        k5: 2.0,
        k6: 2.0,
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