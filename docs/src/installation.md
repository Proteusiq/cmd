# Installation

## Prerequisites

- [Rust](https://rustup.rs/) (for building from source)
- One of the supported LLM providers configured

## Build from Source

```bash
git clone https://github.com/Proteusiq/cmd.git
cd cmd
cargo build --release
```

## Install Binary

### macOS / Linux

```bash
# Create local bin directory
mkdir -p ~/.local/bin

# Move binary
mv target/release/cmd ~/.local/bin/

# Add to PATH (add to ~/.zshrc or ~/.bashrc for persistence)
export PATH="$HOME/.local/bin:$PATH"
```

**One-liner:**

```bash
cargo build --release && mkdir -p ~/.local/bin && mv target/release/cmd ~/.local/bin/
```

### Windows (PowerShell)

```powershell
# Create directory for binaries
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.local\bin"

# Move binary
Move-Item -Path "target\release\cmd.exe" -Destination "$env:USERPROFILE\.local\bin\"

# Add to PATH (current session)
$env:PATH += ";$env:USERPROFILE\.local\bin"

# Add to PATH permanently (run as Administrator or add manually)
[Environment]::SetEnvironmentVariable(
    "PATH",
    "$env:PATH;$env:USERPROFILE\.local\bin",
    [EnvironmentVariableTarget]::User
)
```

### Windows (CMD)

```cmd
:: Create directory
mkdir "%USERPROFILE%\.local\bin"

:: Move binary
move target\release\cmd.exe "%USERPROFILE%\.local\bin\"

:: Add to PATH permanently (requires restart or new terminal)
setx PATH "%PATH%;%USERPROFILE%\.local\bin"
```

## Verify Installation

```bash
cmd --version
cmd --help
```

## Updating

### macOS / Linux

```bash
cd cmd
git pull
cargo build --release
mv target/release/cmd ~/.local/bin/
```

### Windows

```powershell
cd cmd
git pull
cargo build --release
Move-Item -Force -Path "target\release\cmd.exe" -Destination "$env:USERPROFILE\.local\bin\"
```
