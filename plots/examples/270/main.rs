use gre::*;
use svg::node::element::{Group, path::Data};

fn art() -> Vec<Group> {
    let height = 297.0;
    let width = 210.0;
    let pad = 15.0;
    let stroke_width = 0.35;
    let digits_count = 1000;
    let digits = pi::pi(digits_count);

    let numbers: Vec<u32> = digits.chars().filter_map(|c| c.to_digit(10)).collect();

    let mut routes = Vec::new();

    let h = 19.0;
    let padbetween = 4.0;
    let mut y = pad;
    let mut nf: f64 = 1.0;
    loop {
        let nfc = nf.ceil();
        let n = nfc as usize;
        if y > height - padbetween - h {
            break;
        }
        let mut moving_avg: Vec<f64> = Vec::new();
        let mut last = Vec::new();
        let mut min = 9.0;
        let mut max = 0.0;
        for &a in numbers.iter() {
            last.push(a as f64);
            if last.len() <= n {
                continue;
            }
            last.remove(0);
            let f = last.iter().sum::<f64>() / nfc; // (not optimal algo)
            if f < min {
                min = f;
            }
            if f > max {
                max = f;
            }
            moving_avg.push(f);
        }

        let xmul = (width - 2. * pad) / (moving_avg.len() as f64);
        let mut route = Vec::new();
        for (i, &a) in moving_avg.iter().enumerate() {
            let anorm = (a - min) / (max - min);
            route.push((
                pad + xmul * i as f64,
                y + h * anorm as f64
            ));
        }
        routes.push(route);

        y += h + padbetween;
        nf *= 1.8;
    }

    let mut layers = Vec::new();
    let color = "black";
    let mut l = layer(color);
    let mut data = Data::new();
    for route in routes {
        data = render_route_curve(data, route);
    }
    l = l.add(base_path(color, stroke_width, data));
    layers.push(l);

    layers
    
}

fn main() {
    let groups = art();
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
