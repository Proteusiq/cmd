use std::time::Duration;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use ureq::Agent;

use super::SYSTEM_PROMPT;
use crate::core::Config;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Serialize)]
struct Request {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: &'static str,
    content: String,
}

#[derive(Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: String,
}

pub fn call_openai(config: &Config, prompt: &str) -> Result<String> {
    let request = Request {
        model: config.model.clone(),
        messages: vec![
            Message {
                role: "system",
                content: SYSTEM_PROMPT.to_string(),
            },
            Message {
                role: "user",
                content: prompt.to_string(),
            },
        ],
    };

    let agent: Agent = ureq::Agent::config_builder()
        .timeout_global(Some(REQUEST_TIMEOUT))
        .build()
        .into();

    let mut req = agent
        .post(&config.endpoint)
        .header("content-type", "application/json");

    if let Some(api_key) = config.api_key_exposed() {
        req = req.header("Authorization", &format!("Bearer {}", api_key));
    }

    let response = req.send_json(&request);

    match response {
        Ok(mut resp) => {
            let api_resp: Response = resp
                .body_mut()
                .read_json()
                .context("Failed to parse API response")?;
            api_resp
                .choices
                .first()
                .map(|c| c.message.content.trim().to_string())
                .context("No response from LLM")
        }
        Err(ureq::Error::StatusCode(status)) => {
            bail!("API error ({})", status);
        }
        Err(e) => Err(e).context("Network error"),
    }
}
