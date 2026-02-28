use arboard::Clipboard;
use owo_colors::OwoColorize;
use spinoff::{spinners, Color};

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
    eprintln!(
        "\n{} {}\n",
        "Run:".yellow().bold(),
        "cmd setup".cyan().bold()
    );

    eprintln!("{}", "Or set one of the following:".dimmed());

    eprintln!("\n  {}", "Claude (Anthropic)".cyan());
    eprintln!("  export ANTHROPIC_API_KEY={}", "sk-ant-...".dimmed());

    eprintln!("\n  {}", "OpenAI".cyan());
    eprintln!("  export OPENAI_API_KEY={}", "sk-...".dimmed());

    eprintln!("\n  {}", "Ollama (local)".cyan());
    eprintln!("  export OLLAMA_HOST={}", "http://localhost:11434".dimmed());

    eprintln!();
}

pub fn copy_to_clipboard(text: &str) -> bool {
    Clipboard::new()
        .and_then(|mut cb| cb.set_text(text.to_string()))
        .is_ok()
}
