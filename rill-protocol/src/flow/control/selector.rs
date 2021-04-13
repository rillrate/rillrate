use crate::flow::core::{Flow, TimedEvent};
use crate::io::provider::{StreamType, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectorState {
    // IMMUTABLE
    pub label: String,
    /// It's `Vec` to keep the order.
    pub options: Vec<String>,

    // MUTABLE
    pub selected: String,
    pub updated: Option<Timestamp>,
}

#[allow(clippy::new_without_default)]
impl SelectorState {
    pub fn new(label: String, options: Vec<String>, selected: String) -> Self {
        Self {
            label,
            options,
            selected,
            updated: None,
        }
    }
}

impl Flow for SelectorState {
    type Event = SelectorEvent;

    fn stream_type() -> StreamType {
        StreamType::from("rillrate.flow.control.selector.v0")
    }

    fn apply(&mut self, event: TimedEvent<Self::Event>) {
        let new_value = event.event.select;
        if self.options.contains(&new_value) {
            self.selected = new_value;
        } else {
            log::error!("No option {} in the selector: {}.", new_value, self.label);
        }
        self.updated = Some(event.timestamp);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectorEvent {
    pub select: String,
}

impl SelectorEvent {
    pub fn select(value: String) -> Self {
        Self { select: value }
    }
}