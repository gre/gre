use audioviz::io::{Device, Input};
use audioviz::spectrum::{
  config::{Interpolation, ProcessorConfig, StreamConfig},
  stream::Stream,
};
use clap::*;
use gre::*;
use std::thread::sleep;
use std::time::Duration;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "420.0")]
  pub height: f64,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "2.0")]
  pub dy: f64,
  #[clap(short, long, default_value = "20.0")]
  pub amp: f64,
  #[clap(short, long, default_value = "8.0")]
  pub gravity: f32,
  #[clap(short, long, default_value = "0.1")]
  pub volume: f32,
  #[clap(short, long, default_value = "1024")]
  pub fft_resolution: usize,
  #[clap(short, long, default_value = "0")]
  pub freq_from: usize,
  #[clap(short, long, default_value = "12000")]
  pub freq_to: usize,
  #[clap(short, long, default_value = "50")]
  pub interval_ms: u64,
}

fn read_from_mic(opts: &Opts, interval: Duration, n: usize) -> Vec<Vec<f32>> {
  // captures audio from system using cpal
  let mut audio_input = Input::new();
  let (channel_count, _sampling_rate, input_controller) =
    audio_input.init(&Device::DefaultInput).unwrap();

  // spectrum visualizer stream
  let mut stream: Stream = Stream::new(StreamConfig {
    channel_count,
    gravity: Some(opts.gravity),
    fft_resolution: opts.fft_resolution,
    processor: ProcessorConfig {
      frequency_bounds: [opts.freq_from, opts.freq_to],
      interpolation: Interpolation::Cubic,
      volume: opts.volume,
      ..ProcessorConfig::default()
    },
    ..StreamConfig::default()
  });
  let mut res = vec![];
  loop {
    if res.len() >= n {
      break;
    }
    if let Some(data) = input_controller.pull_data() {
      stream.push_data(data);
      stream.update();

      let frequencies = stream.get_frequencies();
      // channel 0
      if let Some(freq) = frequencies.get(0) {
        res.push(freq.iter().map(|freq| freq.volume).collect::<Vec<_>>());
      }
    }
    sleep(interval);
  }

  res
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let dy = opts.dy;
  let count = (height / dy) as usize;

  let record =
    read_from_mic(&opts, Duration::from_millis(opts.interval_ms), count);

  let mut passage = Passage::new(0.5, width, height);
  let passage_max = 6;

  let mut routes = Vec::new();
  let mut ybase = height - pad;
  let amp = opts.amp;
  let precision = 0.3;
  let min_route = 3;

  for mountain_curve in record {
    let mut curve = vec![];
    let w = width - pad * 2.0;
    let mut x = pad;
    let xincr = w / (mountain_curve.len() as f64 - 1.0);
    for v in mountain_curve {
      let y = ybase - v as f64 * amp;
      let p = (x, y);
      curve.push(p);
      x += xincr;
    }

    let mut last = curve[0];
    let mut route = vec![];
    for &c in curve.iter().skip(1) {
      let mut x = last.0;
      loop {
        if x > c.0 {
          break;
        }
        let p = if c.0 == last.0 {
          0.0
        } else {
          (x - last.0) / (c.0 - last.0)
        };
        let y = mix(last.1, c.1, p);
        if y < pad || passage.count((x, y)) > passage_max {
          if route.len() >= min_route {
            routes.push(route);
            route = vec![];
          } else if route.len() > 0 {
            route = vec![];
          }
        } else {
          route.push((x, y));
        }
        x += precision;
      }
      last = c;
    }
    if route.len() >= min_route {
      routes.push(route);
    }

    ybase -= dy;
  }

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

#[derive(Clone)]
struct Passage {
  precision: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage {
  pub fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision).ceil() as usize;
    let hi = (height / precision).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage {
      precision,
      width,
      height,
      counters,
    }
  }

  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.precision).ceil() as usize;
    let hi = (self.height / self.precision).ceil() as usize;
    let xi = ((x / self.precision).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.precision).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }

  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }
}
