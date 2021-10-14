use std::io::BufRead;

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
        let req = Request::builder()
            .method(Method::GET)
            .uri(self.url.clone())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(Default::default())
            .unwrap();

        let client = Client::new();
        let resp = client.request(req).await?;
        println!("Response: {}", resp.status());
        Ok(())
    }

    pub async fn extract_metrics(&self) -> types::Result<()> {
        let req = Request::builder()
            .method(Method::POST)
            .uri(self.url.clone() + "/api/datasources/proxy/1/api/v1/query_range")
            .header("content-type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(Body::from(json!({ 
                "query":"go_memstats_alloc_bytes_total", 
                "start": "1634163645",
                "end": "1634163945", 
                "step": "15"
            }).to_string())
            )
                
            .unwrap();

        let client = Client::new();
        let res = client.request(req).await?;
        println!("Response: {}", res.status());
            
        // while let Some(next) = resp.body().data().await {
        //     let chunk = next?;
        //     io::stdout().write_all(&chunk).await?;
        // }
        
        let body = hyper::body::aggregate(res).await?;
        let mut reader = body.reader();
        let mut line = String::new();
        loop {
            match reader.read_line(&mut line) {
                Ok(_) => println!("{}", line),
                Err(_) => break
            }
        }
        

        Ok(())
    }
}
