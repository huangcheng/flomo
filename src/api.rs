use std::collections::HashMap;
use std::error::Error;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize};

use crate::config::read_config;

#[derive(Deserialize)]
struct Memo {
    slug: String,
}

#[derive(Deserialize)]
struct Response {
    code: i32,
    message: String,
    memo: Memo,
}

pub fn send(memo: &str) -> Result<String, Box<dyn Error>> {
    let config = match read_config() {
        Some(config) => config,
        None => return Err("Failed to read config!".into()),
    };

    let url = match config.api {
        Some(url) => url,
        None => return Err("No API URL found!".into()),
    };

    let mut headers = HeaderMap::new();
    headers.append(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let mut body = HashMap::new();
    body.insert("content", memo);

    let response = match reqwest::blocking::Client::new()
        .post(url)
        .json(&body)
        .send() {
        Ok(response) => response,
        Err(_) => return Err("Failed to send request!".into()),
    };

    let resp = match response.json::<Response>() {
        Ok(resp) => resp,
        Err(_) => return Err("Failed to parse response!".into()),
    };

    if resp.code != 0 {
        return Err(resp.message.into());
    }

    let id = resp.memo.slug;

    return Ok(format!("https://v.flomoapp.com/mine/?memo_id={}", id));
}
