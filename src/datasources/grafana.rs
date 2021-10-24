use crate::metric::Metric;
use crate::{metric::MetricResult, types};

use hyper::{Body, Client, Method, Request, StatusCode};

use bytes::Buf as _;

mod prometheus;

use serde_json;



pub struct Grafana {
    url: String,
    api_key: String,
}

impl Grafana {
    pub fn new(url: String, api_key: String) -> Grafana {
        Grafana { api_key, url }
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

    pub async fn extract_metrics(
        &self,
        datasource_url: &str,
        query: &str,
        from: u64,
        to: u64,
        step: u64,
    ) -> types::Result<MetricResult> {
        let pm = prometheus::Prometheus::new(self, datasource_url, query);
        // TODO: split big query to chunks
        let r = pm.query(from, to, step).await?;
        Ok(r)
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
