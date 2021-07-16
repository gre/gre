use noise::*;
use clap::Clap;
use gre::*;
use svg::node::element::*;
use svg::node::element::path::Data;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.0")]
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
    let f = |(x, y): (f64, f64)| {
        let p = (x, y);
        let c = get_color(p);
        smoothstep(0.0, 1.0, grayscale(c))
    };
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
            let l = f(
                ((x - circle.x + circle.r) / (2. * circle.r), (base_y - circle.y + circle.r) / (2. * circle.r))
            );
            let mut y = base_y;
            y += 8.0 * l *
                perlin.get([
                    0.02 * x,
                    0.6 * y,
                    seed + 2. * l * perlin.get([
                        0.2 * y,
                        0.1 * x + 0.5 * perlin.get([
                            0.5 * y,
                            0.2 * x,
                            100. + 7.3 * seed
                        ]),
                        10. + 0.3 * seed
                    ])
                ]);
            y += 0.2 * (1. - l) * perlin.get([
                    seed,
                    0.03 * x,
                    0.02 * base_y,
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
                        if line % 2 == 0 {
                            route.reverse();
                        }
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
    (vec![c.clone()], waves)
}

fn art(opts: Opts) -> Vec<Group> {
    let width = 297.0;
    let height = 210.0;
    let pad = 10.0;
    let stroke_width = 0.35;

    let circle = VCircle::new(width/2.0, height/2.0, height / 2.0 - pad);
    let (circles, routes) = waveball(opts, &circle);

    let mut layers = Vec::new();
    let colors = vec!["black", "black"];
    for (ci, &color) in colors.iter().enumerate() {
        let mut l = layer(color);
        let mut data = Data::new();
        for (i, r) in routes.iter().enumerate() {
            let route = r.clone();
            if i % 2 == ci {
                data = render_route(data, route);
            }
        }
        l = l.add(base_path(color, stroke_width, data));
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



// Reusing code from plots 009 !!!


///// raymarching a "Signed Distance Function" ///// (see http://jamie-wong.com/2016/07/15/ray-marching-signed-distance-functions/)
// This implements a raymarcher, similar to the one used at https://greweb.me/shaderday/56

// this is the "main" coloring function. for a given uv, returns a color.
fn get_color(uv: Vec2) -> Vec3 {
    let (x, y) = uv;
    // raymarching
    let origin = (0.0, 0.0, -3.0);
    let dir = normalize3((x - 0.5, y - 0.5, 1.0));
    let mut t = 0.0;
    let mut hit = 99.0;
    for _i in 0..100 {
        let h = map(add3(origin, mul3f(dir, t)));
        t += h;
        if h.abs() < 0.001 {
            hit = h;
            break;
        }
    }
    let p = add3(origin, mul3f(dir, t));
    let n = normal(p);
    return lighting(hit, p, n, dir);
}

// this is our "3D scene" distance function:
// for a given point in space, tells the distance to closest object
fn map(mut p: Vec3) -> f64 {
    // x axis rotation
    let r = rot2((p.1, p.2), 0.8);
    p = (p.0, r.0, r.1);
    // y axis rotation
    let r = rot2((p.0, p.2), 0.8);
    p = (r.0, p.1, r.1);
    let k = 0.4;
    let d = 1.0;
    f_op_union_round(
        f_box(p, (0.5, 0.5, 0.5)),
        f_op_union_round(
            f_sphere(add3(p, (0.0, d, 0.0)), 0.3),
            f_op_union_round(
                f_sphere(add3(p, (0.0, 0.0, d)), 0.3),
                f_sphere(add3(p, (d, 0.0, 0.0)), 0.3),
                k
            ),
            k
        ),
    k)
}

// distance to a sphere
fn f_sphere(p: Vec3, r: f64) -> f64 {
    length3(p) - r
}

// distance to a box
fn f_box(p: Vec3, b: Vec3) -> f64 {
    let d = add3(abs3(p), neg3(b));
    return length3(max3(d, (0.0, 0.0, 0.0))) + vmax3(min3(d, (0.0, 0.0, 0.0)));
}

// apply a rotation on 2d
fn rot2(p: Vec2, a: f64) -> Vec2 {
    add2(mul2f(p, (a).cos()), mul2f((p.1, -p.0), (a).sin()))
}

// this implements lighting of the 3D scene. 2 lights here.
fn lighting(_hit: f64, p: Vec3, n: Vec3, _dir: Vec3) -> Vec3 {
    let mut c = 0.0;
    let ldir = (-1.0, 1.0, -2.0);
    c += 0.1 + diffuse(p, n, ldir);
    let ldir = (1.0, 0.0, -1.0);
    c += 0.4 * (0.1 + diffuse(p, n, ldir));
    c = clamp(c, 0.0, 1.0);
    return (c, c, c);
}

// a bunch of vectors helpers (in future, I need a library =D)
type Vec2 = (f64, f64);
type Vec3 = (f64, f64, f64);
fn length3((x, y, z): Vec3) -> f64 {
    (x * x + y * y + z * z).sqrt()
}
fn normalize3(p: Vec3) -> Vec3 {
    let l = length3(p);
    return (p.0 / l, p.1 / l, p.2 / l);
}
fn add2(a: Vec2, b: Vec2) -> Vec2 {
    (a.0 + b.0, a.1 + b.1)
}
fn add3(a: Vec3, b: Vec3) -> Vec3 {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}
fn neg3(a: Vec3) -> Vec3 {
    (-a.0, -a.1, -a.2)
}
fn mul3f(a: Vec3, f: f64) -> Vec3 {
    (a.0 * f, a.1 * f, a.2 * f)
}
fn mul2f(a: Vec2, f: f64) -> Vec2 {
    (a.0 * f, a.1 * f)
}
fn normal(p: Vec3) -> Vec3 {
    return normalize3((
        map(add3(p, (0.0005, 0.0, 0.0))) - map(add3(p, (-0.0005, 0.0, 0.0))),
        map(add3(p, (0.0, 0.0005, 0.0))) - map(add3(p, (0.0, -0.0005, 0.0))),
        map(add3(p, (0.0, 0.0, 0.0005))) - map(add3(p, (0.0, 0.0, -0.0005))),
    ));
}
fn clamp(a: f64, from: f64, to: f64) -> f64 {
    (a).max(from).min(to)
}
fn dot3(a: Vec3, b: Vec3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}
fn abs3(a: Vec3) -> Vec3 {
    (a.0.abs(), a.1.abs(), a.2.abs())
}
fn diffuse(p: Vec3, n: Vec3, lpos: Vec3) -> f64 {
    let l = normalize3(add3(lpos, neg3(p)));
    let dif = clamp(dot3(n, l), 0.01, 1.);
    return dif;
}
fn vmax3(v: Vec3) -> f64 {
    (v.0).max(v.1).max(v.2)
}
fn min3(a: Vec3, b: Vec3) -> Vec3 {
    (a.0.min(b.0), a.1.min(b.1), a.2.min(b.2))
}
fn max3(a: Vec3, b: Vec3) -> Vec3 {
    (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2))
}
