use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
#[derive(Clone, Copy)]
struct TConfig {
    left_a_div: f64,
    right_a_div: f64,
    left_length_mul: f64,
    right_length_mul: f64,
    threshold_min: f64,
}
impl TConfig {
    fn new(
        left_a_div: f64,
        right_a_div: f64,
        left_length_mul: f64,
        right_length_mul: f64,
        threshold_min: f64,
    ) -> Self {
        TConfig {
            left_a_div,
            right_a_div,
            left_length_mul,
            right_length_mul,
            threshold_min,
        }
    }
}

#[derive(Clone, Copy)]
struct TLine {
    origin: (f64, f64),
    angle: f64,
    length: f64,
}

impl TLine {
    fn new(
        origin: (f64, f64),
        angle: f64,
        length: f64,
    ) -> Self {
        TLine {
            origin,
            angle,
            length,
        }
    }
    fn draw(self: Self, d: Data) -> Data {
        let mut data = d.move_to(self.origin);
        let x = self.length * self.angle.cos();
        let y = self.length * self.angle.sin();
        data = data.line_by((x, y));
        data
    }
    fn fork(self: Self, config: TConfig) -> Vec<TLine> {
        let mut v = Vec::new();
        let end = (
            self.origin.0 + self.length * self.angle.cos(),
            self.origin.1 + self.length * self.angle.sin(),
        );
        v.push(TLine::new(
            end,
            self.angle - config.left_a_div,
            self.length * config.left_length_mul,
        ));
        v.push(TLine::new(
            end,
            self.angle + config.right_a_div,
            self.length * config.right_length_mul,
        ));
        v
    }
    fn build(
        self: Self,
        level: usize,
        get_config: &dyn Fn(usize) -> TConfig,
    ) -> Vec<TLine> {
        let mut v = Vec::new();
        v.push(self);
        if level <= 0 {
            return v;
        }
        let c = get_config(level);
        if self.length < c.threshold_min {
            return v;
        }
        let children = self.fork(c);
        for child in children {
            let mut lines =
                child.build(level - 1, get_config);
            v.append(&mut lines);
        }
        v
    }
}

fn art(_seed: f64) -> Vec<Group> {
    let mut groups = Vec::new();
    let mut data = Data::new();

    let origin = (148., 105.);
    let max_level = 20;
    let initial_length = 36.;
    let angle_off = PI / 6.;
    let n = 3;

    let get_config = |level| {
        let l = level as f64 / (max_level as f64);
        TConfig::new(0.5, 0.3 + 0.6 * l, 0.5, 0.82, 1.)
    };

    let mut tlines = Vec::new();
    for i in 0..n {
        let mut lines = TLine::new(
            origin,
            angle_off + i as f64 * 2. * PI / (n as f64),
            initial_length,
        )
        .build(max_level, &get_config);
        tlines.append(&mut lines);
    }

    data = tlines
        .iter()
        .fold(data, |data, &tline| tline.draw(data));

    let color = "white";
    groups.push(
        layer(color).add(base_path(color, 0.5, data)),
    );

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let groups = art(seed);
    let mut document = base_a4_landscape("black");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (260.0, 190.0),
        "white",
    ));
    svg::save("image.svg", &document).unwrap();
}
