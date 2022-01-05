use main::*;
use rand::prelude::*;
use image::io::Reader as ImageReader;

fn main() {
    let mut rng = rand::thread_rng();
    let img = ImageReader::open("../eiffel_silhouette_clipart.png").unwrap().decode().unwrap();
    let rgba = img.to_rgba8();
    let data = rgba.to_vec();
    let (w, h) = rgba.dimensions();
    let width = w as usize;
    let height = h as usize;
    let image = ImageData { data, width, height };
    let doc = art(&Opts {
        seed: rng.gen_range(0.0, 100.0),
        primary_name: String::from("P"),
        secondary_name: String::from("S"),
        image,
        distribmode: 0,
        voronoi_size: 600
    });
    svg::save("image.svg", &doc).unwrap();
}