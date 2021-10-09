use gre::*;
use svg::node::element::*;

fn main() {
    let mut g = Group::new();
    let a = art(&Opts {
        seed: 100.,
    });
    for e in a {
        g = g.add(e);
    }
    let doc = svg::Document::new()
    .set("viewBox", (0, 0, 200, 200))
    .set("width", "200mm")
    .set("height", "200mm")
    .set("style", "background:white")
    .set("xmlns:inkscape", "http://www.inkscape.org/namespaces/inkscape")
    .set("xmlns", "http://www.w3.org/2000/svg" )
    .add(g);
    svg::save("image.svg", &doc).unwrap();
}