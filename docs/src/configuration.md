# Configuration

## Interactive Setup

```bash
cmd setup
```

This will:
1. Let you choose your LLM provider
2. Guide you through entering your API key (hidden input)
3. Store your credentials securely in the system keychain

## Credential Storage

API keys are stored securely in your system's native credential store:

| Platform | Storage |
|----------|---------|
| macOS | Keychain Access |
| Linux | Secret Service (GNOME Keyring, KWallet) |
| Fallback | Encrypted file (`~/.config/cmd/credentials.enc`) |

### Managing Credentials

```bash
# View stored API keys (masked)
cmd config --show-keys

# Delete a stored key
cmd config --delete-key anthropic
cmd config --delete-key openai
cmd config --delete-key ollama_host
```

### Priority Order

When loading credentials, `cmd` checks in this order:

1. **Environment variables** (allows temporary overrides)
2. **System keychain** (primary storage)
3. **Encrypted file** (fallback for headless environments)

This means you can override keychain credentials with environment variables for CI or temporary use.

## Execution Settings

Configure how `cmd` handles command execution:

```bash
# View current settings
cmd config --show

# Enable execution mode (still prompts for confirmation)
cmd config --enable-execution

# Skip confirmation prompts (use with caution)
cmd config --skip-confirmation

# Reset to safe defaults
cmd config --disable-execution --require-confirmation
```

Settings are stored in `~/.config/cmd/settings.toml`:

```toml
enable_execution = false
skip_confirmation = false
```

## Environment Variables

You can also configure providers via environment variables:

### Anthropic (Claude)

```bash
export ANTHROPIC_API_KEY=sk-ant-api03-...
```

Get your API key at [console.anthropic.com/settings/keys](https://console.anthropic.com/settings/keys)

**Default model:** `claude-sonnet-4-20250514`

### OpenAI

```bash
export OPENAI_API_KEY=sk-proj-...
```

Get your API key at [platform.openai.com/api-keys](https://platform.openai.com/api-keys)

**Default model:** `gpt-4o`

### Ollama (Local)

Ollama runs models locally - no API key needed.

```bash
# Install Ollama
brew install ollama

# Pull a model
ollama pull qwen2.5-coder

# Set the host
export OLLAMA_HOST=http://localhost:11434
```

**Default model:** `qwen2.5-coder`

## Provider Priority

If multiple providers are configured, `cmd` uses this priority:

1. Anthropic
2. OpenAI
3. Ollama

## Overriding Defaults

Use CLI flags to override the default model or endpoint:

```bash
# Use a different model
cmd -m claude-3-haiku "list files"

# Use a custom endpoint
cmd -e https://my-proxy.com/v1/messages "list files"
```

## Azure OpenAI

For Azure-hosted OpenAI, use the setup wizard:

```bash
cmd setup
# Select "Azure OpenAI"
# Enter your API key, resource name, and deployment name
```

Or manually:

```bash
export OPENAI_API_KEY=your-azure-api-key

cmd -e "https://your-resource.openai.azure.com/openai/deployments/gpt-4/chat/completions?api-version=2024-02-15-preview" \
    -m gpt-4 \
    "list files"
```

See [Providers](./providers.md#azure-openai) for more details.
