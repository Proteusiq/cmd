use anyhow::{bail, Result};
use dialoguer::{Input, Select};
use owo_colors::OwoColorize;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
struct ProviderConfig {
    name: &'static str,
    env_var: &'static str,
    needs_endpoint: bool,
    key_url: Option<&'static str>,
    default_endpoint: Option<&'static str>,
    default_model: Option<&'static str>,
}

const PROVIDERS: &[ProviderConfig] = &[
    ProviderConfig {
        name: "Claude (Anthropic)",
        env_var: "ANTHROPIC_API_KEY",
        needs_endpoint: false,
        key_url: Some("https://console.anthropic.com/settings/keys"),
        default_endpoint: None,
        default_model: None,
    },
    ProviderConfig {
        name: "OpenAI",
        env_var: "OPENAI_API_KEY",
        needs_endpoint: false,
        key_url: Some("https://platform.openai.com/api-keys"),
        default_endpoint: None,
        default_model: None,
    },
    ProviderConfig {
        name: "Ollama (local)",
        env_var: "OLLAMA_HOST",
        needs_endpoint: false,
        key_url: None,
        default_endpoint: Some("http://localhost:11434"),
        default_model: Some("qwen2.5-coder"),
    },
    ProviderConfig {
        name: "Azure OpenAI",
        env_var: "OPENAI_API_KEY",
        needs_endpoint: true,
        key_url: None,
        default_endpoint: None,
        default_model: None,
    },
    ProviderConfig {
        name: "Groq",
        env_var: "OPENAI_API_KEY",
        needs_endpoint: true,
        key_url: Some("https://console.groq.com/keys"),
        default_endpoint: Some("https://api.groq.com/openai/v1/chat/completions"),
        default_model: Some("llama-3.1-70b-versatile"),
    },
    ProviderConfig {
        name: "Other (custom)",
        env_var: "OPENAI_API_KEY",
        needs_endpoint: true,
        key_url: None,
        default_endpoint: None,
        default_model: None,
    },
];

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
    let mut exports: Vec<String> = Vec::new();
    let mut usage_args = String::new();

    println!();

    if let Some(url) = provider.key_url {
        println!("{} {}\n", "Get your API key at:".yellow(), url.underline());
    }

    // Handle Ollama specially - it's a host, not a key
    if provider.env_var == "OLLAMA_HOST" {
        let default = provider
            .default_endpoint
            .unwrap_or("http://localhost:11434");
        let host: String = Input::new()
            .with_prompt("Ollama host")
            .default(default.into())
            .interact_text()?;

        exports.push(format!("export OLLAMA_HOST=\"{}\"", host));

        println!("\n{}", "Recommended: ollama pull qwen2.5-coder".dimmed());
    } else {
        // Get API key
        let key: String = Input::new().with_prompt("API key").interact_text()?;

        if key.is_empty() {
            bail!("API key is required");
        }

        exports.push(format!("export {}=\"{}\"", provider.env_var, key));
    }

    // Get endpoint if needed
    if provider.needs_endpoint {
        let endpoint: String = if provider.name == "Azure OpenAI" {
            let resource: String = Input::new()
                .with_prompt("Azure resource name")
                .interact_text()?;
            let deployment: String = Input::new()
                .with_prompt("Deployment name")
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
                .interact_text()?
        } else {
            Input::new()
                .with_prompt("API endpoint URL")
                .interact_text()?
        };

        usage_args.push_str(&format!(" -e \"{}\"", endpoint));

        // For custom endpoints, store in a comment for reference
        println!("\n{} {}", "Endpoint:".dimmed(), endpoint.dimmed());
    }

    // Get model if provider has a default
    if let Some(model) = provider.default_model {
        if provider.needs_endpoint {
            usage_args.push_str(&format!(" -m {}", model));
        }
        println!("{} {}", "Model:".dimmed(), model.dimmed());
    }

    // Detect shell config
    let shell_config = detect_shell_config();
    let export_block = exports.join("\n");

    println!("\n{}\n", "Add to your shell config:".yellow());
    println!("{}\n", export_block.green());

    let add_to_config = Select::new()
        .with_prompt(format!("Add to {}?", shell_config.display()))
        .items(&["Yes", "No"])
        .default(0)
        .interact()?;

    if add_to_config == 0 {
        let mut file = OpenOptions::new().append(true).open(&shell_config)?;

        writeln!(file)?;
        writeln!(file, "# Vibe CLI ({})", provider.name)?;
        for export in &exports {
            writeln!(file, "{}", export)?;
        }

        println!("\n{} {}", "Added to".green(), shell_config.display());
        println!(
            "Run: {}\n",
            format!("source {}", shell_config.display()).cyan()
        );
    }

    println!("{}", "Setup complete!".green().bold());
    println!(
        "Test with: {}\n",
        format!("cmd{} \"list files\"", usage_args).cyan()
    );

    Ok(())
}

fn detect_shell_config() -> std::path::PathBuf {
    let home = dirs::home_dir().unwrap_or_default();

    // Check SHELL env var
    if let Ok(shell) = std::env::var("SHELL") {
        if shell.contains("zsh") {
            return home.join(".zshrc");
        }
        if shell.contains("bash") {
            let bashrc = home.join(".bashrc");
            if bashrc.exists() {
                return bashrc;
            }
            return home.join(".bash_profile");
        }
    }

    // Default to .profile
    home.join(".profile")
}
