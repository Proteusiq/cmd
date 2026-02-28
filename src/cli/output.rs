use owo_colors::OwoColorize;
use spinoff::{Color, spinners};
use std::io::Write;
use std::process::Command;

pub struct Spinner(spinoff::Spinner);

impl Spinner {
    pub fn start() -> Self {
        Self(spinoff::Spinner::new(spinners::Dots, "", Color::Cyan))
    }

    pub fn stop(mut self) {
        self.0.clear();
    }
}

pub fn print_setup_help() {
    eprintln!("\n{}", "No LLM provider configured".red().bold());
    eprintln!("\n{}", "Setup one of the following:".yellow().bold());

    eprintln!("\n  {}", "Claude (Anthropic)".cyan().bold());
    eprintln!("  export ANTHROPIC_API_KEY={}", "sk-ant-...".dimmed());
    eprintln!(
        "  {}",
        "https://console.anthropic.com/settings/keys"
            .dimmed()
            .underline()
    );

    eprintln!("\n  {}", "OpenAI".cyan().bold());
    eprintln!("  export OPENAI_API_KEY={}", "sk-...".dimmed());
    eprintln!(
        "  {}",
        "https://platform.openai.com/api-keys".dimmed().underline()
    );

    eprintln!("\n  {}", "Ollama (local)".cyan().bold());
    eprintln!("  export OLLAMA_HOST={}", "http://localhost:11434".dimmed());
    eprintln!("  {}", "https://ollama.ai".dimmed().underline());

    eprintln!();
}

pub fn copy_to_clipboard(text: &str) {
    #[cfg(target_os = "macos")]
    let cmd = "pbcopy";
    #[cfg(target_os = "linux")]
    let cmd = "xclip";
    #[cfg(target_os = "windows")]
    let cmd = "clip";

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        eprintln!("Clipboard not supported on this platform");
        return;
    }

    let Ok(mut child) = Command::new(cmd)
        .stdin(std::process::Stdio::piped())
        .spawn()
    else {
        eprintln!("Failed to copy to clipboard (is {} available?)", cmd);
        return;
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(text.as_bytes());
    }

    if child.wait().is_ok_and(|s| s.success()) {
        println!("{}", "cmd copied to clipboard".dimmed());
    } else {
        eprintln!("cmd not copied to clipboard");
    }
}
