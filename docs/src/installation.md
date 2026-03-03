# Installation

## Requirements

| Requirement | Version | Notes |
|-------------|---------|-------|
| **Rust** | 1.75+ | [Install via rustup](https://rustup.rs/) |
| **OS** | macOS / Linux | Windows support coming soon |

---

## Quick Install

```bash
# Clone and build
git clone https://github.com/Proteusiq/cmd.git
cd cmd
cargo build --release

# Install to local bin
mkdir -p ~/.local/bin
cp target/release/cmd ~/.local/bin/
```

### Add to PATH

If `~/.local/bin` isn't in your PATH, add it:

```bash
# For Zsh (~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"

# For Bash (~/.bashrc)
export PATH="$HOME/.local/bin:$PATH"
```

Then reload your shell:

```bash
source ~/.zshrc  # or source ~/.bashrc
```

---

## Verify Installation

```bash
$ cmd --version
cmd 0.5.0

$ cmd --help
Natural language CLI - translate intentions into terminal commands
...
```

---

## First-Time Setup

After installation, run the setup wizard:

```bash
cmd setup
```

This will:
1. Let you choose your LLM provider (Claude, OpenAI, Ollama, etc.)
2. Securely store your API key in the system keychain
3. Test the connection

---

## Update

To update to the latest version:

```bash
cd cmd
git pull
cargo build --release
cp target/release/cmd ~/.local/bin/
```

---

## Uninstall

```bash
# Remove binary
rm ~/.local/bin/cmd

# Remove configuration (optional)
rm -rf ~/.config/cmd

# Remove keychain entries (optional)
cmd config --delete-key anthropic
cmd config --delete-key openai
# Or use Keychain Access on macOS
```

---

## Troubleshooting

### "command not found: cmd"

Make sure `~/.local/bin` is in your PATH:

```bash
echo $PATH | grep -q '.local/bin' && echo "OK" || echo "Add ~/.local/bin to PATH"
```

### Build errors

Make sure you have the latest Rust:

```bash
rustup update
```

### Keychain errors on Linux

Install the Secret Service backend:

```bash
# Ubuntu/Debian
sudo apt install libsecret-1-dev gnome-keyring

# Fedora
sudo dnf install libsecret-devel gnome-keyring

# Arch
sudo pacman -S libsecret gnome-keyring
```

### Permission denied

Make sure the binary is executable:

```bash
chmod +x ~/.local/bin/cmd
```
