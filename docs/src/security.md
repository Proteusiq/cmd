# Security

## Overview

`cmd` generates shell commands using LLMs and can execute them on your system. This page documents the security features designed to protect you from accidental or malicious command execution.

## Safe by Default

By default, `cmd` runs in **dry-run mode**:

- Shows the generated command
- Copies it to your clipboard
- Does NOT execute it

To execute commands, you must explicitly opt-in with `--enable-execution`.

## Execution Modes

### Dry-Run (Default)

```bash
$ cmd "list all files"
╭──────────────────────────────────────────────────────╮
│ ls -la                                               │
╰──────────────────────────────────────────────────────╯
  ↳ copied to clipboard
  ↳ use --enable-execution to run this command
```

### With Confirmation

```bash
$ cmd --enable-execution "list all files"
╭──────────────────────────────────────────────────────╮
│ ls -la                                               │
╰──────────────────────────────────────────────────────╯

? Execute this command? (y/N)
```

### Skip Confirmation

```bash
$ cmd --enable-execution --skip-confirmation "list all files"
# Executes immediately (use with caution!)
```

## Destructive Command Detection

`cmd` automatically detects potentially dangerous commands and takes protective action.

### Warning Level

Commands that modify files or system state trigger warnings:

- `rm` - File deletion
- `mv` - File moves (can overwrite)
- `chmod`, `chown` - Permission changes
- `sudo` - Privileged execution
- `git push --force` - History rewriting
- `docker rm`, `docker rmi` - Container/image removal

### Dangerous Level

High-risk commands that can cause significant damage:

- `rm -rf` - Recursive forced deletion
- `dd` - Low-level disk operations
- `mkfs` - Filesystem formatting
- `curl | sh` - Remote code execution
- `kill -9`, `killall` - Force kill processes

### Critical Level (Blocked)

Commands that could destroy your system are **blocked entirely**:

- `rm -rf /` or `rm -rf /*` - Filesystem destruction
- `rm -rf ~` - Home directory destruction
- Fork bombs (`:(){:|:&};:`)
- Direct writes to `/dev/sda`

```bash
$ cmd --enable-execution "delete everything"
╭──────────────────────────────────────────────────────╮
│ rm -rf /                                             │
╰──────────────────────────────────────────────────────╯

🛑 CRITICAL
  • CRITICAL: Removes entire filesystem
  • DANGER: Recursive forced deletion

BLOCKED: This command is too dangerous to execute.
If you really need to run this, copy and execute it manually.
```

### Forced Confirmation

For destructive commands, confirmation is **always required**, even if you've set `--skip-confirmation` in your config:

```bash
$ cmd --enable-execution "delete node_modules"
╭──────────────────────────────────────────────────────╮
│ rm -rf node_modules                                  │
╰──────────────────────────────────────────────────────╯

⚠️  DANGEROUS
  • DANGER: Recursive forced deletion

? This is a destructive command. Execute anyway? (y/N)
```

## Credential Storage

API keys can be stored securely using AES-256-GCM encryption with Argon2id key derivation.

### Encrypted Storage

Credentials are stored in `~/.config/cmd/credentials.enc`:

- Encrypted with AES-256-GCM
- Key derived using Argon2id (password hashing)
- File permissions set to `600` (owner only)
- Directory permissions set to `700`

### Environment Variables

You can also use environment variables (existing behavior):

```bash
export ANTHROPIC_API_KEY=sk-ant-...
export OPENAI_API_KEY=sk-...
export OLLAMA_HOST=http://localhost:11434
```

## Best Practices

1. **Always review commands** before executing, especially those from LLMs
2. **Start with dry-run** mode until you're comfortable with the tool
3. **Use confirmation prompts** - don't skip them unless automating trusted workflows
4. **Be careful with wildcards** - `rm *.log` is safer than `rm -rf *`
5. **Test destructive commands** with `echo` first: `echo rm -rf folder/`
6. **Use version control** - commit before running potentially destructive commands

## Reporting Security Issues

If you discover a security vulnerability, please report it by opening an issue on GitHub or contacting the maintainers directly.
