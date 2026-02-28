# Vibe CLI

> Sometimes we know what we want but just forgot the command

Vibe CLI is a natural language command-line tool that translates your intentions into terminal commands. Instead of remembering complex syntax, just describe what you want to do.

## Why Vibe CLI?

Ever found yourself thinking "I want to find all files larger than 100MB" but can't remember the exact `find` syntax? Vibe CLI bridges that gap by letting you describe what you want in plain English.

```bash
cmd "find files larger than 100MB in current directory"
# execute:
#     find . -size +100M -type f
```

## Features

- **Natural language input** - Describe tasks in plain English
- **Multiple LLM providers** - Claude, OpenAI, or local Ollama
- **Dry run mode** - Preview commands before execution
- **Clipboard integration** - Copy commands with `--dry` flag
- **Cross-platform** - macOS, Linux, Windows support

## Quick Example

```bash
# Set your API key
export ANTHROPIC_API_KEY=sk-ant-...

# Use natural language
cmd "show disk usage sorted by size"
cmd "find all rust files modified today"
cmd "compress this folder into a tar.gz"

# Preview without executing
cmd --dry "delete all node_modules folders"
```
