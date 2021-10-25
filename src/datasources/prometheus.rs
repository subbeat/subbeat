use async_trait::async_trait;
use hyper::{Body, Client, Method, Request, StatusCode};

use serde_derive::{Deserialize, Serialize};

use crate::{
    metric::{Metric, MetricResult},
    types,
};

use serde_qs as qs;

use bytes::Buf as _;

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

    // TODO: move to utils
    async fn get(&self, suburl: &str) -> types::Result<(StatusCode, serde_json::Value)> {
        let req = Request::builder()
            .method(Method::GET)
            .uri(self.url.to_owned() + suburl)
            .header("Accept", "application/json")
            .body(Body::empty())
            .unwrap();

        let client = Client::new();
        let res = client.request(req).await?;
        let status = res.status();

        let body = hyper::body::aggregate(res).await?;
        let reader = body.reader();
        let result: serde_json::Value = serde_json::from_reader(reader)?;
        Ok((status, result))
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

        println!("prom str: {}", qs);
        // TODO: query
        // TODO: parse
        return Ok(Default::default());
    }
}
