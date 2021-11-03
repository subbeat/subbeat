use std::collections::HashMap;

use async_trait::async_trait;

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::{metric::{Metric, MetricResult}, types::{self, InfluxConfig}, utils::{self, normalize_url}};

use serde_qs as qs;

#[derive(Clone)]
pub struct Influx {
    url: String,
    org_id: String,
    query: String,
    token: String,
}


impl Influx {
    pub fn new(cfg: &InfluxConfig) -> Influx {
        Influx {
            url: cfg.url.to_owned(),
            org_id: cfg.org_id.to_owned(),
            token: cfg.token.to_owned(),
            query: cfg.query.to_owned(),
        }
    }
}

pub fn parse_result(value: Value) -> types::Result<MetricResult> {
    Err(anyhow::format_err!("not implemented"))
}

#[async_trait]
impl Metric for Influx {
    async fn query_chunk(&self, from: u64, to: u64, step: u64) -> types::Result<MetricResult> {
        
        let url = format!(
            "{}/api/v2/query?orgID={}",
            normalize_url(self.url.to_owned()),
            self.org_id
        );

        println!("URL: {}", url);
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "application/csv".to_owned());
        headers.insert("Authorization".to_string(), format!("Token {}", self.token).to_owned());
        headers.insert("Content-type".to_string(), "application/vnd.flux".to_owned());
        let (_status_code, value) = utils::post_with_headers(&url, headers).await?;

        return parse_result(value);
    }
}


// curl -XPOST "localhost:8086/api/v2/query?orgID=5abe4759f7360f1c" -sS \
//   -H 'Accept:application/csv' \
//   -H 'Authorization: Token sCAB2MVo8TJxhUH8UDJZIeCPwf-cykBtO0jhr207qCQSZ9d43JXObCYK_uAml2BL26JBYFauz95yIeC51kxQog==' \
//   -H 'Content-type:application/vnd.flux' \
//   -d 'from(bucket:"main-backet")
//         |> range(start:-5m) 
//         |> filter(fn:(r) => r._measurement == "cpu")'
