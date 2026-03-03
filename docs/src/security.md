# Security

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║   ███████╗███████╗ ██████╗██╗   ██╗██████╗ ██╗████████╗██╗   ██╗          ║
║   ██╔════╝██╔════╝██╔════╝██║   ██║██╔══██╗██║╚══██╔══╝╚██╗ ██╔╝          ║
║   ███████╗█████╗  ██║     ██║   ██║██████╔╝██║   ██║    ╚████╔╝           ║
║   ╚════██║██╔══╝  ██║     ██║   ██║██╔══██╗██║   ██║     ╚██╔╝            ║
║   ███████║███████╗╚██████╗╚██████╔╝██║  ██║██║   ██║      ██║             ║
║   ╚══════╝╚══════╝ ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝   ╚═╝      ╚═╝             ║
║                                                                           ║
║   Security is not an afterthought—it's built into every layer.            ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

---

## Safe by Default

```
┌─────────────────────────────────────────────────────────────────┐
│  DEFAULT MODE: DRY-RUN                                          │
│                                                                 │
│  [✓] Shows the generated command                                │
│  [✓] Copies to clipboard                                        │
│  [✗] Does NOT execute                                           │
└─────────────────────────────────────────────────────────────────┘
```

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

```
┌──────────────────────────────────────────────────────────────────┐
│  CREDENTIAL STORAGE                                              │
├──────────────┬───────────────────────────────────────────────────┤
│  macOS       │  Keychain Access                                  │
│  Linux       │  Secret Service (GNOME Keyring, KWallet)          │
│  Windows     │  Credential Manager                               │
│  Fallback    │  AES-256-GCM encrypted file                       │
└──────────────┴───────────────────────────────────────────────────┘
```

**Why this matters:**

```
  ┌─ OLD WAY (insecure) ────────────────────────────────┐
  │                                                     │
  │   $ echo 'export ANTHROPIC_API_KEY=sk-...' >> ~/.zshrc
  │                                                     │
  │   [✗] Plain text on disk                            │
  │   [✗] Readable by any process                       │
  │   [✗] Ends up in shell history                      │
  │   [✗] Accidentally committed to git                 │
  │                                                     │
  └─────────────────────────────────────────────────────┘

  ┌─ cmd WAY (secure) ──────────────────────────────────┐
  │                                                     │
  │   $ cmd setup                                       │
  │   ? API key: ········                               │
  │   ✓ Saved to system keychain                        │
  │                                                     │
  │   [✓] Encrypted at rest by OS                       │
  │   [✓] Protected by system login                     │
  │   [✓] Never in plain text files                     │
  │   [✓] Never in shell history                        │
  │                                                     │
  └─────────────────────────────────────────────────────┘
```

### Encrypted File Fallback

For headless servers without keychain access:

```
┌─────────────────────────────────────────────────────────────────┐
│  ENCRYPTION SPEC                                                │
├─────────────────────────────────────────────────────────────────┤
│  Algorithm      │  AES-256-GCM (authenticated encryption)       │
│  Key Derivation │  Argon2id (memory-hard, GPU-resistant)        │
│  Location       │  ~/.config/cmd/credentials.enc                │
│  Permissions    │  600 (owner read/write only)                  │
└─────────────────────────────────────────────────────────────────┘
```

### Hidden Input

API keys are never visible when typing:

```bash
$ cmd setup
? Select your LLM provider: Claude (Anthropic)
Get your API key at: https://console.anthropic.com/settings/keys

? API key: ········································
           ▲
           └── Characters hidden, not echoed

✓ API key saved to system keychain
```

### Memory Safety

```
┌─────────────────────────────────────────────────────────────────┐
│  MEMORY PROTECTION                                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   ┌──────────┐     ┌──────────┐     ┌──────────┐                │
│   │  Input   │ ──▶ │   Use    │ ──▶ │  Zero    │                │
│   │  Secret  │     │  Secret  │     │  Memory  │                │
│   └──────────┘     └──────────┘     └──────────┘                │
│                                           │                     │
│                                           ▼                     │
│                                    ┌──────────┐                 │
│                                    │  0x0000  │                 │
│                                    │  0x0000  │                 │
│                                    │  0x0000  │                 │
│                                    └──────────┘                 │
│                                                                 │
│   • SecretKey wrapper zeros memory on drop                      │
│   • Compiler fences prevent optimization skip                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Input Validation

```bash
# Blocked suspicious endpoints
$ cmd setup
? API endpoint URL: https://evil.ngrok.io/steal

