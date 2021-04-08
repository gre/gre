use gre::*;
use ndarray::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

// https://docs.rs/svg/0.8.0/svg/

fn mandelbrot(init: (f64, f64)) -> f64 {
    let mut p = init;
    let it = 500;
    for i in 0..it {
        p = (
            init.0 + p.0 * p.0 - p.1 * p.1,
            init.1 + 2.0 * p.0 * p.1,
        );
        if p.0 * p.0 + p.1 * p.1 >= 4.0 {
            return (i as f64) / (it as f64);
        }
    }
    return 1.0;
}

pub fn render_route_curve(
    data: Data,
    route: Vec<(f64, f64)>,
) -> Data {
    let mut first = true;
    let mut d = data;
    let mut last = route[0];
    for p in route {
        if first {
            first = false;
            d = d.move_to(p);
        } else {
            d = d.quadratic_curve_to((
                last.0,
                last.1,
                (p.0 + last.0) / 2.,
                (p.1 + last.1) / 2.,
            ));
        }
        last = p;
    }
    return d;
}

fn art(
    seed0: u8,
    seconds: i64,
    clouds_count: usize,
    cloud_size: usize,
    cloud_clusters: usize,
) -> Vec<Group> {
    let clouds: Vec<Vec<(f64, f64)>> = (0..clouds_count)
        .into_par_iter()
        .flat_map(|i| {
            let mut rng = SmallRng::from_seed([
                seed0, i as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ]);

            let samples = sample_2d_candidates_f64(
                &|(x, y)| {
                    (0.05
                        * ((mandelbrot((
                            2.5 * (x - 0.8),
                            2.5 * (y - 0.5),
                        )) - 0.1)
                            .max(0.0)
                            - mandelbrot((
                                5.0 * (x - 0.8),
                                5.0 * (y - 0.5),
                            ))))
                    .max(0.0)
                },
                600,
                cloud_size,
                &mut rng,
            );

            let arr = Array2::from_shape_vec(
                (samples.len(), 2),
                samples
                    .iter()
                    .flat_map(|&(x, y)| vec![x, y])
                    .collect(),
            )
            .unwrap();

            let (means, clusters) = rkm::kmeans_lloyd(
                &arr.view(),
                cloud_clusters,
            );

            let all: Vec<Vec<(f64, f64)>> = means
                .outer_iter()
                .enumerate()
                .map(|(c, _coord)| {
                    clusters
                        .iter()
                        .enumerate()
                        .filter(|(_i, &cluster)| {
                            cluster == c
                        })
                        .map(|(i, _c)| samples[i])
                        .collect()
                })
                .collect();

            return all;
        })
        .collect();

    let routes: Vec<Vec<(f64, f64)>> = clouds
        .into_par_iter()
        .map(|pts| {
            let mut route =
                tsp(pts, time::Duration::seconds(seconds));

            route.push(route[0]);

            route
        })
        .collect();

    let bounds = (0., 0., 297., 210.);

    let mut groups = Vec::new();

    let mut data = Data::new();
    for route in routes {
        let pts = route
            .iter()
            .map(|&p| project_in_boundaries(p, bounds))
            .collect();

        data = render_route_curve(data, pts);
    }

    let color = "black";
    groups.push(
        layer(color).add(base_path(color, 0.2, data)),
    );

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(0);
    let seconds = args
        .get(2)
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    let clouds_count = args
        .get(3)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(4);
    let cloud_size = args
        .get(4)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(2000);
    let cloud_cluster = args
        .get(5)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(60);
    let groups = art(
        seed,
        seconds,
        clouds_count,
        cloud_size,
        cloud_cluster,
    );
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (240.0, 160.0),
        "black",
    ));
    svg::save("image.svg", &document).unwrap();
}
