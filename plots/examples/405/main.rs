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
}

fn art(opts: &Opts) -> Vec<Group> {
    let width = 297.0;
    let height = 210.0;
    let pad = 20.0;
    let stroke_width = 0.3;
    let mut layers = Vec::new();
    let colors = vec!["cyan", "yellow"];
    let perlin = Perlin::new();

    for (ci, &color) in colors.iter().enumerate() {
        let mut routes = Vec::new();

        let line_dist = 0.6;
        let precision = 0.5;
        let mut basey = pad;
        loop {
            let mut x = pad;
            let mut route = Vec::new();
            let mut is_up = false;
            loop {
                let y1 = basey + 10.0 * perlin.get([
                    7.7 * opts.seed,
                    0.01 * basey,
                    0.02 * x
                ]);
                let y = y1 + 2.0 * perlin.get([
                    0.1 * y1,
                    0.5 * x,
                    perlin.get([
                        0.2 * y1,
                        opts.seed / 3.3,
                        0.3 * x,
                    ]) - 9.9 * opts.seed
                ]);
                let flagmode = y > height / 2.0;
                let heartmode = heart_distance(
                    (20.0 * (x/width-0.5),
                     15.0 * (0.33-y/height)
                )) < 0.0;
                let mode = flagmode == heartmode;
                let active = mode == (ci == 0);
                if active {
                    if is_up {
                        is_up = false;
                        if route.len() > 1 {
                            routes.push(route);
                        }
                        route = Vec::new();
                    }
                    route.push((x, y));
                }
                else {
                    is_up = true;
                }
                x += precision;
                if x > width - pad {
                    break;
                }
            }
            if route.len() > 1 {
                routes.push(route);
            }
            basey += line_dist;
            if basey > height - pad {
                break;
            }
        }

        let mut l = layer(color);
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
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save(opts.file, &document).unwrap();
}
