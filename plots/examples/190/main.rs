use noise::*;
use rayon::prelude::*;
use clap::Clap;
use gre::*;
use rand::prelude::*;
use svg::node::element::*;
use svg::node::element::path::Data;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "50.0")]
    seed: f64,
}

#[derive(Clone, Copy, Debug)]
struct VCircle {
    x: f64,
    y: f64,
    r: f64,
}

impl VCircle {
    fn new(x: f64, y: f64, r: f64) -> Self {
        VCircle { x, y, r }
    }
    fn dist(self: &Self, c: &VCircle) -> f64 {
        euclidian_dist((self.x,self.y), (c.x, c.y)) - c.r - self.r
    }
    fn collides(self: &Self, c: &VCircle) -> bool {
        self.dist(c) <= 0.0
    }
    fn contains(self: &Self, c: &VCircle) -> bool {
        euclidian_dist((self.x,self.y), (c.x, c.y)) - self.r + c.r < 0.0
    }
    fn includes(self: &Self, p: (f64, f64)) -> bool {
        euclidian_dist((self.x,self.y), p) < self.r
    }
}

fn scaling_search<F: FnMut(f64) -> bool>(
    mut f: F,
    min_scale: f64,
    max_scale: f64,
) -> Option<f64> {
    let mut from = min_scale;
    let mut to = max_scale;
    loop {
        if !f(from) {
            return None;
        }
        if to - from < 0.1 {
            return Some(from);
        }
        let middle = (to + from) / 2.0;
        if !f(middle) {
            to = middle;
        }
        else {
            from = middle;
        }
    }
}

fn search_circle_radius(
    container: &VCircle,
    circles: &Vec<VCircle>,
    height_map: Vec<f64>,
    x: f64,
    y: f64,
    min_scale: f64,
    max_scale: f64,
) -> Option<f64> {
    let f = |size| {
        let c = VCircle::new(x, y, size);
        let factor = 2. * container.r / (height_map.len() as f64);
        let collides_height_map = height_map.iter().enumerate().any(|(i, &y)| {
            let x = container.x - container.r + i as f64 * factor;
            (c.x - x).abs() < c.r && y < c.y || c.includes((x, y))
        });
        !collides_height_map && container.contains(&c) && !circles.iter().any(|other| {
            c.collides(other)
        })
    };
    scaling_search(f, min_scale, max_scale)
}

fn packing(
    seed: f64,
    iterations: usize,
    desired_count: usize,
    pad: f64,
    container: &VCircle,
    height_map: &Vec<f64>,
    min_scale: f64,
    max_scale: f64,
) -> Vec<VCircle> {
    let mut circles = Vec::new();
    let mut rng = rng_from_seed(seed);
    let x1 = container.x - container.r;
    let y1 = container.y - container.r;
    let x2 = container.x + container.r;
    let y2 = container.y + container.r;
    let max_scale = max_scale.min(container.r);
    for _i in 0..iterations {
        let x: f64 = rng.gen_range(x1, x2);
        let y: f64 = rng.gen_range(y1, y2);
        if let Some(size) = search_circle_radius(&container, &circles, height_map.clone(), x, y, min_scale, max_scale) {
            let circle = VCircle::new(x, y, size - pad);
            circles.push(circle.clone());
        }
        if circles.len() > desired_count {
            break;
        }
    }
    circles
}

