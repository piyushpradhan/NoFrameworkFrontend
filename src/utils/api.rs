use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Error, Response,
};
use serde_json::Value;

use crate::store;

use super::storage::get_item;

pub async fn get(url: &str, mut headers: HeaderMap) -> Result<Response, Error> {
    let client = store::get::<Client>();
    let cookie = get_item("cookie");
    headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());

    return client
        .get(url)
        .bearer_auth::<String>(cookie)
        .headers(headers)
        .send()
        .await;
}

pub async fn post(url: &str, mut headers: HeaderMap, body: Value) -> Result<Response, Error> {
    let client = store::get::<Client>();
    let cookie = get_item("cookie");
    headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());

    return client
        .post(url)
        .bearer_auth(cookie)
        .headers(headers)
        .json(&body)
        .send()
        .await;
}
