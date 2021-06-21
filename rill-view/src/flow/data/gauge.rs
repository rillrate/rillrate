use rill_protocol::flow::core::{Flow, TimedEvent};
use rill_protocol::io::provider::{StreamType, Timestamp};
use rill_protocol::range::Range;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaugeState {
    // IMMUTABLE:
    pub range: Range,

    // MUTABLE:
    pub timestamp: Option<Timestamp>,
    pub value: f64,
}

impl GaugeState {
    pub fn new(range: Range) -> Self {
        Self {
            range,
            timestamp: None,
            value: 0.0,
        }
    }

    pub fn last(&self) -> Option<TimedEvent<f64>> {
        self.timestamp.map(|ts| TimedEvent {
            timestamp: ts,
            event: self.value,
        })
    }
}

impl Flow for GaugeState {
    type Action = ();
    type Event = GaugeEvent;

    fn stream_type() -> StreamType {
        StreamType::from("rillrate.data.gauge.v0")
    }

    fn apply(&mut self, event: TimedEvent<Self::Event>) {
        match event.event {
            GaugeEvent::Set(delta) => {
                self.timestamp = Some(event.timestamp);
                self.value = delta;
            }
        }
    }
}

pub type GaugeDelta = Vec<TimedEvent<GaugeEvent>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GaugeEvent {
    Set(f64),
}