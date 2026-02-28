use anyhow::{Result, bail};
use clap::Parser;
use human_panic::setup_panic;
use owo_colors::OwoColorize;
use std::process::Command;

use cmd::cli::{Spinner, copy_to_clipboard, print_setup_help};
use cmd::core::Config;
use cmd::providers::call_llm;

#[derive(Parser)]
#[command(
    version,
    about = "Natural language CLI - translate intentions into terminal commands"
)]
struct Cli {
    /// Describe what you want to do in natural language
    #[arg(required = true, num_args = 1..)]
    command: Vec<String>,

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

fn main() {
    setup_panic!();

    if let Err(e) = run() {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(exitcode::SOFTWARE);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let env = |key: &str| std::env::var(key).ok();
    let config = match Config::detect(cli.model.as_deref(), cli.endpoint.as_deref(), &env) {
        Some(c) => c,
        None => {
            print_setup_help();
            std::process::exit(exitcode::CONFIG);
        }
    };

    let prompt = cli.command.join(" ");

    let spinner = Spinner::start();
    let result = call_llm(&config, &prompt);
    spinner.stop();

    let cmd_to_execute = result?;

    println!("{}\n\t{}", "execute:".green().bold(), cmd_to_execute);

    if cli.dry {
        copy_to_clipboard(&cmd_to_execute);
    } else {
        let status = Command::new("sh").arg("-c").arg(&cmd_to_execute).status()?;

        if !status.success() {
            bail!(
                "Command failed with exit code: {}",
                status.code().unwrap_or(-1)
            );
        }
    }

    Ok(())
}
