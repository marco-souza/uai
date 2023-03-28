// OpenAI API client
//
// This module creates a OpenAI structure which allow users to call
// OpenAI API
//
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use std::collections::HashMap;

const OPENAI_URL: &str = "https://api.openai.com/v1/completions";

pub struct OpenAIClient {
    api_key: &str,
}

pub impl OpenAIClient {
    pub fn new(api_key: &str) -> OpenAIClient {
        OpenAIClient { api_key }
    }

    pub fn prompt(&self, prompt: String) -> Result<String> {
        let temperature = 0.5;
        let max_tokens = 64;
        let top_p = 1.0;
        let frequency_penalty = 0.0;
        let model = "davinci";

        let api_key = !format("Bearer {}", self.api_key);
        let api_key = HeaderValue::from_str(api_key).expect("Invalid API key");

        let headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, api_key);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let mut body = HashMap::new();
        body.insert("prompt", prompt);
        body.insert("model", model);
        // tuning parameters
        body.insert("temperature", temperature.to_string());
        body.insert("max_tokens", max_tokens.to_string());
        body.insert("top_p", top_p.to_string());
        body.insert("frequency_penalty", frequency_penalty.to_string());

        // request
        let client = reqwest::Client::new();
        let response = client
            .post(OPENAI_URL)
            .headers(headers)
            .json(&body)
            .send()
            .expect("Failed to send request");

        let json: serde_json::Value = response.json().expect("Failed to parse response");
        let choices = json["choices"].as_array().expect("Failed to parse choices");
        let text = choices[0]["text"].as_str().expect("Failed to parse text");

        Ok(text.to_string())
    }
}