fn waves_in_circle(
    seed: f64,
    circle: &VCircle,
    sy: f64,
    dy: f64
) -> (Vec<Vec<(f64, f64)>>, Vec<f64>) {
    let offset_y = 3.0 + 0.8 * circle.r;
    let mut routes = Vec::new();
    let mut base_y = circle.y + circle.r + offset_y;
    let perlin = Perlin::new();
    let mut passage = Passage2DCounter::new(0.4, circle.r * 2.0, circle.r * 2.0);
    let passage_limit = 8;
    let mut height_map: Vec<f64> = Vec::new();
    loop {
        if base_y < circle.y + sy * circle.r {
            break;
        }
        let precision = 0.2;
        let mut route = Vec::new();
        let mut x = circle.x - circle.r;
        let mut was_outside = true;
        let mut i = 0;
        loop {
            if x > circle.x + circle.r {
                break;
            }
            let y = base_y + offset_y * perlin.get([
                0.01 * x,
                0.04 * base_y,
                seed + 0.3 * perlin.get([
                    0.2 * base_y + 0.2 * perlin.get([
                        0.12 * x,
                        0.3 * base_y,
                    ]),
                    0.05 * x,
                    10. + 0.3 * seed
                ])
            ]);
            let mut collides = false;
            if i >= height_map.len() {
                height_map.push(y);
            }
            else {
                if y > height_map[i] {
                    collides = true;
                }
                else {
                    height_map[i] = y;
                }
            }
            let inside = !collides &&
              circle.includes((x, y)) &&
              passage.count(( x - circle.x + circle.r, y - circle.y + circle.r )) < passage_limit;
            if inside {
                if was_outside {
                    if route.len() > 2 {
                        routes.push(route);
                    }
                    route = Vec::new();
                }
                was_outside = false;
                route.push((x, y));
            }
            else {
                was_outside = true;
            }
            x += precision;
            i += 1;
        }
        routes.push(route);

        base_y -= dy;
    }
    (routes, height_map)
}

type WaveballRes = (Vec<(usize, VCircle)>, Vec<(usize, Vec<(f64, f64)>)>);

fn waveball(n: usize, seed: f64, c: &VCircle) -> WaveballRes {
    if n > 3 {
        return (Vec::new(), Vec::new());
    }
    let (waves, height_map) = waves_in_circle(seed, c, 0.0, 0.3);
    
    let res = packing(seed, 100000, 10, 4.0 / (1. + n as f64), c, &height_map, 16.0 / (1. + 4. * n as f64), 100.0)
        .par_iter()
        .filter(|circle| circle.r > 2.0)
        .map(|circle| waveball(n + 1, seed + circle.x * 9. + circle.y / 29., circle))
        .collect::<Vec<_>>();

    let mut circles_acc = Vec::new();
    let mut routes_acc = Vec::new();
    circles_acc.push(vec![ (n, c.clone()) ]);
    routes_acc.push(waves.iter().map(|w| (n, w.clone())).collect::<Vec<_>>());
    for (circles, routes) in res {
        circles_acc.push(circles);
        routes_acc.push(routes);
    }
    let circles = circles_acc.concat();
    let routes = routes_acc.concat();
    (circles, routes)
}

fn art(opts: Opts) -> Vec<Group> {
    let width = 300.0;
    let height = 240.0;
    let pad = 10.0;
    let stroke_width = 0.3;

    let circle = VCircle::new(width/2.0, height/2.0, height / 2.0 - pad);
    let (circles, routes) = waveball(0, opts.seed, &circle);

    let mut layers = Vec::new();
    let colors = vec!["firebrick", "grey", "grey", "grey", "grey"];

    for (ci, &color) in colors.iter().enumerate() {
        let mut l = layer(color);
        for (i, c) in circles.clone() {
            if i != ci { continue; }
            l = l.add(
                Circle::new()
                .set("r", c.r)
                .set("cx", c.x)
                .set("cy", c.y)
                .set("stroke", color)
                .set(
                    "stroke-width",
                    stroke_width,
                )
                .set("fill", "none")
                .set("style", "mix-blend-mode: multiply;")
            );
        }
        let mut data = Data::new();
        for (i, r) in routes.clone() {
            if i != ci { continue; }
            data = render_route(data, r);
        }
        l = l.add(base_path(color, stroke_width, data));
        if ci == 0 {
            l = l.add(signature(
                0.8,
                (185.0, 224.0),
                color,
            ));
        }
        layers.push(l);
    }

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
