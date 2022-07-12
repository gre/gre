/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Plottable Era: (I) Primitive
 */
mod utils;
use byteorder::*;
use geo::prelude::*;
use geo::*;
use noise::*;
use rand::prelude::*;
use rand::Rng;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use svg::Document;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
pub struct Opts {
    pub seed: f64,
    pub hash: String,
    pub primary_name: String,
    pub secondary_name: String,
}

fn reflect_shapes<R: Rng>(
    rng: &mut R,
    routes: &mut Vec<Vec<(f64, f64)>>,
    probability: f64,
    ycenter: f64,
    boundaries: (f64, f64, f64, f64),
) {
    let base_stroke = 0.4;
    for route in routes.clone() {
        for p in route {
            if !rng.gen_bool(probability) {
                continue;
            }
            let sx = base_stroke / 2.0 + 8.0 * rng.gen_range(0f64, 1.0).powf(2.0);
            let sy = 0.3 * rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0);
            let x =
                p.0 + rng.gen_range(0.0, 30.0) * rng.gen_range(0.0, 1.0) * rng.gen_range(-1.0, 1.0);
            let y = 2.0 * ycenter - p.1 + rng.gen_range(0.0, 70.0) * rng.gen_range(-1.0, 1.0);
            if y > ycenter && y < boundaries.3 {
                let x1 = (x - sx).max(boundaries.0).min(boundaries.2);
                let x2 = (x + sx).max(boundaries.0).min(boundaries.2);
                if x2 - x1 > base_stroke {
                    routes.push(vec![(x1, y - sy), (x2, y + sy)]);
                }
            }
        }
    }
}

fn random_menhir<R: Rng>(
    rng: &mut R,
    passage: &mut Passage2DCounter,
    routes: &mut Vec<Vec<(f64, f64)>>,
    (x, y): (f64, f64),
) {
    let size = rng.gen_range(4.0, 6.0);
    let count = rng.gen_range(3, 5);
    let rep = rng.gen_range(0f64, 1.4).floor() as usize + 1;
    let diffx = rng.gen_range(1.6, 3.0);
    for r in 0..rep {
        let dxbase = rng.gen_range(0.2, 0.4);
        let dxup = rng.gen_range(0.1, 0.3);
        let yv = rng.gen_range(0.0, 0.3);
        let rdx = if rep == 1 {
            0.0
        } else {
            r as f64 - (rep as f64 - 1.0) * 0.5
        } * diffx;
        for xi in 0..count {
            let xiv = xi as f64 - count as f64 * 0.5;
            let xvup = x + rdx + xiv * dxup;
            let xvbase = x + rdx + xiv * dxbase;
            let a = (xvbase, y);
            let b = (xvup, y - size * (1.0 - yv * rng.gen_range(0.0, 1.0)));
            passage.count(a);
            passage.count(b);
            routes.push(vec![a, b]);
        }
    }
    if rep > 1 {
        let count = rng.gen_range(3, 5);
        let ybase = y - 0.9 * size;
        let ytwist = rng.gen_range(-0.3, 0.5);
        let dy = rng.gen_range(0.2, 0.3);
        let extrax = 0.5;
        let dx = rep as f64 * diffx * 0.5 + extrax;
        for yi in 0..count {
            let yiv = yi as f64 - (count as f64 - 1.0) * 0.5;
            let y = ybase + yiv * dy;
            let y1 = y - ytwist;
            let y2 = y + ytwist;
            let x1 = x - dx;
            let x2 = x + dx;
            let a = (x1, y1);
            let b = (x2, y2);
            passage.count(a);
            passage.count(b);
            routes.push(vec![a, b]);
        }
    }
}

fn random_mammoth<R: Rng>(
    rng: &mut R,
    passage: &mut Passage2DCounter,
    routes: &mut Vec<Vec<(f64, f64)>>,
    (x, y): (f64, f64),
    size: f64,
) {
    let pts = vec![
        (0., 0.),
        (33.38, -86.48),
        (123.45, -123.19),
        (179.17, -107.19),
        (230.75, -138.37),
        (285.83, -87.63),
        (286.4, 11.69),
        (321.99, 64.65),
        (311.86, 70.39),
        (272.41, 28.13),
        (268.55, -36.57),
        (234.52, -30.69),
        (233.19, 84.8),
        (196.82, 22.91),
        (178.15, 91.69),
        (154.58, -3.12),
        (100.41, 17.1),
        (89.8, 87.04),
        (53.33, 45.69),
        (16.2, 86.04),
    ];
    let translate = (-160.0, -90.0);
    let scale = size / 200.0;
    let xflip = if rng.gen_bool(0.3) { -1.0 } else { 1.0 };
    let mut path: Vec<(f64, f64)> = pts
        .iter()
        .map(|&p| {
            let p = (
                x + (p.0 + translate.0) * scale * xflip + rng.gen_range(-0.3, 0.3),
                y + (p.1 + translate.1) * scale + rng.gen_range(-0.4, 0.4),
            );
            p
        })
        .collect();
    path.push(path[0]);
    let route = path_subdivide_to_curve(path, 2, 0.8);

    // we set pixels that are inside the polygon of the mammoth so we don't make line going through it
    let poly = Polygon::new(LineString::from(route.clone()), vec![]);
    let precision = 0.5;
    let mut xv = x - 0.8 * size;
    loop {
        if xv > x + 0.8 * size {
            break;
        }
        let mut yv = y - size;
        loop {
            if yv > y {
                break;
            }
            let p = (xv, yv);
            if poly.contains(&Point::from(p)) {
                passage.count_n(p, 10);
            }
            yv += precision;
        }
        xv += precision;
    }

    // pass 3 times on the mammoth shape
    routes.push(
        route
            .iter()
            .map(|p| {
                let q = (p.0 + 0.1, p.1 + 0.1);
                passage.count_n(q, 10);
                q
            })
            .collect(),
    );
    routes.push(
        route
            .iter()
            .map(|p| {
                let q = (p.0, p.1 - 0.15);
                passage.count_n(q, 10);
                q
            })
            .collect(),
    );
    routes.push(
        route
            .iter()
            .map(|p| {
                let q = (p.0 - 0.1, p.1 + 0.1);
                passage.count_n(q, 10);
                q
            })
            .collect(),
    );

    // defenses
    let mut p = (x + 0.44 * size * xflip, y - 0.65 * size);
    let mut a: f64 = PI / 2.0;
    let mut i = 0;
    let imax = rng.gen_range(12, 25);
    let amp = 0.08 * size;
    let aincr = xflip * rng.gen_range(0.28, 0.32);
    let mut aincrmul = 1.0;
    let mut route = Vec::new();
    loop {
        if i > imax {
            break;
        }
        p.0 += amp * a.cos();
        p.1 += amp * a.sin();
        route.push(p);
        i += 1;
        a -= aincr * aincrmul + i as f64 * rng.gen_range(-0.008, 0.01) * rng.gen_range(0.0, 1.0);
        aincrmul *= 0.95;
    }
    routes.push(route.clone());
    routes.push(
        route
            .iter()
            .map(|p| {
                let q = (p.0 + 0.2, p.1 + 0.2);
                passage.count_n(q, 10);
                q
            })
            .collect(),
    );
    routes.push(
        route
            .iter()
            .map(|p| {
                let q = (p.0, p.1 - 0.4);
                passage.count_n(q, 10);
                q
            })
            .collect(),
    );
    routes.push(
        route
            .iter()
            .map(|p| {
                let q = (p.0 - 0.2, p.1 + 0.2);
                passage.count_n(q, 10);
                q
            })
            .collect(),
    );
}

