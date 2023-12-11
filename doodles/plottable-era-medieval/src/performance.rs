use instant::Instant;
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::collections::HashMap;

pub struct Span {
  label: String,
  start: Instant,
  stop: Instant,
  start_pts_len: usize,
  stop_pts_len: usize,
}
pub struct PerfRecords {
  debug: bool,
  started: HashMap<String, Instant>,
  started_pts_len: HashMap<String, usize>,
  spans: Vec<Span>,
}

#[derive(Serialize)]
pub struct PerfEntry {
  label: String,
  duration_ms: i32,
  points: usize,
}
pub struct PerfResult {
  per_label: Vec<PerfEntry>,
}
impl PerfRecords {
  /**
   * let mut perf = PerfRecords::start();
   */
  pub fn start(debug: bool) -> Self {
    let r = PerfRecords {
      debug,
      started: HashMap::new(),
      spans: Vec::new(),
      started_pts_len: HashMap::new(),
    };
    r
  }
  /**
   * perf.span("calc_circles");
   */
  pub fn span(
    self: &mut Self,
    s: &str,
    routes: &Vec<(usize, Vec<(f32, f32)>)>,
  ) {
    if self.debug {
      let k = String::from(s);
      self.started.insert(k.clone(), Instant::now());
      self.started_pts_len.insert(k, pts_len(routes));
    }
  }
  /**
   * perf.span_end("calc_circles");
   */
  pub fn span_end(
    self: &mut Self,
    s: &str,
    routes: &Vec<(usize, Vec<(f32, f32)>)>,
  ) {
    if self.debug {
      let label = String::from(s);
      if let Some(&start) = self.started.get(&label) {
        let start_pts_len = self.started_pts_len[&label];
        self.spans.push(Span {
          label,
          start,
          stop: Instant::now(),
          start_pts_len,
          stop_pts_len: pts_len(routes),
        });
      }
    }
  }
  /**
   * let perf_res = perf.end();
   */
  pub fn end(self: &mut Self) -> PerfResult {
    let mut per_label = HashMap::new();
    if self.debug {
      self.spans.iter().for_each(|span| {
        let maybe = per_label.get(&span.label).unwrap_or(&(0., 0));
        per_label.insert(
          span.label.clone(),
          (
            maybe.0 + span.stop.duration_since(span.start).as_secs_f32(),
            maybe.1 + span.stop_pts_len - span.start_pts_len,
          ),
        );
      });
    }
    let per_label = per_label
      .iter()
      .map(|(label, &(duration, points))| PerfEntry {
        label: label.clone(),
        duration_ms: (duration * 1000.0).round() as i32,
        points,
      })
      .collect();
    PerfResult { per_label }
  }
}

impl Serialize for PerfResult {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state = serializer.serialize_struct("Perf", 1)?;
    state.serialize_field("per_label", &self.per_label)?;
    state.end()
  }
}

fn pts_len(routes: &Vec<(usize, Vec<(f32, f32)>)>) -> usize {
  routes.iter().fold(0, |acc, (_, pts)| acc + pts.len())
}
