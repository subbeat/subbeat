use bytes::Buf as _;
use hyper::{Body, Client, Method, Request, StatusCode};
use std::io::Read;

use crate::types;

// TODO: move to utils
pub async fn get(url: &String) -> types::Result<(StatusCode, serde_json::Value)> {
    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("Accept", "application/json")
        .body(Body::empty())
        .unwrap();

    let client = Client::new();
    let res = client.request(req).await?;
    let status = res.status();

    let body = hyper::body::aggregate(res).await?;
    let mut reader = body.reader();

    {
        let mut dst = [0; 1024];
        let num = reader.read(&mut dst).unwrap();
        let mut vec = Vec::<u8>::new();
        for i in 0..num {
            vec.push(dst[i]);
        }
        let str = String::from_utf8(vec).unwrap();
        println!("{}", str);
    }

    panic!("asdas");

    // String::from_utf8(reader.bytes());

    // let result: serde_json::Value = serde_json::from_reader(reader)?;
    // Ok((status, result))
}
