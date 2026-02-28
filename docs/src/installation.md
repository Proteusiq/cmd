# Installation

## Prerequisites

- [Rust](https://rustup.rs/)
- macOS or Linux

## Install

```bash
git clone https://github.com/Proteusiq/cmd.git
cd cmd
cargo build --release
mkdir -p ~/.local/bin && mv target/release/cmd ~/.local/bin/
```

Add to PATH (if not already):

```bash
# ~/.zshrc or ~/.bashrc
export PATH="$HOME/.local/bin:$PATH"
```

## Verify

```bash
cmd --version
cmd --help
```

## Update

```bash
cd cmd
git pull
cargo build --release && mv target/release/cmd ~/.local/bin/
```
