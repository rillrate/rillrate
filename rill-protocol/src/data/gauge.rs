use super::{ConvertError, Delta, Metric, TimedEvent};
use crate::frame::Frame;
use crate::io::provider::{StreamDelta, StreamState};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub struct GaugeMetric;

impl Metric for GaugeMetric {
    type State = GaugeState;
    type Event = GaugeEvent;

    fn apply(state: &mut Self::State, event: TimedEvent<Self::Event>) {
        match event.event {
            GaugeEvent::Increment(delta) => {
                state.value += delta;
            }
            GaugeEvent::Decrement(delta) => {
                state.value -= delta;
            }
            GaugeEvent::Set(value) => {
                state.value = value;
            }
        }
        let point = GaugePoint { value: state.value };
        let timed_event = TimedEvent {
            timestamp: event.timestamp,
            event: point,
        };
        state.frame.insert(timed_event);
    }

    fn wrap(events: Delta<Self::Event>) -> StreamDelta {
        StreamDelta::from(events)
    }

    fn try_extract(delta: StreamDelta) -> Result<Delta<Self::Event>, ConvertError> {
        delta.try_into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaugePoint {
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaugeState {
    pub frame: Frame<TimedEvent<GaugePoint>>,
    value: f64,
}

impl Default for GaugeState {
    fn default() -> Self {
        Self {
            // TODO: Use duration for removing obsolete values instead
            frame: Frame::new(100),
            value: 0.0,
        }
    }
}

impl TryFrom<StreamState> for GaugeState {
    type Error = ConvertError;

    fn try_from(state: StreamState) -> Result<Self, ConvertError> {
        match state {
            StreamState::Gauge(state) => Ok(state),
            _ => Err(ConvertError),
        }
    }
}

pub type GaugeDelta = Vec<TimedEvent<GaugeEvent>>;

impl TryFrom<StreamDelta> for GaugeDelta {
    type Error = ConvertError;

    fn try_from(delta: StreamDelta) -> Result<Self, ConvertError> {
        match delta {
            StreamDelta::Gauge(delta) => Ok(delta),
            _ => Err(ConvertError),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GaugeEvent {
    Increment(f64),
    Decrement(f64),
    Set(f64),
}
