# cmd

**Turn words into commands.**

```bash
$ cmd "find all rust files modified today"
execute:
    find . -name "*.rs" -mtime 0
```

You know what you want. You just forgot the syntax.

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

Or export your API key directly:

```bash
# Anthropic
export ANTHROPIC_API_KEY=sk-ant-...

# OpenAI  
export OPENAI_API_KEY=sk-...

# Ollama (local, free)
export OLLAMA_HOST=http://localhost:11434
```

## Use

```bash
cmd "compress this folder"
cmd "find files larger than 100MB"
cmd "show git commits from last week"
cmd "kill process on port 3000"
cmd "disk usage sorted by size"
```

Preview before running:

```bash
cmd --dry "delete all node_modules"
```

## Options

```
cmd [OPTIONS] <query>

Options:
  -d, --dry        Preview command, copy to clipboard
  -m, --model      Override model
  -e, --endpoint   Custom API endpoint
  -h, --help       Help
  -V, --version    Version

Commands:
  setup            Configure provider interactively
```

## Providers

| Provider | Env Variable | Default Model |
|----------|--------------|---------------|
| Anthropic | `ANTHROPIC_API_KEY` | claude-sonnet-4-20250514 |
| OpenAI | `OPENAI_API_KEY` | gpt-4o |
| Ollama | `OLLAMA_HOST` | qwen2.5-coder |

Custom endpoints (Azure, Groq, etc.):

```bash
export OPENAI_API_KEY=your-key
cmd -e "https://your-endpoint/v1/chat/completions" "list files"
```

## License

Apache 2.0
