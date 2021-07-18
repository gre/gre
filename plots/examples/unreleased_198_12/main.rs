use gre::*;
use svg::node::element::{Group, path::Data};

fn art() -> Vec<Group> {
    let width = 297.0;
    let height = 210.0;
    let pad = 30.0;
    let stroke_width = 0.35;
    let digits_count = 1000;
    let digits = pi::pi(digits_count);

    let route = digits.chars().enumerate().filter_map(|(i, c)| {
        c.to_digit(10).map(|n| 
            (
                pad + (width - 2. * pad) * i as f64 / (digits_count as f64),
                pad + (height - 2. * pad) * n as f64 / 9f64
            ))
    }).collect();

    let mut layers = Vec::new();
    let color = "black";
    let mut l = layer(color);
    l = l.add(signature(
        0.8,
        (245.0, 165.0),
        color,
    ));
    let mut data = Data::new();
    data = render_route_curve(data, route);
    l = l.add(base_path(color, stroke_width, data));
    layers.push(l);

    layers
    
}

fn main() {
    let groups = art();
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