fn random_tipi<R: Rng>(
    rng: &mut R,
    passage: &mut Passage2DCounter,
    routes: &mut Vec<Vec<(f64, f64)>>,
    (x, y): (f64, f64),
) {
    let h = rng.gen_range(3.0, 5.0);
    let dx = 0.35 * h;
    let dx2 = -0.1 * h;
    for _i in 0..2 {
        let rdx = rng.gen_range(-0.2, 0.2);
        let rdy = rng.gen_range(-0.2, 0.2);
        let a = (rdx + x - dx, rdy + y);
        let b = (rdx + x - dx2, rdy + y - h);
        passage.count(a);
        passage.count(b);
        routes.push(vec![a, b]);
        let rdx = rng.gen_range(-0.2, 0.2);
        let rdy = rng.gen_range(-0.2, 0.2);
        let a = (rdx + x + dx, rdy + y);
        let b = (rdx + x + dx2, rdy + y - h);
        passage.count(a);
        passage.count(b);
        routes.push(vec![a, b]);
    }
}

fn random_fire<R: Rng>(
    rng: &mut R,
    passage: &mut Passage2DCounter,
    routes: &mut Vec<Vec<(f64, f64)>>,
    routes_smoke: &mut Vec<Vec<(f64, f64)>>,
    (x, y): (f64, f64),
) {
    for i in 0..7 {
        let dx = rng.gen_range(2.0, 3.0) / (1.0 + i as f64 * 0.3);
        let dy = i as f64 * 0.25 + 0.5;
        let ytwist = rng.gen_range(0.0, 0.5);
        let a = (x - dx, y - dy + ytwist);
        let b = (x + dx, y - dy - ytwist);
        passage.count(a);
        passage.count(b);
        routes.push(vec![a, b]);
    }

    let mut smokex = x;
    let xbaseincr = rng.gen_range(-0.2, 0.2);
    let mut smokey = y - 3.0;
    let perlin = Perlin::new();
    let seed = rng.gen_range(0.0, 1000.0);
    let mut w_mul = 2.0;
    let mut incrymul = 1.0;
    loop {
        if smokey < 10.0 {
            break;
        }
        let w = rng.gen_range(0.3, 1.1) * w_mul;
        routes_smoke.push(vec![(smokex - w * 0.5, smokey), (smokex + w * 0.5, smokey)]);
        smokex += xbaseincr
            + 0.8 * (0.1 * smokey).cos()
            + 2.0 * perlin.get([seed, 0.1 * smokex, 0.1 * smokey]);
        smokey -= rng.gen_range(1.0, 1.5) * incrymul;
        if rng.gen_bool(0.04) {
            smokey -= 10.0;
        }
        w_mul = (w_mul * 1.06).min(30.0);
        incrymul *= 1.03;
    }
}

fn random_bridge<R: Rng>(
    rng: &mut R,
    routes: &mut Vec<Vec<(f64, f64)>>,
    a: (f64, f64),
    b: (f64, f64),
    nb_people: usize,
) {
    let left = if a.0 < b.0 { a } else { b };
    let right = if a.0 > b.0 { a } else { b };

    // people on it
    for _p in 0..nb_people {
        let xp = rng.gen_range(0.0, 1.0);
        let x = mix(left.0, right.0, xp);
        let y = mix(left.1, right.1, xp);
        let h = rng.gen_range(2.0, 3.0);
        random_people(rng, routes, (x, y), h, false);
    }

    // poles
    for i in 0..3 {
        let dx = i as f64 * 0.2 - 0.3;
        routes.push(vec![
            (a.0 + dx, a.1),
            (a.0 + rng.gen_range(-0.3, 0.3), a.1 - 4.0),
        ]);
        routes.push(vec![
            (b.0 + dx, b.1),
            (b.0 + rng.gen_range(-0.3, 0.3), b.1 - 4.0),
        ]);
    }
    // base
    for i in 0..2 {
        let lx = 2.0;
        let rx = 1.0;
        let dy = i as f64 * 0.2 - 0.3;
        routes.push(vec![(left.0 + rx, left.1 + dy), (left.0 - lx, left.1 + dy)]);
        routes.push(vec![
            (right.0 - rx, right.1 + dy),
            (right.0 + lx, right.1 + dy),
        ]);
    }

    // 2 lines for the bridge on bottom
    routes.push(vec![a, b]);
    routes.push(vec![(a.0, a.1 + 0.2), (b.0, b.1 + 0.2)]);
    // 1 lines for the bridge top
    routes.push(vec![(a.0, a.1 - 1.0), (b.0, b.1 - 1.0)]);
}

