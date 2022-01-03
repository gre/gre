use gre::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

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

fn art(seed: f64) -> Vec<Group> {
    let colors = vec!["black", "red"];
    let pad = 20.0;
    let width = 297.0;
    let height = 210.0;
    let size = 60.0;
    let bounds = (pad, pad, width - pad, height - pad);

    let line_length = 200.0;
    let granularity = 1.0;
    let samples = 2000;

    let track_count = 8;
    let track_dist = 0.8;

    /*
    let mut road = Road::new(
        (width / 2.0, height / 2.0),
        track_count,
        track_dist
    );

    let mut rng = rng_from_seed(seed);

    let mut a = rng.gen_range(0.0, 2. * PI);
    let amp = 0.6;
    let da = PI / 16.0;
    for _i in 0..200 {
        road.mv(a, amp);
        if rng.gen_range(0.0, 1.0) < 0.7 {
            if rng.gen_range(0.0, 1.0) < 0.5 {
                a += da;
            }
            else {
                a -= da;
            }
        }
    }
    */

    /*
    let mut routes =
    // lines
    build_routes_with_collision_par(
        initial_positions,
        &build_route,
    );
    */

    // let routes = road.get_routes_to_draw();

    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let data = routes
                .iter()
                .enumerate()
                .filter(|(j, _route)| i == 0)
                .fold(Data::new(), |data, (_j, route)| {
                    render_route(data, route.clone())
                });
            let mut g = layer(color);
            g = g.add(base_path(color, 0.35, data));
            return g;
        })
        .collect()
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let groups = art(seed);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
