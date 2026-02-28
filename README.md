# cmd

> "What's the find command for files over 100MB again?"

```bash
$ cmd find files larger than 100MB
execute:
    find . -size +100M -type f
```

You know what you want. You just forgot the syntax. We all do.

## The Problem

You're in the zone. Deep in code. Then you need to find something, compress something, grep something. And suddenly you're on Stack Overflow for 10 minutes trying to remember if it's `-mtime` or `-mmin` or `-newer`.

**cmd** lets you stay in the zone. Just say what you want.

## Install

```bash
git clone https://github.com/Proteusiq/cmd.git && cd cmd
cargo build --release
mkdir -p ~/.local/bin && mv target/release/cmd ~/.local/bin/
```

## Setup

```bash
cmd setup
```

Pick your LLM. Enter your key. Done.

Works with **Claude**, **OpenAI**, **Ollama** (free, local), or any OpenAI-compatible API.

## Examples

```bash
# Files & directories
cmd find all rust files modified today
cmd show disk usage sorted by size
cmd count lines of code in this project

# Git
cmd show commits from last week by author
cmd undo last commit but keep changes
cmd what files changed in the last 3 commits

# Processes & system
cmd kill whatever is running on port 3000
cmd show top 10 processes by memory
cmd how much ram is chrome using

# Docker
cmd stop all running containers
cmd remove all dangling images
cmd show logs from the api container

# Text & data
cmd find all TODOs in python files
cmd replace tabs with spaces in all .js files
cmd extract emails from this file
```

## Preview Mode

Not sure what it'll do? Preview first:

```bash
$ cmd --dry delete all node_modules folders recursively

execute:
    find . -type d -name "node_modules" -exec rm -rf {} +
copied to clipboard
```

The command is shown, copied to clipboard, but **not executed**. Paste when ready.

## How It Works

```
┌─────────────────┐     ┌─────────────┐     ┌──────────────┐
│  "compress      │ ──▶ │    LLM      │ ──▶ │ tar -czvf    │ ──▶ runs
│   this folder"  │     │  (Claude/   │     │ folder.tar.gz│
│                 │     │   GPT/etc)  │     │ folder/      │
└─────────────────┘     └─────────────┘     └──────────────┘
```

That's it. Your words become commands.

## Options

```
cmd [query]           Run a natural language command
cmd setup             Configure your LLM provider
cmd --dry [query]     Preview without executing
cmd -m MODEL          Use a specific model
cmd -e ENDPOINT       Use a custom API endpoint
```

## Providers

| Provider | Setup |
|----------|-------|
| Claude | `export ANTHROPIC_API_KEY=sk-ant-...` |
| OpenAI | `export OPENAI_API_KEY=sk-...` |
| Ollama | `export OLLAMA_HOST=http://localhost:11434` |

Or just run `cmd setup` and follow the prompts.

## Requirements

- macOS or Linux
- Rust (to build)
- An LLM provider (or Ollama for free local inference)

## License

Apache 2.0

---

*Stop googling. Start doing.*
