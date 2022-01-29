use std::f64::consts::PI;

use clap::Clap;
use gre::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let colors = vec!["turquoise", "red"];
    colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
            let mut rng = rng_from_seed(opts.seed);
            let mut l = layer(color);
            let width = 210.;
            let height = 297.;
            let div = 5;
            let w = width;
            let h = height / (div as f64 + 1.);
            let cfi = ci as f64;
            let fdiv = div as f64;

            for (width, height, dx, dy, progress) in
                (0..div)
                .map(|i| {
                    let fi = i as f64;
                    (w, h,
                        1.4 * cfi * (fi / fdiv - 0.5),
                        0.5 * h + 0.8 * cfi * (fi / fdiv - 0.5) +
                        fi * h,
                        (fi + 1.) / (fdiv + 2.))
                }) {
                let maxr = height * 0.55 - 0.5 * cfi;
                let dxp = 0.2;
                let c1 = (width * (0.5-dxp), height * 0.5);
                let c2 = (width * (0.5+dxp), height * 0.5);

                let mut route = Vec::new();
                let c = c1;
                let pmax = 1. - progress;
                let mut r = 0f64;
                let mut a = 0f64;
                let incr = 0.06;
                let angincr = 0.3;
                loop {
                    route.push((c.0 + r * a.cos() + dx, c.1 + r * a.sin() + dy));
                    a = (a + angincr) % (2.0 * PI);
                    r += incr;
                    if r > pmax * maxr && a > 1.5 * PI {
                        break;
                    }
                }
                let x1 = c.0 + r * a.cos() + dx;
                let y1 = c.1 + r * a.sin() + dy;

                let c = c2;
                let pmax = progress;
                let mut r = pmax * maxr;
                a = 1.5 * PI;
                let x2 = c.0 + r * a.cos() + dx;
                let y2 = c.1 + r * a.sin() + dy;

                let waveincr = 7.0 + 0.5 * cfi;
                let waveamp = 16.0;
                let mut x = x1;
                loop {
                    let xp = lerp(x1, x2, x);
                    let y = mix(y1, y2, xp) +
                        (0.5 - (0.5 - xp).abs()) *
                        rng.gen_range(-waveamp, waveamp) *
                        rng.gen_range(0., 1.);
                    route.push((x, y));
                    x += waveincr;
                    if x > x2 {
                        break;
                    }
                }

                loop {
                    route.push((c.0 + r * a.cos() + dx, c.1 + r * a.sin() + dy));
                    a = (a + angincr) % (2.0 * PI);
                    r -= incr;
                    if r <= 0.1 {
                        break;
                    }
                }

                let data = render_route_curve(Data::new(), route);
                l = l.add(base_path(color, 0.35, data));

            }

            l
        })
        .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "1.0")]
    seed: f64,
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
