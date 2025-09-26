# 🦥 Vibe CLI Commands

> Sometimes we know what we want but just forgot the command

A natural language CLI tool that translates your intentions into terminal commands, powered by Simon Willison's [llm](https://github.com/simonw/llm).

## Why Vibe CLI?

Ever found yourself thinking "I want to find all files larger than 100MB" but can't remember the exact `find` syntax? Vibe CLI bridges that gap by letting you describe what you want in plain English.

## Prerequisites

- **uv** - Python package installer
- **ollama** (optional) - For local LLM hosting
- **Rust/Cargo** - For building the CLI tool
- **pbcopy** (macOS) - For clipboard functionality with `--dry` flag

## Quick Start

### 1. Install and Configure LLM

```bash
# Install the llm tool
uv tool install llm

# Option A: Use local models with Ollama
llm install llm-ollama
ollama pull qwen2.5-coder
llm models default qwen2.5-coder:latest

# Option B: Use Anthropic's Claude (requires API key)
llm install llm-anthropic
llm keys set anthropic
llm models default claude-3-5-sonnet-latest
```

### 2. Create Command Template

```bash
# Save a reusable template for command generation
 llm --system 'Reply with linux terminal commands only, no extra information. No ```bash ...```' --save cmd

```

### 3. Build and Install Vibe CLI

```bash
# Clone and build
git clone https://github.com/Proteusiq/cmd.git
cd cmd
cargo build --release

# Install to your PATH
mkdir -p ~/.local/bin
mv target/release/cmd ~/.local/bin/cmd

# Ensure ~/.local/bin is in your PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Usage Examples

```bash
# Execute commands directly
cmd "find files larger than 100MB in current directory"

# Preview commands without executing (copies to clipboard)
cmd --dry "stop all running containers"

# Docker commands made simple  
cmd "stop all running containers"

# Git operations in natural language
cmd "show commits from last week with author names"

# System monitoring
cmd "show top 10 processes using most CPU"

# Safe way to preview potentially dangerous operations
cmd --dry "delete all .log files older than 30 days"
```

## Command Options

- **Default behavior**: Executes the generated command immediately
- **`--dry` / `-d` flag**: Shows the command and copies it to clipboard without execution
  - Perfect for reviewing commands before running them
  - Useful for learning the actual syntax
  - Safe way to handle potentially destructive operations

## Platform Notes

**macOS**: Requires `pbcopy` (usually pre-installed) for clipboard functionality with the `--dry` flag
**Linux/Windows**: Currently not supported - you'll need to modify the clipboard command in the source code for your system

## Configuration Tips

- **For privacy**: Use local models with Ollama
- **For accuracy**: Use cloud models like Claude or GPT
- **For speed**: Keep frequently used models cached locally

## Contributing

Found a bug or have a suggestion? Check out the [repository](https://github.com/Proteusiq/cmd) to contribute!

## License

Apache
