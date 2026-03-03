# Security

`cmd` generates shell commands using LLMs and can execute them on your system. Security is not an afterthought—it's built into every layer.

---

## Safe by Default

By default, `cmd` runs in **dry-run mode**:

- Shows the generated command
- Copies it to your clipboard
- **Does NOT execute it**

```bash
$ cmd "list all files"
╭──────────────────────────────────────────────────────╮
│ ls -la                                               │
╰──────────────────────────────────────────────────────╯
  ↳ copied to clipboard
  ↳ use --enable-execution to run this command
```

To execute commands, you must explicitly opt-in with `--enable-execution`.

---

## Credential Security

### System Keychain Storage

API keys are stored in your operating system's native secure storage:

| Platform | Storage Backend |
|----------|-----------------|
| macOS | Keychain Access |
| Linux | Secret Service (GNOME Keyring, KWallet) |
| Windows | Credential Manager |

**Benefits:**
- Encrypted at rest by the OS
- Protected by your system login
- Never stored in plain text files
- Never written to `.zshrc` or `.bashrc`

### Encrypted File Fallback

For headless servers without keychain access, `cmd` falls back to encrypted file storage:

- **Algorithm:** AES-256-GCM (authenticated encryption)
- **Key Derivation:** Argon2id (memory-hard, GPU-resistant)
- **Location:** `~/.config/cmd/credentials.enc`
- **Permissions:** `600` (owner read/write only)

### Hidden Input

API keys are never visible when typing:

```bash
$ cmd setup
? Select your LLM provider: Claude (Anthropic)
Get your API key at: https://console.anthropic.com/settings/keys

? API key: ········································
✓ API key saved to system keychain
```

### Memory Safety

Sensitive data is zeroed from memory when no longer needed:

- Encryption keys use a custom `SecretKey` wrapper
- Memory is overwritten with zeros before deallocation
- Compiler fences prevent optimization from skipping zeroing

### Input Validation

During setup, `cmd` validates:

- **API key format** — Correct prefix (`sk-ant-` for Anthropic, `sk-` for OpenAI)
- **API key length** — Minimum expected length per provider
- **URL format** — Valid URL structure for custom endpoints
- **Suspicious endpoints** — Blocks known data exfiltration services

```bash
# Blocked suspicious endpoints
$ cmd setup
? API endpoint URL: https://evil.ngrok.io/steal
Error: Suspicious endpoint detected: ngrok.io. This could be an attempt to steal your API key.
```

---

## Execution Safety

### Execution Modes

| Mode | Command | Behavior |
|------|---------|----------|
| Dry-run | `cmd "query"` | Show only, copy to clipboard |
| With confirmation | `cmd --enable-execution "query"` | Prompt before executing |
| Skip confirmation | `cmd --enable-execution --skip-confirmation "query"` | Execute immediately |

### Destructive Command Detection

`cmd` analyzes generated commands and categorizes them by risk level:

#### Warning Level

Commands that modify files or system state:

- `rm` — File deletion
- `mv` — File moves (can overwrite)
- `chmod`, `chown` — Permission changes
- `sudo` — Privileged execution
- `git push --force` — History rewriting
- `docker rm`, `docker rmi` — Container/image removal

#### Dangerous Level

High-risk commands that can cause significant damage:

- `rm -rf` — Recursive forced deletion
- `dd` — Low-level disk operations
- `mkfs` — Filesystem formatting
- `curl | sh` — Remote code execution
- `kill -9`, `killall` — Force kill processes

#### Critical Level (Blocked)

Commands that could destroy your system are **blocked entirely**:

- `rm -rf /` or `rm -rf /*` — Filesystem destruction
- `rm -rf ~` — Home directory destruction
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

For destructive commands, confirmation is **always required**, even with `--skip-confirmation`:

```bash
$ cmd --enable-execution --skip-confirmation "delete node_modules"
╭──────────────────────────────────────────────────────╮
│ rm -rf node_modules                                  │
╰──────────────────────────────────────────────────────╯

⚠️  DANGEROUS
  • DANGER: Recursive forced deletion

? This is a destructive command. Execute anyway? (y/N)
```

---

## Managing Credentials

### View Stored Keys

```bash
$ cmd config --show-keys

Stored API keys:
  Anthropic: ********************************
  OpenAI: (not set)
  Ollama: (not set)
```

Keys are masked—only asterisks are shown, never actual content.

### Delete Keys

```bash
$ cmd config --delete-key anthropic
✓ Deleted anthropic API key
```

### Using Native Tools

You can also manage credentials using your OS's native tools:

- **macOS:** Keychain Access app (search for "cmd-cli")
- **Linux:** `secret-tool` CLI or Seahorse GUI
- **Windows:** Credential Manager

---

## Best Practices

1. **Always review commands** before executing, especially those from LLMs
2. **Start with dry-run mode** until you're comfortable with the tool
3. **Use confirmation prompts** — don't skip them unless automating trusted workflows
4. **Be careful with wildcards** — `rm *.log` is safer than `rm -rf *`
5. **Test destructive commands** with `echo` first: `echo rm -rf folder/`
6. **Use version control** — commit before running potentially destructive commands
7. **Use environment variables** for CI — don't store keys in build configs

---

## Reporting Security Issues

If you discover a security vulnerability, please report it by:

1. Opening a private security advisory on GitHub
2. Emailing the maintainers directly

Please do **not** open public issues for security vulnerabilities.
