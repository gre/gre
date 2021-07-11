use clap::Clap;
use gre::*;
use svg::node::element::*;
use svg::node::element::path::Data;

#[derive(Clap)]
#[clap()]
struct Opts {
}

fn art(_opts: Opts) -> Vec<Group> {
    let width = 300.0;
    let height = 240.0;
    let pad = 10.0;
    let stroke_width = 0.35;
    let digits = pi::pi(1000);
    let mut route = Vec::new();
    let mut i = 0;
    let mut x = 0;
    for c in digits.chars() {
        if let Some(n) = c.to_digit(10) {
            if i % 2 == 0 {
                x = n;
            }
            else {
                let y = n;
                let p = pad + i as f64 / 10.0;
                let w = width - 2. * p;
                let h = height - 2. * p;
                route.push((
                    p + w * (x as f64 / 9.0),
                    p + h * (y as f64 / 9.0)
                ));
            }
            i += 1;
        }
    }
    
    let mut layers = Vec::new();

    let mut data = Data::new();
    data = render_route(data, route);
    let color = "black";
    let mut l = layer(color);
    
    l = l.add(signature(
        2.0,
        (172.0, 210.0),
        color,
    ));
    l = l.add(base_path(color, stroke_width, data));
    layers.push(l);
    
    layers
    
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_24x30_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
