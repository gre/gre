use noise::*;
use clap::Clap;
use gre::*;
use rand::Rng;
use svg::node::element::*;
use svg::node::element::path::Data;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "24.0")]
    seed: f64,
}

fn art(opts: Opts) -> Vec<Group> {
    let colors = vec!["#f90", "#09f"];
    let width = 420.0;
    let height = 297.0;
    let pad = 20.0;
    let stroke_width = 0.35;
    let seed = opts.seed;
    let mut layers = Vec::new();
    let mut rng = rng_from_seed(seed);
    let color_alt = height / 4.0;
    let r1 = rng.gen_range(0.0, 1.0);
    let r2 = rng.gen_range(0.0, 1.0);
    let r3 = rng.gen_range(0.0, 1.0);
    let r4 = rng.gen_range(0.0, 1.0);
    let r5 = rng.gen_range(0.0, 1.0);
    let r6 = rng.gen_range(0.0, 1.0);
    let r7 = rng.gen_range(0.0, 1.0);

    for (ci, &color) in colors.iter().enumerate() {
        let from = height * 2.;
        let to = 0.0;
        let mut routes = Vec::new();
        let mut base_y = from;
        let perlin = Perlin::new();
        let mut passage = Passage2DCounter::new(0.4, width, height);
        let passage_limit = 12 ;
        let precision = 0.2;
        let dy = 0.3;

        let mut height_map: Vec<f64> = Vec::new();
        loop {
            if base_y < to {
                break;
            }
            let is_color = (base_y / color_alt) as usize % 2 != ci;
            let mut route = Vec::new();
            let mut x = pad;
            let mut was_outside = true;
            let mut i = 0;
            loop {
                if x > width - pad {
                    break;
                }
                let y = base_y + ((0.1 + 0.6 * r5) * height - 0.6 * r7 * euclidian_dist((0.5 * width, 0.7 * height), (x, base_y))).max(5.0) * (
                    perlin.get([
                        mix(0.05, 0.005, r1) * x,
                        mix(0.005, 0.05, r1) * base_y,
                        seed + 4.0 * r2 * perlin.get([
                            0.1 * base_y * r3,
                            0.02 * x,
                            1. + 0.7 * seed + 0.01 * perlin.get([
                                r4 * base_y,
                                0.2 * x,
                                10. + 5.3 * seed
                            ])
                        ])
                    ])
                    - 10.0 * r6 * perlin.get([
                        mix(0.2, 0.8, r5) * 0.006 * x,
                        mix(0.8, 0.2, r5) * 0.006 * base_y,
                        -7. + 9. * seed + 0.02 * perlin.get([
                            0.02 * base_y,
                            0.02 * x,
                            seed / 7. - 9.
                        ])
                    ]).powf(2.0)
                );
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
                pad < x && x < width - pad &&
                pad < y && y < height - pad &&
                passage.count((x, y)) < passage_limit;
                if inside {
                    if was_outside {
                        if route.len() > 2 {
                            if is_color {
                                routes.push(route);
                            }
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
            
            if is_color {
                routes.push(route);
            }

            base_y -= dy;
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
    let groups = art(opts);
    let mut document = base_a3_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
