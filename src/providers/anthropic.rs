use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use super::SYSTEM_PROMPT;
use crate::core::Config;

const ANTHROPIC_VERSION: &str = "2023-06-01";

#[derive(Serialize)]
struct Request {
    model: String,
    max_tokens: u32,
    system: &'static str,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: &'static str,
    content: String,
}

#[derive(Deserialize)]
struct Response {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: String,
}

pub fn call_anthropic(config: &Config, prompt: &str) -> Result<String> {
    let request = Request {
        model: config.model.clone(),
        max_tokens: 1024,
        system: SYSTEM_PROMPT,
        messages: vec![Message {
            role: "user",
            content: prompt.to_string(),
        }],
    };

    let api_key = config.api_key.as_ref().context("Missing API key")?;

    let response = ureq::post(&config.endpoint)
        .header("x-api-key", api_key)
        .header("anthropic-version", ANTHROPIC_VERSION)
        .header("content-type", "application/json")
        .send_json(&request);

    match response {
        Ok(mut resp) => {
            let api_resp: Response = resp
                .body_mut()
                .read_json()
                .context("Failed to parse API response")?;
            api_resp
                .content
                .first()
                .map(|c| c.text.trim().to_string())
                .context("No response from LLM")
        }
        Err(ureq::Error::StatusCode(status)) => {
            bail!("API error ({})", status);
        }
        Err(e) => Err(e).context("Network error"),
    }
}
