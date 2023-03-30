// OpenAI API client
//
// This module creates a OpenAI structure which allow users to call
// OpenAI API
//
use const_env::from_env;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[from_env]
const OPENAI_API_KEY: &'static str = "";
#[from_env]
const OPENAI_URL: &'static str = "https://api.openai.com/v1/completions";

pub struct OpenAIClient {
    api_key: String,
}

impl OpenAIClient {
    pub fn new() -> OpenAIClient {
        OpenAIClient {
            api_key: OPENAI_API_KEY.to_string(),
        }
    }

    pub async fn prompt(&self, prompt: String) -> Result<String, ()> {
        let api_key = format!("Bearer {}", self.api_key);
        let api_key = HeaderValue::from_str(api_key.as_str()).expect("Invalid API key");

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, api_key);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let body = OpenAISettings {
            prompt: prompt.to_string(),
            model: "text-davinci-003".to_string(),
            temperature: 0.9,
            max_tokens: 150,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.6,
            stop: vec!["Human:".to_string(), "AI:".to_owned()],
        };

        // request
        let client = reqwest::Client::new();
        let response = client
            .post(OPENAI_URL)
            .headers(headers)
            .json::<OpenAISettings>(&body)
            .send()
            .await
            .expect("Failed to send request");

        match response.status() {
            reqwest::StatusCode::OK => {
                let json = response
                    .json::<ResponseBody>()
                    .await
                    .expect("Failed to parse response");
                let choice = json.choices.first().expect("No choices");
                Ok(choice.text.clone())
            }
            _ => {
                println!(
                    "Error: {} {}",
                    response.status(),
                    response.text().await.unwrap()
                );
                return Err(());
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAISettings {
    pub prompt: String,
    pub model: String,
    pub temperature: f64,
    pub max_tokens: u32,
    pub top_p: f64,
    pub frequency_penalty: f64,
    pub presence_penalty: f64,
    pub stop: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseBody {
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    pub text: String,
    pub index: u32,
}
