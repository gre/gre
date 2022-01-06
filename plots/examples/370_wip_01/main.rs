use clap::Clap;
use geo::prelude::*;
use gre::*;
use noise::*;
use rayon::prelude::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let colors = vec!["black"];
    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            
    let voronoi_size = 600;
    let max_samples = 80;
    let samples_r = 0.08;
    let res = 100;
    let pad = 10.;
    let width = 260.;
    let height = 190.;
    let poly_threshold = 0.5;

    let project = |(x, y): (f64, f64)| {
        (pad + x * width, pad + y * height)
    };

    let mut rng = rng_from_seed(opts.seed);

    let mut data = Data::new();

    for i in 0..1 {
        let candidates = sample_2d_candidates_f64(
            &|p| 1. - 1.5 * euclidian_dist(p, (0.5, 0.5)),
            800,
            (voronoi_size as f64 * rng.gen_range(0.1, 1.2))
                as usize,
            &mut rng,
        );

        let mut polys =
            sample_square_voronoi_polys(candidates, 0.1);

        // filter out big polygons (by their "squared" bounds)
        polys.retain(|poly| {
            poly_bounding_square_edge(poly) < poly_threshold
        });


        let pow = rng.gen_range(1.0, 3.0) as f64;
        let get = |p| {
            0.45 - euclidian_dist(p, (0.5, 0.5))
        };

        let routes: Vec<Vec<(f64, f64)>> = polys
            .par_iter()
            .map(|poly| {
                let mut rng = rng_from_seed(opts.seed);
                let bounds = poly.bounding_rect().unwrap();
                let min = bounds.min();
                let width = bounds.width();
                let height = bounds.height();
                let map_p = |(lx, ly)| {
                    (
                        min.x + width * lx,
                        min.y + height * ly,
                    )
                };
                let mut candidates =
                    sample_2d_candidates_f64(
                        &|p| {
                            let ap = map_p(p);
                            if poly.contains(
                                &geo::Point::new(
                                    ap.0, ap.1,
                                ),
                            ) {
                                samples_r * get(ap)
                            } else {
                                0.0
                            }
                        },
                        res,
                        max_samples,
                        &mut rng,
                    );
                candidates
                    .iter()
                    .map(|&p| project(map_p(p)))
                    .collect()
            })
            .collect();

        for route in routes {
            data = render_route_curve(
                data,
                route,
            );
        }
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
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
