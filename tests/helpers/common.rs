use std::collections::HashMap;

use anyhow::Result;
use reqwest::{Client, Method, StatusCode};
use serde::Deserialize;
use serde_json::{from_str, Value};

#[derive(Deserialize)]
struct Config {
    base_url: String,
}

fn load_config() -> Config {
    let config =
        std::fs::read_to_string("tests/helpers/config.toml").expect("failed to read config.toml");
    toml::from_str(&config).expect("failed to parse config.toml")
}

pub struct TestClient {
    pub client: Client,
    pub base_url: String,
}

impl TestClient {
    pub async fn new() -> Self {
        let config = load_config();

        dotenv::dotenv().ok();

        let client = reqwest::Client::new();
        Self {
            client,
            base_url: config.base_url,
        }
    }

    pub async fn make_request(
        &self,
        method: Method,
        path: &str,
        query: Option<HashMap<&str, &str>>,
        body: Option<Value>,
    ) -> Result<(StatusCode, Option<Value>)> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.request(method, &url);

        if let Some(q) = query {
            req = req.query(&q);
        }

        if let Some(b) = body {
            req = req.json(&b);
        }

        let res = req.send().await?;
        let status = res.status();
        let text = res.text().await?;

        let response_body = if !text.is_empty() {
            Some(from_str(&text).unwrap_or(Value::String(text)))
        } else {
            None
        };

        Ok((status, response_body))
    }

    #[allow(dead_code)]
    pub async fn get(
        &self,
        path: &str,
        query: Option<HashMap<&str, &str>>,
    ) -> Result<(StatusCode, Option<Value>)> {
        self.make_request(Method::GET, path, query, None).await
    }

    #[allow(dead_code)]
    pub async fn post(
        &self,
        path: &str,
        query: Option<HashMap<&str, &str>>,
        body: Option<Value>,
    ) -> Result<(StatusCode, Option<Value>)> {
        self.make_request(Method::POST, path, query, body).await
    }
}
