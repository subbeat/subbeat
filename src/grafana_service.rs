use std::{io::BufRead};

use hyper::{Body, Client, Method, Request};
use tokio::io::{stdout, AsyncWriteExt as _};
use serde_json::json;

use crate::types;
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
        self.get("/api").await?;
        Ok(())
    }

    pub async fn get_datasources(&self) -> types::Result<()> {
        self.get("/api/datasources").await?;
        Ok(())
    }

    pub async fn extract_metrics(&self) -> types::Result<()> {
        let req = Request::builder()
            .uri(self.url.clone() + "/api/datasources/proxy/1/api/v1/query_range")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .method(Method::POST)
            .header("content-type", "application/json")
            .body(Body::from(json!({ 
                "query":"go_memstats_alloc_bytes_total", 
                "from": "1634163645",
                "to": "1634163945",
                "step": "15"
            }).to_string())
            )

            .unwrap();

        Ok(())
    }

    async fn get(&self, suburl:&str) -> types::Result<serde_json::Value> {
        let req = Request::builder()
            .method(Method::GET)
            .uri(self.url.to_owned() + suburl)
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(Body::empty())
            .unwrap();

        let client = Client::new();
        let res = client.request(req).await?;
        println!("Response: {}", res.status());
        println!("");

        let body = hyper::body::aggregate(res).await?;
        let reader = body.reader();
        let result: serde_json::Value = serde_json::from_reader(reader)?;
        // let mut line = String::new();
        // loop {
        //     match reader.read_line(&mut line) {
        //         Ok(s) => {
        //             if s == 0 {
        //                 break;
        //             }
        //             println!("{}", line);
        //             line.clear();
        //         },
        //         Err(_) => break
        //     }
        // }
        Ok(result)
    }
}
