use std::ptr::NonNull;

use async_trait::async_trait;

use crate::{
    metric::{Metric, MetricResult},
    types,
};


struct Prometheus {

}

impl Prometheus {
    pub fn new() -> Prometheus {
        Prometheus{}
    }
}

#[async_trait]
impl Metric for Prometheus {
    async fn query_chunk(&self, from: u64, to: u64, step: u64) -> types::Result<MetricResult> {
        return None;
    }
}