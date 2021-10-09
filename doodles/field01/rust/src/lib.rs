mod utils;
use noise::*;
use std::f64::consts::PI;
use byteorder::*;
use svg::node::element::path::Data;
use svg::node::element::*;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Opts {
    pub seed: f64,
}

pub fn art(opts: &Opts) -> Vec<Group> {
    let width = 200.0;
    let height = 200.0;
    let stroke_width = 0.3;
    let pad = 0.0;
    let perlin = Perlin::new();
    let bounds = (
        pad,
        pad,
        width - pad,
        height - pad,
    );
    let mut passage = Passage2DCounter::new(
        0.005, 1.0, 1.0,
    );
    
    fn length(l: (f64, f64)) -> f64 {
        (l.0 * l.0 + l.1 * l.1).sqrt()
    }
    fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
        (
            a.cos() * p.0 + a.sin() * p.1,
            a.cos() * p.1 - a.sin() * p.0,
        )
    }
    fn op_union_round(a: f64, b: f64, r: f64) -> f64 {
        r.max(a.min(b)) - length(((r - a).max(0.), (r - b).max(0.)))
    }
    let  sdf_box2 = |(x,y):(f64,f64), (w, h):(f64,f64)| {
        let dx = x.abs() - w;
        let dy = y.abs() - h;
        length((dx.max(0.), dy.max(0.))) + dx.min(0.).max(dy.min(0.))
    };

    let mut rng = rng_from_seed(opts.seed);
    let rot = rng.gen_range(0.0, 2. * PI);
    let f1 = rng.gen_range(0.5, 5.0);
    let f2 = rng.gen_range(0.0, 12.0);
    let f3 = rng.gen_range(0.0, 16.0);
    let a1 = rng.gen_range(0.0, 1.0) * rng.gen_range(0f64, 1.0);
    let a2 = rng.gen_range(0.0, 3.0) * rng.gen_range(0.0, 1.0);
    let a3 = rng.gen_range(0.0, 3.0) * rng.gen_range(0.0, 1.0);
    let xmirror = rng.gen_range(0.0, 1.0) < 0.5;
    let ymirror = rng.gen_range(0.0, 1.0) < 0.3;
    let count = rng.gen_range(2.0, 16.0) as usize;
    let rects: Vec<((f64,f64), (f64, f64), f64, f64)> = (0..count).map(|_i| {
        let amp = rng.gen_range(0.0, 0.3);
        let r = rng.gen_range(0.0, 2. * PI);
        let offset = (r.cos() * amp, r.sin() * amp);
        let dim = (rng.gen_range(0.1, 0.3), rng.gen_range(0.1, 0.3));
        let ang = rng.gen_range(0f64, PI);
        let k = rng.gen_range(0.001, 0.2);
        (offset, dim, ang, k)
    }).collect();
    let outside = 0.0;
    let inside = rng.gen_range(0.4, 0.7);
    
    let f = |(x, y): (f64, f64)| {
        let mut c = ((x-0.5) * width / height, y-0.5);
        let mut s = 100.0;
        if xmirror {
            c.0 = c.0.abs();
        }
        if ymirror {
            c.1 = c.1.abs();
        }
        c = p_r(c, rot);
        for &(offset, dim, ang, k) in rects.iter() {
          let mut p = (c.0, c.1);
          p = p_r(p, ang);
          p.0 += offset.0;
          p.1 += offset.1;
          s = op_union_round(s, sdf_box2(p, dim), k);
        }
        let n = a1 * perlin.get([
            f1 * c.0,
            f1 * c.1,
            opts.seed
            + a2 * perlin.get([
                4. + opts.seed,
                f2 * c.0 + a3 * perlin.get([f3 * c.0, f3 * c.1, 20. + opts.seed]),
                f2 * c.1 + a3 * perlin.get([f3 * c.0, f3 * c.1, 30. + opts.seed])
              ])
            ]);
        lerp(-inside, outside, s) + n
    };
    
    let colors = vec!["#0FF", "#F0F"];
    let mut layers = Vec::new();
    for (ci, &color) in colors.iter().enumerate() {
        let mut routes = Vec::new();
        
        let samples = sample_2d_candidates(&|p| (f(p)<0.5) == (ci==0), 400, 3000, &mut rng);
        
        let precision = 0.008;
        let deltarot = 1.0;
        let target_size = 50;

        for sample in samples {
            let mut route = Vec::new();
            let mut p = sample;
            let mut ang = rng.gen_range(0.0, 2. * PI);
            loop {
                if out_of_boundaries(p, (0.0, 0.0, 1.0, 1.0)) {
                    break;
                }
                if route.len() >= target_size {
                    break;
                }
                if passage.count(p) > 8 {
                    break;
                }
                route.push(
                    project_in_boundaries(
                        p,
                        bounds
                    )
                );
                let front = (
                    p.0 + precision * ang.cos(),
                    p.1 + precision * ang.sin(),
                );
                let frontleft = (
                    p.0 + precision * (ang-deltarot).cos(),
                    p.1 + precision * (ang-deltarot).sin(),
                );
                let frontright = (
                    p.0 + precision * (ang+deltarot).cos(),
                    p.1 + precision * (ang+deltarot).sin(),
                );
                let value = f(p);
                let frontvalue = (f(front) - value).abs();
                let frontleftvalue = (f(frontleft) - value).abs();
                let frontrightvalue = (f(frontleft) - value).abs();
                if frontvalue < frontleftvalue && frontvalue < frontrightvalue {
                    p = front;
                }
                else if frontleftvalue < frontrightvalue {
                    p = frontleft;
                    ang -= deltarot;
                }
                else {
                    p = frontright;
                    ang += deltarot;
                }
            }
            routes.push(route);
        }

        let mut l = Group::new()
            .set("inkscape:groupmode", "layer")
            .set("inkscape:label", color)
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
        layers.push(l);
    }
        
    layers 
}

