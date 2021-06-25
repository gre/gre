use clap::Clap;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;


#[derive(Clap)]
#[clap()]
struct Opts {
}

pub fn fill<F: FnMut((f64, f64)) -> f64>(
    data: Data,
    bounds: (f64, f64, f64, f64),
    increment: (f64,f64),
    mut f: F,
    threshold: f64,
    threshold_dir: bool
) -> Data {
    let mut d = data;
    let mut p = (0.0, 0.0);
    let min_line_threshold = 1.0;
    loop {
        let mut q = p;
        let incr2 = (0.05 * increment.1, 0.05 * increment.0); // heuristic as it won't work with all kind of angles
        let mut r = Vec::new();
        loop {
            let next = (q.0 + incr2.0, q.1 + incr2.1);
            let v = f(normalize_in_boundaries(q, bounds));
            let write = threshold_dir && v < threshold || !threshold_dir && v > threshold;
            if write && r.len() == 0 || !write && r.len() == 1 {
                r.push(q);
            }
            if r.len() == 2 {
                if euclidian_dist(r[0], r[1]) > min_line_threshold {
                    d = render_route(d, r);
                }
                r = Vec::new();
            }
            q = next;
            if out_of_boundaries(q, bounds) {
                if r.len() == 1 {
                    r.push(q);
                }
                break;
            }
        }

        if r.len() == 2 && euclidian_dist(r[0], r[1]) > min_line_threshold {
            d = render_route(d, r);
        }

        p = (p.0 + increment.0, p.1 + increment.1);
        if out_of_boundaries(p, bounds) {
            break;
        }
    }
    d
}


fn shape (path: &str, inside: bool, i: usize) -> Path {
    let (w, h) = (100, 100);
    let bounds = (0.0, 0.0, w as f64, h as f64);
    let precision = 0.5;
    let width = (w as f64 / precision) as u32;
    let height = (h as f64 / precision) as u32;
    let perlin = Perlin::new();
    let get_color = image_get_color(path).unwrap();
    let threshold = 0.5;
    let f = |origin: (f64, f64)| {
      grayscale(get_color(origin))
    };
    let mut sum = 0.0;
    for x in 0..10 {
        for y in 0..10 {
            sum += f((x as f64/10., y as f64/10.));
        }
    }
    sum /= 100.0;
    let res = contour(width, height, f, &vec![0.5 * sum, 0.9 * sum]);
    let mut routes = features_to_routes(res, precision);
    routes = crop_routes(&routes, bounds);
    let mut data = Data::new();
    let thresholds = (0.15 * sum, 0.5 * sum, 0.9 * sum);
    data = fill(data, bounds, (0.0, 0.7), f, thresholds.0, inside);
    data = fill(data, bounds, (0.7, 0.0), f, thresholds.0, inside);
    data = fill(data, bounds, (1.2, 0.0), f, thresholds.1, inside);
    data = fill(data, bounds, (0.0, 1.2), f, thresholds.1, inside);
    data = fill(data, bounds, (0.0, 2.), f, thresholds.2, inside);
    data = fill(data, bounds, (2., 0.0), f, thresholds.2, inside);
    if !inside {
        for _i in 0..3 {
            data = render_route(data, boundaries_route(bounds));
        }
    }
    for route in routes {
        data = render_route(data, route);
    }
    base_path("black", 0.35, data)
}

fn art(_opts: &Opts) -> Vec<Group> {
    vec![
        "/Users/grenaudeau/Desktop/avatars/3/2.jpeg",
        "/Users/grenaudeau/Desktop/avatars/3/3.png",
    ].iter().enumerate().map(|(i, path)| {
        let x = i as f64 * 120.0 + 40.;
        let y = 70.;
        Group::new().set("transform", format!("translate({},{})", x, y)).add(shape(path, true, i))
    }).collect()
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(&opts);
    let mut document = base_24x30_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
