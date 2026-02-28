# Usage

## Basic Usage

Simply describe what you want to do:

```bash
cmd "your natural language description"
```

The command will be generated and executed immediately.

## Command Options

```
cmd [OPTIONS] <COMMAND>...

Arguments:
  <COMMAND>...  Describe what you want to do in natural language

Options:
  -d, --dry                  Show command without executing (copies to clipboard)
  -m, --model <MODEL>        Override the default model
  -e, --endpoint <ENDPOINT>  Override the API endpoint
  -h, --help                 Print help
  -V, --version              Print version
```

## Dry Run Mode

Use `--dry` or `-d` to preview commands without executing:

```bash
cmd --dry "delete all .log files older than 30 days"
# execute:
#     find . -name "*.log" -mtime +30 -delete
# cmd copied to clipboard
```

The command is copied to your clipboard for manual execution.

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
