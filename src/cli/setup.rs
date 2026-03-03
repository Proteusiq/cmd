use anyhow::{Result, bail};
use dialoguer::{Input, Password, Select};
use owo_colors::OwoColorize;
use secrecy::{ExposeSecret, SecretString};
use url::Url;

use crate::core::SecureStorage;

#[derive(Debug)]
struct ProviderConfig {
    name: &'static str,
    storage_key: &'static str,
    needs_endpoint: bool,
    key_url: Option<&'static str>,
    default_endpoint: Option<&'static str>,
    default_model: Option<&'static str>,
    key_prefix: Option<&'static str>,
    min_key_length: usize,
}

const PROVIDERS: &[ProviderConfig] = &[
    ProviderConfig {
        name: "Claude (Anthropic)",
        storage_key: "anthropic",
        needs_endpoint: false,
        key_url: Some("https://console.anthropic.com/settings/keys"),
        default_endpoint: None,
        default_model: None,
        key_prefix: Some("sk-ant-"),
        min_key_length: 40,
    },
    ProviderConfig {
        name: "OpenAI",
        storage_key: "openai",
        needs_endpoint: false,
        key_url: Some("https://platform.openai.com/api-keys"),
        default_endpoint: None,
        default_model: None,
        key_prefix: Some("sk-"),
        min_key_length: 40,
    },
    ProviderConfig {
        name: "Ollama (local)",
        storage_key: "ollama_host",
        needs_endpoint: false,
        key_url: None,
        default_endpoint: Some("http://localhost:11434"),
        default_model: Some("qwen2.5-coder"),
        key_prefix: None,
        min_key_length: 0,
    },
    ProviderConfig {
        name: "Azure OpenAI",
        storage_key: "openai",
        needs_endpoint: true,
        key_url: None,
        default_endpoint: None,
        default_model: None,
        key_prefix: None,
        min_key_length: 32,
    },
    ProviderConfig {
        name: "Groq",
        storage_key: "openai",
        needs_endpoint: true,
        key_url: Some("https://console.groq.com/keys"),
        default_endpoint: Some("https://api.groq.com/openai/v1/chat/completions"),
        default_model: Some("llama-3.1-70b-versatile"),
        key_prefix: Some("gsk_"),
        min_key_length: 40,
    },
    ProviderConfig {
        name: "Other (custom)",
        storage_key: "openai",
        needs_endpoint: true,
        key_url: None,
        default_endpoint: None,
        default_model: None,
        key_prefix: None,
        min_key_length: 8,
    },
];

