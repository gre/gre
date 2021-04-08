use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clone, Copy)]
struct Planet {
    mass: f64,
    position: (f64, f64),
}

#[derive(Clone, Copy)]
struct VLine {
    route: Vec<(f64, f64)>,
    direction: (f64,f64)
}

impl VLine {
    fn new(start: (f64, f64)) -> Self {
        let mut route = Vec::new();
        route.push(start);
        VLine { route }
    }
    fn go(self: &Self, p: (f64, f64)) -> Self {
        let mut route = self.route.clone();
        route.push(p);
        VLine { route }
    }
}


fn art(_seed: f64) -> Vec<Group> {
    let mut groups = Vec::new();

    let mut data = Data::new();

    data = data.move_to((0., 0.));

    let color = "black";
    groups.push(layer(color).add(base_path(color, 0.2, data)));

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let groups = art(seed);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(1.0, (260.0, 190.0), "black"));
    svg::save("image.svg", &document).unwrap();
}