fn random_people<R: Rng>(
    rng: &mut R,
    routes: &mut Vec<Vec<(f64, f64)>>,
    (x, y): (f64, f64),
    body_height: f64,
    with_weapon: bool,
) {
    let body_ratio = 0.2;
    let foot_dist = rng.gen_range(0.2, 1.0);
    let arm_dist = rng.gen_range(0.2, 1.0);
    let arm_balance = rng.gen_range(0.0, 1.0);
    let foot_balance = rng.gen_range(0.0, 1.0);
    let elder_pos = 0.4;
    let shoulder_pos = 0.8;
    for i in 0..2 {
        let dir = if i == 0 { 1.0 } else { -1.0 };
        let bw = body_height * body_ratio * dir;
        let fb = (0.5f64 - foot_balance) * dir;
        let ab = (0.5f64 - arm_balance) * dir;
        routes.push(vec![
            (x - bw * (foot_dist + fb), y),
            (x, y - elder_pos * body_height),
            (x, y - shoulder_pos * body_height),
            (
                x - bw * (arm_dist + ab),
                y - mix(elder_pos, shoulder_pos, (arm_dist + ab).max(0.0).min(1.0)) * body_height,
            ),
        ]);
    }
    routes.push(vec![
        (x, y - elder_pos * body_height),
        (x, y - body_height),
        (x + 0.05 * body_height, y - 0.95 * body_height),
        (x - 0.05 * body_height, y - 0.95 * body_height),
        (x, y - body_height),
    ]);

    if with_weapon {
        let yv = rng.gen_range(0.1, 0.9);
        routes.push(vec![
            (x - rng.gen_range(2.0, 5.0), y - (yv + 0.4) * body_height),
            (
                x + rng.gen_range(2.0, 5.0),
                y - (1.0 - yv + 0.3) * body_height,
            ),
        ])
    }
}

// floating rocks attached by cords

fn art_boats<R: Rng>(
    rng: &mut R,
    routes: &mut Vec<Vec<(f64, f64)>>,
    count: usize,
    width: f64,
    height: f64,
) {
    let w1base = rng.gen_range(3., 7.);
    let w2base = w1base + rng.gen_range(-1.0, 1.0);
    for _j in 0..count {
        let x = width / 2. + rng.gen_range(-1.0, 1.0) * rng.gen_range(0.1, 0.4) * width;
        let dist = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
        let y = height * 0.6 + dist * 0.2 * height;
        let scale = 0.4 + dist;
        let curvy1dt = scale * rng.gen_range(-2., 1.);
        let curvy2dt = scale * rng.gen_range(-2.0, 1.0);
        let curvy1 = curvy1dt + scale * rng.gen_range(-1.0, 1.0);
        let curvy2 = curvy2dt + scale * rng.gen_range(-1.0, 1.0);
        for i in 0..6 {
            let dy = i as f64 * 0.2 - 0.1;
            let w1 = scale * (w1base + rng.gen_range(-0.6, 0.6));
            let w2 = scale * (w2base + rng.gen_range(-0.3, 0.3));
            let h1 = scale * (3.0 + 2.0 * rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0));
            let h2 = scale * (3.0 + 2.0 * rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0));
            let base_route = vec![
                (x - w1 + curvy1, y + dy - h1),
                (x - w1, y + dy - h1),
                (x - w1, y + dy),
                (x + w2, y + dy),
                (x + w2, y + dy - h2),
                (x + w2 - curvy2, y + dy - h2),
            ];

            let route =
                path_subdivide_to_curve(base_route, 2, mix(0.72, 0.78, rng.gen_range(0.0, 1.0)));
            routes.push(route);
            /*
            routes.push(vec![
                (x + xoff, y + dy),
                (x + polexoff + xoff, y + dy - poleh * ymul),
            ]);*/
        }

        let xoff = scale * 2.0 * (rng.gen_range(0.0, 1.0) - 0.5);
        let bodyh = scale * rng.gen_range(5.0, 7.0);
        random_people(rng, routes, (x + xoff, y), bodyh, false);
        let polexoff = scale * 5.0 * rng.gen_range(-0.5, 0.5);
        routes.push(vec![
            (x - polexoff + xoff, y - bodyh * rng.gen_range(1.0, 1.5)),
            (x + polexoff + xoff, y + bodyh * rng.gen_range(0.3, 0.6)),
        ]);
    }
}

