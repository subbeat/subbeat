use std::collections::HashMap;

use async_trait::async_trait;

use bytes::{buf::Reader, Buf};

use crate::{
    metric::{Metric, MetricResult},
    types::{self, InfluxConfig},
    utils::{self, normalize_url},
};

#[derive(Clone)]
pub struct Influx {
    url: String,
    org_id: String,
    query: String,
    token: String,
}

impl Influx {
    pub fn new(cfg: &InfluxConfig) -> Influx {
        if !cfg.query.contains("$range") {
            panic!(
                "Bad query: missing $range variable. Example: \n 'from(bucket: \"main-backet\") |> $range |> filter(fn: (r) => r[\"_measurement\"] == \"cpu\") |> filter(fn: (r) => r[\"_field\"] == \"usage_softirq\") |> filter(fn: (r) => r[\"cpu\"] == \"cpu-total\") |> filter(fn: (r) => r[\"host\"] == \"roid\") |> yield(name: \"mean\")')"
            );
        }

        Influx {
            url: cfg.url.to_owned(),
            org_id: cfg.org_id.to_owned(),
            token: cfg.token.to_owned(),
            query: cfg.query.to_owned(),
        }
    }
}

pub fn parse_result(reader: Reader<impl Buf>) -> types::Result<MetricResult> {

    // println!("---------------");
    // utils::print_buf(reader);
    // println!("xxxxxxxxxxxxxxx");
    // return Ok(Default::default());

    let mut rdr = csv::Reader::from_reader(reader);

    let hdrs = rdr.headers();
    if hdrs.is_err() {
        return Err(anyhow::format_err!(
            "Cant' extract metric: headers are empty"
        ));
    }

    let hdrs = hdrs.unwrap();
    if hdrs.len() == 0 {
        return Ok(Default::default());
    }

    // TODO: get it from actual response
    let measurement_name_position = 8usize;

    if hdrs.get(measurement_name_position).is_none() {
        // println!("HEADERS:");
        // for h in hdrs {
        //     println!("{}", h);
        // }
        return Err(anyhow::format_err!(
            "Cant' extract metric: no measurement at position {}", measurement_name_position
        ));
    }
    // println!("len: {:?}", rdr.headers().unwrap().get(9));
    let measurement = hdrs.get(measurement_name_position).unwrap().to_owned();

    // println!("_measurement {}", measurement);

    let mut ts = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let t = chrono::DateTime::parse_from_rfc3339(record.get(5).unwrap())
            .unwrap()
            .timestamp() as u64;
        let v = record.get(6).unwrap().parse::<f64>().unwrap();
        ts.push((t, v));
        // println!("{:?} > {:?}", t, v);
        // println!("{:?}", record);
    }

    let mut result: MetricResult = Default::default();
    result.data.insert(measurement, ts);
    // result.data.insert(metric_name, values.to_owned());

    Ok(result)
}

#[async_trait]
impl Metric for Influx {
    async fn query_chunk(&self, from: u64, to: u64, step: u64) -> types::Result<MetricResult> {
        // TODO: use step

        let url = format!(
            "{}/api/v2/query?orgID={}",
            normalize_url(self.url.to_owned()),
            self.org_id
        );

        // println!("{}", url);

        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "application/csv".to_owned());
        headers.insert(
            "Authorization".to_string(),
            format!("Token {}", self.token).to_owned(),
        );
        headers.insert(
            "Content-type".to_string(),
            "application/vnd.flux".to_owned(),
        );

        let range_str = format!("range(start:{},stop:{})", from, to);
        let query = self.query.replace("$range", range_str.as_str());

        // println!("query: {}", query);
        let body = hyper::Body::from(query);

        let (_status_code, reader) = utils::post_with_headers(&url, &headers, body).await?;

        parse_result(reader)
    }

    fn boxed_clone(&self) -> Box<dyn Metric + Sync + Send> {
        Box::new(self.clone())
    }
}

// curl -XPOST "localhost:8086/api/v2/query?orgID=5abe4759f7360f1c" -sS \
//   -H 'Accept:application/csv' \
//   -H 'Authorization: Token sCAB2MVo8TJxhUH8UDJZIeCPwf-cykBtO0jhr207qCQSZ9d43JXObCYK_uAml2BL26JBYFauz95yIeC51kxQog==' \
//   -H 'Content-type:application/vnd.flux' \
//   -d 'from(bucket:"main-backet")
//         |> range(start:-5m)
//         |> filter(fn:(r) => r._measurement == "cpu")'
