use clap::Clap;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;
use std::f64::consts::PI;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "35.0")]
    seed: f64,
    #[clap(short, long, default_value = "0.25")]
    amp: f64,
    #[clap(short, long, default_value = "12.0")]
    freq: f64,
    #[clap(short, long, default_value = "0.0")]
    offset: f64,
    #[clap(short, long, default_value = "160")]
    samples: usize,
    #[clap(short, long, default_value = "6000")]
    samples2: usize,
}

fn art(opts: Opts) -> Vec<Group> {
    let colors = vec!["Black", "Black"];
    let stroke_width = 0.35;
    let pad = 20.0;
    let width = 297.0;
    let height = 210.0;
    let size = 60.0;
    let bounds = (pad, pad, width - pad, height - pad);
    
    let line_length = 220.0;
    let granularity = 1.0;

    let get_angle = |initial_angle, length, j| {
        initial_angle + 0.3 - length as f64 * 0.005
    };

    let perlin = Perlin::new();

    let parametric = |t: f64| {
        (
            1.5 * (2. * PI * t).cos()
            + 0.3 * (6. * PI * t).cos(),
            1.1 * (2. * PI * t).sin()
            + 0.3 * (6. * PI * t).sin()
        )
    };

    let parametric2 = |t| {
        let p = parametric(t);
        let x = (t + opts.offset) % 1.;
        let n = perlin.get([
            opts.freq * x,
            opts.seed + 
                0.1 * perlin.get([
                    opts.seed,
                    2. * opts.freq * x
                ]),
        ]);
        return (
            0.7 * p.0
            - 0.3 * (504. * PI * t).cos(),
            0.5 * p.1
            + 0.2 * (6. * PI * t).sin()
            + 0.02 * (1000. * PI * t).sin()
            + opts.amp * n
        );
    };

    let samples_data: Vec<(f64, (f64, f64))> = (0..opts.samples)
        .map(|i| {
            let sp = (opts.offset + i as f64) / (opts.samples as f64);
            let o = parametric(sp);
            let dt = 0.001;
            let o2 = parametric(sp + dt);
            let initial_angle =
                (o.1 - o2.1).atan2(o.0 - o2.0);
            let p = (
                width * 0.5 + size * o.0,
                height * 0.5 + size * o.1,
            );
            (initial_angle, p)
        })
        .collect();

    let initial_positions =
        samples_data.iter().map(|&(_a, p)| p).collect();

    let mut parametric_route: Vec<(f64, f64)> =
        samples_data.iter().map(|&(_a, p)| p).collect();
    parametric_route.push(parametric_route[0]);

    let mut particles = Vec::new();

    let mut build_route = |p, i, j| {
        let length = i as f64 * granularity;
        let (initial_angle, _o) = samples_data[j];
        let angle = get_angle(initial_angle, length, j);
        let nextp = follow_angle(p, angle, granularity);
        let ends = length >= line_length;
        if ends {
            return None; // line ends
        }
        let rep = 40;
        if j % 2 == 0 && (i + 8 * j + (rep as f64 * (1. - opts.offset)) as usize) % rep == 0 {
            particles.push(nextp);
        }
        if let Some(edge_p) =
            collide_segment_boundaries(p, nextp, bounds)
        {
            return Some((edge_p, true));
        }
        if i > 1 {
            if let Some(c) = collide_route_segment(
                &parametric_route,
                p,
                nextp,
            ) {
                return Some((c, true));
            }
        }
        return Some((nextp, false));
    };

    let mut routes =
    // lines
    build_routes_with_collision_par(
        initial_positions,
        &mut build_route,
    );

    // parametric curve itself
    routes.push(parametric_route);

    // frame
    routes.push(boundaries_route(bounds));
    
    let mut second: Vec<(f64,f64)> = 
        (0..opts.samples2)
        .map(|i| {
            let sp = i as f64 / (opts.samples2 as f64);
            let o = parametric2(sp);
            let p = (
                width * 0.5 + size * o.0,
                height * 0.5 + size * o.1,
            );
            p
        })
        .collect();
    second.push(second[0]);

    let all = vec![
        routes,
        vec![second]
    ];
    
    let mut passage = Passage2DCounter::new(0.5, width, height);
    let max_passage = 3;
    let mut should_draw_line = |a, _b| {
        passage.count(a) < max_passage
    };

    colors
        .iter()
        .enumerate()
        .map(|(i, &color)| {
            let data = all[i]
                .iter()
                .enumerate()
                .fold(Data::new(), |data, (_j, route)| {
                    render_route_when(data, route.clone(), &mut should_draw_line)
                });
            let mut g = layer(color);
            g = g.add(base_path(color, stroke_width, data));
            if i == 0 {
                // do it twice for the ink to be more intense!
                for _i in 0..2 {
                    for p in particles.iter() {
                        g = g.add(
                            Circle::new()
                            .set("cx", p.0)
                            .set("cy", p.1)
                            .set("r", stroke_width)
                            .set("stroke", color)
                            .set(
                                "stroke-width",
                                stroke_width,
                            )
                            .set("fill", "none")
                            .set("style", "mix-blend-mode: multiply;"),
                        );
                    }
                }
                g = g.add(signature(
                    1.0,
                    (252.0, 190.0),
                    color,
                ))
            }
            return g;
        })
        .collect()
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
