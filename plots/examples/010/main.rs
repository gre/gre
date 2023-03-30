use image::io::Reader as ImageReader;
use image::GenericImageView;
use rand::prelude::*;
use std::env;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

// vectorize an image using contiguous random lines
// and where the lines have a small oscillation with a frequency and amplitude that depends on the image color
// done for each of the 4 colors of CMYK.

fn main() {
  let args: Vec<String> = env::args().collect();
  let path = if args.len() > 1 {
    &args[1]
  } else {
    "images/profile.jpg"
  };
  let document = make_svg(path).expect("failed to load image");
  svg::save("image.svg", &document).expect("failed to generate svg");
}

fn make_svg(path: &str) -> Result<Document, image::ImageError> {
  let get_color = image_get_color(path)?;

  let size = 120.0;
  // we put each color into a Inkscape layer to facilitate the plotting work
  let mut magenta = layer("magenta");
  let mut cyan = layer("cyan");
  let mut yellow = layer("yellow");
  let mut black = layer("black");

  // we will do 6 images (3x2)
  for x in 0..3 {
    for y in 0..2 {
      // rng will differ on each x,y
      let mut rng =
        SmallRng::from_seed([x, y, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
      // the total length of the line will increase on x and y
      let length = 3000.0 * (2 + x + 3 * y) as f32;

      let mut vectorize =
        |map: &dyn Fn((f32, f32, f32)) -> f32, color: &str, lmul: f32| {
          vectorize_as_random_waves(
            (size, size),
            &get_color,
            &map,
            3.0,
            3.0,
            0.05,
            0.25,
            length * lmul,
            &mut rng,
            color,
            format!("(img{},{})", x, y),
          )
          .set(
            "transform",
            format!(
              "translate({x},{y})",
              x = 30.0 + size * (x as f32),
              y = 30.0 + size * (y as f32)
            ),
          )
        };
      // apply vectorize for each layer
      magenta =
        magenta.add(vectorize(&move |clr| rgb_to_cmyk(clr).1, "magenta", 1.0));
      cyan = cyan.add(vectorize(&move |clr| rgb_to_cmyk(clr).0, "cyan", 1.0));
      yellow =
        yellow.add(vectorize(&move |clr| rgb_to_cmyk(clr).2, "yellow", 1.0));
      black =
        black.add(vectorize(&move |clr| rgb_to_cmyk(clr).3, "black", 0.8));
    }
  }

  return Ok(
    Document::new()
      .set(
        "xmlns:inkscape",
        "http://www.inkscape.org/namespaces/inkscape",
      )
      .set("viewBox", (0, 0, 420, 297))
      .set("height", "297mm")
      .set("width", "420mm")
      .add(magenta)
      .add(cyan)
      .add(yellow)
      .add(black),
  );
}

// vectorize an image using contiguous random lines where the lines have a small oscillation with a frequency and amplitude that depends on the image color
fn vectorize_as_random_waves(
  // rect size to vectorize
  (width, height): (f32, f32),
  // a function that for a given (x,y) in 0..1 range will give the (r,g,b) color to rasterize with
  get_color: impl Fn((f32, f32)) -> (f32, f32, f32),
  // a function that select a float value out from the (r,g,b) color
  map_color: impl Fn((f32, f32, f32)) -> f32,
  // maximum oscillation amplitude
  max_amp: f32,
  // maximum oscillation frequency
  max_freq: f32,
  // in 0..1. a value close to 0 makes the line staying down, a higher value makes a lot of "up and down" on the path.
  debouncing: f32,
  // under which value we should assume a value is not to be drown (pen goes up)
  debouncing_threshold: f32,
  // the total length, in mm, of the line
  length: f32,
  // a random number generator
  rng: &mut impl Rng,
  // the color to use
  color: &str,
  id: String,
) -> Group {
  let mut data = Data::new();
  let mut totalMMstatus = 0.0;

  let mut dist = 0.0;
  let mut t: f32 = 0.0;
  let mut x = rng.gen_range(0.0, width);
  let mut y = rng.gen_range(0.0, height);
  data = data.move_to((x, y));
  loop {
    let nx = rng.gen_range(0.0, width);
    let ny = rng.gen_range(0.0, height);
    let dx = nx - x;
    let dy = ny - y;
    let d = (dx * dx + dy * dy).sqrt();
    let da = dy.atan2(dx);

    // essentially we do a line_to((nx,ny))
    // but we need to make it oscillate
    let nb = d * max_freq;
    let mut debouncedValue = 0.0;
    let mut penUp = false;
    for i in 0..(nb as usize) {
      // NB it's likely we could optimize the nb of points here, it makes the SVG pretty big. but it's ok.
      let p = (i as f32) / nb;
      let mut xi = x + dx * p;
      let mut yi = y + dy * p;
      let value = map_color(get_color((xi / width, yi / height)));
      let amp = max_amp * value;
      // this makes the oscillation, perpendicular to the current line. it's a rotation of (0.0,t.cos()) of angle 'da'. see https://fr.wikipedia.org/wiki/Rotation_vectorielle
      xi -= amp * t.cos() * (da).sin();
      yi += amp * t.cos() * (da).cos();
      // logic to debounce the pen up & down based on the image color
      debouncedValue += debouncing * (value - debouncedValue);
      let wasUp = penUp;
      penUp = debouncedValue < debouncing_threshold;
      if !penUp {
        if wasUp {
          // if it just went down, we will "jump" without writing
          data = data.move_to((xi, yi));
        } else {
          // otherwise, we draw a line
          data = data.line_to((xi, yi));
          totalMMstatus += d / nb;
        }
      }
      // using pow of 2 makes the oscillation frequency going higher only when the color is really near 1.0
      t += value.powf(2.0);
    }

    dist += d;
    x = nx;
    y = ny;
    if dist > length {
      println!(
        "{} {}: {} meters",
        id,
        color,
        totalMMstatus.round() / 1000.0,
      );
      return Group::new().add(
        Path::new()
          .set("fill", "none")
          .set("stroke", color)
          .set("stroke-width", 0.2)
          .set("d", data),
      );
    }
  }
}

fn layer(id: &str) -> Group {
  return Group::new()
    .set("inkscape:groupmode", "layer")
    .set("inkscape:label", id);
}

// see also https://en.wikipedia.org/wiki/CMYK_color_model
fn rgb_to_cmyk((r, g, b): (f32, f32, f32)) -> (f32, f32, f32, f32) {
  let k = 1.0 - r.max(g).max(b);
  let c = (1.0 - r - k) / (1.0 - k);
  let m = (1.0 - g - k) / (1.0 - k);
  let y = (1.0 - b - k) / (1.0 - k);
  return (c, m, y, k);
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
