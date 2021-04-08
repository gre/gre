use clap::Clap;
use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(
    opts: &Opts
) -> Vec<Group> {
    let (width, height) = if opts.square {
        (210., 210.)
    } else if opts.portrait {
        (210., 297.)
    } else {
        (297., 210.)
    };

    let get_color = image_get_color(opts.path.as_str()).unwrap();
    let pad = 20.0;
    let boundaries = (pad, pad, width - pad, height - pad);

    let map = vec![0, 3, 5, 1, 2, 4];

    vec!["cyan", "magenta", "yellow", "black"]
        .iter()
        .enumerate()
        .map(|(g, clr)| {
            let mut cang = g as f64 * 2. * PI / 3.;
            let mut camp = 0.8;
            if g == 3 {
                camp = 1.0;
                cang = PI / 2.0;
            }
            let c = (
                0.5 + camp * cang.cos(),
                0.5 + camp * cang.sin(),
            );
            let samples = 4000 * opts.spins;
            let radius = 2. * camp;
            let mut routes = Vec::new();
            let mut route = Vec::new();
            let mut pen_up = true;
            for i in 0..samples {
                let ii = (i as f64) / (samples as f64);
                let a = 2. * PI * (opts.spins as f64) * ii;
                let r = radius
                    * (ii
                        + (g as f64
                            / (4. * (opts.spins as f64))));
                let pn =
                    (c.0 + r * a.cos(), c.1 + r * a.sin());
                let p =
                    project_in_boundaries(pn, boundaries);
                let cmyk = rgb_to_cmyk_vec(get_color(
                    preserve_ratio_outside(
                        pn,
                        (width, height),
                    ),
                ));
                let c = cmyk[g];
                let draw = c > 0.03
                    + 0.9
                        * ((map[((opts.spins as f64 * ii)
                            as usize)
                            % map.len()]
                            as f64)
                            / (map.len() as f64))
                    && !out_of_boundaries(p, boundaries);
                if draw {
                    if pen_up {
                        if route.len() > 2 {
                            routes.push(route);
                        }
                        route = Vec::new();
                        pen_up = false;
                    }
                    route.push(p);
                } else {
                    pen_up = true;
                }
            }
            if route.len() > 2 {
                routes.push(route);
            }

            let data = routes.iter().fold(
                Data::new(),
                |acc, route| {
                    render_route(acc, route.clone())
                },
            );

            let mut l =
                layer(clr).add(base_path(clr, 0.25, data).set("opacity", 0.8));
            if g == 3 {
                l = l.add(signature(
                    1.0,
                    (
                        width - 35. - opts.padsig.0,
                        height - 15. - opts.padsig.1,
                    ),
                    clr,
                ))
            }
            l
        })
        .collect()
}



fn parse_pad(s: &str) -> Result<(f64,f64), String> {
    let all: Vec<f64> = s
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|str| str.parse::<f64>().unwrap())
        .collect();
    if all.len() == 0 {
        return Ok((0., 0.));
    }
    if all.len() == 1 {
        return Ok((all[0], all[0]));
    }
    return Ok((all[0], all[1]));
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "images/pattern_02_e.png")]
    path: String,
    #[clap(short, long, default_value = "1000")]
    spins: usize,
    #[clap(short, long)]
    portrait: bool,
    #[clap(short, long)]
    square: bool,
    #[clap(short, long, default_value = "12,5", parse(try_from_str = parse_pad))]
    padsig: (f64,f64),
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(&opts);
    let mut document = if !opts.portrait {
        base_a4_landscape("white")
    } else {
        base_a4_portrait("white")
    };
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
