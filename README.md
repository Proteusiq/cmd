# Vibe CLI

> Sometimes we know what we want but just forgot the command

A natural language CLI tool that translates your intentions into terminal commands.

## Quick Start

### Install

**macOS / Linux:**

```bash
git clone https://github.com/Proteusiq/cmd.git
cd cmd
cargo build --release
mkdir -p ~/.local/bin && mv target/release/cmd ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"  # add to ~/.zshrc or ~/.bashrc
```

**Windows (PowerShell):**

```powershell
git clone https://github.com/Proteusiq/cmd.git
cd cmd
cargo build --release
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.local\bin"
Move-Item -Path "target\release\cmd.exe" -Destination "$env:USERPROFILE\.local\bin\"
$env:PATH += ";$env:USERPROFILE\.local\bin"
```

### Configure

Run the interactive setup:

```bash
./scripts/setup.sh
```

Or manually set **one** of the following:

```bash
# Claude (Anthropic)
export ANTHROPIC_API_KEY=sk-ant-...

# OpenAI
export OPENAI_API_KEY=sk-...

# Ollama (local, free)
ollama pull qwen2.5-coder
export OLLAMA_HOST=http://localhost:11434
```

### Use

```bash
cmd "find files larger than 100MB"
cmd "show commits from last week"
cmd --dry "delete old log files"  # preview only
```

## Documentation

Full documentation available at [proteusiq.github.io/cmd](https://proteusiq.github.io/cmd)

## Architecture

```
src/
  main.rs           # CLI entry point (thin shell)
  lib.rs            # Module exports
  core/
    config.rs       # Provider detection (pure, testable)
  providers/
    anthropic.rs    # Anthropic API client
    openai.rs       # OpenAI/Ollama API client
  cli/
    output.rs       # Spinner, colors, clipboard
```

## Options

```
cmd [OPTIONS] <COMMAND>...

Arguments:
  <COMMAND>...  Describe what you want to do in natural language

Options:
  -d, --dry                  Show command without executing
  -m, --model <MODEL>        Override the default model
  -e, --endpoint <ENDPOINT>  Override the API endpoint
  -h, --help                 Print help
  -V, --version              Print version
```

## Configuration

| Provider | Environment Variable | Default Model |
|----------|---------------------|---------------|
| Anthropic | `ANTHROPIC_API_KEY` | claude-sonnet-4-20250514 |
| OpenAI | `OPENAI_API_KEY` | gpt-4o |
| Ollama | `OLLAMA_HOST` | qwen2.5-coder |

## License

Apache 2.0
