# cmd

**Turn words into commands.**

```bash
$ cmd "find all rust files modified today"
╭──────────────────────────────────────────────────────╮
│ find . -name "*.rs" -mtime 0                         │
╰──────────────────────────────────────────────────────╯
  ↳ copied to clipboard
  ↳ use --enable-execution to run this command
```

You know what you want. You just forgot the syntax.

## Why cmd?

- **Natural language** → shell commands
- **Multiple providers** → Claude, OpenAI, Ollama
- **Single binary** → no Python, no dependencies
- **Safe by default** → dry-run mode, destructive command detection
- **Secure** → encrypted credential storage

## Quick Start

```bash
# Install
git clone https://github.com/Proteusiq/cmd.git && cd cmd
cargo build --release
mkdir -p ~/.local/bin && mv target/release/cmd ~/.local/bin/

# Setup
cmd setup

# Use (dry-run by default)
cmd "find files larger than 100MB"

# Execute with confirmation
cmd --enable-execution "find files larger than 100MB"
```
