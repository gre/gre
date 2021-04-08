use gre::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

type Conf = (&'static str, (f64, f64, f64), f64);
type ConfRoute = Vec<(Conf, Vec<(f64, f64)>)>;

fn art(seconds: i64) -> Vec<Group> {
    let passes = 2;
    let samples_count = 4000;
    let kmeans_clusters = 20;
    let get_color =
        image_get_color("images/moon.jpg").unwrap();

    let mut groups = Vec::new();

    let configs: Vec<Conf> = vec![
        ("white", (0.9, 0.9, 0.9), 0.9),
        ("orange", (0.7, 0.4, 0.1), 0.4),
    ];

    let bounds = (40.0, 0.0, 260.0, 210.0);

    let routes: Vec<(Conf, Vec<(f64, f64)>)> = configs
        .par_iter()
        .flat_map(|&config| {
            let mut rng = thread_rng();

            let all: ConfRoute = (0..passes)
                .flat_map(|_i| {
                    let (r, g, b) = config.1;
                    let sampler = |p| {
                        let (cr, cg, cb) = get_color(p);
                        let dr = cr - r;
                        let dg = cg - g;
                        let db = cb - b;
                        0.7 * smoothstep(
                            config.2,
                            0.0,
                            (dr * dr + dg * dg + db * db)
                                .sqrt(),
                        )
                    };

                    let samples = sample_2d_candidates_f64(
                        &sampler,
                        400,
                        samples_count,
                        &mut rng,
                    );

                    if samples.len() < kmeans_clusters * 3 {
                        return vec![];
                    }

                    let groups: ConfRoute =
                        group_with_kmeans(
                            samples,
                            kmeans_clusters,
                        )
                        .iter()
                        .map(|pts| (config, pts.clone()))
                        .collect();

                    groups
                })
                .collect();

            all
        })
        .map(|(config, pts)| {
            let mut route = tsp(
                pts.iter()
                    .map(|&p| {
                        project_in_boundaries(p, bounds)
                    })
                    .collect(),
                time::Duration::seconds(seconds),
            );
            route.push(route[0]);
            (config, route)
        })
        .collect();

    for (config, route) in routes {
        let mut data = Data::new();
        data = render_route_curve(data, route);
        groups.push(
            layer(config.0)
                .add(base_path(config.0, 0.5, data)),
        );
    }

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seconds = args
        .get(1)
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(10);
    let groups = art(seconds);
    let mut document = base_a4_landscape("black");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (220.0, 170.0),
        "white",
    ));
    svg::save("image.svg", &document).unwrap();
}
