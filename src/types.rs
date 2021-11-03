use anyhow;

// A simple type alias so as to DRY.

pub type Result<T> = anyhow::Result<T>;


pub struct PrometheusConfig {
    pub url: String,
    pub query: String,
}


pub struct GrafanaConfig {
    pub url: String,
    pub api_key: String,
    pub datasource_url: String,
    pub query: String,
}


pub enum DatasourceConfig {
    Grafana(GrafanaConfig),
    Prometheus(PrometheusConfig)
}

pub struct QueryConfig {
    pub datasource_config: DatasourceConfig,
    pub from: u64,
    pub to: u64,
    pub step: u64
}


