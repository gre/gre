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
    group: usize,
    depth: usize,
}

impl TLine {
    fn new(
        origin: (f64, f64),
        angle: f64,
        length: f64,
        group: usize,
        depth: usize,
    ) -> Self {
        TLine {
            origin,
            angle,
            length,
            group,
            depth,
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
        let group = self.group;
        let depth = self.depth + 1;
        v.push(TLine::new(
            end,
            self.angle - config.left_a_div,
            self.length * config.left_length_mul,
            if depth < 2 { group + 1 } else { group },
            depth,
        ));
        v.push(TLine::new(
            end,
            self.angle + config.right_a_div,
            self.length * config.right_length_mul,
            group,
            depth,
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
    let origin = (150., 105.);
    let angle_off = PI / 2.;
    let n = 4;

    let mut tlines = Vec::new();
    for i in 0..n {
        let max_level = 30 - i * 5;
        let get_config = |level| {
            let l = level as f64 / (max_level as f64);
            let left_a_div = 1.0 - i as f64 * 0.2;
            let left_length_mul = 0.4 + 0.2 * l;
            let right_a_div =
                0.2 + 0.8 * l + i as f64 * 0.05;
            let right_length_mul = 0.82 + i as f64 * 0.015;
            let threshold_min = 1.;
            TConfig::new(
                left_a_div,
                right_a_div,
                left_length_mul,
                right_length_mul,
                threshold_min,
            )
        };
        let mut lines = TLine::new(
            origin,
            angle_off + i as f64 * 2. * PI / (n as f64),
            (i % 2) as f64 * 24. + 16.,
            i,
            0,
        )
        .build(max_level, &get_config);
        tlines.append(&mut lines);
    }

    let colors = vec!["white", "gold"];

    let mut groups = Vec::new();

    for (i, color) in colors.iter().enumerate() {
        let data = tlines
            .iter()
            .filter(|tline| tline.group % colors.len() == i)
            .fold(Data::new(), |data, &tline| {
                tline.draw(data)
            });
        groups.push(
            layer(color).add(base_path(color, 0.5, data)),
        );
    }

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
