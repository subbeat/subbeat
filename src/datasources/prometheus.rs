use async_trait::async_trait;
use hyper::{Body, Client, Method, Request, StatusCode};

use serde_derive::{Deserialize, Serialize};

use crate::{
    metric::{Metric, MetricResult},
    types, utils,
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

        let url = format!("{}/api/v1/query_range?{}", self.url, qs);

        let v = utils::get(&url).await?;

        println!("Prom value:");
        println!("{:?}", &v);
        // TODO: query
        // TODO: parse
        return Ok(Default::default());
    }
}