pub fn art(opts: &Opts) -> Document {
    let width: f64 = 210.0;
    let height: f64 = 297.0;
    let pad: f64 = 10.0;
    let ycenter = height / 2.0;
    let bounds = (pad, pad, width - pad, height - pad);
    let mut rng = rng_from_seed(opts.seed);

    let mut passage = Passage2DCounter::new(0.5, width, height);
    let max_passage = 10;

    let precision = 0.1;
    let noise_amp = 0.46 + rng.gen_range(-0.3, 0.4) * rng.gen_range(0.0, 1.0);
    let perlinpow = rng.gen_range(0.5, 4.0);
    let max_mountain_layers = 10;
    let mountain_density = 4.0;
    let max_sun_radius: f64 = 40.;
    let sun_density = 16.;
    let max_group_of_birds = 3;
    let boats =
        (rng.gen_range(0.0, 60.0) * rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0)) as usize;
    let reflection_probability = 0.04;
    let perlin = Perlin::new();

    let cloud_add = rng.gen_range(-0.7, 1.8) * rng.gen_range(0.0, 1.0);

    let people_samples = (rng.gen_range(0., 50.) * rng.gen_range(0.0, 1.0)) as usize;
    let people_f = rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0);
    let people_threshold = rng.gen_range(0.1, 0.7);
    let mut people_count = 0;
    let mut birds_total = 0;

    let objs_f = rng.gen_range(0.01, 0.03);
    let menhir_threshold = rng.gen_range(0.15, 0.9);
    let menhir_stability_threshold = 0.1;
    let menhir_stability_lookup = 4;
    let objs_xincr = rng.gen_range(6.0, 20.0);
    let mut mammoth_threshold = 0.7 - rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);

    let fire_randomness = rng.gen_range(0.2, 1.0) * rng.gen_range(0.01, 1.0);

    let mut menhir_count = 0;
    let mut mammoth_count = 0;
    let mut fire_count = 0;
    let mut tipi_count = 0;

    let mut layer_primary: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut layer_secondary: Vec<Vec<(f64, f64)>> = Vec::new();

    // ~~~ STEP ~~~ build the mountains

    let x_increment = precision; // precision of the strokes in the mountain
    let mut y_increment = 1.0 / mountain_density; // the base distance between lines in the mountain
                                                  // store the highest points reached by a mountain to implement collision
    let mut heights = vec![height - pad; (width / x_increment) as usize];
    // For each mountain layer...
    let mountains = rng.gen_range(1, max_mountain_layers); // number of mountains layers
    let mountains_delta = 40.0 - rng.gen_range(0.0, 36.0) * rng.gen_range(0.2, 1.0)
        + rng.gen_range(0., 20.0) * rng.gen_range(0.0, 1.0) * rng.gen_range(0.2, 1.0); // defines the "stops" of each mountains layer
    let ystoppow = rng.gen_range(0.5, 2.);
    let precision_delta = (rng.gen_range(-0.1f64, 0.04)).max(0.0);
    for i in 0..mountains {
        let yfreqmul =
            0.1 + rng.gen_range(0.0, 4.0) + rng.gen_range(0.0, 16.0) * rng.gen_range(0.0, 1.0);
        y_increment *= 1.2;
        // the distance between lines in mountain will fade away with distance
        // we pick random perlin noise frequencies (f*) and amplitudes (amp*)
        // the different level of noises are composed with domain warping
        let f1 = rng.gen_range(0.002, 0.004);
        let f2 = 0.001 + rng.gen_range(0.001, 0.01) * rng.gen_range(0.2, 1.0);
        let f3 = rng.gen_range(0.01, 0.05) * rng.gen_range(0.5, 1.0);
        let amp1 = noise_amp / f1;
        let amp2 = rng.gen_range(0.3, 3.0) * rng.gen_range(0.1, 1.0);
        let amp3 = rng.gen_range(0.5, 2.0) * rng.gen_range(0.3, 1.0);
        let sf1 = rng.gen_range(0.005, 0.01) * rng.gen_range(0.5, 1.0);
        let sf2 = rng.gen_range(0.005, 0.02) * rng.gen_range(0.5, 1.0);
        let ampnoise2 = rng.gen_range(0.0, 0.1) * rng.gen_range(0.2, 1.0);
        let ampnoise3 = rng.gen_range(0.0, 0.05) * rng.gen_range(0.2, 1.0);
        let samp2 = noise_amp * rng.gen_range(3.0, 5.0);
        let ystop =
            0.5 * height - ((i as f64 + 1.0) / (mountains as f64)).powf(ystoppow) * mountains_delta;
        let perlin_seed = rng.gen_range(0.0, 1000.0);
        // For each line of the mountain
        let mut ybase = ycenter;
        loop {
            if ybase < ystop {
                break;
            }
            let amp1mul = 0.8 * smoothstep(ycenter - 0.5, ycenter - 10., ybase)
                + 0.2 * smoothstep(ycenter, 0., ybase);
            if i > 0 {
                //amp1mul = 0.5 + 0.5 * amp1mul;
            }
            let mut route = Vec::new();
            let freqmul = 0.6 - (ybase - ycenter) / height;
            let mut xi = 0;
            let mut x = pad;
            let x_increment_modified = if precision_delta < 0.00001 {
                1.0
            } else {
                rng.gen_range(1. - precision_delta, 1. + precision_delta)
            } * x_increment;
            // we iterate on X to build up the mountain
            loop {
                if x > width - pad {
                    break;
                }
                let dx = x - width / 2.;
                let dy = ybase - height / 3.;

                let amp1mul2 = 1. - 0.5 * smoothstep(0., 100., (dx * dx + dy * dy).sqrt());
                let y = ybase + perlin.get([x * 0.02, 7.7 * perlin_seed])
                    - amp1mul
                        * amp1mul2
                        * amp1
                        * (0.3
                            + (3. * perlin.get([perlin_seed, 0.002 * x]))
                                .abs()
                                .powf(perlinpow)
                                .min(1.0))
                        * (0.8
                            * perlin.get([
                                f1 * x * freqmul,
                                yfreqmul * f1 * ybase * freqmul,
                                perlin_seed / 3.3
                                    + amp2
                                        * perlin.get([
                                            -5.5 * perlin_seed,
                                            f2 * x * freqmul,
                                            yfreqmul * f2 * ybase * freqmul,
                                        ])
                                    - amp3
                                        * perlin.get([
                                            f3 * x * freqmul,
                                            perlin_seed,
                                            yfreqmul * f3 * ybase * freqmul,
                                        ]),
                            ])
                            - 0.2
                                * perlin
                                    .get([
                                        sf1 * x * freqmul,
                                        yfreqmul * sf1 * ybase * freqmul,
                                        -perlin_seed
                                            + samp2
                                                * perlin.get([
                                                    sf2 * x * freqmul,
                                                    yfreqmul * sf2 * ybase * freqmul,
                                                    8. * perlin_seed,
                                                ]),
                                    ])
                                    .abs()
                                    .powf(perlinpow)
                            - ampnoise2
                                * perlin.get([
                                    0.3 * x * freqmul,
                                    yfreqmul * 0.6 * ybase * freqmul,
                                    perlin_seed / 1.7,
                                ])
                            - ampnoise3
                                * perlin.get([
                                    0.6 * x * freqmul,
                                    yfreqmul * 0.2 * ybase * freqmul,
                                    -perlin_seed * 3.3,
                                ]));
                let h = heights[xi];
                let p = (x, y);
                if y < h + 0.2 && y > pad && passage.count(p) < max_passage {
                    heights[xi] = y;
                    route.push(p);
                } else {
                    if route.len() > 1 {
                        layer_primary.push(route);
                    }
                    route = Vec::new();
                }
                xi = (xi + 1).min(heights.len() - 1);
                x += x_increment_modified;
            }

            if route.len() > 1 {
                layer_primary.push(route);
            }

            ybase -= y_increment;
        }

        let mut mammoths_positions = Vec::new();
        let mut menhir_positions = Vec::new();

        let mut x = pad + 10.0;
        loop {
            if x > width - pad - 10.0 {
                break;
            }
            let xi = ((x - pad) / x_increment) as usize;
            let y1 = heights[(xi - menhir_stability_lookup) % heights.len()];
            let y2 = heights[(xi + menhir_stability_lookup) % heights.len()];
            let y = heights[xi];
            if y < pad + 10.0 || (y1 - y2).abs() > menhir_stability_threshold {
                x += objs_xincr;
                continue;
            }
            let v = perlin.get([objs_f * x, objs_f * y, 5.6 * opts.seed]);
            if v > menhir_threshold {
                random_menhir(&mut rng, &mut passage, &mut layer_primary, (x, y));
                menhir_positions.push((x, y));
                menhir_count += 1;
                x += 5.0;
            } else if -v > mammoth_threshold && i == 0 {
                let body_height = 3.0 + rng.gen_range(0.0, 4.0) * rng.gen_range(0.3, 1.0);
                random_mammoth(
                    &mut rng,
                    &mut passage,
                    &mut layer_primary,
                    (x, y),
                    body_height,
                );
                mammoth_count += 1;
                mammoths_positions.push((x, y));
                mammoth_threshold += rng.gen_range(0.0, 0.08);
                x += 10.0;
            }
            x += objs_xincr;
        }

        let mut people_positions = Vec::new();

        for _p in 0..people_samples {
            let x = rng.gen_range(pad + 10.0, width - pad - 10.0);
            let xi = ((x - pad) / x_increment) as usize;
            let y = heights[xi];
            let v = perlin.get([people_f * x, people_f * y, 4.4 + 0.2 * opts.seed]);
            let ymul = 0.5
                + perlin
                    .get([
                        0.5 * people_f * x,
                        0.5 * people_f * y,
                        5.5 - 7.7 * opts.seed,
                    ])
                    .abs();
            if v > people_threshold && y > pad + 10.0 {
                let body_height = rng.gen_range(2.0, 2.5) * ymul;
                let pos = (x, y);
                let with_weapon = mammoths_positions
                    .iter()
                    .any(|&p| euclidian_dist(p, pos) < 50.0);
                random_people(&mut rng, &mut layer_primary, pos, body_height, with_weapon);
                people_positions.push((x, y));
                people_count += 1;
            }
        }
        if people_positions.len() > 0 {
            let extra_objects = (rng.gen_range(0.0, 1.0 + ((people_positions.len() as f64) * 0.8))
                * rng.gen_range(0.0, 1.0)
                * rng.gen_range(0.0, 1.0))
            .min(20.0) as usize;
            if extra_objects > 0 {
                // find all possible slots
                let mut x = pad + 10.0;
                let mut slots = Vec::new();
                let avoiding_objects = vec![menhir_positions, mammoths_positions].concat();
                let obj_distance_bailout = 20.0;
                let min_distance_to_people_bailout = 10.0;
                let max_distance_to_people_bailout = 50.0;
                let xincr = rng.gen_range(4.0, 6.0);
                loop {
                    if x > width - pad - 10.0 {
                        break;
                    }
                    if avoiding_objects
                        .iter()
                        .any(|&p| (p.0 - x).abs() < obj_distance_bailout)
                        || people_positions
                            .iter()
                            .any(|&p| (p.0 - x).abs() < min_distance_to_people_bailout)
                        || people_positions
                            .iter()
                            .all(|&p| (p.0 - x).abs() > max_distance_to_people_bailout)
                    {
                        x += xincr;
                        continue;
                    }

                    let xi = ((x - pad) / x_increment) as usize;
                    let y1 = heights[(xi - 3) % heights.len()];
                    let y2 = heights[(xi + 3) % heights.len()];
                    let y = heights[xi];
                    let dyabs = (y1 - y2).abs();

                    let stability_threshold = 0.1;

                    if y < pad + 10.0 || dyabs > stability_threshold {
                        x += xincr;
                        continue;
                    }

                    slots.push((x, y));
                    x += xincr;
                }

                rng.shuffle(&mut slots);
                slots.truncate(extra_objects);
                for p in slots {
                    if rng.gen_bool(fire_randomness) {
                        random_fire(
                            &mut rng,
                            &mut passage,
                            &mut layer_primary,
                            &mut layer_secondary,
                            p,
                        );
                        fire_count += 1;
                    } else {
                        random_tipi(&mut rng, &mut passage, &mut layer_primary, (p.0, p.1 + 0.3));
                        tipi_count += 1;
                    }
                }
            }
        }
    }

    // look for a possible bridge candidate, using moving average
    let smooth = 64;
    let sf = smooth as f64;
    let mut sum = 0.0;
    let mut acc = Vec::new();
    let mut smooth_heights = Vec::new();
    for (i, h) in heights.iter().enumerate() {
        if acc.len() == smooth {
            let avg = sum / sf;
            let xtheoric = pad + (i as f64 - sf / 2.0) * x_increment;
            smooth_heights.push((xtheoric, avg));
            let prev = acc.remove(0);
            sum -= prev;
        }
        acc.push(h);
        sum += h;
    }

    smooth_heights.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let tries = 100;
    let max_bridge_width = rng.gen_range(30.0, 80.0);
    let mut has_bridge = false;
    for t in 0..tries {
        let i = (t * 10) % smooth_heights.len();
        let a = smooth_heights[i];
        let maybe_b = smooth_heights.iter().find(|&&b| {
            let d = (a.0 - b.0).abs();
            if 10.0 < d && d < max_bridge_width {
                let dx = (a.0 - b.0).abs();
                let dy = (a.1 - b.1).abs();
                if dy < 1.0 || dx / dy > 4.0 {
                    let left = if a.0 < b.0 { a } else { b };
                    let right = if a.0 > b.0 { a } else { b };
                    let leftxi = ((left.0 - pad) / x_increment) as usize;
                    let rightxi = ((right.0 - pad) / x_increment) as usize;
                    let mut area = 0.0;
                    let l = (rightxi - leftxi) as f64;
                    for xi in leftxi..rightxi {
                        let xp = (xi - leftxi) as f64 / l;
                        let liney = mix(left.1, right.1, xp);
                        let dy = heights[xi] - liney;
                        if dy < 0.0 {
                            area += -dy * dy; // square of the era if it's traversing the bridge
                        } else {
                            area += dy;
                        }
                    }
                    area *= x_increment;
                    if area / dx > 10.0 {
                        return true;
                    }
                }
            }
            return false;
        });

        if let Some(&b) = maybe_b {
            layer_primary.push(vec![a, b]);
            let nb_people = (rng.gen_range(0.0, 9.0) * rng.gen_range(0.0, 1.0)) as usize;
            people_count += nb_people;
            random_bridge(&mut rng, &mut layer_primary, a, b, nb_people);
            has_bridge = true;
            break;
        }
    }

    // ~~~ STEP ~~~ chose a place for the possible sun.
    // find the lowest point of the mountain
    let mut lowxi = 0;
    let mut lowy = 0.0;
    let padend = (2. * (pad / x_increment)) as usize;
    let padxi = (heights.len() as f64 * rng.gen_range(0.1, 0.25)) as usize;
    for xi in padxi..(heights.len() - padend - padxi) {
        let y = heights[xi];
        if y > lowy {
            lowy = y;
            lowxi = xi;
        }
    }
    let lowx = pad + lowxi as f64 * x_increment;

    let low_cap = rng.gen_range(0.1, 1.0);
    let center = (lowx, lowy * rng.gen_range(low_cap, 1.1));
    let radius = (max_sun_radius * rng.gen_range(0.2, 1.0))
        .min(width - pad - center.0)
        .min(center.0 - pad)
        .min(center.1 - pad);

    let mut sun_part = 0.0;

    if radius > 5.0 {
        let mut route = Vec::new();
        let spins = sun_density;
        let mut rbase = radius + 0.5;
        let mut a: f64 = 0.0;
        let mut inside_count = 0;
        let mut total_count = 0;
        loop {
            if rbase < 0.05 {
                break;
            }
            let r = rbase.min(radius);
            let aincr = precision / (r + 1.0);
            let rincr = (0.9 * aincr) / spins;
            let p = (center.0 + r * a.cos(), center.1 + r * a.sin());
            let xi = ((p.0 - pad) / x_increment) as usize;
            let h = heights[xi];
            total_count += 1;
            if p.1 < h {
                inside_count += 1;
                route.push(p);
            } else {
                if route.len() > 1 {
                    layer_secondary.push(route);
                }
                route = Vec::new();
            }
            rbase -= rincr;
            a += aincr;
        }

        if route.len() > 1 {
            layer_secondary.push(route);
        }

        sun_part = (inside_count as f64) / (total_count as f64);
    }

    // ~~~ STEP ~~~ place birbs
    let groups = rng.gen_range(0, max_group_of_birds);
    for _i in 0..groups {
        let lowx = width / 2.0 + rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, width - 2.0 * pad);
        let xi = ((lowx - pad) / x_increment) as usize;
        let lowy = heights[xi];
        let center = (lowx, lowy * rng.gen_range(0.2, 0.8));
        let radius = // radius of a circle in which we place some birds
          rng.gen_range(0f64, 100.0).min(
            width - pad - center.0).min(
            center.0 - pad).min(
            center.1 - pad) * rng.gen_range(0.5, 1.0);
        let golden_angle = PI * (3. - (5f64).sqrt());
        let count_birds = (rng.gen_range(0.0, 2.0) * radius - rng.gen_range(0.0, 4.0)) as usize;
        let radius_from = 2.0;

        for i in 0..count_birds {
            let f = i as f64;
            let a = f * golden_angle;
            let amp = radius_from + (radius - radius_from) * (f / (count_birds as f64)).powf(0.6);
            let x = center.0 + amp * a.cos() + rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 10.);
            let y = center.1 + amp * a.sin() + rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 10.);
            let size = rng.gen_range(1.5, 3.0);
            let dx = size * rng.gen_range(0.3, 0.5);
            let dy = size * rng.gen_range(0.3, 0.5);
            layer_primary.push(path_subdivide_to_curve(
                vec![(x - dx, y - dy), (x, y + dy), (x + dx, y - dy)],
                2,
                rng.gen_range(0.7, 0.9),
            ));
        }
        birds_total += count_birds;
    }

    // ~~~ STEP ~~~ clouds
    let mut cloud_count = 0;
    let dy = 1.0;
    let dx = 1.0;
    let thresholds = vec![0.0, 0.4, 0.2, 0.6];
    let mut y = pad;
    let mut yi = 0;
    loop {
        let mut route = Vec::new();
        if y > ycenter {
            break;
        }
        let ythreshold = thresholds[yi % thresholds.len()];

        let mut x = pad;
        loop {
            if x > width - pad {
                break;
            }

            let xi = ((x - pad) / x_increment) as usize;
            let lowy = heights[xi];

            let p = (x, y);

            let should_draw = (0.2 * cloud_add
                + 0.7
                    * perlin.get([
                        opts.seed
                            + perlin.get([
                                //
                                0.02 * x,
                                7.7 * opts.seed,
                                0.01 * y,
                            ]),
                        0.005 * x,
                        0.02 * y,
                    ])
                + 0.3 * perlin.get([0.002 * x, 0.1 * y, opts.seed / 5.]))
                * (0.5 * cloud_add
                    + 2.0 * perlin.get([-3. - opts.seed / 7., 0.001 * x, 0.001 * y]))
                * (cloud_add + perlin.get([5. * opts.seed / 7., 0.004 * x, 0.004 * y]))
                > ythreshold + 0.001 * (route.len() as f64) + 0.3 * smoothstep(0.0, ycenter, y);

            if y < lowy - 1.0 && should_draw {
                cloud_count += 1;
                route.push(p);
            } else {
                if route.len() > 1 {
                    layer_secondary.push(route);
                }
                route = Vec::new();
            }

            x += dx;
        }

        if route.len() > 1 {
            layer_secondary.push(route);
        }

        y += dy;
        yi += 1;
    }

    let cloud_density = cloud_count as f64 / (dx * dy * (width - 2.0 * pad) * (height / 2.0 - pad));

    reflect_shapes(
        &mut rng,
        &mut layer_primary,
        reflection_probability,
        ycenter,
        bounds,
    );

    reflect_shapes(
        &mut rng,
        &mut layer_secondary,
        reflection_probability,
        ycenter,
        bounds,
    );

    art_boats(&mut rng, &mut layer_primary, boats, width, height);

    let mut inks = Vec::new();
    let layers: Vec<Group> = vec![
        ("#0FF", opts.primary_name.clone(), layer_primary),
        ("#F0F", opts.secondary_name.clone(), layer_secondary),
    ]
    .iter()
    .filter(|(_color, _label, routes)| routes.len() > 0)
    .map(|(color, label, routes)| {
        inks.push(label.clone());
        let mut l = Group::new()
            .set("inkscape:groupmode", "layer")
            .set("inkscape:label", label.clone())
            .set("fill", "none")
            .set("stroke", color.clone())
            .set("stroke-linecap", "round")
            .set("stroke-width", 0.35);

        let opacity: f64 = 0.65;
        let opdiff = 0.15 / (routes.len() as f64);
        let mut trace = 0f64;
        for route in routes.clone() {
            trace += 1f64;
            let data = render_route(Data::new(), route);
            l = l.add(
                Path::new()
                    .set(
                        "opacity",
                        (1000. * (opacity - trace * opdiff)).floor() / 1000.0,
                    )
                    .set("d", data),
            );
        }

        l
    })
    .collect();

    inks.sort();
    if inks.len() == 2 && inks[0].eq(&inks[1]) {
        inks.remove(1);
    }

    let mut map = Map::new();
    map.insert(String::from("Inks Count"), json!(inks.len()));
    map.insert(String::from("Inks"), json!(inks.join(" + ")));
    if boats > 0 {
        map.insert(
            String::from("Boats"),
            json!(if boats <= 1 {
                "Solo"
            } else if boats <= 2 {
                "Duo"
            } else if boats <= 9 {
                "Few"
            } else {
                "Many"
            }),
        );
    }
    if birds_total > 0 {
        map.insert(
            String::from("Birds"),
            json!(if birds_total <= 1 {
                "Alone"
            } else if birds_total <= 9 {
                "Few"
            } else {
                "Colony"
            }),
        );
    }
    if people_count > 0 {
        map.insert(
            String::from("People"),
            json!(if people_count <= 1 {
                "Solo"
            } else if people_count <= 2 {
                "Duo"
            } else if people_count <= 9 {
                "Few"
            } else {
                "Many"
            }),
        );
    }
    if cloud_density > 0.00001 {
        map.insert(
            String::from("Cloud Density"),
            json!(if cloud_density < 0.01 {
                "Small"
            } else if cloud_density <= 0.05 {
                "Medium"
            } else if cloud_density <= 0.2 {
                "High"
            } else {
                "Very High"
            }),
        );
    }

    if mammoth_count > 0 {
        map.insert(
            String::from("Mammoth"),
            json!(if mammoth_count <= 1 {
                "One"
            } else if mammoth_count <= 2 {
                "Two"
            } else if mammoth_count <= 5 {
                "Few"
            } else {
                "Many"
            }),
        );
    }
    if fire_count > 0 {
        map.insert(
            String::from("Fire"),
            json!(if fire_count <= 1 {
                "One"
            } else if fire_count <= 2 {
                "Two"
            } else if fire_count <= 5 {
                "Few"
            } else {
                "Many"
            }),
        );
    }
    if tipi_count > 0 {
        map.insert(
            String::from("Tipi"),
            json!(if tipi_count <= 1 {
                "One"
            } else if tipi_count <= 2 {
                "Two"
            } else if tipi_count <= 5 {
                "Few"
            } else {
                "Many"
            }),
        );
    }
    if menhir_count > 0 {
        map.insert(
            String::from("Menhir"),
            json!(if menhir_count <= 1 {
                "One"
            } else if menhir_count <= 2 {
                "Two"
            } else if menhir_count <= 5 {
                "Few"
            } else {
                "Many"
            }),
        );
    }

    let mut lowy = height;
    for v in heights {
        if v < lowy {
            lowy = v;
        }
    }
    let heightfactor = (lowy - pad) / (ycenter - pad);
    map.insert(
        String::from("Elevation"),
        json!(if heightfactor < 0.05 {
            "Extreme"
        } else if heightfactor < 0.2 {
            "Very High"
        } else if heightfactor < 0.4 {
            "High"
        } else if heightfactor < 0.66 {
            "Regular"
        } else if heightfactor < 0.8 {
            "Low"
        } else {
            "Very Low"
        }),
    );

    map.insert(
        String::from("Sunset"),
        json!(if sun_part < 0.05 {
            "Finished"
        } else if sun_part < 0.2 {
            "Ending"
        } else if sun_part < 0.8 {
            "In-between"
        } else if sun_part < 0.99 {
            "Beginning"
        } else {
            "Not Started"
        }),
    );

    map.insert(
        String::from("Sun Radius"),
        json!(if radius < 14.0 {
            "Small"
        } else if radius < 25.0 {
            "Medium"
        } else if radius < 35.0 {
            "Big"
        } else {
            "Very big"
        }),
    );

    if has_bridge {
        map.insert(String::from("Bridge"), json!(String::from("Yes")));
    }

    let traits = Value::Object(map);

    let mut document = svg::Document::new()
        .set("data-hash", opts.hash.to_string())
        .set("data-traits", traits.to_string())
        .set("viewBox", (0, 0, width, height))
        .set("width", format!("{}mm", width))
        .set("height", format!("{}mm", height))
        .set("style", "background:white")
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("xmlns", "http://www.w3.org/2000/svg");
    for l in layers {
        document = document.add(l);
    }
    document
}

