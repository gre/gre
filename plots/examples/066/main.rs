use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(_seed: f64) -> Vec<Group> {
    let mut groups = Vec::new();

    let mut data = Data::new();

    let padx = 5.0;
    let pady = 20.0;
    let width = 200.0;
    let height = 250.0;
    let amp = 3.;
    let samples = 1000;
    let threshold = 0.7;

    let get_color =
        image_get_color("images/interchange.jpg").unwrap();

    let lines = 60;
    for l in 0..lines {
        let ry = l as f64 / (lines as f64);
        let base_y = pady + height * ry;
        let mut pts = Vec::new();
        for s in 0..samples {
            let rx = s as f64 / (samples as f64);
            let clr = get_color((rx, ry));
            let v = amp
                * smoothstep(
                    threshold,
                    0.0,
                    grayscale(clr),
                );
            let x = padx + width * rx;
            let y = base_y
                + v * (2. * PI * rx * width / 2.).cos();
            pts.push((x, y));
        }

        let should_draw_line =
            |from: (f64, f64), to: (f64, f64)| {
                let x = (from.0 + to.0) / 2.;
                let y = (from.1 + to.1) / 2.;
                let rx = (x - padx) / width;
                let ry = (y - pady) / height;
                let c = get_color((rx, ry));
                let g = grayscale(c);
                g < threshold
            };
        data =
            render_route_when(data, pts, &should_draw_line);
    }

    let color = "red";
    groups.push(
        layer(color).add(base_path(color, 0.5, data)),
    );

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let groups = art(seed);
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (180.0, 270.0),
        "black",
    ));
    svg::save("image.svg", &document).unwrap();
}
