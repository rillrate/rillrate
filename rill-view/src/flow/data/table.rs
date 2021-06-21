use derive_more::{From, Into};
use rill_protocol::flow::core::{Flow, TimedEvent};
use rill_protocol::io::provider::StreamType;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::convert::{TryFrom, TryInto};
use std::fmt;

/// Id of a column in a table.
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, From, Into, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Col(pub u64);

impl fmt::Display for Col {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<usize> for Col {
    type Error = <u64 as TryFrom<usize>>::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}

/// Id of a row in a table.
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, From, Into, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Row(pub u64);

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<usize> for Row {
    type Error = <u64 as TryFrom<usize>>::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableState {
    // IMMUTABLE:
    #[serde(with = "vectorize")]
    pub columns: BTreeMap<Col, ColRecord>,

    // MUTABLE:
    #[serde(with = "vectorize")]
    pub rows: BTreeMap<Row, RowRecord>,
}

impl TableState {
    pub fn new(columns: BTreeMap<Col, ColRecord>) -> Self {
        Self {
            columns,
            rows: BTreeMap::new(),
        }
    }
}

impl Flow for TableState {
    type Action = ();
    type Event = TableEvent;

    fn stream_type() -> StreamType {
        StreamType::from("rillrate.data.table.v0")
    }

    fn apply(&mut self, event: TimedEvent<Self::Event>) {
        match event.event {
            TableEvent::AddRow { row } => {
                let record = RowRecord {
                    cols: BTreeMap::new(),
                };
                self.rows.insert(row, record);
            }
            TableEvent::DelRow { row } => {
                self.rows.remove(&row);
            }
            TableEvent::SetCell { row, col, value } => {
                if let Some(record) = self.rows.get_mut(&row) {
                    if self.columns.contains_key(&col) {
                        record.cols.insert(col, value);
                    }
                }
            }
        }
    }
}

pub type TableDelta = Vec<TimedEvent<TableEvent>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableEvent {
    AddRow { row: Row },
    DelRow { row: Row },
    SetCell { row: Row, col: Col, value: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ColRecord {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RowRecord {
    #[serde(with = "vectorize")]
    pub cols: BTreeMap<Col, String>,
}