#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
    let opts = val.into_serde().unwrap();
    let doc = art(&opts);
    let str = doc.to_string();
    return str;
}

fn render_route(data: Data, route: Vec<(f64, f64)>) -> Data {
    if route.len() == 0 {
        return data;
    }
    let first_p = route[0];
    if f64::is_nan(first_p.0) {
        let mut copy = route.clone();
        copy.remove(0);
        return render_route_curve(data, copy);
    }
    let mut d = data.move_to((significant_str(first_p.0), significant_str(first_p.1)));
    for p in route {
        d = d.line_to((significant_str(p.0), significant_str(p.1)));
    }
    return d;
}

pub fn render_route_curve(data: Data, route: Vec<(f64, f64)>) -> Data {
    if route.len() == 0 {
        return data;
    }
    let mut first = true;
    let mut d = data;
    let mut last = route[0];
    for p in route {
        if first {
            first = false;
            d = d.move_to((significant_str(p.0), significant_str(p.1)));
        } else {
            d = d.quadratic_curve_to((
                significant_str(last.0),
                significant_str(last.1),
                significant_str((p.0 + last.0) / 2.),
                significant_str((p.1 + last.1) / 2.),
            ));
        }
        last = p;
    }
    return d;
}

#[inline]
fn significant_str(f: f64) -> f64 {
    (f * 100.0).floor() / 100.0
}

