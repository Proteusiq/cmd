use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use human_panic::setup_panic;
use owo_colors::OwoColorize;
use std::process::Command;

use cmd::cli::{Spinner, copy_to_clipboard, print_setup_help, run_setup};
use cmd::core::Config;
use cmd::providers::call_llm;

#[derive(Parser)]
#[command(
    version,
    about = "Natural language CLI - translate intentions into terminal commands"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Describe what you want to do in natural language
    #[arg(required = false, num_args = 1..)]
    query: Vec<String>,

    /// Show the command without executing (copies to clipboard)
    #[arg(short, long)]
    dry: bool,

    /// Model to use (auto-detected from provider, or override)
    #[arg(short, long)]
    model: Option<String>,

    /// API endpoint URL (auto-detected from provider)
    #[arg(short, long)]
    endpoint: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure LLM provider interactively
    Setup,
}

fn main() {
    setup_panic!();

    if let Err(e) = run() {
        eprintln!("\n{} {}\n", "error:".red().bold(), e);
        std::process::exit(exitcode::SOFTWARE);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    if let Some(Commands::Setup) = cli.command {
        return run_setup();
    }

    if cli.query.is_empty() {
        print_setup_help();
        std::process::exit(exitcode::CONFIG);
    }

    let env = |key: &str| std::env::var(key).ok();
    let config = match Config::detect(cli.model.as_deref(), cli.endpoint.as_deref(), &env) {
        Some(c) => c,
        None => {
            print_setup_help();
            std::process::exit(exitcode::CONFIG);
        }
    };

    let prompt = cli.query.join(" ");

    let spinner = Spinner::start();
    let result = call_llm(&config, &prompt);
    spinner.stop();

    let cmd_to_execute = result?;

    if cli.dry {
        let copied = copy_to_clipboard(&cmd_to_execute);
        print_command_box(&cmd_to_execute, copied);
    } else {
        print_command_box(&cmd_to_execute, false);
        println!();

        let status = Command::new("sh").arg("-c").arg(&cmd_to_execute).status()?;

        if !status.success() {
            println!();
            bail!("command exited with code {}", status.code().unwrap_or(-1));
        }
    }

    Ok(())
}

fn print_command_box(cmd: &str, copied: bool) {
    let width = terminal_width().min(80);
    let border = "─".repeat(width - 2);

    println!();
    println!("{}", format!("╭{}╮", border).dimmed());

    // Wrap long commands
    for line in wrap_text(cmd, width - 4) {
        println!(
            "{} {:<w$} {}",
            "│".dimmed(),
            line.cyan().bold(),
            "│".dimmed(),
            w = width - 4
        );
    }

    println!("{}", format!("╰{}╯", border).dimmed());

    if copied {
        println!("  {} {}", "↳".dimmed(), "copied to clipboard".dimmed());
    }
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if current.is_empty() {
            current = word.to_string();
        } else if current.len() + 1 + word.len() <= max_width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

fn terminal_width() -> usize {
    terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80)
}
