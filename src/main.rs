use clap::{Arg, Command};
use std::process::Command as StdCommand;

fn main() {
    let matches = Command::new("cmd")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("Execute commands with LLM")
        .arg(
            Arg::new("command")
                .help("The command to execute with LLM, wrapped in quotes")
                .required(true)
                .num_args(1..)
                .value_delimiter(' '),
        )
        .arg(
            Arg::new("dry")
                .short('d')
                .long("dry")
                .action(clap::ArgAction::SetTrue)
                .help("Show the command to be executed without actually executing it"),
        )
        .get_matches();

    let command = matches
        .get_many::<String>("command")
        .unwrap()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    if matches.get_flag("dry") {
        println!("Command to execute:\n\t{}", command);
    } else {
        execute_llm_command(&command);
    }
}

fn execute_llm_command(command: &str) {
    let output = StdCommand::new("llm")
        .arg("-t")
        .arg("cmd")
        .arg(command)
        .output()
        .expect("Failed to execute LLM command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stderr.is_empty() {
        eprintln!("Error executing command:\n{}", stderr);
        return;
    }

    let cmd_to_execute = stdout.trim();
    
    println!("execute:\n\t{}", cmd_to_execute);
    
    let status = StdCommand::new("sh")
        .arg("-c")
        .arg(cmd_to_execute)
        .status()
        .expect("Failed to execute command");
    
    if !status.success() {
        eprintln!("Command failed with exit code: {}", status.code().unwrap_or(-1));
    }
}
