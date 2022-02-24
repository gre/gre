use clap::*;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;


#[derive(Clap)]
#[clap()]
pub struct Opts {
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


fn parametric(p: f64, opts: &Opts) -> (f64, f64) {
    let f1 = (12.0 + 4. * opts.seed1).floor();
    let f2 = (12.0 + 4. * opts.seed2).floor();
    let a = mix(0.1, 0.9, 0.5 + 0.5 * opts.seed3.cos());
    (
        a * (2.0 * PI * p).cos()
            + (1.0 - a) * (f1 * PI * p).cos(),
        a * (2.0 * PI * p).sin()
            + (1.0 - a) * (f2 * PI * p).sin(),
    )
}

fn art(opts: &Opts) -> Vec<Group> {
    let colors = vec!["red", "cyan", "darkblue", "#F90", "black", "pink", "green", "purple"];

    let color = colors[(opts.seed1 as usize) % colors.len()];

    let pad = 20.0;
    let width = 210.0;
    let height = 297.0;
    let size = 60.0;
    let bounds = (pad, pad, width - pad, height - pad);

    let line_length = 1000.0;
    let granularity = 1.0;
    let samples = 1200;

    let perlin = Perlin::new();
    let get_angle = |p, initial_angle, length| -> f64 {
        initial_angle + 0.5
            - length
                * 0.00005
                * euclidian_dist(
                    p,
                    (width / 2., height / 2.),
                )
    };

    let initial_data: Vec<((f64, f64), f64)> = (0..samples)
        .map(|s| {
            let sp = s as f64 / (samples as f64);
            let o = parametric(sp, opts);
            let dt = 0.0001;
            let o2 = parametric(sp + dt, opts);
            let initial_angle =
                (o.1 - o2.1).atan2(o.0 - o2.0);
            let p = (
                width * 0.5 + size * o.0,
                height * 0.5 + size * o.1,
            );
            (p, initial_angle)
        })
        .collect();

    let initial_positions: Vec<(f64, f64)> =
        initial_data.iter().map(|&(p, a)| p).collect();

    let initial_angles: Vec<f64> =
        initial_data.iter().map(|&(p, a)| a).collect();

    let mut build_route = |p: (f64, f64), l, route_i| {
        let normalized = normalize_in_boundaries(p, bounds);
        let initial_angle = initial_angles[route_i];
        let angle = get_angle(
            p,
            initial_angle,
            l as f64 * granularity,
        );
        let next = (
            p.0 + granularity * angle.cos(),
            p.1 + granularity * angle.sin(),
        );
        let ends = l as f64 / granularity > line_length;
        if let Some(c) =
            collide_segment_boundaries(p, next, bounds)
        {
            return Some((c, true));
        }
        if ends {
            None
        } else {
            Some((next, false))
        }
    };

    let mut routes = build_routes_with_collision_par(
        initial_positions.clone(),
        &build_route,
    );

    routes = routes
        .iter()
        .map(|route| round_route(route.clone(), 0.01))
        .collect();

    let mut parametric = initial_positions.clone();
    parametric.push(parametric[0]);
    routes.push(parametric);

    routes.push(boundaries_route(bounds));

    let mut groups = Vec::new();

    let mut data = Data::new();
    for (j, route) in routes.iter().enumerate() {
        data = render_route(data, route.clone());
    }

    let mut g = layer(color);

    g = g.add(base_path(color, 0.2, data));

    groups.push(g);

    groups
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
