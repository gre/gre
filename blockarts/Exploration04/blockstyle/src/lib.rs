mod utils;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;
use wasm_bindgen::prelude::*;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Opts {
    pub seed: f64,
    pub opacity: f64,
    pub sdivisions: usize,
    pub lines: usize,
    pub sublines: usize,
    pub osc_amp: (f64, f64),
    pub osc_freq: f64,
    pub margin: (f64, f64),
    pub padding: (f64, f64),
    pub off: (f64, f64),
    pub lines_axis: Vec<bool>,
    pub mirror_axis: Vec<bool>,
    pub line_dir: f64,
    pub mirror_axis_weight: f64,
    pub lower: f64,
    pub upper: f64,
    pub lowstep: f64,
    pub highstep: f64,
    pub rotation: f64,
    pub border: f64,
    pub m: f64,
    pub k: f64,
    pub k1: f64,
    pub k2: f64,
    pub k3: f64,
    pub k4: f64,
    pub second_color_div: usize,
}


fn close_last_curve (curves: &mut Vec<Vec<(f64,f64)>>) {
    if let Some(curve) = curves.pop() {
        let mut c = curve;
        let l = c.len();
        if l > 1 {
            let a = c[l-1];
            let b = c[l-2];
            if a.0 != b.0 || a.1 != b.1 {
                c = c.clone();
                c.push(a);
            }
            curves.push(c);
        }
    }
    curves.push(Vec::new());
}

