use clap::Clap;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let width = 210.0;
    let height = 297.0;
    let pad = 10.0;
    let padleft = 20.0;
    let count = 28;
    let mut layers = Vec::new();
    let cfi = 1.0 / (count as f64);
    for c in 0..count  {
        let cf = c as f64 / (count as f64);
        let color = format!("{}", c);
        let mut data = Data::new();

        // color only
        let y1 = pad + (height - 2. * pad) * (cf + 0.05 * cfi);
        let y2 = pad + (height - 2. * pad) * (cf + 0.95* cfi);
        let x1 = pad;
        let x2 = padleft - 2.0;
        let mut rev = false;
        let div = 32;
        for yi in 0..div {
            let y = mix(y1, y2, (yi as f64 / (div as f64)).powf(0.9));
            if !rev {
                data = data.move_to((x1, y));
                data = data.line_to((x2, y));
            }
            else {
                data = data.move_to((x2, y));
                data = data.line_to((x1, y));
            }
            rev = !rev;
        }

        // column
        let x1 = padleft + (width - pad - padleft) * cf;
        let x2 = padleft + (width - pad - padleft) * (cf + cfi);
        let y1 = pad;
        let y2 = height - pad;
        let mut rev = false;
        for xi in vec![0.3, 0.5, 0.7] {
            let x = mix(x1, x2, xi);
            if !rev {
                data = data.move_to((x, y1));
                data = data.line_to((x, y2));
            }
            else {
                data = data.move_to((x, y2));
                data = data.line_to((x, y1));
            }
            rev = !rev;
        }
        // row
        let y1 = pad + (height - 2. * pad) * cf;
        let y2 = pad + (height - 2. * pad) * (cf + cfi);
        let x1 = padleft - 2.0;
        let x2 = width - pad;
        let mut rev = false;
        for yi in vec![0.2, 0.32, 0.42, 0.55, 0.6] {
            let y = mix(y1, y2, yi);
            if !rev {
                data = data.move_to((x1, y));
                data = data.line_to((x2, y));
            }
            else {
                data = data.move_to((x2, y));
                data = data.line_to((x1, y));
            }
            rev = !rev;
        }

        
        let mut l = layer(color.as_str());
        l = l.add(base_path("#000", 0.35, data));
        layers.push(l);
    }
    layers
}

#[derive(Clap)]
#[clap()]
struct Opts {
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
