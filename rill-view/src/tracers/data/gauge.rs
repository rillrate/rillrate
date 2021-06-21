use crate::flow::data::gauge::{GaugeEvent, GaugeState};
use derive_more::{Deref, DerefMut};
use rill_engine::tracers::tracer::Tracer;
use rill_protocol::io::provider::Path;
use rill_protocol::range::Range;
use std::time::SystemTime;

/// Tracers `Gauge` metrics.
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct GaugeTracer {
    tracer: Tracer<GaugeState>,
}

impl GaugeTracer {
    /// Creates a new tracer instance.
    pub fn new(path: Path, min: f64, max: f64) -> Self {
        let range = Range::new(min, max);
        let state = GaugeState::new(range);
        let tracer = Tracer::new_tracer(state, path, None);
        Self { tracer }
    }

    /// Set value of the gauge.
    pub fn set(&self, value: f64, timestamp: Option<SystemTime>) {
        let data = GaugeEvent::Set(value);
        self.tracer.send(data, timestamp);
    }
}