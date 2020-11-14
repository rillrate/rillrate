use crate::protocol::{EntryId, RillData};
use crate::state::{ControlEvent, RILL_STATE};
use futures::channel::mpsc;
use meio::Action;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub struct DataEnvelope {
    pub data: RillData,
}

impl Action for DataEnvelope {}

pub type DataSender = mpsc::UnboundedSender<DataEnvelope>;
pub type DataReceiver = mpsc::UnboundedReceiver<DataEnvelope>;

/// Used to control the streams and interaction between a sender and a receiver.
#[derive(Debug, Default)]
pub struct Joint {
    idx: AtomicUsize,
    active: AtomicBool,
}

impl Joint {
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }

    pub fn switch(&self, active: bool) {
        self.active.store(active, Ordering::Relaxed);
    }
}

#[derive(Debug)]
pub struct Provider {
    joint: Arc<Joint>,
    sender: DataSender,
}

impl Provider {
    pub fn new(entry_id: EntryId) -> Self {
        let (tx, rx) = mpsc::unbounded();
        let joint = Arc::new(Joint::default());
        let this = Provider {
            joint: joint.clone(),
            sender: tx,
        };
        let event = ControlEvent::RegisterJoint2 {
            entry_id,
            joint,
            rx,
        };
        let state = RILL_STATE.get().expect("rill not installed!");
        state.send(event);
        this
    }
}