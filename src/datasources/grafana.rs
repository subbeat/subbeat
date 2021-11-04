use crate::metric::Metric;
use crate::types::GrafanaConfig;
use crate::{metric::MetricResult, types};
use async_trait::async_trait;

use hyper::{Body, Client, Method, Request, StatusCode};

use bytes::Buf as _;

mod prometheus;

use serde_json;

#[derive(Clone)]
pub struct Grafana {
    url: String,
    api_key: String,
    datasource_url: String,
    query: String,
}

impl Grafana {
    pub fn new(config: &GrafanaConfig) -> Grafana {
        Grafana {
            api_key: config.api_key.to_owned(),
            url: config.url.to_owned(),
            datasource_url: config.datasource_url.to_owned(),
            query: config.query.to_owned(),
        }
    }

    pub async fn test_connection(&self) -> types::Result<()> {
        println!("Test connection response");
        let (s, p) = self.get("/api").await?;
        println!("{}", p.to_string());
        Ok(())
    }

    pub async fn get_datasources(&self) -> types::Result<()> {
        let (s, p) = self.get("/api/datasources").await?;
        println!("{}", p);
        Ok(())
    }

    async fn get(&self, suburl: &str) -> types::Result<(StatusCode, serde_json::Value)> {
        let req = Request::builder()
            .method(Method::GET)
            .uri(self.url.to_owned() + suburl)
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
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

    async fn post_form(
        &self,
        suburl: &str,
        value: &str,
    ) -> types::Result<(StatusCode, serde_json::Value)> {
        let req = Request::builder()
            .method(Method::POST)
            .uri(self.url.to_owned() + suburl)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(Body::from(value.to_string()))
            .unwrap();

        let client = Client::new();
        let res = client.request(req).await?;
        let status = res.status();

        let body = hyper::body::aggregate(res).await?;
        let reader = body.reader();
        let result: serde_json::Value = serde_json::from_reader(reader)?;
        Ok((status, result))
    }

    async fn post_json(
        &self,
        suburl: &str,
        value: serde_json::Value,
    ) -> types::Result<(StatusCode, serde_json::Value)> {
        let req = Request::builder()
            .method(Method::POST)
            .uri(self.url.to_owned() + suburl)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(Body::from(value.to_string()))
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
impl Metric for Grafana {
    async fn query_chunk(&self, from: u64, to: u64, step: u64) -> types::Result<MetricResult> {
        let pm = prometheus::Prometheus::new(self.clone(), &self.datasource_url, &self.query);
        let r = pm.query(from, to, step).await?;
        Ok(r)
    }

    fn boxed_clone(&self) -> Box<dyn Metric + Sync + Send> {
        return Box::new(self.clone());
    }
}
