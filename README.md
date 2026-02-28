<img width="2710" height="1628" alt="CleanShot 2026-02-28 at 13 27 32@2x" src="https://github.com/user-attachments/assets/b75ffb7a-2807-4be2-9a62-e75c110d6fe2" />


# cmd

**Your words become commands.**

> "What's the find command for files over 100MB again?"

```bash
$ cmd find files larger than 100MB
╭──────────────────────────────────────────────────────╮
│ find . -size +100M -type f                           │
╰──────────────────────────────────────────────────────╯
```

You know what you want. You just forgot the syntax. We all do.

## Install

```bash
git clone https://github.com/Proteusiq/cmd.git && cd cmd
cargo build --release
mkdir -p ~/.local/bin && mv target/release/cmd ~/.local/bin/
```

> [!TIP]
> Make sure `~/.local/bin` is in your `PATH`

## Setup

```bash
cmd setup
```

Pick your LLM. Enter your key. Done.

> [!NOTE]
> Works with **Claude**, **OpenAI**, **Ollama** (free, local), or any OpenAI-compatible API.

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
╭──────────────────────────────────────────────────────╮
│ find . -type d -name "node_modules" -exec rm -rf {} +│
╰──────────────────────────────────────────────────────╯
  ↳ copied to clipboard
```

> [!CAUTION]
> Always use `--dry` for destructive operations. Review before running.

## How It Works

```
┌─────────────────┐     ┌─────────────┐     ┌──────────────┐
│  "compress      │ ──▶ │    LLM      │ ──▶ │ tar -czvf    │ ──▶ runs
│   this folder"  │     │  (Claude/   │     │ folder.tar.gz│
│                 │     │   GPT/etc)  │     │ folder/      │
└─────────────────┘     └─────────────┘     └──────────────┘
```

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

> [!TIP]
> For **Azure OpenAI**, **Groq**, or other providers, use `cmd setup` and select "Other (custom)" to configure your endpoint.

## Requirements

- macOS or Linux
- Rust (to build)
- An LLM provider (or Ollama for free local inference)

## License

Apache 2.0

---

*Stop googling. Start doing.*
