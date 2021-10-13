use std::collections::HashMap;

pub type MetricId = String;

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

trait Metric {
    fn query();
}