pub fn art(opts: &Opts) -> Vec<Group> {
    // variables
    let sdivisions = opts.sdivisions; // how much to split the width space
    let sdivisionsf = sdivisions as f64;
    let lines = opts.lines; // how much to split the height space
    let linesf = lines as f64;
    let sublines = opts.sublines; // for each line, how much do we make "sublines" to make it grow
    let sublinesf = sublines as f64;
    let osc_amp = opts.osc_amp;
    let osc_freq = opts.osc_freq;
    let margin = opts.margin;
    let padding = opts.padding;
    let lines_axis = opts.lines_axis.clone(); // true: h lines, false: v lines
    let line_dir = opts.line_dir;
    let mirror_axis_weight = opts.mirror_axis_weight;
    let lower = opts.lower;
    let upper = opts.upper;
    let lowstep = opts.lowstep;
    let highstep = opts.highstep;
    let rotation = opts.rotation;
    let border = opts.border;
    let m = opts.m;
    let k = opts.k;
    let k1 = opts.k1;
    let k2 = opts.k2;
    let k3 = opts.k3;
    let k4 = opts.k4;
    let second_color_div = opts.second_color_div;
    
    // statics
    let stroke_width = 0.3;
    let height = 200.0;
    let width = 200.0;
    // calculated
    let boundaries = (margin.0, margin.1, width - margin.0, height - margin.1);
    let crop = (padding.0.max(0.), padding.1.max(0.), width - padding.0.max(0.), height - padding.1.max(0.));
    let ratio = (boundaries.2 - boundaries.0) / (boundaries.3 - boundaries.1);
    let noise = OpenSimplex::new();

    let f = |o: (f64, f64)| {
        let mut point = (o.0 - 0.5, o.1 - 0.5);
        point = p_r(point, rotation);
        point.0 += 0.5;
        point.1 += 0.5;
        let mut s = 0.0;
        for is_axis_y in opts.mirror_axis.clone() {
            if !is_axis_y {
                s += mirror_axis_weight * (0.33 - (point.0-0.5).abs());
                point.0 = 0.5 + (point.0 - 0.5).abs();
            }
            else {
                s += mirror_axis_weight * (0.33 - (point.1-0.5).abs());
                point.1 = 0.5 + (point.1 - 0.5).abs();
            }
        }

        let p = ( point.0 * m * ratio, point.1 * m );
        let a1 = noise.get([3. + 0.9 * opts.seed, p.0, p.1 ]);
        let a2 = noise.get([p.0, p.1, 7.3 * opts.seed]);
        let b1 = noise.get([
            p.0 + 4. * k * a1 + 7.8 + opts.seed,
            p.1 + k * a2 ]);
        let b2 = noise.get([
            p.0 + k * a1 + 2.1 - opts.seed,
            p.1 + 2. * k * a2 - 1.7 ]);
        s += noise.get([
            -opts.seed,
            p.0 + 0.2 * k * a1 + 0.4 * k * b1,
            p.1 + 0.2 * k * a2 + 0.4 * k * b2
        ]);
        smoothstep(lowstep, highstep, s) * (upper - lower) + lower
    };

    let displacement = |p: (f64, f64)| -> (f64, f64) {
        let a = 3.0 * noise.get([k1 * p.0, k2 * p.1, 6.7 * opts.seed]);
        let b = 2.0 * noise.get([k4 * p.1 + a, k3 * p.0 - a]);
        (
            p.0 + opts.off.0 * noise.get([p.0 + a - opts.seed, p.1 + 10. + b]) + osc_amp.0 * (osc_freq * p.1).cos(),
            p.1 + opts.off.1 * noise.get([p.1 + b + opts.seed, p.0 -10. - b]) + osc_amp.1 * (osc_freq * p.0).sin(),
        )
    };

    let growing_lines = |
        j: usize,
        curves: &mut Vec<Vec<(f64,f64)>>,
        sdivisions: usize,
        lpi: f64,
        spfrom: f64,
        spto: f64,
        h: f64,
        is_axis_y: bool,
        line_dir: f64,
        continuing: bool
    | {
        let mut curve = if continuing {
            if let Some(curve) = curves.pop() {
                curve
            }
            else {
                close_last_curve(curves);
                Vec::new()
            }
        }
        else {
            close_last_curve(curves);
            Vec::new()
        };

        let p = h * (j as f64 - line_dir * (sublinesf)) / (sublinesf);
        let lp = lpi + p;
        for k in 0..sdivisions {
            let sp = mix(spfrom, spto, (k as f64) / (sdivisionsf - 1.));
            let origin = displacement(if is_axis_y { (sp, lpi) } else { (lpi, sp) });
            let target = displacement(if is_axis_y { (sp, lp) } else { (lp, sp) });
            let v = f(target); // lookup from a normalized function

            if v < 0.0 {
                let l = curve.len();
                if l > 0 {
                    if l > 1 {
                        curve.push(curve[l-1]); // as it's a curve, we need to add last point again
                        curves.push(curve);
                    }
                    curve = Vec::new();
                }
            }
            else {
                    // our final point (normalized in 0..1)
                let p = if is_axis_y {
                    ( origin.0, mix(origin.1, target.1, v) )
                } else {
                    ( mix(origin.0, target.0, v), origin.1 )
                };
                curve.push(project_in_boundaries(p, boundaries));
            }
        }
        let l = curve.len();
        if l > 1 {
            curve.push(curve[curve.len()-1]); // as it's a curve, we need to add last point again
            curves.push(curve);
        }
    };

    let mut layers = Vec::new();
    let colors = if second_color_div > 0 {
        vec!["#0FF", "#F0F"]
    } else {
        vec!["#0FF"]
    };
    let clen = colors.len();
    for (ci, &color) in colors.iter().enumerate() {
        let mut curves = Vec::new(); // all the lines

        // rectangle spiral filling
        if lines_axis.len() == 0 && ci == clen-1 {
            let max = (crop.2 - crop.0).max(crop.3 - crop.1);
            let step = max / (linesf);
            let h = 1. / (linesf);
            for j in 0..sublines {
                let mut bounds = crop;
                loop {
                    let mut a = normalize_in_boundaries((bounds.0, bounds.1), crop);
                    let mut b = normalize_in_boundaries((bounds.2, bounds.1), crop);
                    
                    if bounds.0 < bounds.2 {
                        let sdiv = (sdivisionsf * ((bounds.2 - bounds.0) / (crop.2 - crop.0))) as usize;
                        // left to right
                        growing_lines(
                            j,
                            &mut curves,
                            sdiv,
                            a.1,
                            a.0,
                            b.0,
                            h,
                            true,
                            line_dir,
                            true
                        );
                        bounds.0 += step;
                    }

                    a = b;
                    b = normalize_in_boundaries((bounds.2, bounds.3), crop);
                    if bounds.1 < bounds.3 {
                        let sdiv = (sdivisionsf * ((bounds.3 - bounds.1) / (crop.3 - crop.1))) as usize;
                        // top to down
                        growing_lines(
                            j,
                            &mut curves,
                            sdiv,
                            a.0,
                            a.1,
                            b.1,
                            h,
                            false,
                            1. - line_dir,
                            true
                        );
                        bounds.1 += step;
                    }

                    a = b;
                    b = normalize_in_boundaries((bounds.0, bounds.3), crop);
                    if bounds.0 < bounds.2 {
                        let sdiv = (sdivisionsf * ((bounds.2 - bounds.0) / (crop.2 - crop.0))) as usize;
                        // right to left
                        growing_lines(
                            j,
                            &mut curves,
                            sdiv,
                            a.1,
                            a.0,
                            b.0,
                            h,
                            true,
                            1. - line_dir,
                            true
                        );
                        bounds.2 -= step;
                    }

                    a = b;
                    b = normalize_in_boundaries((bounds.0, bounds.1), crop);
                    if bounds.1 < bounds.3 {
                        let sdiv = (sdivisionsf * ((bounds.3 - bounds.1) / (crop.3 - crop.1))) as usize;
                        // bottom to up
                        growing_lines(
                            j,
                            &mut curves,
                            sdiv,
                            a.0,
                            a.1,
                            b.1,
                            h,
                            false,
                            line_dir,
                            true
                        );
                        bounds.3 -= step;
                    }

                    if bounds.0 >= bounds.2 || bounds.1 >= bounds.3 {
                        break;
                    }
                }
                close_last_curve(&mut curves);
            }
        }
        // otherwise it's a line filling
        for is_axis_y in lines_axis.clone() {
            for j in 0..sublines {
                for i in 0..lines {
                    if second_color_div == 0 || (i / second_color_div) % clen == ci {
                        let lpi = (i as f64 + line_dir) / (linesf - 1.);
                        let (from, to) = if i % 2 == 0 {
                            (0.0, 1.0)
                        } else {
                            (1.0, 0.0)
                        };
                        growing_lines(
                            j,
                            &mut curves,
                            sdivisions,
                            lpi,
                            from,
                            to,
                            1.0 / (linesf),
                            is_axis_y,
                            line_dir,
                            false
                        );
                    }
                }
                close_last_curve(&mut curves);
            }
        }

        let mut l = Group::new()
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-linecap", "round")
            .set("stroke-linejoin", "round")
            .set("stroke-width", stroke_width);
            for r in curves {
                if r.len() < 2 {
                    continue;
                }
                let data = render_route_curve(Data::new(), r, crop);
                l = l.add(
                    Path::new()
                    .set("opacity", opts.opacity)
                    .set("d", data)
                );
            }
            let border_dist = 0.25;
            let borders = (border / border_dist).ceil() as usize;
            if ci == 0 {
                for b in 0..borders {
                    let bd = b as f64 * border_dist;
                    l = l.add(
                        Rectangle::new()
                        .set("opacity", opts.opacity)
                        .set("x", crop.0 - bd)
                        .set("y", crop.1 - bd)
                        .set("width", (crop.2 - crop.0) + bd * 2.)
                        .set("height", (crop.3 - crop.1) + bd * 2.)
                    );
                }
            }
        layers.push(l);
    }
    layers
}

