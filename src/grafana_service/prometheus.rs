use async_trait::async_trait;
use serde_json::Value;

use crate::{
    metric::{Metric, MetricResult},
    types,
};

use serde_derive::{Deserialize, Serialize};

use serde_qs as qs;

use super::GrafanaService;

pub struct Prometheus<'a> {
    url: String,
    query: String,
    grafana_service: &'a GrafanaService,
}

#[derive(Deserialize, Serialize)]
struct Query {
    query: String,
    start: u64,
    end: u64,
    step: u64
}

impl<'a> Prometheus<'a> {
    pub fn new(
        grafana_service: &'a GrafanaService,
        url: &str,
        query: &str,
    ) -> Prometheus<'a> {
        Prometheus {
            url: url.to_owned(),
            grafana_service,
            query: query.to_string(),
        }
    }
}

fn parse_result(value: Value) -> types::Result<MetricResult> {
    let metric = &value["data"]["result"][0]["metric"];
    let metric_name = metric
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| format!("{}=\"{}\"", k, v.as_str().unwrap()))
        .collect::<Vec<String>>()
        .join(",");

    let metric_name = format!("{{{}}}", metric_name);

    let values = &value["data"]["result"][0]["values"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            let r = e.as_array().unwrap();
            return (
                r[0].as_u64().unwrap(),
                r[1].as_str().unwrap().to_string().parse::<f64>().unwrap(),
            );
        })
        .collect::<Vec<(u64, f64)>>();

    let mut result = MetricResult::new();
    result.insert(metric_name, values.to_owned());

    // println!("{:?}", result);

    return Ok(result);
}

#[async_trait]
impl Metric for Prometheus<'_> {
    async fn query(&self, from: u64, to: u64, step: u64) -> types::Result<MetricResult> {
        let q = Query {
            query: self.query.to_owned(),
            step: step,
            start: from,
            end: to,
        };

        // TODO: use serialisatoin from serde

        let rq = qs::to_string(&q)?;
        let (status_code, value) = self.grafana_service.post_form(&self.url, &rq).await?;
        // TODO: return error
        // if status_code != StatusCode::OK {
        //     return std::error::("Bad status code");
        // }

        // println!("{:?}", value);

        return parse_result(value);
    }
}
