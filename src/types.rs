use anyhow;

// A simple type alias so as to DRY.

pub type Result<T> = anyhow::Result<T>;
pub type TimeSerie = Vec<(u64, f64)>;

#[derive(Clone)]
pub struct PrometheusConfig {
    pub url: String,
    pub query: String,
}

#[derive(Clone)]
pub struct InfluxConfig {
    pub url: String,
    pub org_id: String,
    pub token: String,
    pub query: String,
}

#[derive(Clone)]
pub struct GrafanaConfig {
    pub url: String,
    pub api_key: String,
    pub datasource_url: String,
    pub query: String,
}

#[derive(Clone)]
pub enum DatasourceConfig {
    Grafana(GrafanaConfig),
    Prometheus(PrometheusConfig),
    Influx(InfluxConfig),
}

pub struct QueryConfig {
    pub datasource_config: DatasourceConfig,
    pub from: u64,
    pub to: u64,
    pub step: u64,
}


