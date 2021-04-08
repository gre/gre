use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::env;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "images/photo-kid-hand1.jpg"
    };
    match make_svg(path) {
        Ok(document) => svg::save("image.svg", &document).unwrap(),
        Err(err) => println!("failed! {}", err),
    }
}

fn make_svg(path: &str) -> Result<Document, image::ImageError> {
    let get_color = image_get_color(path)?;
    let map_color = move |clr| 2.0 * smoothstep(0.8, 0.1, grayscale(clr)).powf(2.0);
    let group = vectorize_as_wave_rows((160.0, 160.0), get_color, map_color, 40, 800.0, "black")
        .set("transform", "translate(20,20)");

    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(group);

    return Ok(document);
}

fn vectorize_as_wave_rows(
    (width, height): (f32, f32),
    get_color: impl Fn((f32, f32)) -> (f32, f32, f32),
    map_color: impl Fn((f32, f32, f32)) -> f32,
    rows: u32,
    wave_freq: f32,
    color: &str,
) -> Group {
    let mut group = Group::new();
    for yi in 0..rows {
        let yp = (0.5 + yi as f32) / (rows as f32);
        let y = height * yp;
        let mut data = Data::new().move_to((0, y));
        let nb = (3.0 * wave_freq) as u32;
        // TODO: this could be optimized to have less datapoints
        for i in 1..nb {
            let xp = (i as f32) / (nb as f32);
            let x = width * xp;
            let clr = get_color((xp, yp));
            let amp = 0.5 * map_color(clr) * (height as f32) / (rows as f32);
            let dy = amp * (wave_freq * xp).cos();
            data = data.line_to((x, y + dy));
        }
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", 0.2)
            .set("d", data);
        group = group.add(path)
    }

    return group;
}

fn grayscale((r, g, b): (f32, f32, f32)) -> f32 {
    return 0.299 * r + 0.587 * g + 0.114 * b;
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let k = ((x - a) / (b - a)).max(0.0).min(1.0);
    return k * k * (3.0 - 2.0 * k);
}

// point is normalized in 0..1
// returned value is a rgb tuple in 0..1 range
fn image_get_color(
    path: &str,
) -> Result<impl Fn((f32, f32)) -> (f32, f32, f32), image::ImageError> {
    let img = ImageReader::open(path)?.decode()?;
    let (width, height) = img.dimensions();
    return Ok(move |(x, y)| {
        let xi = (x * (width as f32)) as u32;
        let yi = (y * (height as f32)) as u32;
        let pixel = img.get_pixel(xi, yi);
        let r = (pixel[0] as f32) / 255.0;
        let g = (pixel[1] as f32) / 255.0;
        let b = (pixel[2] as f32) / 255.0;
        return (r, g, b);
    });
}
