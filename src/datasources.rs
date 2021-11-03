use crate::{metric::Metric, types::{DatasourceConfig}};

pub mod grafana;
pub mod prometheus;

pub fn resolve(config: &DatasourceConfig) -> Box<dyn Metric + Sync> {
    match config {
        DatasourceConfig::Grafana(cfg) => Box::new(grafana::Grafana::new(cfg)),
        DatasourceConfig::Prometheus(cfg) => Box::new(prometheus::Prometheus::new(cfg))
    }
}
