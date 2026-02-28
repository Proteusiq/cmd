# cmd

**Turn words into commands.**

```bash
$ cmd "find all rust files modified today"
execute:
    find . -name "*.rs" -mtime 0
```

You know what you want. You just forgot the syntax.

## Why cmd?

- **Natural language** → shell commands
- **Multiple providers** → Claude, OpenAI, Ollama
- **Single binary** → no Python, no dependencies
- **Preview mode** → see before you run

## Quick Start

```bash
# Install
git clone https://github.com/Proteusiq/cmd.git && cd cmd
cargo build --release
mkdir -p ~/.local/bin && mv target/release/cmd ~/.local/bin/

# Setup
cmd setup

# Use
cmd "find files larger than 100MB"
```
