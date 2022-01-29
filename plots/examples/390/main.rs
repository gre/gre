use clap::Clap;
use gre::*;
use ndarray::Array2;
use rayon::prelude::*;
use svg::node::element::{Group, path::Data};

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.0")]
    seed: f64,
    #[clap(short, long, default_value = "40")]
    kmeans_clusters: usize,
    #[clap(short, long, default_value = "5")]
    seconds: i64,
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
}

fn art(opts: Opts) -> Vec<Group> {
    let height = 210.0;
    let width = 297.0;
    let size = 190.0;
    let stroke_width = 0.35;
    let mut rng = rng_from_seed(opts.seed);

    let get_color = image_get_color("images/profile2.jpg").unwrap();
    let dim = 512;
    let samples = 6000;

    let f = |_p: (f64, f64)| { 1.0 };

    let samples = sample_2d_candidates_f64(&f, dim, samples, &mut rng);

    let circles = samples.iter().map(|&(x, y)| {
        let color = get_color((x, y));
        let v = grayscale(color);
        let r = 4. * (1.03 - v);
        VCircle::new(
            (width - size) / 2.0 + x * size,
            (height - size) / 2.0 + y * size,
            r
        )
    }).collect();

    let routes: Vec<Vec<(f64, f64)>> =
    group_with_kmeans(circles, opts.kmeans_clusters)
        .par_iter()
        .map(|cin| {
            let points: Vec<(f64, f64)> = cin.iter().map(|c| (c.x, c.y)).collect();
            let tour =
                travelling_salesman::simulated_annealing::solve(&points, time::Duration::seconds(opts.seconds));
            let circles: Vec<VCircle> = tour
                .route
                .iter()
                .map(|&i| cin[i])
                .collect();
            let route: Vec<(f64, f64)> =
            circles
                .par_iter()
                .flat_map(|c| {
                    let s = opts.seed + c.x * 3.1 + c.y / 9.8;
                    let mut rng = rng_from_seed(s);
                    let pow = 1.4;
                    let samples = sample_2d_candidates_f64(&|p| {
                        let dx = p.0 - 0.5;
                        let dy = p.1 - 0.5;
                        let d2 = dx * dx + dy * dy;
                        if d2 > 0.25 {
                            0.0
                        }
                        else {
                            d2
                        }
                    }, (6. * c.r) as usize, (8. + (0.4 * c.r).powf(pow)) as usize, &mut rng);
                let candidates = samples.iter().map(|(x, y)| {
                    (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y)
                }).collect();
                route_spiral(candidates)
            })
            .collect();
            route
        })
        .collect();

    let color = "black";
    let mut data = Data::new();
    for route in routes {
      data = render_route_curve(data, route);
    }
    vec![
        layer(color)
        .add(base_path(color, stroke_width, data))
    ]
    
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

fn group_with_kmeans(
    samples: Vec<VCircle>,
    n: usize,
) -> Vec<Vec<VCircle>> {
    let arr = Array2::from_shape_vec(
        (samples.len(), 2),
        samples
            .iter()
            .flat_map(|c| vec![c.x, c.y])
            .collect(),
    )
    .unwrap();

    let (means, clusters) =
        rkm::kmeans_lloyd(&arr.view(), n);

    let all: Vec<Vec<VCircle>> = means
        .outer_iter()
        .enumerate()
        .map(|(c, _coord)| {
            clusters
                .iter()
                .enumerate()
                .filter(|(_i, &cluster)| cluster == c)
                .map(|(i, _c)| samples[i])
                .collect()
        })
        .collect();

    all
}