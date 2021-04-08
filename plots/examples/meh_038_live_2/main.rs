use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;
use time::Duration;

fn main() {
    let mut layers = Vec::new();

    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(0);

    let mut rng = SmallRng::from_seed([
        seed, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    let get_color =
        image_get_color("images/dragoon.jpg").unwrap();

    let mut data = Data::new();

    let resolution = 500;
    let g = 10;
    for i in 0..g {
        let mut candidates = Vec::new();
        for x in 0..resolution {
            for y in 0..resolution {
                let xp = (x as f64) / (resolution as f64);
                let yp = (y as f64) / (resolution as f64);
                let color = get_color((xp, yp));
                let g = grayscale(color);
                if g < 0.4
                    * ((0.5 + (i as f64)) / (g as f64))
                {
                    candidates.push((xp, yp));
                }
            }
        }
        rng.shuffle(&mut candidates);
        candidates.truncate(600 - 50 * i);

        let groups = group_by_proximity(candidates, 0.05);

        for g in groups {
            if g.len() < 5 {
                continue;
            }
            let mut candidates = g;

            if i % 3 == 0 {
                candidates = route_by_spiral(candidates);
            }
            if i % 3 == 1 {
            } else {
                let tour =
                    travelling_salesman::hill_climbing::solve(&candidates, Duration::seconds(1));
                candidates = tour
                    .route
                    .iter()
                    .map(|&i| candidates[i])
                    .collect();
            }

            for (i, (x, y)) in candidates.iter().enumerate()
            {
                let p = (10.0 + x * 190., 10.0 + y * 190.);
                if i == 0 {
                    data = data.move_to(p);
                } else {
                    data = data.line_to(p);
                }
            }
        }
    }

    let color = "black";
    layers.push(
        layer(color).add(
            Path::new()
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 0.2)
                .set("d", data),
        ),
    );

    // Make svg
    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(signature(1.0, (260.0, 190.0), "black"));
    for g in layers {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}

fn euclidian_dist(
    (x1, y1): (f64, f64),
    (x2, y2): (f64, f64),
) -> f64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    return (dx * dx + dy * dy).sqrt();
}

fn group_by_proximity(
    candidates: Vec<(f64, f64)>,
    threshold: f64,
) -> Vec<Vec<(f64, f64)>> {
    let mut groups: Vec<Vec<(f64, f64)>> = Vec::new();
    let list = candidates.clone();

    for item in list {
        let mut found = false;
        for group in &mut groups {
            let matches = group.iter().any(|p| {
                euclidian_dist(*p, item) < threshold
            });
            if matches {
                found = true;
                group.push(item);
                break;
            }
        }
        if !found {
            let group = vec![item];
            groups.push(group);
        }
    }

    return groups;
}

fn route_by_spiral(
    candidates: Vec<(f64, f64)>,
) -> Vec<(f64, f64)> {
    if candidates.len() == 0 {
        return candidates;
    }
    let mut result = Vec::new();
    let mut list = candidates.clone();
    let mut p = *(candidates
        .iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap());
    let mut a = 0.0;
    result.push(p);
    loop {
        list =
            list.into_iter().filter(|&x| x != p).collect();

        let maybe_match = list.iter().min_by_key(|q| {
            let qp_angle = (p.1 - q.1).atan2(p.0 - q.0);
            // HACK!!! no Ord for f64 :(
            return (1000000.0
                * ((2. * PI + qp_angle - a) % (2.0 * PI)))
                as i32;
        });
        if let Some(new_p) = maybe_match {
            a = (p.1 - new_p.1).atan2(p.0 - new_p.0);
            p = *new_p;
            result.push(p);
        } else {
            break;
        }
    }
    return result;
}
