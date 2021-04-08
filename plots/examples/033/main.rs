use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;
use time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default = &String::from("images/bird.png");
    let path = args.get(1).unwrap_or(default);
    let seconds = args.get(2).and_then(|s| s.parse::<i64>().ok()).unwrap_or(5);
    let count = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(3000);
    let get_color = image_get_color(path).unwrap();
    let mut rng = SmallRng::from_seed([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    // select all pixels that are suitable to drawing
    let mut candidates = Vec::new();
    let dim = 200;
    for x in 0..dim {
        for y in 0..dim {
            let p = ((x as f64) / (dim as f64), (y as f64) / (dim as f64));
            let c = get_color(p);
            let g = grayscale(c);
            if g < 0.5 {
                candidates.push(p);
            }
        }
    }

    // pick <count> random samples out
    rng.shuffle(&mut candidates);
    candidates.truncate(count);

    // we find the best route to connect the points
    let tour = travelling_salesman::hill_climbing::solve(&candidates, Duration::seconds(seconds));

    // and we plot it
    let mut data = Data::new();
    let mut first = true;
    for i in tour.route {
        let point = candidates[i];
        let p = (250.0 * point.0 + 20.0, 250.0 * point.1 - 20.0);
        if first {
            first = false;
            data = data.move_to(p);
        } else {
            data = data.line_to(p);
        }
    }

    // Make svg
    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: black")
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(
            Path::new()
                .set("fill", "none")
                .set("stroke", "white")
                .set("stroke-width", 0.2)
                .set("d", data),
        )
        .add(signature(1.0, (260.0, 190.0), "white"));
    svg::save("tsp.svg", &document).unwrap();
}
