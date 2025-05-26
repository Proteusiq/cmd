use clap::{Arg, Command};
use std::process::Command as StdCommand;

fn main() {
    // Define command-line argument parser
    let matches = Command::new("cmd")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("Execute commands with LLM")
        .arg(
            Arg::new("command")
                .help("The command to execute with LLM, wrapped in quotes")
                .required(true)
                .multiple_occurrences(true) // Allow multiple word command
                .value_delimiter(' '), // Treat space-separated values as part of the command
        )
        .arg(
            Arg::new("dry")
                .short('d')
                .long("dry")
                .help("Show the command to be executed without actually executing it"),
        )
        .get_matches();

    // Collect command arguments and join them into a single command string
    let command = matches
        .get_many::<String>("command")
        .unwrap()
        .map(|s| s.to_string()) // Convert &String to String
        .collect::<Vec<String>>() // Collect into Vec<String>
        .join(" "); // Join the vector into a single string

    if matches.get_flag("dry") {
        // Just show the command that would be executed
        println!("Command to execute:\n\t{}", command);
    } else {
        // Execute the command using the llm command
        execute_llm_command(&command);
    }
}

fn execute_llm_command(command: &str) {
    // Call the LLM command with the command provided
    let output = StdCommand::new("llm")
        .arg("-t")
        .arg("cmd")
        .arg(command)
        .output()
        .expect("Failed to execute LLM command");

    // Convert output bytes to string
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stderr.is_empty() {
        eprintln!("Error executing command:\n{}", stderr);
        return;
    }

    // Get the command to execute from stdout
    let cmd_to_execute = stdout.trim();
    
    // Show the command that will be executed
    println!("execute:\n\t{}", cmd_to_execute);
    
    // Execute the command
    let status = StdCommand::new("sh")
        .arg("-c")
        .arg(cmd_to_execute)
        .status()
        .expect("Failed to execute command");
    
    if !status.success() {
        eprintln!("Command failed with exit code: {}", status.code().unwrap_or(-1));
    }

    // Format and print the executed command
    for line in stdout.lines() {
        println!("execute:\n\t{}", line);
    }
}
