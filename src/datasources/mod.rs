pub mod grafana;
pub mod prometheus;
pub mod influx;

use crate::{metric::Metric, types::{DatasourceConfig}};


pub fn resolve(config: &DatasourceConfig) -> Box<dyn Metric + Sync> {
    match config {
        DatasourceConfig::Grafana(cfg) => Box::new(grafana::Grafana::new(cfg)),
        DatasourceConfig::Prometheus(cfg) => Box::new(prometheus::Prometheus::new(cfg)),
        DatasourceConfig::Influx(cfg) => Box::new(influx::Influx::new(cfg))
    }
}
