use async_trait::async_trait;
use std::collections::HashMap;

use crate::types;

pub type MetricId = String;

struct DatasourceParams {
    db: String,
    q: String,
    epoch: String,
}

struct Datasource {
    url: String,
    dtype: String,
    params: Option<DatasourceParams>,
    data: Option<HashMap<String, String>>,
    datasource_id: Option<String>,
}

struct MetricQuery {
    url: String,
    method: String,
    schema: HashMap<String, String>,
    headers: Option<HashMap<String, String>>,
}

pub type MetricResult = HashMap<String, Vec<(u64, f64)>>;

#[async_trait]
pub trait Metric {
    async fn query(&self, from: u64, to: u64) -> types::Result<MetricResult>;
}
