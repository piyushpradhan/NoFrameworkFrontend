use reqwest::{header::HeaderMap, Client, Error, Response};
use serde_json::Value;

use crate::store;

pub async fn get(url: &str, mut headers: HeaderMap) -> Result<Response, Error> {
    let client = store::get::<Client>();
    return client.get(url).headers(headers).send().await;
}

pub async fn post(url: &str, mut headers: HeaderMap, body: Value) -> Result<Response, Error> {
    let client = store::get::<Client>();
    return client.post(url).headers(headers).json(&body).send().await;
}
