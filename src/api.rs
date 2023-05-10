use std::collections::HashMap;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize};

use crate::config::read_config;

#[derive(Deserialize)]
struct Memo {
    slug: String,
}

#[derive(Deserialize)]
struct FlomoResponse {
    code: i32,
    message: String,
    memo: Memo,
}

pub fn send(memo: &str) -> Option<String> {
    if let Some(config) = read_config() {
        if let Some(url) = config.api {
            let mut headers = HeaderMap::new();
            headers.append(CONTENT_TYPE, HeaderValue::from_static("application/json"));

            let mut body = HashMap::new();
            body.insert("content", memo);

            let client = reqwest::blocking::Client::new();

            let response = match client.post(url).json(&body).send() {
                Ok(response) => response,
                Err(err) => panic!("{}", err),
            };

            let resp = match response.json::<FlomoResponse>() {
                Ok(content) => {
                    if content.code != 0 {
                        panic!("{}", content.message);
                    }

                    content
                },
                Err(_) => panic!("Failed to obtain response"),
            };

            let id = resp.memo.slug;

            return Some(format!("https://v.flomoapp.com/mine/?memo_id={}", id));
        }

        return None;
    }

    None
}
