mod actor;
pub use actor::Exporter;

mod link;
pub use link::{ExporterLinkForCtrl, ExporterLinkForData};

mod graphite;
use graphite::GraphiteExporter;

mod prometheus;
use prometheus::PrometheusExporter;

use rill::protocol::{Path, RillData};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum ExportEvent {
    SetInfo {
        path: Path,
        info: String,
    },
    BroadcastData {
        path: Path,
        timestamp: Duration,
        data: RillData,
    },
}