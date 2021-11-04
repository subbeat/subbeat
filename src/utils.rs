use bytes::{buf::Reader, Buf};
use hyper::{body, Body, Client, Method, Request, StatusCode};
use std::{collections::HashMap, io::Read};

use crate::types;

fn print_buf(mut reader: Reader<impl Buf>) {
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
    //.header("Accept", "application/json");

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
