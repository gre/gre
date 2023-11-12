use instant::Instant;
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::collections::HashMap;

pub struct Span {
  label: String,
  start: Instant,
  stop: Instant,
}
pub struct PerfRecords {
  debug: bool,
  started: HashMap<String, Instant>,
  spans: Vec<Span>,
}
pub struct PerfResult {
  per_label: HashMap<String, f64>,
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
    };
    r
  }
  /**
   * perf.span("calc_circles");
   */
  pub fn span(self: &mut Self, s: &str) {
    if self.debug {
      self.started.insert(String::from(s), Instant::now());
    }
  }
  /**
   * perf.span_end("calc_circles");
   */
  pub fn span_end(self: &mut Self, s: &str) {
    if self.debug {
      let label = String::from(s);
      if let Some(&start) = self.started.get(&label) {
        self.spans.push(Span {
          label,
          start,
          stop: Instant::now(),
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
        let maybe_time = per_label.get(&span.label).unwrap_or(&0.);
        per_label.insert(
          span.label.clone(),
          maybe_time + span.stop.duration_since(span.start).as_secs_f64(),
        );
      });
    }
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
