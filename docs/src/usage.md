# Usage

## Basic Usage

Simply describe what you want to do:

```bash
cmd "your natural language description"
```

By default, `cmd` runs in **dry-run mode** - it shows the generated command and copies it to your clipboard, but does not execute it.

```bash
$ cmd find files larger than 100MB
╭──────────────────────────────────────────────────────╮
│ find . -size +100M -type f                           │
╰──────────────────────────────────────────────────────╯
  ↳ copied to clipboard
  ↳ use --enable-execution to run this command
```

## Command Options

```
cmd [OPTIONS] <QUERY>...
cmd setup                    Configure your LLM provider
cmd config [OPTIONS]         Manage persistent settings

Arguments:
  <QUERY>...  Describe what you want to do in natural language

Options:
      --enable-execution     Enable command execution (required to run commands)
      --skip-confirmation    Skip confirmation prompt (requires --enable-execution)
  -m, --model <MODEL>        Override the default model
  -e, --endpoint <ENDPOINT>  Override the API endpoint
  -h, --help                 Print help
  -V, --version              Print version
```

## Execution Mode

To actually execute commands, use `--enable-execution`:

```bash
# Shows command and prompts for confirmation
cmd --enable-execution "delete all .log files older than 30 days"

# Skip confirmation (use with caution)
cmd --enable-execution --skip-confirmation "list all files"
```

## Persistent Settings

Save your preferences so you don't have to pass flags every time:

```bash
# Enable execution mode permanently (still prompts for confirmation)
cmd config --enable-execution

# Also skip confirmation (not recommended)
cmd config --skip-confirmation

# Check current settings
cmd config --show

# Reset to safe defaults
cmd config --disable-execution --require-confirmation
```

Settings are stored in `~/.config/cmd/settings.toml`.

## Quoting

You can use quotes around your description or not:

```bash
# Both work
cmd "find large files"
cmd find large files
```

For descriptions with special characters, use quotes:

```bash
cmd "find files with 'test' in the name"
```

## Multiple Words

All arguments are joined into a single prompt:

```bash
cmd show me the disk usage
# Same as:
cmd "show me the disk usage"
```
