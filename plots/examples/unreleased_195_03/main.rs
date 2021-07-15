use noise::*;
use clap::Clap;
use gre::*;
use svg::node::element::*;
use svg::node::element::path::Data;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "206.0")]
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
    opts: Opts,
    circle: &VCircle,
) -> (Vec<Vec<(f64, f64)>>, Vec<f64>) {
    let seed = opts.seed;
    let mut routes = Vec::new();
    let mut base_y = circle.y + 2. * circle.r;
    let perlin = Perlin::new();
    let mut passage = Passage2DCounter::new(0.4, circle.r * 2.0, circle.r * 2.0);
    let passage_limit = 10;
    let mut height_map: Vec<f64> = Vec::new();
    let mut line = 0;
    loop {
        if base_y < circle.y - circle.r - 10.0 {
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
            let amp = 4.0 + (line % 4) as f64 * 20.0;
            let freq = 1.0 / amp;
            let y = base_y + amp * perlin.get([
                freq * x,
                freq * base_y,
                0.3 * seed + 0.8 * perlin.get([
                    0.7 * seed + 0.4 * perlin.get([ 6. * freq * x, seed, 8. * freq * base_y ]),
                    2. * freq * x,
                    2. * freq * base_y,
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
        if line % 2 == 0 {
            route.reverse();
        }
        routes.push(route);
        base_y -= 0.3;
        line += 1;
    }
    (routes, height_map)
}

type WaveballRes = (Vec<VCircle>, Vec<Vec<(f64, f64)>>);

fn waveball(opts: Opts, c: &VCircle) -> WaveballRes {
    let (waves, _height_map) = waves_in_circle(opts, c);
    (vec![c.clone(),c.clone(),c.clone()], waves)
}

fn art(opts: Opts) -> Vec<Group> {
    let width = 297.0;
    let height = 210.0;
    let pad = 10.0;
    let stroke_width = 0.3;

    let circle = VCircle::new(width/2.0, height/2.0, height / 2.0 - pad);
    let (circles, routes) = waveball(opts, &circle);

    let mut layers = Vec::new();
    let colors = vec!["black"];
    for (ci, &color) in colors.iter().enumerate() {
        let mut l = layer(color);
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
            l = l.add(signature(
                0.8,
                (178.0, 194.0),
                color,
            ));
        }
        let mut data = Data::new();
        for r in routes.clone() {
            data = render_route(data, r);
        }
        l = l.add(base_path(color, stroke_width, data));
        layers.push(l);
    }
    
    layers
    
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
