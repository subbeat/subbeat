use crate::{
    metric::Metric,
    types::{DatasourceType, QueryConfig},
};

pub mod grafana;
pub mod prometheus;

pub fn resolve(query_config: &QueryConfig) -> Box<dyn Metric> {
    if query_config.datasource_type == DatasourceType::Grafana {
        let gs = grafana::Grafana::new(
            query_config.url.to_string(),
            query_config.key.to_string(),
            query_config.datasource_url.to_string(),
            query_config.query.to_string(),
        );
        return Box::new(gs);
    } else {
        let pm = prometheus::Prometheus::new(&query_config.url, &query_config.query);
        return Box::new(pm);
    }
}
