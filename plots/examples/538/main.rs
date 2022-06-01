use clap::Clap;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
pub struct Opts {
    #[clap(short, long, default_value = "image.svg")]
    file: String,
    #[clap(
        short,
        long,
        default_value = "/Users/grenaudeau/Desktop/farmer.gif"
    )]
    animation: String,
    #[clap(short, long, default_value = "297.0")]
    pub width: f64,
    #[clap(short, long, default_value = "210.0")]
    pub height: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
    let colors = vec!["black"];
    let mut routes = Vec::new();

    let frames = 5;
    let pad = 10.0;
    let size = (opts.width - pad * 2.0) / (frames as f64);
    for rev in 0..2 {
        let dy = (opts.height - size * 2.) / 2.0
            + rev as f64 * size;
        for i in 0..frames {
            let get_color = image_gif_get_color(
                opts.animation.as_str(),
                i,
            )
            .unwrap();
            let dx = pad + i as f64 * size;
            let bounds = (1.0, 1.0, size - 1.0, size - 1.0);

            let pixel_pad = 0.1;
            let f = |(x, y)| {
                let p = (
                    -pixel_pad
                        + x * (1.0 + 2.0 * pixel_pad),
                    -pixel_pad
                        + y * (1.0 + 2.0 * pixel_pad),
                );
                if out_of_boundaries(
                    p,
                    (0.0, 0.0, 1.0, 1.0),
                ) {
                    return rev as f64;
                }
                if rev == 0 {
                    get_color(p).0
                } else {
                    1. - get_color(p).0
                }
            };

            let thresholds = vec![0.5];
            let precision = 0.5;
            let w = (size as f64 / precision) as u32;
            let h = (size as f64 / precision) as u32;
            let res = contour(w, h, f, &thresholds);
            let mut all =
                features_to_routes(res, precision);
            all = crop_routes(&all, bounds);

            let aincr = 0.005;
            let rincr = aincr / 12.0;
            let mut r = 0.1;
            let mut a = 0f64;
            let center = (
                size * 0.47,
                size * (0.35
                    + match i {
                        2 | 4 => 0.05,
                        3 => 0.1,
                        _ => 0.0,
                    }),
            );
            let mut route = Vec::new();
            let min_stroke = 0.1;
            loop {
                if r > size {
                    break;
                }
                let p = (
                    center.0 + r * a.cos(),
                    center.1 + r * a.sin(),
                );
                let n = (p.0 / size, p.1 / size);

                let should_draw = strictly_in_boundaries(
                    n,
                    (0.0, 0.0, 1.0, 1.0),
                ) && f(n) > 0.5;

                if !should_draw {
                    if route.len() > 1 {
                        all.push(route);
                    }
                    route = Vec::new();
                } else {
                    let l = route.len();
                    if l == 0 {
                        route.push(p);
                    } else if euclidian_dist(
                        route[l - 1],
                        p,
                    ) > min_stroke
                    {
                        route.push(p);
                    }
                }

                r += rincr;
                a += aincr;
            }
            if route.len() > 1 {
                all.push(route);
            }

            all = translate_routes(all, (dx, dy));
            routes = vec![routes, all].concat();
            routes.push(vec![(dx, dy), (dx, dy + size)]);
        }
        let lastx = pad + size * (frames as f64);
        routes.push(vec![(lastx, dy), (lastx, dy + size)]);
        routes.push(vec![(pad, dy), (lastx, dy)]);
    }
    let dy = opts.height / 2.0 + size;
    routes.push(vec![(pad, dy), (opts.width - pad, dy)]);

    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut data = Data::new();
            let mut l = layer(color);
            for route in routes.clone() {
                data = render_route(data, route);
            }
            l = l.add(base_path(color, 0.35, data));
            l
        })
        .collect()
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(&opts);
    let mut document =
        base_document("white", opts.width, opts.height);
    for g in groups {
        document = document.add(g);
    }
    svg::save(opts.file, &document).unwrap();
}

fn translate_routes(
    routes: Vec<Vec<(f64, f64)>>,
    (tx, ty): (f64, f64),
) -> Vec<Vec<(f64, f64)>> {
    routes
        .iter()
        .map(|route| {
            route
                .iter()
                .map(|&(x, y)| (x + tx, y + ty))
                .collect()
        })
        .collect()
}
