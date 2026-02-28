# Contributing

Contributions are welcome! Here's how to get started.

## Development Setup

```bash
# Clone the repository
git clone https://github.com/Proteusiq/cmd.git
cd cmd

# Build
cargo build

# Run tests
cargo test

# Run with development build
cargo run -- "list files"
```

## Code Style

We follow the conventions in [AGENTS.md](https://github.com/Proteusiq/dotfiles/blob/main/AGENTS.md):

- **Simplicity is king** - the simplest solution that works is the best
- **Functional over OOP** - pure functions, composition, immutability
- **No inline comments** - code should be self-documenting

### Before Committing

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

## Project Structure

```
src/
  main.rs           # CLI entry point (keep thin)
  lib.rs            # Module exports
  core/             # Pure business logic (testable)
  providers/        # LLM API clients (side effects)
  cli/              # Terminal I/O (side effects)
```

### Adding a New Provider

1. Create `src/providers/newprovider.rs`
2. Implement the request/response types
3. Add to `src/providers/mod.rs`
4. Update config detection in `src/core/config.rs`

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test detects_anthropic

# Run with output
cargo test -- --nocapture
```

## Documentation

Documentation uses [mdBook](https://rust-lang.github.io/mdBook/).

```bash
# Install mdBook
cargo install mdbook

# Serve locally
cd docs
mdbook serve

# Build
mdbook build
```

## Commit Messages

Follow the format:

```
type: short description
```

| Type | Use |
|------|-----|
| `feat:` | New feature |
| `fix:` | Bug fix |
| `docs:` | Documentation |
| `refactor:` | Code restructure |
| `test:` | Tests |
| `chore:` | Maintenance |

## Pull Requests

1. Fork the repository
2. Create a feature branch (`feat/my-feature`)
3. Make your changes
4. Run tests and linting
5. Submit a PR with a clear description

## License

By contributing, you agree that your contributions will be licensed under the Apache 2.0 License.
