use clap::Clap;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;


#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "3.0")]
    seed: f64,
    #[clap(short, long, default_value = "10")]
    samples: usize,
    #[clap(short, long)]
    reversed: bool,
}

#[derive(Clone, Debug)]
struct Road {
    tracks_count: usize,
    tracks_dist: f64,
    routes: Vec<(f64, f64)>,
    tracks: Vec<Vec<(f64, f64)>>
}

impl Road {
    fn new(
        head: (f64, f64),
        tracks_count: usize,
        tracks_dist: f64
    ) -> Self {
        let routes = vec![head];
        let tracks = (0..tracks_count).map(|_i| Vec::new()).collect();
        Road { tracks_count, tracks_dist, routes, tracks }
    }
    fn get_routes_to_draw(self: &Self) -> Vec<Vec<(f64, f64)>> {
        self.tracks.clone()
    }
    fn add(self: &mut Self, p: (f64, f64)) {
        self.routes.push(p);
    }
    fn mv(self: &mut Self, ang: f64, amp: f64) {
        // add a point into the main route
        let from = self.routes[self.routes.len()-1];
        let mut p = from;
        p.0 += amp * ang.cos();
        p.1 += amp * ang.sin();
        self.routes.push(p);
        // progress the tracks too
        let ang2 = ang + PI / 2.0;
        let dx = ang2.cos();
        let dy = ang2.sin();
        let mut dc = self.tracks_dist * (0.5 - self.tracks_count as f64 / 2.0);
        self.tracks = self.tracks.iter().map(|track| {
            let mut copy = track.clone();
            let l = copy.len();
            if l == 0 {
                copy.push((from.0 + dx * dc, from.1 + dy * dc));
            }
            copy.push((p.0 + dx * dc, p.1 + dy * dc));
            dc += self.tracks_dist;
            copy
        }).collect();
    }
    fn dist_point(self: &Self, p: (f64, f64)) -> f64 {
        let minleft = self.tracks[0].iter().map(|a| {
            let dx = a.0 - p.0;
            let dy = a.1 - p.1;
            return dx * dx + dy * dy;
        }).reduce(|a: f64, b: f64| a.min(b)).unwrap();
        let minright = self.tracks[self.tracks_count - 1].iter().map(|a| {
            let dx = a.0 - p.0;
            let dy = a.1 - p.1;
            return dx * dx + dy * dy;
        }).reduce(|a: f64, b: f64| a.min(b)).unwrap();
        minleft.min(minright).sqrt()
    }
}


fn art(opts: Opts) -> Vec<Group> {
    let colors = vec!["red", "black"];
    let track_count = 20;
    let track_dist = 0.8;
    let crop = 5;
    let pad = 20.0;
    let width = 297.0;
    let height = 210.0;
    let line_length = 200.0;
    let granularity = 0.8;

    let bounds = (pad, pad, width - pad, height - pad);

    let samples = opts.samples;
    let seed = opts.seed;
    let perlin = Perlin::new();
    let mut passage =
        Passage2DCounter::new(1.0, width, height);

    let mut rng = rng_from_seed(opts.seed);

    let e = rng.gen_range(0.1, 1.0);
    let f = rng.gen_range(0.1, 1.0);
    let g = rng.gen_range(0.1, 1.0);

    let get_angle = |p: (f64, f64), initial_angle, i| {
        initial_angle +
        e * perlin.get([
            2. * p.0 / width,
            2. * p.1 / height,
            10. + seed
            + f * perlin.get([
                5. * p.0 / width,
                5. * p.1 / height,
                10. + seed
                    + g * perlin.get([
                        8. * p.0 / width,
                        8. * p.1 / height,
                        seed + i as f64 / 100.0,
                    ])
            ])
        ])
    };

    let mut samples_data: Vec<(f64, (f64, f64))> = Vec::new();
    for x in 0..samples {
        let xp = (0.5 + x as f64) / (samples as f64);
        for y in 0..samples {
            let yp = (0.5 + y as f64 )/ (samples as f64);
            let initial_angle = if rng.gen_range(0.0, 1.0) < 0.2 { 0.0 } else { PI };
            let p = (
                pad + xp * (width - 2. * pad),
                pad + yp * (height - 2. * pad),
            );
            samples_data.push((initial_angle, p));
        }
    }

    rng.shuffle(&mut samples_data);

    let initial_positions =
        samples_data.iter().map(|&(_a, p)| p).collect();

    let mut build_route = |p, i, j| {
        let length = i as f64 * granularity;
        if length >= line_length {
            return None; // line ends
        }
        let (initial_angle, _o) = samples_data[j];
        let angle = get_angle(p, initial_angle, j);
        let nextp = follow_angle(p, angle, granularity);
        if let Some(edge_p) =
            collide_segment_boundaries(p, nextp, bounds)
        {
            return Some((edge_p, true));
        }
        let count = passage.count(nextp);
        if count > 2 {
            return None; // too much passage here
        }
        return Some((nextp, false));
    };

    let lines =
    build_routes(
        initial_positions,
        &mut build_route,
    );

    let mut roads = Vec::new();

    for line in lines {
        if line.len() < crop * 3 {
            continue;
        }
        let mut last = line[crop];
        let mut road = Road::new(last, track_count, track_dist);
        for &p in line.iter().skip(crop + 1).take(line.len() - 2 * crop) {
            let dx = p.0 - last.0;
            let dy = p.1 - last.1;
            let ang = dy.atan2(dx);
            let amp = (dx * dx + dy * dy).sqrt();
            road.mv(ang, amp);
            last = p;
        }
        roads.push(road);
    }

    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut data = Data::new();
            for road in roads.iter() {
                for route in road.get_routes_to_draw() {
                    let mut rt = Vec::new();
                    let half = route.len() / 2;
                    let from = if i == 0 { 0 } else { half };
                    let to = if i == 0 { half } else { route.len() };
                    for i in from..to {
                        rt.push(route[i]);
                    }
                    data = render_route(data, rt);
                }
            }
            let mut g = layer(color);
            g = g.add(base_path(color, 0.35, data));
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
