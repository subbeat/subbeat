use async_trait::async_trait;
use std::{collections::HashMap, result};

use crate::types;

pub type MetricId = String;

const CHUNK_SIZE: u64 = 10_000;

struct DatasourceParams {
    db: String,
    q: String,
    epoch: String,
}

struct Datasource {
    url: String,
    dtype: String,
    params: Option<DatasourceParams>,
    data: Option<HashMap<String, String>>,
    datasource_id: Option<String>,
}

struct MetricQuery {
    url: String,
    method: String,
    schema: HashMap<String, String>,
    headers: Option<HashMap<String, String>>,
}

pub struct MetricResult {
    pub data: HashMap<String, Vec<(u64, f64)>>,
}

impl MetricResult {
    pub fn merge_with(&mut self, mr: &MetricResult) {
        for (k, v) in mr.data.iter() {
            // TODO: think how to make it faster
            if self.data.contains_key(k) {
                self.data.get_mut(k).unwrap().extend(v.iter())
            } else {
                // panic!("not contain key");
                self.data.insert(k.to_owned(), v.to_owned());
            }
        }
    }
}

impl Default for MetricResult {
    fn default() -> MetricResult {
        return MetricResult {
            data: HashMap::<String, Vec<(u64, f64)>>::new(),
        };
    }
}

#[async_trait]
pub trait Metric {
    // (to - from) / step < 10000
    async fn query_chunk(&self, from: u64, to: u64, step: u64) -> types::Result<MetricResult>;

    // TODO: mov eit to grafana-spicific logic
    async fn query(&self, from: u64, to: u64, step: u64) -> types::Result<MetricResult> {
        if from >= to {
            panic!("from >= to");
        }

        if step == 0 {
            panic!("step equals 0");
        }

        let chunk_timespan_size = CHUNK_SIZE * step;

        let mut chunks = Vec::<(u64, u64)>::new();
        {
            let mut f = from;
            while f < to {
                let next = to.min(f + chunk_timespan_size);
                chunks.push((f, next));
                f = next;
            }
        }

        let mut result: MetricResult = Default::default();
        for (f, t) in chunks.iter() {
            let r = self.query_chunk(*f, *t, step).await?;
            result.merge_with(&r);
        }

        Ok(result)
    }
}
