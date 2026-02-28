const ANTHROPIC_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
const OPENAI_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
const OLLAMA_ENDPOINT: &str = "http://localhost:11434/v1/chat/completions";

const DEFAULT_ANTHROPIC_MODEL: &str = "claude-sonnet-4-20250514";
const DEFAULT_OPENAI_MODEL: &str = "gpt-4o";
const DEFAULT_OLLAMA_MODEL: &str = "qwen2.5-coder";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Provider {
    Anthropic,
    OpenAI,
    Ollama,
}

#[derive(Debug)]
pub struct Config {
    pub provider: Provider,
    pub api_key: Option<String>,
    pub endpoint: String,
    pub model: String,
}

impl Config {
    pub fn detect(
        model_override: Option<&str>,
        endpoint_override: Option<&str>,
        env_vars: &dyn Fn(&str) -> Option<String>,
    ) -> Option<Self> {
        if let Some(key) = env_vars("ANTHROPIC_API_KEY").filter(|k| !k.is_empty()) {
            return Some(Config {
                provider: Provider::Anthropic,
                api_key: Some(key),
                endpoint: endpoint_override
                    .map(String::from)
                    .unwrap_or_else(|| ANTHROPIC_ENDPOINT.into()),
                model: model_override
                    .map(String::from)
                    .unwrap_or_else(|| DEFAULT_ANTHROPIC_MODEL.into()),
            });
        }

        if let Some(key) = env_vars("OPENAI_API_KEY").filter(|k| !k.is_empty()) {
            return Some(Config {
                provider: Provider::OpenAI,
                api_key: Some(key),
                endpoint: endpoint_override
                    .map(String::from)
                    .unwrap_or_else(|| OPENAI_ENDPOINT.into()),
                model: model_override
                    .map(String::from)
                    .unwrap_or_else(|| DEFAULT_OPENAI_MODEL.into()),
            });
        }

        if env_vars("OLLAMA_HOST").is_some() || endpoint_override == Some(OLLAMA_ENDPOINT) {
            let host = env_vars("OLLAMA_HOST").unwrap_or_else(|| OLLAMA_ENDPOINT.into());
            return Some(Config {
                provider: Provider::Ollama,
                api_key: None,
                endpoint: endpoint_override.map(String::from).unwrap_or(host),
                model: model_override
                    .map(String::from)
                    .unwrap_or_else(|| DEFAULT_OLLAMA_MODEL.into()),
            });
        }

        None
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
        assert_eq!(config.api_key, Some("sk-ant-test".into()));
        assert_eq!(config.model, DEFAULT_ANTHROPIC_MODEL);
    }

    #[test]
    fn detects_openai_from_env() {
        let env = make_env(HashMap::from([("OPENAI_API_KEY", "sk-test")]));
        let config = Config::detect(None, None, &env).unwrap();

        assert_eq!(config.provider, Provider::OpenAI);
        assert_eq!(config.api_key, Some("sk-test".into()));
        assert_eq!(config.model, DEFAULT_OPENAI_MODEL);
    }

    #[test]
    fn detects_ollama_from_env() {
        let env = make_env(HashMap::from([("OLLAMA_HOST", "http://localhost:11434")]));
        let config = Config::detect(None, None, &env).unwrap();

        assert_eq!(config.provider, Provider::Ollama);
        assert_eq!(config.api_key, None);
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
        let config = Config::detect(Some("claude-3-haiku"), None, &env).unwrap();

        assert_eq!(config.model, "claude-3-haiku");
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
}