#[wasm_bindgen]
pub fn blockstyle(val: &JsValue) -> String {
    let opts = val.into_serde().unwrap();

    let mut g = Group::new();
    let a = art(&opts);
    for e in a {
        g = g.add(e);
    }
    let str = g.to_string();
    return str;
}

fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
    (
        a.cos() * p.0 + a.sin() * p.1,
        a.cos() * p.1 - a.sin() * p.0,
    )
}

fn project_in_boundaries(
    p: (f64, f64),
    boundaries: (f64, f64, f64, f64),
) -> (f64, f64) {
    (
        p.0 * (boundaries.2 - boundaries.0) + boundaries.0,
        p.1 * (boundaries.3 - boundaries.1) + boundaries.1,
    )
}

fn out_of_bound(p: (f64, f64), boundaries: (f64, f64, f64, f64)) -> bool {
    p.0 < boundaries.0 || p.0 > boundaries.2 || p.1 < boundaries.1 || p.1 > boundaries.3
}

fn significant_str (f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}

fn render_route_curve(
    data: Data,
    route: Vec<(f64, f64)>,
    boundaries: (f64, f64, f64, f64)
) -> Data {
    if route.len() == 0 {
        return data;
    }
    let mut first = true;
    let mut up = false;
    let mut last = route[0];
    let mut d = data;
    for p in route {
        if first {
            first = false;
            d = d.move_to((
                significant_str(p.0),
                significant_str(p.1)
            ));
        } else {
            let next = ((p.0 + last.0) / 2., (p.1 + last.1) / 2.);
            if !out_of_bound(next, boundaries) {
                if up {
                    up = false;
                    d = d.move_to((
                        significant_str(last.0),
                        significant_str(last.1)
                    ));
                }
                d = d.quadratic_curve_to((
                    significant_str(last.0),
                    significant_str(last.1),
                    significant_str(next.0),
                    significant_str(next.1),
                ));
            } else {
                up = true;
            }
        }
        last = p;
    }
    return d;
}

fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let k = ((x - a) / (b - a)).max(0.0).min(1.0);
    return k * k * (3.0 - 2.0 * k);
}

fn mix(a: f64, b: f64, x: f64) -> f64 {
    (1. - x) * a + x * b
}

pub fn normalize_in_boundaries(
    p: (f64, f64),
    boundaries: (f64, f64, f64, f64),
) -> (f64, f64) {
    (
        (p.0 - boundaries.0)
            / (boundaries.2 - boundaries.0),
        (p.1 - boundaries.1)
            / (boundaries.3 - boundaries.1),
    )
}