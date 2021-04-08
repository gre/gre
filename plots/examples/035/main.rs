use gre::*;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|str| str.parse::<f64>().ok())
        .unwrap_or(0.0);

    let resolution = 800;
    let samples = 10000;
    let group_min_size = 30;
    let group_samples = 400;
    let proximity_threshold = 0.06;
    let noise_frequency1 = 11.;
    let noise_amp1 = 0.6;
    let noise_frequency2 = 2.;
    let noise_amp2 = 0.5;
    let noise_threshold = 0.2;

    let mut rng = SmallRng::from_seed([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let perlin = Perlin::new();

    let mut d = Data::new();

    let mut candidates = Vec::new();
    let ratio = 420. / 297.;
    for x in 0..resolution {
        for y in 0..resolution {
            let xp = ratio * (x as f64) / (resolution as f64);
            let yp = (y as f64) / (resolution as f64);
            let v = noise_amp1 * perlin.get([noise_frequency1 * xp, noise_frequency1 * yp, seed])
                + noise_amp2
                    * perlin.get([
                        2.0 * noise_frequency2 * xp,
                        2.0 * noise_frequency2 * yp,
                        1.0 + seed,
                    ]);
            if v > noise_threshold {
                candidates.push((xp, yp));
            }
        }
    }
    rng.shuffle(&mut candidates);
    candidates.truncate(samples);

    let groups = group_by_proximity(candidates.clone(), proximity_threshold);

    for mut group in groups {
        if group.len() < group_min_size {
            continue;
        }
        group.truncate(group_samples);

        let pts = route_by_spiral(group);

        for (i, (x, y)) in pts.iter().enumerate() {
            let p = (10.0 + x * 280., 10.0 + y * 280.);
            if i == 0 {
                d = d.move_to(p);
            } else {
                d = d.line_to(p);
            }
        }
    }

    let art = Path::new()
        .set("stroke-width", 0.2)
        .set("stroke", "#000")
        .set("fill", "none")
        .set("d", d);

    // Make svg
    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 420, 297))
        .set("height", "297mm")
        .set("width", "420mm")
        .add(art)
        .add(signature(1.0, (380.0, 280.0), "black"));
    svg::save("image.svg", &document).unwrap();
}

fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    return (dx * dx + dy * dy).sqrt();
}

fn group_by_proximity(candidates: Vec<(f64, f64)>, threshold: f64) -> Vec<Vec<(f64, f64)>> {
    let mut groups: Vec<Vec<(f64, f64)>> = Vec::new();
    let list = candidates.clone();

    for item in list {
        let mut found = false;
        for group in &mut groups {
            let matches = group.iter().any(|p| euclidian_dist(*p, item) < threshold);
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

    println!("{}", groups.len());

    return groups;
}

fn route_by_spiral(candidates: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
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
        list = list.into_iter().filter(|&x| x != p).collect();

        let maybe_match = list.iter().min_by_key(|q| {
            let qp_angle = (p.1 - q.1).atan2(p.0 - q.0);
            // HACK!!! no Ord for f64 :(
            return (1000000.0 * ((2. * PI + qp_angle - a) % (2.0 * PI))) as i32;
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
