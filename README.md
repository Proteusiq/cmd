<img width="2710" height="1628" alt="CleanShot 2026-02-28 at 13 27 32@2x" src="https://github.com/user-attachments/assets/b75ffb7a-2807-4be2-9a62-e75c110d6fe2" />

<div align="center">

```
 ██████╗███╗   ███╗██████╗
██╔════╝████╗ ████║██╔══██╗
██║     ██╔████╔██║██║  ██║
██║     ██║╚██╔╝██║██║  ██║
╚██████╗██║ ╚═╝ ██║██████╔╝
 ╚═════╝╚═╝     ╚═╝╚═════╝
```

**Your words become commands.**

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

<details>
<summary><h2>Install</h2></summary>

```bash
git clone https://github.com/Proteusiq/cmd.git && cd cmd
cargo build --release
mkdir -p ~/.local/bin && mv target/release/cmd ~/.local/bin/
```

> [!TIP]
> Make sure `~/.local/bin` is in your `PATH`

</details>

## Setup

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

> [!NOTE]
> Works with **Claude**, **OpenAI**, **Ollama** (free, local), or any OpenAI-compatible API.

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

## Quoting

```bash
# ┌─────────────────────────────────────────────────┐
# │  These NEED quotes (special characters)         │
# └─────────────────────────────────────────────────┘
cmd "what's using port 3000?"      # apostrophe + ?
cmd "find *.log files"             # glob (*)
cmd "show $PATH variable"          # dollar sign

# ┌─────────────────────────────────────────────────┐
# │  These work WITHOUT quotes                      │
# └─────────────────────────────────────────────────┘
cmd list all files
cmd show disk usage
cmd find large files in downloads
```

**Characters that require quoting:** `` ? * [ ] $ & | ; < > ( ) ` ' " \ { } ~ ! ``

> [!TIP]
> When in doubt, use quotes. It never hurts.

## Security

```
╔════════════════════════════════════════════════════════════╗
║  SECURITY MODEL                                            ║
╠════════════════════════════════════════════════════════════╣
║                                                            ║
║  [✓] Dry-run by default     Commands shown, not executed   ║
║  [✓] Keychain storage       No plain text API keys         ║
║  [✓] Destructive detection  Warns about dangerous cmds     ║
║  [✓] Critical blocking      Blocks rm -rf / and similar    ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
```

> [!WARNING]
> **Commands are generated by an LLM and executed on your system.**
>
> By default, `cmd` runs in **dry-run mode** - it shows the command and copies to clipboard, but does NOT execute.
>
> ```bash
> cmd --enable-execution "query"                      # Execute with confirmation
> cmd --enable-execution --skip-confirmation "query"  # Execute without confirmation
> ```
>
> **Always review commands before executing.**

### Destructive Command Detection

```
┌──────────────────────────────────────────────────────────────┐
│  THREAT LEVELS                                               │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  ⚠️  WARNING   rm, mv, chmod, sudo, git push --force         │
│               → Prompts for confirmation                     │
│                                                              │
│  🔥 DANGER    rm -rf, dd, mkfs, curl|sh, kill -9             │
│               → Always prompts, even with --skip             │
│                                                              │
│  🛑 CRITICAL  rm -rf /, rm -rf ~, fork bombs                 │
│               → BLOCKED ENTIRELY                             │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

<img width="898" height="1059" alt="CleanShot 2026-03-03 at 11 39 44" src="https://github.com/user-attachments/assets/82a8dfd9-082b-4f0b-93ef-0e14433719d8" />

## Configuration

```bash
# ───────────────────────────────────────────────────
#  Execution settings
# ───────────────────────────────────────────────────
cmd config --enable-execution       # Enable execution mode
cmd config --skip-confirmation      # Skip prompts (careful!)
cmd config --show                   # View current settings
cmd config --disable-execution --require-confirmation  # Reset

# ───────────────────────────────────────────────────
#  API key management
# ───────────────────────────────────────────────────
cmd config --show-keys              # View stored keys (masked)
cmd config --delete-key anthropic   # Delete a stored key
```

Settings stored in `~/.config/cmd/settings.toml`:

```toml
enable_execution = false    # safe default
skip_confirmation = false   # safe default
```

> [!CAUTION]
> **Setting both `enable_execution` and `skip_confirmation` to `true` is dangerous.**
>
> We've designed `cmd` to be **secure by default** - but we know developers love their `--yolo` flags. If you enable both settings, you're living on the edge. Godspeed.

## How It Works

```
┌─────────────────┐     ┌─────────────┐     ┌──────────────┐
│  "compress      │     │             │     │ tar -czvf    │
│   this folder"  │ ──▶ │     LLM     │ ──▶ │ folder.tar.gz│ ──▶ run
│                 │     │             │     │ folder/      │
└─────────────────┘     └─────────────┘     └──────────────┘
     natural               Claude              shell
     language              GPT                 command
                           Ollama
```

## Options

```
┌────────────────────────────────────────────────────────────────────────────┐
│  USAGE                                                                     │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  cmd [query]                          Dry-run (default)                    │
│  cmd --enable-execution [query]       Execute with confirmation            │
│  cmd --enable-execution --skip-confirmation [query]                        │
│                                       Execute without confirmation         │
│                                                                            │
│  cmd setup                            Configure LLM provider               │
│  cmd config --show                    View settings                        │
│  cmd config --show-keys               View stored API keys                 │
│  cmd config --delete-key PROVIDER     Delete stored key                    │
│                                                                            │
│  cmd -m MODEL                         Use specific model                   │
│  cmd -e ENDPOINT                      Use custom endpoint                  │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

## Providers

```
┌──────────────┬───────────────────────────────────┬────────────────────┐
│  Provider    │  Environment Variable             │  Default Model     │
├──────────────┼───────────────────────────────────┼────────────────────┤
│  Claude      │  ANTHROPIC_API_KEY=sk-ant-...     │  claude-sonnet-4-6 │
│  OpenAI      │  OPENAI_API_KEY=sk-...            │  gpt-5.2           │
│  Ollama      │  OLLAMA_HOST=http://localhost:... │  qwen2.5-coder     │
└──────────────┴───────────────────────────────────┴────────────────────┘
```

> [!NOTE]
> Environment variables take priority over keychain. This allows temporary overrides.

> [!TIP]
> For **Azure OpenAI**, **Groq**, or other providers, use `cmd setup` and select "Other (custom)".

## Requirements

```
┌────────────────────────────────────────────────────┐
│  • macOS or Linux                                  │
│  • Rust (to build)                                 │
│  • LLM provider (or Ollama for free local)         │
└────────────────────────────────────────────────────┘
```

## License

Apache 2.0

---

<div align="center">

*Stop googling. Start doing.*

**[Documentation](https://proteusiq.github.io/cmd/)** · **[Issues](https://github.com/Proteusiq/cmd/issues)** · **[Releases](https://github.com/Proteusiq/cmd/releases)**

</div>
