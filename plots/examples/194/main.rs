use noise::*;
use clap::Clap;
use gre::*;
use rand::Rng;
use svg::node::element::*;
use svg::node::element::path::Data;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "800.")]
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
    fn includes(self: &Self, p: (f64, f64)) -> bool {
        euclidian_dist((self.x,self.y), p) < self.r
    }
}

fn waves_in_circle(
    seed: f64,
    circle: &VCircle
) -> (Vec<Vec<(f64, f64)>>, Vec<f64>) {
    let mut routes = Vec::new();
    let mut base_y = circle.y + 2. * circle.r;
    let perlin = Perlin::new();
    let mut passage = Passage2DCounter::new(0.4, circle.r * 2.0, circle.r * 2.0);
    let passage_limit = 8;
    let mut height_map: Vec<f64> = Vec::new();
    let mut rng = rng_from_seed(seed);
    let a = rng.gen_range(0.0, 0.8);
    let b = rng.gen_range(0.2, 2.8);
    let c = rng.gen_range(0.0, 0.2);
    let d = rng.gen_range(0.0, 0.01);
    let e = rng.gen_range(0.01, 0.2);
    let f = rng.gen_range(0.0, 1.0);
    loop {
        if base_y < circle.y - circle.r - 10.0 {
            break;
        }

        if perlin.get([ seed, 2. * base_y ]) < 0.0 {
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
                    0.5 * d * x,
                    d * base_y,
                    seed + b * perlin.get([
                        0.5 * e * base_y,
                        e * x,
                        10. + 0.3 * seed + c * perlin.get([
                            f * base_y,
                            0.5 * f * x,
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
        }

        base_y -= 0.2;
    }
    (routes, height_map)
}

type WaveballRes = (Vec<VCircle>, Vec<Vec<(f64, f64)>>);

fn waveball(seed: f64, c: &VCircle) -> WaveballRes {
    let (waves, _height_map) = waves_in_circle(seed, c);
    (vec![c.clone()], waves)
}

fn art(opts: Opts) -> Vec<Group> {
    let width = 300.0;
    let height = 240.0;
    let pad = 10.0;
    let stroke_width = 0.3;

    let circle = VCircle::new(width/2.0, height/2.0, height / 2.0 - pad);
    let (circles, routes) = waveball(opts.seed, &circle);

    let mut layers = Vec::new();
    let color = "black";
    let mut l = layer(color);
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
    let mut data = Data::new();
    for r in routes.clone() {
        data = render_route(data, r);
    }
    l = l.add(base_path(color, stroke_width, data));
    l = l.add(signature(
        0.8,
        (185.0, 224.0),
        color,
    ));
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
