use clap::Clap;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "827.0")]
    seed: f64,
    #[clap(short, long, default_value = "3.0")]
    index: f64,
    #[clap(short, long, default_value = "8.0")]
    frames: f64,
}

fn art(opts: Opts) -> Vec<Group> {
    let p = opts.index / opts.frames;
    let height = 210f64;
    let width = 297f64;
    let offy = 42.;
    let h = height - 80. - offy;
    let w = width - 40.;
    let granularity = 2f64;
    let max_count = 3000;
    let freq = 0.0022;
    let divergence = 0.0001;
    let mut rng = rng_from_seed(opts.seed);
    let perlin = Perlin::new();
    let passage_gran = 0.5;
    let mut passage = Passage2DCounter::new(passage_gran, width, height);
    let max_passage = 3;
    let mut should_draw_line = |a: (f64,f64), b: (f64,f64)| {
        let m = (mix(a.0, b.0, 0.5), mix(a.1, b.1, 0.5));
        passage.count(m) < max_passage
    };

    let sx = (width - w) / 2.;
    let sy = offy + (height - h) / 2.;
    let bounds = (20., 20., width-20., height-20.);

    let f = |t: f64| {
        // quad
        let a = if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        };

        return a * 0.3 + t * 0.7;
    };
    let amp = |p| {
        1. + 100.0
            * (1.0
                - 2. * euclidian_dist(
                    (width / 2., height / 2.),
                    p,
                ) / width)
                .max(0.0)
                .powf(2.)
    };

    let candidates: Vec<Vec<(f64, f64)>> = (0..max_count)
        .map(|i| {
            let mut route = Vec::new();
            let s1 = f(rng.gen_range(0., 1.0));
            let s2 = 1.3 * f(rng.gen_range(0.0, 1.0)) - 0.3;
            let l1 = rng.gen_range(0.4, 0.8);
            let x = sx + w * s2;
            let y = sy + h * s1;
            let len = w * l1;
            let x_from = x.max(sx);
            let x_to = (x + len).min(sx + w);
            let mut xp = x_from;
            loop {
                let xx = xp;
                let a = freq * xx;
                let b = freq * y + 2. * perlin.get([
                    2. * freq * xx + 1.5 *
                        perlin.get([
                            4. * freq * xx,
                            4. * freq * y,
                            10.33 + opts.seed
                        ]),
                    2. * freq * y + 2. *
                    perlin.get([
                        3. * freq * xx,
                        3. * freq * y,
                        8.13 + opts.seed
                    ]),
                    100.45 + opts.seed
                ]).abs();
                let n1 = perlin.get([ a, b, opts.seed + i as f64 * divergence + 200.2 ]);
                let n2 = perlin.get([ a, b, opts.seed + i as f64 * divergence + 300.514 ]);
                let n3 = perlin.get([ a, b, opts.seed + i as f64 * divergence + 400.31 ]);
                let n = 0.8 * 
                    (
                        n1 * (2. * PI * p).cos() +
                        n2 * (2. * PI * (p + 0.33)).cos() +
                        n3 * (2. * PI * (p + 0.66)).cos()
                    ) +
                    // global disp
                    0.2 * perlin.get([
                        freq * xp,
                        freq * y,
                        opts.seed
                    ]);
                let yp = y + amp((xp, y)) * n;
                route.push((xp, yp));
                let old = xp;
                xp = (xp + granularity).min(x_to);
                if xp-old < 0.0001 {
                    break;
                }
            }
            route
        })
        .filter(|r| r.len() >= 2)
        .collect();

    let counts = [800, 500];
    let colors = vec!["firebrick", "darkturquoise"];
    
    let mut layers: Vec<Group> =
    colors
        .iter()
        .enumerate()
        .map(|(g, color)| {
            let mut l = layer(color);
            let count = counts[g];
            let mut routes = candidates.clone();
            rng.shuffle(&mut routes);
            routes.truncate(count);
            let data = routes.iter().fold(
                Data::new(),
                |data, route| {
                    render_route_when(data, route.clone(), &mut should_draw_line)
                },
            );
            l = l.add(base_path(color, 0.3, data));
            if g == 0 {
                l = l.add(signature(
                    1.0,
                    (250.0, 192.0),
                    color,
                ));
            }
            l
        })
        .collect();


    let sunphase = p - 0.2;
    let sunp = (w / 2. + 75. * (2. * PI * sunphase).cos(), 50. - 30. * (2. * PI * sunphase).sin().abs());
    let sunr = 16.;
    let rays = 600;
    let sun_routes: Vec<Vec<(f64,f64)>> = (0..rays).filter_map(|i| {
        let a = 2. * PI * (p + (i as f64) / (rays as f64));
        let mut extra = 0.;
        if i % 3 != 0 {
            extra += 8. + ((i * 29) % 121) as f64;
        }
        let r = sunr + extra;
        let from = (sunp.0 + r * a.cos(), sunp.1 + r * a.sin());
        if !strictly_in_boundaries(from, bounds) {
            return None;
        }
        let mut to = (sunp.0 + 300. * a.cos(), sunp.1 + 300. * a.sin());
        if let Some(p) = collide_segment_boundaries(from, to, bounds) {
            to = p;
        }
        let mut i = 0.0;
        let len = euclidian_dist(from, to);
        loop {
            i += passage_gran;
            let m = i / len;
            let p = (mix(from.0, to.0, m), mix(from.1, to.1, m));
            if passage.get(p) > 0 {
                to = p;
                break;
            }
            if i > len {
                break;
            }
        }
        return Some(vec![from, to]);
    }).collect();
    let mut sun_layer = layer("orange");
    let data = sun_routes.iter().fold(
        Data::new(),
        |data, route| {
            render_route(data, route.clone())
        },
    );
    sun_layer = sun_layer.add(base_path("orange", 0.3, data));
    layers.push(sun_layer);

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
