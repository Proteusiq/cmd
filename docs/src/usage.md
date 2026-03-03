# Usage

## Basic Usage

Describe what you want to do in plain English:

```bash
cmd "your natural language description"
```

By default, `cmd` runs in **dry-run mode**—it shows the generated command and copies it to your clipboard, but does not execute it.

```bash
$ cmd find files larger than 100MB
╭──────────────────────────────────────────────────────╮
│ find . -size +100M -type f                           │
╰──────────────────────────────────────────────────────╯
  ↳ copied to clipboard
  ↳ use --enable-execution to run this command
```

---

## Command Reference

```
USAGE:
    cmd [OPTIONS] <QUERY>...
    cmd setup
    cmd config [OPTIONS]

ARGUMENTS:
    <QUERY>...    Describe what you want to do in natural language

OPTIONS:
        --enable-execution     Execute the generated command
        --skip-confirmation    Skip the confirmation prompt
    -m, --model <MODEL>        Override the default model
    -e, --endpoint <ENDPOINT>  Override the API endpoint
    -h, --help                 Print help
    -V, --version              Print version

SUBCOMMANDS:
    setup     Configure your LLM provider interactively
    config    Manage settings and API keys
```

---

## Execution Modes

### Dry-Run (Default)

Shows the command without executing:

```bash
$ cmd "compress all jpg files"
╭──────────────────────────────────────────────────────╮
│ tar -czvf images.tar.gz *.jpg                        │
╰──────────────────────────────────────────────────────╯
  ↳ copied to clipboard
```

### Execute with Confirmation

```bash
$ cmd --enable-execution "compress all jpg files"
╭──────────────────────────────────────────────────────╮
│ tar -czvf images.tar.gz *.jpg                        │
╰──────────────────────────────────────────────────────╯

? Execute this command? (y/N) y
```

### Execute without Confirmation

```bash
$ cmd --enable-execution --skip-confirmation "list files"
╭──────────────────────────────────────────────────────╮
│ ls -la                                               │
╰──────────────────────────────────────────────────────╯

total 24
drwxr-xr-x  5 user  staff  160 Mar  3 10:00 .
...
```

> **Note:** Destructive commands always require confirmation, even with `--skip-confirmation`.

---

## Configuration Commands

### Setup Wizard

```bash
cmd setup
```

Interactive setup to configure your LLM provider and store credentials securely.

### View Settings

```bash
$ cmd config --show

Current settings:
  enable_execution: false
  skip_confirmation: false

  config: /Users/you/.config/cmd/settings.toml
```

### Change Settings

```bash
# Enable execution mode (still prompts for confirmation)
cmd config --enable-execution

# Skip confirmation prompts (use with caution)
cmd config --skip-confirmation

# Reset to safe defaults
cmd config --disable-execution --require-confirmation
```

### Manage API Keys

```bash
# View stored keys (masked)
cmd config --show-keys

# Delete a stored key
cmd config --delete-key anthropic
cmd config --delete-key openai
cmd config --delete-key ollama_host
```

---

## Model and Endpoint Override

### Use a Different Model

```bash
# Use Claude Haiku instead of default Sonnet
cmd -m claude-haiku-4-5 "list files"

# Use GPT-5.2 mini for faster responses
cmd -m gpt-5.2-mini "show disk usage"
```

### Use a Custom Endpoint

```bash
# Use a proxy
cmd -e https://my-proxy.com/v1/messages "list files"

# Use LM Studio locally
cmd -e http://localhost:1234/v1/chat/completions "list files"

# Use a different Ollama host
cmd -e http://192.168.1.100:11434/v1/chat/completions "list files"
```

---

## Tips and Tricks

### Quoting

Simple queries work without quotes:

```bash
cmd find large files
cmd show disk usage
cmd list running containers
```

**Use quotes when your query contains shell special characters:**

```bash
cmd "what's using port 3000?"      # apostrophe and ?
cmd "find *.log files"             # glob (*)
cmd "show $PATH variable"          # dollar sign
cmd "find files with 'test' in name"  # nested quotes
cmd "why isn't this working?"      # apostrophe and ?
```

**Characters that require quoting:**

| Characters | Name | Shell behavior |
|------------|------|----------------|
| `?` `*` `[` `]` | Globs | Pattern matching |
| `$` | Dollar | Variable expansion |
| `` ` `` `$()` | Backticks | Command substitution |
| `&` | Ampersand | Background process |
| `\|` | Pipe | Command piping |
| `;` | Semicolon | Command separator |
| `<` `>` | Redirects | I/O redirection |
| `(` `)` | Parens | Subshell |
| `{` `}` | Braces | Brace expansion |
| `~` | Tilde | Home directory |
| `!` | Bang | History expansion |
| `'` `"` `\` | Quotes/escape | Quoting characters |

> **Tip:** When in doubt, use quotes. It never hurts.

### Be Specific

More specific queries get better results:

```bash
# Less specific
cmd "delete old files"

# More specific
cmd "delete all .log files in /var/log older than 30 days"
```

### Iterate

If the first command isn't quite right, refine your query:

```bash
# First attempt
cmd "find big files"
# → find . -size +1G

# Refined
cmd "find files larger than 100MB, show size in human readable format"
# → find . -size +100M -exec ls -lh {} \;
```

### Preview First

Always use dry-run mode for unfamiliar commands:

```bash
# See what it generates
cmd "recursively change permissions"

# Review the command, then execute if correct
cmd --enable-execution "recursively chmod 755 all directories"
```
