#![allow(dead_code)]
use log::debug;
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};

const R8_VERSION: &str = "a9758cbfbd5f3c2094457d996681af52552901775aa2d6dd0b17fd15df959bef";

const R8_URL: &str = "https://api.replicate.com/v1/predictions";

#[derive(Deserialize, Debug)]
struct PredictionResponse {
    completed_at: Option<String>,
    created_at: Option<String>,
    error: Option<String>,
    hardware: String,
    id: String,
    input: Input,
    logs: String,
    metrics: Metrics,
    output: Option<Vec<String>>,
    started_at: Option<String>,
    status: String,
    urls: Urls,
    version: String,
    webhook_completed: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Metrics {
    predict_time: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Input {
    prompt: String,
    seed: Option<u32>,
    num_inference_steps: Option<u32>,
    guidance_scale: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Urls {
    get: String,
    cancel: String,
}

#[derive(Serialize, Debug)]
struct R8Request {
    version: String,
    input: Input,
    webhook_completed: Option<String>,
}

pub struct R8Client {
    client: Client,
}

impl R8Client {
    pub fn new() -> Self {
        let client = Client::new();
        Self { client }
    }

    pub async fn request(&self, prompt: String) {
        let input = Input {
            prompt,
            seed: None,
            num_inference_steps: None,
            guidance_scale: None,
        };

        let mut webhook = std::env::var("WEBHOOK_URL")
            .expect("WEBHOOK_URL must be set and point to current address");

        webhook.push_str("/webhook/test-id");

        let body = R8Request {
            version: R8_VERSION.to_string(),
            input,
            webhook_completed: Some(webhook),
        };

        let body = serde_json::to_string(&body).unwrap();

        let token =
            std::env::var("R8_TOKEN").expect("Replicate's token must be set at R8_TOKEN var");

        let response = self
            .client
            .post(R8_URL.to_string())
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, "Token ".to_string() + &token)
            .body(body)
            .send()
            .await;

        debug!("{:#?}", response);
    }
}
