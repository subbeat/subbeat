use anyhow;

// A simple type alias so as to DRY.

pub type Result<T> = anyhow::Result<T>;

pub enum DatasourceType {
    Grafana,
    Prometheus,
}

pub struct QueryConfig {
    pub datasource_type: DatasourceType,
    pub url: String,
    pub key: String,
    pub datasource_url: String,
    pub query: String,
    pub from: u64,
    pub to: u64,
    pub step: u64,
}
