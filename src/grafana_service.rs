use hyper::{Body, Client, Method, Request};
use tokio::io::{stdout, AsyncWriteExt as _};

use crate::types;

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
}
