use bytes::{buf::Reader, Buf};
use hyper::{body, Body, Client, Method, Request, StatusCode};
use std::{collections::HashMap, io::Read};

use crate::types::{self, TimeSerie};

pub fn print_buf(mut reader: Reader<impl Buf>) {
    let mut dst = [0; 1024];
    let mut vec = Vec::<u8>::new();

    loop {
        let num = reader.read(&mut dst).unwrap();
        if num == 0 {
            break;
        }
        for i in 0..num {
            vec.push(dst[i]);
        }
    }

    let str = String::from_utf8(vec).unwrap();
    println!("{}", str);
}

pub fn normalize_url(url: String) -> String {
    if url.ends_with("/") {
        let res = url.strip_suffix("/").unwrap();
        return res.to_string();
    }
    return url;
}

pub async fn get(url: &String) -> types::Result<(StatusCode, serde_json::Value)> {
    let req_result = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("Accept", "application/json")
        .body(Body::empty());

    if req_result.is_err() {
        println!("{:?}", req_result);
        panic!("Can`t connect");
    }

    let req = req_result.unwrap();

    let client = Client::new();
    let res = client.request(req).await?;
    let status = res.status();

    let body = hyper::body::aggregate(res).await?;
    let reader = body.reader();

    let result: serde_json::Value = serde_json::from_reader(reader)?;
    Ok((status, result))
}

pub async fn post_with_headers(
    url: &String,
    headers: &HashMap<String, String>,
    body: hyper::Body,
) -> types::Result<(StatusCode, Reader<impl Buf>)> {
    let mut builder = Request::builder().method(Method::POST).uri(url);

    for (k, v) in headers {
        builder = builder.header(k, v);
    }

    let req_result = builder.body(body);

    if req_result.is_err() {
        println!("{:?}", req_result);
        panic!("Can`t connect");
    }

    let req = req_result.unwrap();

    let client = Client::new();
    let res = client.request(req).await?;
    let status = res.status();

    let body = hyper::body::aggregate(res).await?;
    let reader = body.reader();

    Ok((status, reader))
}


// TODO: cover this function with tests
pub fn interpolate_nans_and_gaps_with_zeros(ts: &TimeSerie, from: u64, to: u64, step: u64) -> TimeSerie {
    // add zeros in the begining
    let mut result: TimeSerie = Vec::new();
    let mut t = from;

    if ts.len() == 0 {
        while t + step < to {
            result.push((t, 0.0));
            t += step;
        }
        return result;
    }

    while t + step < ts[0].0 {
        result.push((t, 0.0));
        t += step;
    }

    let mut i = 0usize;
    while i + 1 < ts.len() {
        let mut my_t = ts[i].0;
        let v = if f64::is_nan(ts[i].1) { 0.0 } else { ts[i].1 };
        result.push((my_t, v));

        let next_t = ts[i + 1].0;

        while my_t + step < next_t {
            my_t += step;
            result.push((my_t, 0.0));
        }

        i += 1;
    }

    let mut t = ts.last().unwrap().0;
    let v = if f64::is_nan(ts.last().unwrap().1) { 0.0 } else { ts.last().unwrap().1 };

    result.push((t, v));

    while t + step < to {
        t += step;
        result.push((t, 0.0));
    }

    return result;
}
