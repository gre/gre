mod utils;
use geo::*;
use prelude::{BoundingRect, Contains};
use svg::node::element::path::Data;
use svg::node::element::{Path, Group};
use svg::Document;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use byteorder::*;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Opts {
    pub seed: f64,
    pub primary_name: String,
    pub secondary_name: String,
    pub image: ImageData,
    pub distribmode: usize,
    pub voronoi_size: usize
}

pub fn art(opts: &Opts) -> Document {
    let width = 210.0;
    let height = 210.0;
    let stroke_width = 0.35;

    let voronoi_size = opts.voronoi_size;
    let max_samples = 100;
    let samples_r = 0.05;
    let res = 80;
    let poly_threshold = 0.5;

    let mut rng = rng_from_seed(opts.seed);

    let project =
        |(x, y): (f64, f64)| (x * width, y * height);

    let distrib = match opts.distribmode {
        1 => {|p: (f64,f64)| (0.5 - euclidian_dist(p, (0.5, 0.5))).max(0.0)}
        2 => {|p: (f64,f64)| 0.5 - p.0.min(1.-p.0).min(p.1.min(1.-p.1))}
        3 => {|p: (f64,f64)| 0.5-(p.0-0.5).abs()}
        4 => {|p: (f64,f64)| 0.5-(p.1-0.5).abs()}
        _ => {|p: (f64,f64)| 1.0}
    };

    let candidates = sample_2d_candidates_f64(
        &distrib,
        800,
        voronoi_size,
        &mut rng,
    );

    let mut polys =
        sample_square_voronoi_polys(candidates, 0.05);

    // filter out big polygons (by their "squared" bounds)
    polys.retain(|poly| {
        poly_bounding_square_edge(poly) < poly_threshold
    });

    let get_color = dynamic_image_get_color(&opts.image);

    let get = |p| { 1. - get_color(p).0  };

    let routes: Vec<Vec<(f64, f64)>> = polys
        .iter()
        .map(|poly| {
            let bounds = poly.bounding_rect().unwrap();
            let min = bounds.min();
            let width = bounds.width();
            let height = bounds.height();
            let map_p = |(lx, ly)| {
                (min.x + width * lx, min.y + height * ly)
            };
            let mut candidates = sample_2d_candidates_f64(
                &|p| {
                    let ap = map_p(p);
                    if poly.contains(&geo::Point::new(
                        ap.0, ap.1,
                    )) {
                        samples_r * get(ap)
                    } else {
                        0.0
                    }
                },
                res,
                max_samples,
                &mut rng,
            );
            candidates = candidates
                .iter()
                .map(|&p| project(map_p(p)))
                .collect();
            if candidates.len() < 5 {
                vec![]
            } else {
                candidates.sort_by(|&a, &b| {
                    (a.0 - a.1)
                        .partial_cmp(&(b.0 - b.1))
                        .unwrap()
                        .then(
                            a.1.partial_cmp(&b.1).unwrap(),
                        )
                });
                candidates
            }
        })
        .collect();

    let colors = vec!["#0FF"];
    let layers = colors.iter().enumerate().map(|(ci, &color)| {

        let label = if ci == 0 { opts.primary_name.clone() } else { opts.secondary_name.clone() };

        let mut l = Group::new()
            .set("inkscape:groupmode", "layer")
            .set("inkscape:label", label)
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", stroke_width);

        let opacity: f64 = 0.6;
        
        let opdiff = 0.15 / (routes.len() as f64);
        let mut trace = 0f64;
        for r in routes.clone() {
            trace += 1f64;
            if r.len() < 2 {
                continue;
            }
            let data = render_route(Data::new(), r);
            l = l.add(
                Path::new()
                .set("opacity", (1000. * (opacity - trace * opdiff)).floor()/1000.0)
                .set("d", data)
            );
        }
        l
    });
        
    let mut doc = svg::Document::new()
    .set("viewBox", (0, 0, 210, 210))
    .set("width", "210mm")
    .set("height", "210mm")
    .set("style", "background:white")
    .set("xmlns:inkscape", "http://www.inkscape.org/namespaces/inkscape")
    .set("xmlns", "http://www.w3.org/2000/svg" );
    for g in layers {
        doc = doc.add(g);
    }
    return doc;
}

#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
    let opts = val.into_serde().unwrap();
    let doc = art(&opts);
    let str = doc.to_string();
    return str;
}

fn render_route(
    data: Data,
    route: Vec<(f64, f64)>
) -> Data {
    if route.len() == 0 {
        return data;
    }
    let first_p = route[0];
    let mut d = data.move_to((
        significant_str(first_p.0),
        significant_str(first_p.1)
    ));
    for p in route {
        d = d.line_to((
            significant_str(p.0),
            significant_str(p.1),
        ));
    }
    return d;
}

#[inline]
fn significant_str (f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}

fn sample_2d_candidates(
    f: &dyn Fn((f64, f64)) -> bool,
    dim: usize,
    samples: usize,
    rng: &mut impl Rng,
) -> Vec<(f64, f64)> {
    let mut candidates = Vec::new();
    for x in 0..dim {
        for y in 0..dim {
            let p = (
                (x as f64) / (dim as f64),
                (y as f64) / (dim as f64),
            );
            if f(p) {
                candidates.push(p);
            }
        }
    }
    rng.shuffle(&mut candidates);
    candidates.truncate(samples);
    return candidates;
}