#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
    let opts = val.into_serde().unwrap();

    let mut g = Group::new();
    let a = art(&opts);
    for e in a {
        g = g.add(e);
    }
    let str = g.to_string();
    return str;
}

#[inline]
pub fn project_in_boundaries(
    p: (f64, f64),
    boundaries: (f64, f64, f64, f64),
) -> (f64, f64) {
    (
        significant_str(p.0 * (boundaries.2 - boundaries.0) + boundaries.0),
        significant_str(p.1 * (boundaries.3 - boundaries.1) + boundaries.1),
    )
}

#[inline]
pub fn strictly_in_boundaries(
    p: (f64, f64),
    boundaries: (f64, f64, f64, f64),
) -> bool {
    p.0 > boundaries.0
        && p.0 < boundaries.2
        && p.1 > boundaries.1
        && p.1 < boundaries.3
}

pub fn crop_route(
    route: &Vec<(f64, f64)>,
    boundaries: (f64, f64, f64, f64),
) -> Option<Vec<(f64, f64)>> {
    if route.len() < 2
        || route
            .iter()
            .all(|&p| !strictly_in_boundaries(p, boundaries))
    {
        return None;
    }
    return Some(route.clone());
}

pub fn crop_routes(
    routes: &Vec<Vec<(f64, f64)>>,
    boundaries: (f64, f64, f64, f64),
) -> Vec<Vec<(f64, f64)>> {
    return routes
        .iter()
        .filter_map(|route| crop_route(&route, boundaries))
        .collect();
}

#[inline]
fn significant_str (f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}

pub fn sample_2d_candidates(
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
fn lerp(a: f64, b: f64, x: f64) -> f64 {
    (x - a) / (b - a)
}

pub struct Passage2DCounter {
    granularity: f64,
    width: f64,
    height: f64,
    counters: Vec<usize>,
}
impl Passage2DCounter {
    pub fn new(
        granularity: f64,
        width: f64,
        height: f64,
    ) -> Self {
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
        let wi =
            (self.width / self.granularity).ceil() as usize;
        let hi = (self.height / self.granularity).ceil()
            as usize;
        let xi = ((x / self.granularity).round() as usize)
            .max(0)
            .min(wi - 1);
        let yi = ((y / self.granularity).round() as usize)
            .max(0)
            .min(hi - 1);
        yi * wi + xi
    }
    pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
        let i = self.index(p);
        let v = self.counters[i] + 1;
        self.counters[i] = v;
        v
    }
    pub fn get(self: &Self, p: (f64, f64)) -> usize {
        self.counters[self.index(p)]
    }
}
pub fn rng_from_seed(s: f64) -> impl Rng {
    let mut bs = [0; 16];
    bs.as_mut().write_f64::<BigEndian>(s).unwrap();
    let mut rng = SmallRng::from_seed(bs);
    for _i in 0..10 {
        rng.gen::<f64>();
    }
    return rng;
}

pub fn out_of_boundaries(
    p: (f64, f64),
    boundaries: (f64, f64, f64, f64),
) -> bool {
    p.0 < boundaries.0
        || p.0 > boundaries.2
        || p.1 < boundaries.1
        || p.1 > boundaries.3
}