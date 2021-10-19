use crate::types;

use hyper::{Body, Client, Method, Request, StatusCode};
use serde_json::json;
use tokio::io::{stdout, AsyncWriteExt as _};

use bytes::Buf as _;

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

    pub async fn extract_metrics(&self) -> types::Result<()> {
        let (s, p) = self
            .post(
                "/api/datasources/proxy/1/api/v1/query_range",
                // serde_json::json!({
                    // "from": "1634237655",
                    // "to": "1634238555",
                    // "queries": [
                    //     {
                    //         "datasourceId": 1,
                    //         "refId": "A",
                    //         "expr": "rate(go_memstats_alloc_bytes_total[5m])",
                    //         "format": "time_series",
                    //         "step": "15",
                    //         "start": "1634329050",
                    //         "end": "1634329950"
                    //     }
                    // ]
                    // "query": "rate(go_memstats_alloc_bytes_total[5m])",
                    // "start": 1634672070,
                    // "end": 1634672970,
                    // "step": "15"
                // }),
                serde_json::json!({})
                
            )
            .await?;
        println!("{}", p.to_string());

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

    async fn post(
        &self,
        suburl: &str,
        value: serde_json::Value,
    ) -> types::Result<(StatusCode, serde_json::Value)> {
        let req = Request::builder()
            .method(Method::POST)
            .uri(self.url.to_owned() + suburl)
            .header("Accept", "application/json")
            // .header("Content-Type", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Bearer {}", self.api_key))
            // .body(Body::from(value.to_string()))
            .body(Body::from("query=rate%28go_memstats_alloc_bytes_total%5B5m%5D%29&start=1634672070&end=1634672970&step=15"))
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
