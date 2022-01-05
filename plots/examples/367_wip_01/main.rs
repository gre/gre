use clap::Clap;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let colors = vec!["black"];
    let perlin = Perlin::new();

    let displacement = |x, y| {
        0.2 * perlin.get([
            99. + 0.2 * opts.seed + perlin.get([
                0.08 * x,
                0.08 * y,
                opts.seed,
            ]),
            0.01 * x + perlin.get([
                0.06 * x,
                88. - 7.7 * opts.seed,
                0.2 * y
            ]),
            0.01 * y + perlin.get([
                0.2 * x,
                8.8 * opts.seed - 99.,
                0.06 * y
            ])
        ]) +
        0.5 * perlin.get([
            0.01303 * x,
            0.01737 * y  + perlin.get([
                0.08 * x,
                0.08 * y,
                7.7 * opts.seed,
            ]),
            8.88 + 3.3 * opts.seed
        ])
    };

    let incr = |x| {
        0.5 + 0.12 * perlin.get([
            0.171 * x,
            8.88 + 3.3 * opts.seed +
            0.2 * perlin.get([
                opts.seed,
                0.771 * x,
            ])
        ]) + 0.002 * x
    };

    let height = 297.;
    let width = 210.;
    let pad = 20.;
    let center = (width * 0.5, height * 0.26);
    let precision = 0.2;
    let overlap = 0.4;
    
    let mut routes = Vec::new();
    
    let mut reverse = false;
    let mut y = pad;
    loop {
        if y > height - pad {
            break;
        }
        let l = if y < center.1 {
            lerp(pad, center.1, y)
         }
         else {
             lerp(height - pad, center.1, y)
         };
        let xfrom = mix(pad, center.0, l) - overlap;
        let xto = mix(width - pad, center.0, l) + overlap;
        let mut x = if !reverse { xfrom } else { xto };
        let mut route = Vec::new();
        loop {
            if x < xfrom || x > xto {
                break;
            }
            let dy = displacement(x, y);
            let p = (x, y + dy);
            route.push(p);
            if reverse {
                x -= precision;
            }
            else {
                x += precision;
            }
        }
        y += incr((y-center.1).abs());
        routes.push(route);
        reverse = !reverse;
    }

    let mut x = pad;
    loop {
        if x > width - pad {
            break;
        }
        let l = if x < center.0 {
            lerp(pad, center.0, x)
         }
         else {
             lerp(width - pad, center.0, x)
         };
        let yfrom = mix(pad, center.1, l) - overlap;
        let yto = mix(height - pad, center.1, l) + overlap;
        let mut y = if !reverse { yfrom } else { yto };
        let mut route = Vec::new();
        loop {
            if y > yto || y < yfrom {
                break;
            }
            let dx = displacement(x, y);
            let p = (x + dx, y);
            route.push(p);
            if reverse {
                y -= precision;
            }
            else {
                y += precision;
            }
        }
        x += incr((x-center.0).abs());
        routes.push(route);
        reverse = !reverse;
    }

    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut data = Data::new();
            for route in routes.clone() {
                data = render_route(data, route);
            }
            let mut l = layer(color);
            l = l.add(base_path(color, 0.35, data));
            l
        })
        .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.0")]
    seed: f64,
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
