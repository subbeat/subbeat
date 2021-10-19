use async_trait::async_trait;

use crate::{
    metric::{Metric, MetricResult},
    types,
};

use serde_derive::{Deserialize, Serialize};

use super::GrafanaService;

pub struct Prometheus<'a> {
    query: String,
    step: u32,
    grafana_service: &'a GrafanaService,
}

#[derive(Deserialize, Serialize)]
struct Query {
    query: String,
    step: u32,
    start: u64,
    end: u64,
}

impl<'a> Prometheus<'a> {
    pub fn new(grafana_service: &'a GrafanaService, query: &str, step: u32) -> Prometheus<'a> {
        Prometheus {
            grafana_service,
            query: query.to_string(),
            step,
        }
    }
}

#[async_trait]
impl Metric for Prometheus<'_> {
    async fn query(&self, from: u64, to: u64) -> types::Result<MetricResult> {
        let q = Query {
            query: self.query.to_owned(),
            step: self.step,
            start: from,
            end: to,
        };
        let url = "/api/datasources/proxy/1/api/v1/query_range";
        // TODO: use serialisatoin from serde
        let rq = "query=rate%28go_memstats_alloc_bytes_total%5B5m%5D%29&start=1634672070&end=1634672970&step=15";
        let (status_code, value) = self.grafana_service.post_form(&url, &rq).await?;
        // TODO: return error
        // if status_code != StatusCode::OK {
        //     return std::error::("Bad status code");
        // }

        println!("{:?}", value);

        return Ok(Vec::new());
    }
}