fn rng_from_seed(s: f64) -> impl Rng {
    let mut bs = [0; 16];
    bs.as_mut().write_f64::<BigEndian>(s).unwrap();
    let mut rng = SmallRng::from_seed(bs);
    // run it a while to have better randomness
    for _i in 0..50 {
        rng.gen::<f64>();
    }
    return rng;
}

#[inline]
fn mix(a: f64, b: f64, x: f64) -> f64 {
    (1. - x) * a + x * b
}

#[inline]
fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    return (dx * dx + dy * dy).sqrt();
}

#[inline]
fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let k = ((x - a) / (b - a)).max(0.0).min(1.0);
    return k * k * (3.0 - 2.0 * k);
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
    (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn path_subdivide_to_curve_it(path: Vec<(f64, f64)>, interpolation: f64) -> Vec<(f64, f64)> {
    let l = path.len();
    if l < 3 {
        return path;
    }
    let mut route = Vec::new();
    let mut first = path[0];
    let mut last = path[l - 1];
    let looped = euclidian_dist(first, last) < 0.1;
    if looped {
        first = lerp_point(path[1], first, interpolation);
    }
    route.push(first);
    for i in 1..(l - 1) {
        let p = path[i];
        let p1 = lerp_point(path[i - 1], p, interpolation);
        let p2 = lerp_point(path[i + 1], p, interpolation);
        route.push(p1);
        route.push(p2);
    }
    if looped {
        last = lerp_point(path[l - 2], last, interpolation);
    }
    route.push(last);
    if looped {
        route.push(first);
    }
    route
}

fn path_subdivide_to_curve(path: Vec<(f64, f64)>, n: usize, interpolation: f64) -> Vec<(f64, f64)> {
    let mut route = path;
    for _i in 0..n {
        route = path_subdivide_to_curve_it(route, interpolation);
    }
    route
}

#[derive(Clone)]
pub struct Passage2DCounter {
    granularity: f64,
    width: f64,
    height: f64,
    counters: Vec<usize>,
}
impl Passage2DCounter {
    pub fn new(granularity: f64, width: f64, height: f64) -> Self {
        let wi = (width / granularity).ceil() as usize;
        let hi = (height / granularity).ceil() as usize;
        let counters = vec![0; wi * hi];
        Passage2DCounter {
            granularity,
            width,
            height,
            counters,
        }
    }
    fn index(self: &Self, (x, y): (f64, f64)) -> usize {
        let wi = (self.width / self.granularity).ceil() as usize;
        let hi = (self.height / self.granularity).ceil() as usize;
        let xi = ((x / self.granularity).round() as usize).max(0).min(wi - 1);
        let yi = ((y / self.granularity).round() as usize).max(0).min(hi - 1);
        yi * wi + xi
    }
    pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
        let i = self.index(p);
        let v = self.counters[i] + 1;
        self.counters[i] = v;
        v
    }
    pub fn count_n(self: &mut Self, p: (f64, f64), n: usize) -> usize {
        let i = self.index(p);
        let v = self.counters[i] + n;
        self.counters[i] = v;
        v
    }
    pub fn get(self: &Self, p: (f64, f64)) -> usize {
        self.counters[self.index(p)]
    }
}

// skipped ideas

/*
fn art_lantern(routes: &mut Vec<Vec<(f64, f64)>>, (cx, cy): (f64, f64), size: f64) {
    let fromr = 0.2 * size;
    let tor = 0.4 * size;
    let fromy = cy + 0.4 * size;
    let toy = cy - 0.2 * size;
    let loops = 6.0;
    let twopi = 2.0 * PI;
    let mut a = 0.0;
    let mut route = Vec::new();
    loop {
        if a > loops * twopi {
            break;
        }
        let m = a / (twopi * loops);
        let r = mix(fromr, tor, m);
        let dy = mix(fromy, toy, m);
        let x = cx + r * a.cos();
        let y = cy + r * a.sin() + dy;
        route.push((x, y));
        a += 0.4;
    }
    routes.push(route);
}
*/

/*
// ~~~ STEP ~~~ lanterns
for (x, _y) in people_positions {
    let xi = ((x - pad) / x_increment) as usize;
    let h = heights[xi];
    let from = pad + 10.0;
    let to = h - 10.0;
    if from < to {
        let y = rng.gen_range(from, to);
        let size = rng.gen_range(2.0, 3.0);
        art_lantern(&mut layer_secondary, (x, y), size);
    }
}
*/
