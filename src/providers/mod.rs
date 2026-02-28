mod anthropic;
mod openai;

pub use anthropic::call_anthropic;
pub use openai::call_openai;

use crate::core::{Config, Provider};
use anyhow::Result;

pub const SYSTEM_PROMPT: &str = "Reply with linux terminal commands only, no extra information. No ```bash ...``` markdown formatting. Just the raw command.";

pub fn call_llm(config: &Config, prompt: &str) -> Result<String> {
    match config.provider {
        Provider::Anthropic => call_anthropic(config, prompt),
        Provider::OpenAI | Provider::Ollama => call_openai(config, prompt),
    }
}
