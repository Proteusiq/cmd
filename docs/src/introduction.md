# cmd

<div style="text-align: center; margin: 2rem 0;">
<strong style="font-size: 1.5em;">Your words become commands.</strong>
</div>

> "What's the find command for files over 100MB again?"

```bash
$ cmd find files larger than 100MB
╭──────────────────────────────────────────────────────╮
│ find . -size +100M -type f                           │
╰──────────────────────────────────────────────────────╯
  ↳ copied to clipboard
  ↳ use --enable-execution to run this command
```

You know what you want. You just forgot the syntax. We all do.

---

## Why cmd?

| Feature | Description |
|---------|-------------|
| **Natural Language** | Describe what you want in plain English |
| **Multiple Providers** | Claude, OpenAI, Ollama (local), Azure, Groq |
| **Secure by Default** | Keychain storage, no plain text secrets |
| **Safe by Default** | Dry-run mode, destructive command detection |
| **Single Binary** | No Python, no Node, no runtime dependencies |

---

## Quick Start

### 1. Install

```bash
git clone https://github.com/Proteusiq/cmd.git && cd cmd
cargo build --release
mkdir -p ~/.local/bin && cp target/release/cmd ~/.local/bin/
```

### 2. Setup

```bash
cmd setup
```

Choose your provider, enter your API key (hidden input), and you're ready.

### 3. Use

```bash
# Dry-run (default) - shows command, copies to clipboard
cmd "find all rust files modified today"

# Execute with confirmation
cmd --enable-execution "compress this folder"

# Execute without confirmation (use with caution)
cmd --enable-execution --skip-confirmation "list running containers"
```

---

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

---

## Security First

`cmd` is designed with security as a priority:

- **API keys in keychain** — Not in `.zshrc` or plain text files
- **Hidden input** — API keys never visible on screen
- **Memory safety** — Secrets zeroed when no longer needed
- **Dry-run default** — Commands shown but not executed
- **Destructive detection** — Warns about dangerous commands
- **Critical blocking** — Blocks catastrophic commands like `rm -rf /`

---

## Next Steps

- [Installation](./installation.md) — Detailed installation guide
- [Configuration](./configuration.md) — Set up providers and credentials
- [Usage](./usage.md) — Learn all the features
- [Security](./security.md) — Understand the safety features
