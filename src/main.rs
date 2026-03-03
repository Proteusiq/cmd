use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use dialoguer::Confirm;
use human_panic::setup_panic;
use owo_colors::OwoColorize;
use std::process::Command;

use cmd::cli::{Spinner, copy_to_clipboard, print_setup_help, run_setup};
use cmd::core::{Config, SafetyCheck, SecureStorage, Settings, Severity};
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

    /// Enable command execution (required to run commands)
    #[arg(long)]
    enable_execution: bool,

    /// Skip confirmation prompt (requires --enable-execution)
    #[arg(long, requires = "enable_execution")]
    skip_confirmation: bool,

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
    /// Configure execution settings and manage API keys
    Config {
        /// Enable command execution by default
        #[arg(long)]
        enable_execution: bool,
        /// Disable command execution by default
        #[arg(long, conflicts_with = "enable_execution")]
        disable_execution: bool,
        /// Skip confirmation prompts by default
        #[arg(long)]
        skip_confirmation: bool,
        /// Require confirmation prompts (default)
        #[arg(long, conflicts_with = "skip_confirmation")]
        require_confirmation: bool,
        /// Show current settings
        #[arg(long)]
        show: bool,
        /// Show stored API keys (masked)
        #[arg(long)]
        show_keys: bool,
        /// Delete a stored API key (anthropic, openai, ollama_host)
        #[arg(long, value_name = "PROVIDER")]
        delete_key: Option<String>,
    },
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

    match &cli.command {
        Some(Commands::Setup) => return run_setup(),
        Some(Commands::Config {
            enable_execution,
            disable_execution,
            skip_confirmation,
            require_confirmation,
            show,
            show_keys,
            delete_key,
        }) => {
            return run_config(
                *enable_execution,
                *disable_execution,
                *skip_confirmation,
                *require_confirmation,
                *show,
                *show_keys,
                delete_key.as_deref(),
            );
        }
        None => {}
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

    // Load saved settings and merge with CLI flags
    let settings = Settings::load();
    let enable_execution = cli.enable_execution || settings.enable_execution;
    let skip_confirmation = cli.skip_confirmation || settings.skip_confirmation;

    let prompt = cli.query.join(" ");

    let spinner = Spinner::start();
    let result = call_llm(&config, &prompt);
    spinner.stop();

    let cmd_to_execute = result?;

    // Default: dry-run mode (show command, copy to clipboard)
    if !enable_execution {
        let copied = copy_to_clipboard(&cmd_to_execute);
        print_command_box(&cmd_to_execute, copied);
        println!(
            "  {} {}",
            "↳".dimmed(),
            "use --enable-execution to run this command".dimmed()
        );
        return Ok(());
    }

    // Execution mode: show command and confirm before running
    print_command_box(&cmd_to_execute, false);

    // Safety check for destructive commands
    let safety = SafetyCheck::analyze(&cmd_to_execute);

    if safety.is_destructive {
        println!();
        print_safety_warning(&safety);
    }

    // Block critical commands entirely
    if safety.should_block() {
        println!(
            "\n{} {}",
            "BLOCKED:".red().bold(),
            "This command is too dangerous to execute.".red()
        );
        println!(
            "{}",
            "If you really need to run this, copy and execute it manually.".dimmed()
        );
        let _ = copy_to_clipboard(&cmd_to_execute);
        return Ok(());
    }

    println!();

    // Force confirmation for destructive commands, regardless of settings
    let needs_confirmation = safety.requires_confirmation() || !skip_confirmation;

    let confirmed = if needs_confirmation {
        Confirm::new()
            .with_prompt(if safety.is_destructive {
                "This is a destructive command. Execute anyway?"
            } else {
                "Execute this command?"
            })
            .default(false)
            .interact()?
    } else {
        true
    };

    if !confirmed {
        println!("{}", "Aborted.".yellow());
        return Ok(());
    }

    let status = Command::new("sh").arg("-c").arg(&cmd_to_execute).status()?;

    if !status.success() {
        println!();
        bail!("command exited with code {}", status.code().unwrap_or(-1));
    }

    Ok(())
}

fn print_safety_warning(safety: &SafetyCheck) {
    let (icon, label) = match safety.severity {
        Severity::Critical => ("🛑", "CRITICAL"),
        Severity::Dangerous => ("⚠️ ", "DANGEROUS"),
        Severity::Warning => ("⚡", "WARNING"),
        Severity::Safe => return,
    };

    let color_label = match safety.severity {
        Severity::Critical => label.red().bold().to_string(),
        Severity::Dangerous => label.red().to_string(),
        Severity::Warning => label.yellow().to_string(),
        Severity::Safe => return,
    };

    println!("{} {}", icon, color_label);
    for reason in &safety.reasons {
        println!("  {} {}", "•".dimmed(), reason.dimmed());
    }
}

fn run_config(
    enable_execution: bool,
    disable_execution: bool,
    skip_confirmation: bool,
    require_confirmation: bool,
    show: bool,
    show_keys: bool,
    delete_key: Option<&str>,
) -> Result<()> {
    // Handle key deletion
    if let Some(provider) = delete_key {
        match SecureStorage::delete(provider) {
            Ok(()) => {
                println!("{} Deleted {} API key", "✓".green(), provider);
                return Ok(());
            }
            Err(e) => {
                bail!("Failed to delete {}: {}", provider, e);
            }
        }
    }

    // Handle showing keys
    if show_keys {
        println!("\n{}", "Stored API keys:".bold());
        for (id, name, has_key) in SecureStorage::list_providers() {
            if has_key {
                let masked = SecureStorage::get_masked(id).unwrap_or_default();
                println!("  {} {}", format!("{}:", name).cyan(), masked.green());
            } else {
                println!("  {} {}", format!("{}:", name).cyan(), "(not set)".dimmed());
            }
        }
        println!();
        return Ok(());
    }

    let mut settings = Settings::load();
    let mut changed = false;

    if enable_execution {
        settings.set_enable_execution(true);
        changed = true;
    }
    if disable_execution {
        settings.set_enable_execution(false);
        changed = true;
    }
    if skip_confirmation {
        settings.set_skip_confirmation(true);
        changed = true;
    }
    if require_confirmation {
        settings.set_skip_confirmation(false);
        changed = true;
    }

    if changed {
        settings.save()?;
        println!("{}", "Settings saved.".green());
    }

    if show || !changed {
        println!("\n{}", "Current settings:".bold());
        print!("  {} ", "enable_execution:".cyan());
        if settings.enable_execution {
            println!("{}", "true".green());
        } else {
            println!("{}", "false".yellow());
        }
        print!("  {} ", "skip_confirmation:".cyan());
        if settings.skip_confirmation {
            println!("{}", "true".yellow());
        } else {
            println!("{}", "false".green());
        }

        if let Some(path) = Settings::config_path() {
            println!("\n  {} {}", "config:".dimmed(), path.display());
        }
        println!();
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
