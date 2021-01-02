use super::{Config, ReadConfigFile};
use crate::actors::embedded_node::EmbeddedNode;
use crate::actors::exporter::ExporterLinkForCtrl;
use anyhow::Error;
use async_trait::async_trait;
use meio::prelude::{Actor, Context, IdOf, InterruptedBy, StartedBy, TaskEliminated, TaskError};
use rill_protocol::Path;

pub struct Tuner {
    exporter: ExporterLinkForCtrl,
}

impl Tuner {
    pub fn new(exporter: ExporterLinkForCtrl) -> Self {
        Self { exporter }
    }
}

impl Actor for Tuner {
    type GroupBy = ();
}

#[async_trait]
impl StartedBy<EmbeddedNode> for Tuner {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.spawn_task(ReadConfigFile, ());
        Ok(())
    }
}

#[async_trait]
impl InterruptedBy<EmbeddedNode> for Tuner {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.shutdown();
        Ok(())
    }
}

#[async_trait]
impl TaskEliminated<ReadConfigFile> for Tuner {
    async fn handle(
        &mut self,
        _id: IdOf<ReadConfigFile>,
        result: Result<Config, TaskError>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        match result {
            Ok(mut config) => {
                if let Some(export) = config.export.paths.take() {
                    for path_str in export {
                        let path: Path = path_str.parse()?;
                        log::info!("Export path: {}", path);
                        self.exporter.export_path(path).await?;
                    }
                }
                if let Some(_) = config.export.prometheus.take() {
                    self.exporter.start_prometheus().await?;
                }
                if let Some(_) = config.export.graphite.take() {
                    self.exporter.start_graphite().await?;
                }
            }
            Err(err) => {
                log::warn!(
                    "Can't read config file. No special configuration parameters applied: {}",
                    err
                );
            }
        }
        Ok(())
    }
}