/// Run the interactive setup wizard.
pub fn run_setup() -> Result<()> {
    if !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
        bail!("setup requires an interactive terminal");
    }

    println!("\n{}\n", "cmd setup".bold());

    let provider_names: Vec<&str> = PROVIDERS.iter().map(|p| p.name).collect();

    let selection = Select::new()
        .with_prompt("Select your LLM provider")
        .items(&provider_names)
        .default(0)
        .interact()?;

    let provider = &PROVIDERS[selection];
    let mut usage_args = String::new();

    println!();

    if let Some(url) = provider.key_url {
        println!("{} {}\n", "Get your API key at:".yellow(), url.underline());
    }

    let (storage_key, storage_value) = if provider.storage_key == "ollama_host" {
        let default = provider
            .default_endpoint
            .unwrap_or("http://localhost:11434");
        let host: String = Input::new()
            .with_prompt("Ollama host")
            .default(default.into())
            .validate_with(|input: &String| -> Result<(), &str> {
                validate_url(input).map_err(|_| "Invalid URL format")
            })
            .interact_text()?;

        println!("\n{}", "Recommended: ollama pull qwen2.5-coder".dimmed());

        ("ollama_host", host)
    } else {
        let key: SecretString = Password::new()
            .with_prompt("API key")
            .validate_with(|input: &String| -> Result<(), String> {
                validate_api_key(input, provider)
            })
            .interact()?
            .into();

        (provider.storage_key, key.expose_secret().to_string())
    };

    if provider.needs_endpoint {
        let endpoint: String = if provider.name == "Azure OpenAI" {
            let resource: String = Input::new()
                .with_prompt("Azure resource name")
                .validate_with(|input: &String| -> Result<(), &str> {
                    if input.is_empty() {
                        Err("Resource name is required")
                    } else if input.contains(' ') || input.contains('/') {
                        Err("Resource name should not contain spaces or slashes")
                    } else {
                        Ok(())
                    }
                })
                .interact_text()?;
            let deployment: String = Input::new()
                .with_prompt("Deployment name")
                .validate_with(|input: &String| -> Result<(), &str> {
                    if input.is_empty() {
                        Err("Deployment name is required")
                    } else {
                        Ok(())
                    }
                })
                .interact_text()?;
            let version: String = Input::new()
                .with_prompt("API version")
                .default("2024-02-15-preview".into())
                .interact_text()?;

            format!(
                "https://{}.openai.azure.com/openai/deployments/{}/chat/completions?api-version={}",
                resource, deployment, version
            )
        } else if let Some(default) = provider.default_endpoint {
            Input::new()
                .with_prompt("API endpoint")
                .default(default.into())
                .validate_with(|input: &String| -> Result<(), &str> {
                    validate_url(input).map_err(|_| "Invalid URL format")
                })
                .interact_text()?
        } else {
            Input::new()
                .with_prompt("API endpoint URL")
                .validate_with(|input: &String| -> Result<(), &str> {
                    validate_url(input).map_err(|_| "Invalid URL format")
                })
                .interact_text()?
        };

        validate_endpoint_security(&endpoint)?;

        usage_args.push_str(&format!(" -e \"{}\"", endpoint));

        SecureStorage::save("custom_endpoint", &endpoint)?;
        println!("\n{} {}", "Endpoint:".dimmed(), endpoint.dimmed());
    }

    if let Some(model) = provider.default_model {
        if provider.needs_endpoint {
            usage_args.push_str(&format!(" -m {}", model));
        }
        println!("{} {}", "Model:".dimmed(), model.dimmed());
    }

    SecureStorage::save(storage_key, &storage_value)?;

    println!("\n{}", "API key saved to system keychain".green());

    println!("\n{}", "Setup complete!".green().bold());
    println!(
        "Test with: {}\n",
        format!("cmd{} \"list files\"", usage_args).cyan()
    );

    Ok(())
}

fn validate_api_key(key: &str, provider: &ProviderConfig) -> Result<(), String> {
    if key.is_empty() {
        return Err("API key is required".to_string());
    }

    if key.len() < provider.min_key_length {
        return Err(format!(
            "API key seems too short (expected at least {} characters)",
            provider.min_key_length
        ));
    }

    if let Some(prefix) = provider.key_prefix
        && !key.starts_with(prefix)
    {
        return Err(format!(
            "API key should start with '{}' for {}",
            prefix, provider.name
        ));
    }

    if key.contains(' ') {
        return Err("API key should not contain spaces".to_string());
    }

    if key.starts_with("http://") || key.starts_with("https://") {
        return Err("This looks like a URL, not an API key".to_string());
    }

    Ok(())
}

fn validate_url(input: &str) -> Result<(), String> {
    Url::parse(input).map_err(|e| format!("Invalid URL: {}", e))?;
    Ok(())
}

fn validate_endpoint_security(endpoint: &str) -> Result<()> {
    let url = Url::parse(endpoint)?;

    if url.scheme() != "https" {
        let host = url.host_str().unwrap_or("");
        let is_local = host == "localhost" || host == "127.0.0.1" || host.starts_with("192.168.");
        if !is_local {
            println!(
                "\n{} {}",
                "Warning:".yellow().bold(),
                "Using non-HTTPS endpoint. Your API key will be sent in plain text!".yellow()
            );
        }
    }

    let host = url.host_str().unwrap_or("").to_lowercase();
    let suspicious_patterns = [
        "ngrok.io",
        "requestbin",
        "webhook.site",
        "pipedream",
        "hookbin",
    ];

    for pattern in suspicious_patterns {
        if host.contains(pattern) {
            bail!(
                "Suspicious endpoint detected: {}. This could be an attempt to steal your API key.",
                pattern
            );
        }
    }

    Ok(())
}
