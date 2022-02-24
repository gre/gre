use noise::*;
use clap::Clap;
use gre::*;
use svg::node::element::*;
use svg::node::element::path::Data;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "image.svg")]
    file: String,
    #[clap(short, long, default_value = "0.0")]
    pub seed: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed1: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed2: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed3: f64,
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
    opts: &Opts,
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
            let amp = 6.0 + (line % 3) as f64 * 30.0 + opts.seed3;
            let freq = 1.0 / amp;
            let y = base_y + amp * perlin.get([
                freq * x,
                freq * base_y,
                0.3 * seed + 0.8 * perlin.get([
                    0.7 * seed + 1.4 * perlin.get([
                        6. * freq * x,
                        seed,
                        8. * freq * base_y
                    ]),
                    2. * freq * (x + opts.seed1),
                    2. * freq * (base_y + opts.seed2),
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
        base_y -= 0.2;
        line += 1;
    }
    (routes, height_map)
}

type WaveballRes = (Vec<VCircle>, Vec<Vec<(f64, f64)>>);

fn waveball(opts: &Opts, c: &VCircle) -> WaveballRes {
    let (waves, _height_map) = waves_in_circle(opts, c);
    (vec![c.clone(),c.clone(),c.clone()], waves)
}

fn art(opts: &Opts) -> Vec<Group> {
    let width = 210.0;
    let height = 297.0;
    let pad = 10.0;
    let stroke_width = 0.3;

    let circle = VCircle::new(width/2.0, height/2.0, width / 2.0 - pad);
    let (circles, routes) = waveball(opts, &circle);

    let mut layers = Vec::new();
    let colors = vec!["red", "cyan", "darkblue", "#F90", "black", "pink", "green", "purple"];
    let color = colors[(opts.seed1.abs() as usize) % colors.len()];
    let colors = vec![color];
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
    let groups = art(&opts);
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save(opts.file, &document).unwrap();
}
