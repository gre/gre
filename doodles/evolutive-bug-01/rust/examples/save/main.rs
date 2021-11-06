use gre::*;
use svg::node::element::*;

fn main() {
    let mut g = Group::new();
    let args: Vec<String> = std::env::args().collect();
    let a = art(&Opts {
        seed: args
            .get(1)
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(8.0),
        v1: 0.,
        v2: 0.,
        v3: 0.,
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