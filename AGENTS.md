# Agent Instructions for cmd

This document provides AI coding agents with project-specific conventions and guidelines.

## Project Overview

`cmd` is a natural language CLI tool that translates intentions into terminal commands using LLMs. It's written in Rust and prioritizes security and simplicity.

## Architecture

```
src/
  core/           # Pure business logic (no I/O)
    config.rs     # Provider detection and configuration
    credentials.rs # Secure credential storage (keychain + encrypted file)
    encrypted_file.rs # AES-256-GCM encryption fallback
    safety.rs     # Destructive command detection
    settings.rs   # User preferences (TOML)
  cli/            # CLI interface
    setup.rs      # Interactive setup wizard
    output.rs     # Display helpers
  providers/      # LLM API clients
    anthropic.rs  # Claude API
    openai.rs     # OpenAI/compatible APIs
  main.rs         # Entry point
```

## Code Conventions

### Rust Style

- **No inline comments** — code should be self-documenting
- **Doc comments for public APIs only** — use `///` for public functions/types
- **Error handling** — use `Result<T, E>` and `?`, avoid `.unwrap()` in production
- **Types at module boundaries** — explicit types on public function signatures

### Before Commit

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

### Security Principles

1. **Never store secrets in plain text** — use system keychain or encrypted files
2. **Zero sensitive memory** — use `SecretKey` wrapper for cryptographic material
3. **Validate all input** — API keys, URLs, user input
4. **Hidden password input** — use `dialoguer::Password` for sensitive input

## Dependencies

| Crate | Purpose |
|-------|---------|
| `keyring` | System keychain (macOS Keychain, Linux Secret Service) |
| `aes-gcm` | AES-256-GCM encryption |
| `argon2` | Password-based key derivation |
| `secrecy` | Memory-safe secret handling |
| `dialoguer` | Interactive prompts |
| `ureq` | HTTP client |
| `clap` | CLI argument parsing |

## Testing

```bash
cargo test                    # Run all tests
cargo test core::credentials  # Run specific module tests
```

## Common Tasks

### Adding a New Provider

1. Add entry to `PROVIDERS` array in `src/cli/setup.rs`
2. Add credential key to `PROVIDERS` in `src/core/credentials.rs`
3. Update `Config::detect()` in `src/core/config.rs` if needed

### Modifying Credential Storage

The credential system has two layers:
1. **System keychain** (primary) — seamless, no password prompt
2. **Encrypted file** (fallback) — for headless environments

Service name: `com.proteusiq.cmd-cli`