╔════════════════════════════════════════════════════════════════╗
║  [ERROR] Suspicious endpoint detected                          ║
║                                                                ║
║  Domain 'ngrok.io' is commonly used for data exfiltration.     ║
║  This could be an attempt to steal your API key.               ║
╚════════════════════════════════════════════════════════════════╝
```

---

## Execution Safety

### Execution Modes

```
┌────────────────────┬──────────────────────────────────────────────────┐
│  MODE              │  COMMAND                                         │
├────────────────────┼──────────────────────────────────────────────────┤
│  Dry-run           │  cmd "query"                                     │
│  (default)         │  → Show only, copy to clipboard                  │
├────────────────────┼──────────────────────────────────────────────────┤
│  With confirmation │  cmd --enable-execution "query"                  │
│                    │  → Prompt before executing                       │
├────────────────────┼──────────────────────────────────────────────────┤
│  Auto-execute      │  cmd --enable-execution --skip-confirmation ...  │
│  (dangerous)       │  → Execute immediately                           │
└────────────────────┴──────────────────────────────────────────────────┘
```

### Destructive Command Detection

```
┌─────────────────────────────────────────────────────────────────────────┐
│  THREAT LEVELS                                                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│   ┌─────────┐                                                           │
│   │ WARNING │  rm, mv, chmod, chown, sudo, git push --force             │
│   │   ⚠️    │  → Prompts for confirmation                               │
│   └─────────┘                                                           │
│                                                                         │
│   ┌─────────┐                                                           │
│   │ DANGER  │  rm -rf, dd, mkfs, curl|sh, kill -9                       │
│   │   🔥    │  → Always prompts, even with --skip-confirmation          │
│   └─────────┘                                                           │
│                                                                         │
│   ┌─────────┐                                                           │
│   │CRITICAL │  rm -rf /, rm -rf ~, fork bombs, dd /dev/sda              │
│   │   🛑    │  → BLOCKED ENTIRELY                                       │
│   └─────────┘                                                           │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Critical Commands = Blocked

```bash
$ cmd --enable-execution "delete everything"
╭──────────────────────────────────────────────────────╮
│ rm -rf /                                             │
╰──────────────────────────────────────────────────────╯

╔══════════════════════════════════════════════════════════════════╗
║  🛑 CRITICAL - BLOCKED                                           ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                  ║
║  • CRITICAL: Removes entire filesystem                           ║
║  • DANGER: Recursive forced deletion                             ║
║                                                                  ║
║  This command is too dangerous to execute.                       ║
║  If you really need to run this, copy and execute it manually.   ║
║                                                                  ║
╚══════════════════════════════════════════════════════════════════╝
```

### Forced Confirmation

Destructive commands **always** prompt, even with `--skip-confirmation`:

```bash
$ cmd --enable-execution --skip-confirmation "delete node_modules"
╭──────────────────────────────────────────────────────╮
│ rm -rf node_modules                                  │
╰──────────────────────────────────────────────────────╯

⚠️  DANGEROUS
  • DANGER: Recursive forced deletion

? This is a destructive command. Execute anyway? (y/N) _
```

---

## Managing Credentials

```bash
# View stored keys (masked)
$ cmd config --show-keys
┌─────────────────────────────────────────────────────┐
│  Stored API keys:                                   │
│                                                     │
│    Anthropic: sk-ant-*************************      │
│    OpenAI:    (not set)                             │
│    Ollama:    (not set)                             │
└─────────────────────────────────────────────────────┘

# Delete a key
$ cmd config --delete-key anthropic
✓ Deleted anthropic API key
```

---

## Best Practices

```
╔═══════════════════════════════════════════════════════════════════════════╗
║  SECURITY CHECKLIST                                                       ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                           ║
║  [1] Always review commands before executing                              ║
║                                                                           ║
║  [2] Start with dry-run mode until comfortable                            ║
║                                                                           ║
║  [3] Use confirmation prompts — don't skip unless automating              ║
║                                                                           ║
║  [4] Be careful with wildcards — rm *.log safer than rm -rf *             ║
║                                                                           ║
║  [5] Test destructive commands with echo first:                           ║
║      $ echo rm -rf folder/                                                ║
║                                                                           ║
║  [6] Use version control — commit before destructive commands             ║
║                                                                           ║
║  [7] Use env vars for CI — don't store keys in build configs              ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

---

## Reporting Security Issues

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  Found a vulnerability?                                         │
│                                                                 │
│  [1] Open a private security advisory on GitHub                 │
│  [2] Email the maintainers directly                             │
│                                                                 │
│  ⚠️  Please do NOT open public issues for security bugs         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```
