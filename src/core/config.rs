use secrecy::{ExposeSecret, SecretString};
use std::fmt;

use super::credentials::SecureStorage;

const ANTHROPIC_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
const OPENAI_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
const OLLAMA_ENDPOINT: &str = "http://localhost:11434/v1/chat/completions";

const DEFAULT_ANTHROPIC_MODEL: &str = "claude-sonnet-4-6";
const DEFAULT_OPENAI_MODEL: &str = "gpt-5.2";
const DEFAULT_OLLAMA_MODEL: &str = "qwen2.5-coder";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Provider {
    Anthropic,
    OpenAI,
    Ollama,
}

pub struct Config {
    pub provider: Provider,
    pub api_key: Option<SecretString>,
    pub endpoint: String,
    pub model: String,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("provider", &self.provider)
            .field("api_key", &self.api_key.as_ref().map(|_| "[REDACTED]"))
            .field("endpoint", &self.endpoint)
            .field("model", &self.model)
            .finish()
    }
}

impl Config {
    /// Detect provider configuration
    ///
    /// Priority order:
    /// 1. Environment variables (allows override)
    /// 2. System keychain / secure storage
    pub fn detect(
        model_override: Option<&str>,
        endpoint_override: Option<&str>,
        env_vars: &dyn Fn(&str) -> Option<String>,
    ) -> Option<Self> {
        // Priority 1: Environment variables
        if let Some(key) = env_vars("ANTHROPIC_API_KEY").filter(|k| !k.is_empty()) {
            return Some(Self::anthropic(key, model_override, endpoint_override));
        }

        if let Some(key) = env_vars("OPENAI_API_KEY").filter(|k| !k.is_empty()) {
            return Some(Self::openai(key, model_override, endpoint_override));
        }

        if env_vars("OLLAMA_HOST").is_some() || endpoint_override == Some(OLLAMA_ENDPOINT) {
            let host = env_vars("OLLAMA_HOST").unwrap_or_else(|| OLLAMA_ENDPOINT.into());
            return Some(Self::ollama(host, model_override, endpoint_override));
        }

        // Priority 2: Secure storage (keychain)
        if let Some(key) = SecureStorage::load("anthropic") {
            return Some(Self::anthropic(key, model_override, endpoint_override));
        }

        if let Some(key) = SecureStorage::load("openai") {
            return Some(Self::openai(key, model_override, endpoint_override));
        }

        if let Some(host) = SecureStorage::load("ollama_host") {
            return Some(Self::ollama(host, model_override, endpoint_override));
        }

        None
    }

    fn anthropic(
        api_key: String,
        model_override: Option<&str>,
        endpoint_override: Option<&str>,
    ) -> Self {
        Config {
            provider: Provider::Anthropic,
            api_key: Some(SecretString::from(api_key)),
            endpoint: endpoint_override
                .map(String::from)
                .unwrap_or_else(|| ANTHROPIC_ENDPOINT.into()),
            model: model_override
                .map(String::from)
                .unwrap_or_else(|| DEFAULT_ANTHROPIC_MODEL.into()),
        }
    }

    fn openai(
        api_key: String,
        model_override: Option<&str>,
        endpoint_override: Option<&str>,
    ) -> Self {
        Config {
            provider: Provider::OpenAI,
            api_key: Some(SecretString::from(api_key)),
            endpoint: endpoint_override
                .map(String::from)
                .unwrap_or_else(|| OPENAI_ENDPOINT.into()),
            model: model_override
                .map(String::from)
                .unwrap_or_else(|| DEFAULT_OPENAI_MODEL.into()),
        }
    }

    fn ollama(host: String, model_override: Option<&str>, endpoint_override: Option<&str>) -> Self {
        Config {
            provider: Provider::Ollama,
            api_key: None,
            endpoint: endpoint_override.map(String::from).unwrap_or(host),
            model: model_override
                .map(String::from)
                .unwrap_or_else(|| DEFAULT_OLLAMA_MODEL.into()),
        }
    }

    pub fn api_key_exposed(&self) -> Option<&str> {
        self.api_key.as_ref().map(|s| s.expose_secret())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_env(vars: HashMap<&str, &str>) -> impl Fn(&str) -> Option<String> {
        move |key| vars.get(key).map(|v| v.to_string())
    }

    #[test]
    fn detects_anthropic_from_env() {
        let env = make_env(HashMap::from([("ANTHROPIC_API_KEY", "sk-ant-test")]));
        let config = Config::detect(None, None, &env).unwrap();

        assert_eq!(config.provider, Provider::Anthropic);
        assert_eq!(config.api_key_exposed(), Some("sk-ant-test"));
        assert_eq!(config.model, DEFAULT_ANTHROPIC_MODEL);
    }

    #[test]
    fn detects_openai_from_env() {
        let env = make_env(HashMap::from([("OPENAI_API_KEY", "sk-test")]));
        let config = Config::detect(None, None, &env).unwrap();

        assert_eq!(config.provider, Provider::OpenAI);
        assert_eq!(config.api_key_exposed(), Some("sk-test"));
        assert_eq!(config.model, DEFAULT_OPENAI_MODEL);
    }

    #[test]
    fn detects_ollama_from_env() {
        let env = make_env(HashMap::from([("OLLAMA_HOST", "http://localhost:11434")]));
        let config = Config::detect(None, None, &env).unwrap();

        assert_eq!(config.provider, Provider::Ollama);
        assert_eq!(config.api_key_exposed(), None);
    }

    #[test]
    fn returns_none_when_no_provider_configured() {
        let env = make_env(HashMap::new());
        let config = Config::detect(None, None, &env);

        assert!(config.is_none());
    }

    #[test]
    fn model_override_takes_precedence() {
        let env = make_env(HashMap::from([("ANTHROPIC_API_KEY", "sk-ant-test")]));
        let config = Config::detect(Some("claude-sonnet-4.5"), None, &env).unwrap();

        assert_eq!(config.model, "claude-sonnet-4.5");
    }

    #[test]
    fn anthropic_takes_priority_over_openai() {
        let env = make_env(HashMap::from([
            ("ANTHROPIC_API_KEY", "sk-ant-test"),
            ("OPENAI_API_KEY", "sk-test"),
        ]));
        let config = Config::detect(None, None, &env).unwrap();

        assert_eq!(config.provider, Provider::Anthropic);
    }

    #[test]
    fn debug_redacts_api_key() {
        let env = make_env(HashMap::from([("ANTHROPIC_API_KEY", "sk-ant-secret")]));
        let config = Config::detect(None, None, &env).unwrap();
        let debug_output = format!("{:?}", config);

        assert!(debug_output.contains("[REDACTED]"));
        assert!(!debug_output.contains("sk-ant-secret"));
    }
}
