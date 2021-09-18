use gre::*;
use svg::node::element::*;

fn main() {
    let mut g = Group::new();
    let a = art(&Opts {
        seed: 10.,
        amp: 1.0,
        freq: 0.8
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