# Configuration

## Interactive Setup

```bash
cmd setup
```

This will:
1. Let you choose your LLM provider
2. Guide you through entering your API key
3. Optionally add the configuration to your shell profile

## Manual Configuration

Set **one** of the following:

## Anthropic (Claude)

```bash
export ANTHROPIC_API_KEY=sk-ant-api03-...
```

Get your API key at [console.anthropic.com/settings/keys](https://console.anthropic.com/settings/keys)

**Default model:** `claude-sonnet-4-20250514`

## OpenAI

```bash
export OPENAI_API_KEY=sk-proj-...
```

Get your API key at [platform.openai.com/api-keys](https://platform.openai.com/api-keys)

**Default model:** `gpt-4o`

## Ollama (Local)

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

If multiple providers are configured, Vibe CLI uses this priority:

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

For Azure-hosted OpenAI, use the OpenAI API key with a custom endpoint:

```bash
export OPENAI_API_KEY=your-azure-api-key

cmd -e "https://your-resource.openai.azure.com/openai/deployments/gpt-4/chat/completions?api-version=2024-02-15-preview" \
    -m gpt-4 \
    "list files"
```

See [Providers](./providers.md#azure-openai) for more details.

## Shell Configuration

Add your API key to your shell profile for persistence:

```bash
# ~/.bashrc or ~/.zshrc
export ANTHROPIC_API_KEY=sk-ant-...
```

Or use a secrets manager:

```bash
# With 1Password CLI
export ANTHROPIC_API_KEY=$(op read op://API/ClaudeKey/password)
```
