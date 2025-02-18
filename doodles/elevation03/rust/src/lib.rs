mod utils;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;
use wasm_bindgen::prelude::*;
use byteorder::*;
use rand::prelude::*;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Opts {
    pub seed: f64,
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
    fn includes(self: &Self, p: (f64, f64)) -> bool {
        euclidian_dist((self.x,self.y), p) < self.r
    }
}

fn waves_in_circle(
    seed: f64,
    circle: &VCircle
) -> (Vec<Vec<Vec<(f64, f64)>>>, Vec<f64>) {
    let mut routes_acc = Vec::new();
    let mut routes = Vec::new();
    let mut base_y = circle.y + 2. * circle.r;
    let perlin = Perlin::new();
    let mut passage = Passage2DCounter::new(0.4, circle.r * 2.0, circle.r * 2.0);
    let passage_limit = 8;
    let mut height_map: Vec<f64> = Vec::new();
    let mut rng = rng_from_seed(seed);
    let sky = rng.gen_range(0.0, 0.6);
    let a = rng.gen_range(0.2, 1.0);
    let b = rng.gen_range(0.0, 0.1);
    let d1 = rng.gen_range(0.02, 0.08);
    let d2 = rng.gen_range(0.06, 0.16);
    let e1 = rng.gen_range(0.6, 1.2);
    let e2 = rng.gen_range(0.1, 0.3);
    let c = rng.gen_range(0.2, 0.4);
    let f = rng.gen_range(0.5, 2.0);
    loop {
        if base_y < circle.y - circle.r - 10.0 {
            break;
        }

        if true {
        //if perlin.get([ seed, 3.0 * base_y ]) < 0.0 {
            let precision = 0.2;
            let mut route = Vec::new();
            let mut x = circle.x - circle.r;
            let mut was_outside = true;
            let mut i = 0;
            loop {
                if x > circle.x + circle.r {
                    break;
                }
                let y = base_y + a * (circle.r - euclidian_dist((circle.x, circle.y), (x, base_y))) * perlin.get([
                    d1 * x,
                    d2 * base_y,
                    seed + b * perlin.get([
                        e1 * base_y,
                        e2 * x,
                        10. + 0.3 * seed + c * perlin.get([
                            f * base_y,
                            f * x,
                            100. + 7.3 * seed
                        ])
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
            if routes_acc.len() == 0 && base_y < circle.y - sky * circle.r {
                routes_acc.push(routes);
                routes = Vec::new();
            }
        }

        base_y -= 0.4;
    }
    routes_acc.push(routes);

    (routes_acc, height_map)
}

type WaveballRes = (Vec<VCircle>, Vec<Vec<Vec<(f64, f64)>>>);

fn waveball(seed: f64, c: &VCircle) -> WaveballRes {
    let (waves, _height_map) = waves_in_circle(seed, c);
    (vec![c.clone()], waves)
}

pub fn art(opts: &Opts) -> Vec<Group> {
    let width = 200.0;
    let height = 200.0;
    let pad = 10.0;
    let stroke_width = 0.3;

    let circle = VCircle::new(width/2.0, height/2.0, height / 2.0 - pad);
    let (circles, routes_acc) = waveball(opts.seed, &circle);

    let mut layers = Vec::new();
    let colors = vec!["#0FF", "#F0F"];
    for (ci, &color) in colors.iter().enumerate() {
        let mut l = Group::new()
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", stroke_width);
        if ci == 0 {
            for c in circles.clone() {
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
        }
        let opacity: f64 = 0.6;
        
        for (i, routes) in routes_acc.iter().enumerate() {
            let opdiff = 0.15 / (routes.len() as f64);
            let mut trace = 0f64;
            if i % colors.len() == ci {
                for r in routes.clone() {
                    trace += 1f64;
                    if r.len() < 2 {
                        continue;
                    }
                    let data = render_route(Data::new(), r);
                    l = l.add(
                        Path::new()
                        .set("opacity", (1000. * (opacity - trace * opdiff)).floor()/1000.0)
                        .set("d", data)
                    );
                }
            }
        }
        layers.push(l);
    }
    
    layers    
}

#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
    let opts = val.into_serde().unwrap();

    let mut g = Group::new();
    let a = art(&opts);
    for e in a {
        g = g.add(e);
    }
    let str = g.to_string();
    return str;
}

#[inline]
fn significant_str (f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}

fn render_route(
    data: Data,
    route: Vec<(f64, f64)>
) -> Data {
    if route.len() == 0 {
        return data;
    }
    let first_p = route[0];
    let mut d = data.move_to((
        significant_str(first_p.0),
        significant_str(first_p.1)
    ));
    for p in route {
        d = d.line_to((
            significant_str(p.0),
            significant_str(p.1),
        ));
    }
    return d;
}

#[inline]
fn euclidian_dist(
    (x1, y1): (f64, f64),
    (x2, y2): (f64, f64),
) -> f64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    return (dx * dx + dy * dy).sqrt();
}

pub struct Passage2DCounter {
    granularity: f64,
    width: f64,
    height: f64,
    counters: Vec<usize>,
}
impl Passage2DCounter {
    pub fn new(
        granularity: f64,
        width: f64,
        height: f64,
    ) -> Self {
        let wi = (width / granularity).ceil() as usize;
        let hi = (height / granularity).ceil() as usize;
        let counters = vec![0; wi * hi];
        Passage2DCounter {
            granularity,
            width,
            height,
            counters,
        }
    }
    fn index(self: &Self, (x, y): (f64, f64)) -> usize {
        let wi =
            (self.width / self.granularity).ceil() as usize;
        let hi = (self.height / self.granularity).ceil()
            as usize;
        let xi = ((x / self.granularity).round() as usize)
            .max(0)
            .min(wi - 1);
        let yi = ((y / self.granularity).round() as usize)
            .max(0)
            .min(hi - 1);
        yi * wi + xi
    }
    pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
        let i = self.index(p);
        let v = self.counters[i] + 1;
        self.counters[i] = v;
        v
    }
    pub fn get(self: &Self, p: (f64, f64)) -> usize {
        self.counters[self.index(p)]
    }
}

pub fn rng_from_seed(s: f64) -> impl Rng {
    let mut bs = [0; 16];
    bs.as_mut().write_f64::<BigEndian>(s).unwrap();
    let mut rng = SmallRng::from_seed(bs);
    // run it a while to have better randomness
    for _i in 0..50 {
        rng.gen::<f64>();
    }
    return rng;
}