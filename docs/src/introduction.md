# cmd

<pre class="ascii-hero">
 ██████╗███╗   ███╗██████╗
██╔════╝████╗ ████║██╔══██╗
██║     ██╔████╔██║██║  ██║
██║     ██║╚██╔╝██║██║  ██║
╚██████╗██║ ╚═╝ ██║██████╔╝
 ╚═════╝╚═╝     ╚═╝╚═════╝
</pre>

<div class="tagline">
<strong>Your words become commands.</strong>
</div>

```
┌────────────────────────────────────────────────────────────┐
│  $ cmd find files larger than 100MB                        │
│                                                            │
│  ╭──────────────────────────────────────────────────────╮  │
│  │ find . -size +100M -type f                           │  │
│  ╰──────────────────────────────────────────────────────╯  │
│    ↳ copied to clipboard                                   │
│    ↳ use --enable-execution to run this command            │
└────────────────────────────────────────────────────────────┘
```

> You know what you want. You just forgot the syntax. We all do.

---

## Why cmd?

```
┌─────────────────────┬──────────────────────────────────────┐
│ ▸ Natural Language  │ Describe what you want in English    │
├─────────────────────┼──────────────────────────────────────┤
│ ▸ Multi-Provider    │ Claude, OpenAI, Ollama, Azure, Groq  │
├─────────────────────┼──────────────────────────────────────┤
│ ▸ Secure by Default │ Keychain storage, no plain text      │
├─────────────────────┼──────────────────────────────────────┤
│ ▸ Safe by Default   │ Dry-run mode, destructive guard      │
├─────────────────────┼──────────────────────────────────────┤
│ ▸ Single Binary     │ No Python, no Node, just Rust        │
└─────────────────────┴──────────────────────────────────────┘
```

---

## Quick Start

### Installation

```bash
# Clone and build
git clone https://github.com/Proteusiq/cmd.git && cd cmd
cargo build --release

# Install to PATH
mkdir -p ~/.local/bin && cp target/release/cmd ~/.local/bin/
```

### Setup

```bash
$ cmd setup

? Select your LLM provider:
  ▸ Anthropic (Claude)
    OpenAI
    Ollama (local)
    Azure OpenAI

? Enter your API key: ********

✓ API key stored in system keychain
```

### Use

```bash
# Dry-run (default) - shows command, copies to clipboard
$ cmd find all rust files modified today

# Execute with confirmation
$ cmd --enable-execution compress this folder

# Execute without confirmation (careful!)
$ cmd --enable-execution --skip-confirmation list running containers
```

---

## Examples

```bash
# ───────────────────────────────────────────────────
#  Files & directories
# ───────────────────────────────────────────────────
cmd find all rust files modified today
cmd show disk usage sorted by size
cmd count lines of code in this project

# ───────────────────────────────────────────────────
#  Git
# ───────────────────────────────────────────────────
cmd show commits from last week by author
cmd undo last commit but keep changes
cmd what files changed in the last 3 commits

# ───────────────────────────────────────────────────
#  Processes & system
# ───────────────────────────────────────────────────
cmd kill whatever is running on port 3000
cmd show top 10 processes by memory
cmd how much ram is chrome using

# ───────────────────────────────────────────────────
#  Docker
# ───────────────────────────────────────────────────
cmd stop all running containers
cmd remove all dangling images
cmd show logs from the api container
```

---

## Security First

```
╔════════════════════════════════════════════════════════════╗
║  SECURITY                                                  ║
╠════════════════════════════════════════════════════════════╣
║                                                            ║
║  [✓] API keys in keychain   Not in .zshrc or plain text    ║
║  [✓] Hidden input           Keys never visible on screen   ║
║  [✓] Memory safety          Secrets zeroed when done       ║
║  [✓] Dry-run default        Commands shown, not executed   ║
║  [✓] Destructive detection  Warns about dangerous cmds     ║
║  [✓] Critical blocking      Blocks rm -rf / and similar    ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
```

---

## Next Steps

```
┌──────────────────────────────────────────────────────┐
│                                                      │
│  [1] Installation ────── Detailed install guide      │
│  [2] Configuration ───── Set up providers & creds    │
│  [3] Usage ───────────── Learn all the features      │
│  [4] Security ────────── Understand safety features  │
│                                                      │
└──────────────────────────────────────────────────────┘
```

- [Installation](./installation.md)
- [Configuration](./configuration.md)
- [Usage](./usage.md)
- [Security](./security.md)
