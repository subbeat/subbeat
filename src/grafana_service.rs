use crate::types;

use hyper::{Body, Client, Method, Request, StatusCode};
use tokio::io::{stdout, AsyncWriteExt as _};

use bytes::Buf as _;

mod prometheus;

use serde_json;

use crate::metric::Metric;

pub struct GrafanaService {
    url: String,
    api_key: String,
}

impl GrafanaService {
    pub fn new(url: String, api_key: String) -> GrafanaService {
        GrafanaService { api_key, url }
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

    pub async fn extract_metrics(&self, panel_url: &str) -> types::Result<()> {
        let pm = prometheus::Prometheus::new(self, "rate(go_memstats_alloc_bytes_total[5m])", 15);
        let r = pm.query(1634672070, 1634672970).await;

        // println!("{}", p.to_string());

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
