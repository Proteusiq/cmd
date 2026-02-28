# Architecture

Vibe CLI follows the **Functional Core, Imperative Shell** pattern for maintainability and testability.

## Project Structure

```
src/
  main.rs           # CLI entry point (thin shell)
  lib.rs            # Module exports
  core/
    mod.rs
    config.rs       # Provider detection (pure, testable)
  providers/
    mod.rs
    anthropic.rs    # Anthropic API client
    openai.rs       # OpenAI/Ollama API client
  cli/
    mod.rs
    output.rs       # Spinner, colors, clipboard
```

## Design Principles

### Functional Core

The `core/` module contains pure functions with no side effects:

- **Config detection** - Takes environment variable getter as a parameter
- **Testable** - Unit tests don't need mocks for environment variables
- **Predictable** - Same inputs always produce same outputs

```rust
// Pure function - testable without mocking env vars
pub fn detect(
    model_override: Option<&str>,
    endpoint_override: Option<&str>,
    env_vars: &dyn Fn(&str) -> Option<String>,
) -> Option<Config>
```

### Imperative Shell

The `main.rs` is a thin shell that:

1. Parses CLI arguments
2. Wires dependencies together
3. Handles I/O (network, terminal, clipboard)
4. Reports errors

### Side Effects Isolated

The `providers/` and `cli/` modules handle I/O:

- **providers/** - HTTP calls to LLM APIs
- **cli/** - Terminal output, clipboard operations

## Data Flow

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   CLI Args  │ ──▶ │  Config      │ ──▶ │  Provider   │
│   (clap)    │     │  Detection   │     │  API Call   │
└─────────────┘     └──────────────┘     └─────────────┘
                                               │
                                               ▼
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   Execute   │ ◀── │  Display     │ ◀── │  LLM        │
│   Command   │     │  Result      │     │  Response   │
└─────────────┘     └──────────────┘     └─────────────┘
```

## Error Handling

- Uses `anyhow` for error propagation with context
- Semantic exit codes via `exitcode` crate
- Friendly panic messages via `human-panic`

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `anyhow` | Error handling |
| `ureq` | Sync HTTP client |
| `serde` | JSON serialization |
| `owo-colors` | Terminal colors |
| `spinoff` | Loading spinner |
| `human-panic` | Friendly crash reports |
| `exitcode` | Semantic exit codes |
