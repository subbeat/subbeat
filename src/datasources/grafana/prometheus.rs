use async_trait::async_trait;
use hyper::StatusCode;

use crate::{
    datasources::prometheus,
    metric::{Metric, MetricResult},
    types,
};

use serde_derive::{Deserialize, Serialize};

use serde_qs as qs;

use super::Grafana;

#[derive(Clone)]
pub struct Prometheus {
    url: String,
    query: String,
    grafana_service: Grafana,
}

#[derive(Deserialize, Serialize)]
struct Query {
    query: String,
    start: u64,
    end: u64,
    step: u64,
}

impl Prometheus {
    pub fn new(grafana_service: Grafana, url: &str, query: &str) -> Prometheus {
        Prometheus {
            url: url.to_owned(),
            grafana_service,
            query: query.to_string(),
        }
    }
}

#[async_trait]
impl Metric for Prometheus {
    async fn query_chunk(&self, from: u64, to: u64, step: u64) -> types::Result<MetricResult> {
        if from >= to {
            panic!("from >= to");
        }

        let q = Query {
            query: self.query.to_owned(),
            step: step,
            start: from,
            end: to,
        };

        let rq = qs::to_string(&q)?;
        let (status_code, value) = self.grafana_service.post_form(&self.url, &rq).await?;

        if status_code != StatusCode::OK {
            let error = &value["error"].as_str().unwrap();
            return Err(anyhow::anyhow!("Can`t query: {}", error));
        }

        return prometheus::parse_result(value);
    }

    fn boxed_clone(&self) -> Box<dyn Metric> {
        return Box::new(self.clone());
    }
}