fn render_route_when<F: FnMut(
    (f64, f64),
    (f64, f64),
) -> bool>(
    data: Data,
    route: Vec<(f64, f64)>,
    mut should_draw_line: F,
) -> Data {
    let mut first = true;
    let mut up = false;
    let mut last = (0.0, 0.0);
    let mut d = data;
    for p in route {
        if first {
            if should_draw_line(p, p) {
                first = false;
                d = d.move_to(p);
            }
        } else {
            if should_draw_line(last, p) {
                if up {
                    up = false;
                    d = d.move_to(last);
                }
                d = d.line_to(p);
            } else {
                up = true;
            }
        }
        last = p;
    }
    return d;
}

fn sample_square_voronoi_polys(
    candidates: Vec<(f64, f64)>,
    pad: f64,
) -> Vec<Polygon<f64>> {
    let mut points = Vec::new();
    for c in candidates {
        points.push(voronoi::Point::new(
            pad + (1.0 - 2.0 * pad) * c.0,
            pad + (1.0 - 2.0 * pad) * c.1,
        ));
    }
    let dcel = voronoi::voronoi(points, 1.0);
    let polys = voronoi::make_polygons(&dcel)
        .iter()
        .map(|pts| {
            Polygon::new(
                pts.iter()
                    .map(|p| (p.x(), p.y()))
                    .collect::<Vec<_>>()
                    .into(),
                vec![],
            )
        })
        .collect();
    polys
}


fn sample_2d_candidates_f64<R: Rng>(
    f: &dyn Fn((f64, f64)) -> f64,
    dim: usize,
    samples: usize,
    rng: &mut R,
) -> Vec<(f64, f64)> {
    let mut candidates = Vec::new();
    for x in 0..dim {
        for y in 0..dim {
            let p = (
                (x as f64) / (dim as f64),
                (y as f64) / (dim as f64),
            );
            if f(p) > rng.gen_range(0.0, 1.0) {
                candidates.push(p);
            }
        }
    }
    rng.shuffle(&mut candidates);
    candidates.truncate(samples);
    return candidates;
}

fn rasterize_1d<F: FnMut((f64, f64)) -> f64>(
    width: u32,
    height: u32,
    mut f: F,
) -> Vec<f64> {
    (0..height)
        .flat_map(|y| {
            (0..width)
                .map(|x| {
                    f((
                        x as f64 / width as f64,
                        y as f64 / height as f64,
                    ))
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<f64>>()
}

#[derive(Deserialize)]
pub struct ImageData {
  pub data: Vec<u8>,
  pub width: usize,
  pub height: usize
}
// point is normalized in 0..1
// returned value is a rgb tuple in 0..1 range
fn dynamic_image_get_color(
    img: &ImageData
) -> impl Fn((f64, f64)) -> (f64, f64, f64) {
    let width = img.width;
    let height = img.height;
    let d = img.data.clone();
    return move |(x, y): (f64, f64)| {
        // quadratic implementation
        let xi: f64 = x.max(0.0).min(1.0) * ((width - 1) as f64);
        let yi: f64 = y.max(0.0).min(1.0) * ((height - 1) as f64);
        let x1 = xi.floor() as usize;
        let x2 = xi.ceil() as usize;
        let y1 = yi.floor() as usize;
        let y2 = yi.ceil() as usize;
        let w1 = width * y1;
        let p1i = (x1 + w1) * 4;
        let p2i = (x2 + w1) * 4;
        let w2 = width * y2;
        let p3i = (x2 + w2) * 4;
        let p4i = (x1 + w2) * 4;
        let xp = xi - xi.floor();
        let yp = yi - yi.floor();
        let r = (mix(mix(d[p1i + 0] as f64, d[p2i + 0] as f64, xp), mix(d[p4i + 0] as f64, d[p3i + 0] as f64, xp), yp)) / 255.0;
        let g = (mix(mix(d[p1i + 1] as f64, d[p2i + 1] as f64, xp), mix(d[p4i + 1] as f64, d[p3i + 1] as f64, xp), yp)) / 255.0;
        let b = (mix(mix(d[p1i + 2] as f64, d[p2i + 2] as f64, xp), mix(d[p4i + 2] as f64, d[p3i + 2] as f64, xp), yp)) / 255.0;
        return (r, g, b);
    };
}
#[inline]
fn mix(a: f64, b: f64, x: f64) -> f64 {
    (1. - x) * a + x * b
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

fn poly_bounding_square_edge(
    poly: &Polygon<f64>,
) -> f64 {
    let bounds = poly.bounding_rect().unwrap();
    bounds.width().max(bounds.height())
}

#[inline]
fn out_of_boundaries(
    p: (f64, f64),
    boundaries: (f64, f64, f64, f64),
) -> bool {
    p.0 < boundaries.0
        || p.0 > boundaries.2
        || p.1 < boundaries.1
        || p.1 > boundaries.3
}


#[inline]
fn euclidian_dist(
    (x1, y1): (f64, f64),
    (x2, y2): (f64, f64),
) -> f64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    return (dx * dx + dy * dy).sqrt();
}
