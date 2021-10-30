use async_trait::async_trait;
use hyper::{Body, Client, Method, Request, StatusCode};

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    metric::{Metric, MetricResult},
    types,
    utils::{self, normalize_url},
};

use serde_qs as qs;

use bytes::Buf as _;
use std::io::Read;

pub struct Prometheus {
    url: String,
    query: String,
}

#[derive(Deserialize, Serialize)]
struct Query {
    query: String,
    start: u64,
    end: u64,
    step: u64,
}

impl Prometheus {
    pub fn new(url: &String, query: &String) -> Prometheus {
        Prometheus {
            url: url.to_owned(),
            query: query.to_owned(),
        }
    }
}

pub fn parse_result(value: Value) -> types::Result<MetricResult> {
    // TODO: check that metric exists
    // TODO: check status: "error"
    if value.get("data").is_none() {
        return Err(anyhow::format_err!("no data in response"));
    }
    if value["data"].get("result").is_none() {
        return Err(anyhow::format_err!("no result in response"));
    }
    if value["data"]["result"].as_array().unwrap().len() == 0 {
        return Ok(Default::default());
    }
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

    let mut result: MetricResult = Default::default();
    result.data.insert(metric_name, values.to_owned());

    // println!("{:?}", result);

    return Ok(result);
}

#[async_trait]
impl Metric for Prometheus {
    async fn query_chunk(&self, from: u64, to: u64, step: u64) -> types::Result<MetricResult> {
        let q = Query {
            query: self.query.to_owned(),
            start: from,
            end: to,
            step,
        };
        let qs = qs::to_string(&q)?;
        let url = format!(
            "{}/api/v1/query_range?{}",
            normalize_url(self.url.to_owned()),
            qs
        );
        let (_status_code, value) = utils::get(&url).await?;

        return parse_result(value);
    }
}
