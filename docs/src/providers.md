# Providers

Vibe CLI supports multiple LLM providers through a unified interface.

## Supported Providers

| Provider | API Format | Auth | Local |
|----------|-----------|------|-------|
| Anthropic | Messages API | API Key | No |
| OpenAI | Chat Completions | API Key | No |
| Ollama | OpenAI-compatible | None | Yes |

## Anthropic

Uses Claude models via the Messages API.

**Endpoint:** `https://api.anthropic.com/v1/messages`

**Authentication:** `x-api-key` header

**Default model:** `claude-sonnet-4-20250514`

**Available models:**
- `claude-sonnet-4-20250514` (default, balanced)
- `claude-3-haiku-20240307` (fast, cheap)
- `claude-3-opus-20240229` (most capable)

## OpenAI

Uses GPT models via the Chat Completions API.

**Endpoint:** `https://api.openai.com/v1/chat/completions`

**Authentication:** `Authorization: Bearer` header

**Default model:** `gpt-4o`

**Available models:**
- `gpt-4o` (default)
- `gpt-4o-mini` (fast, cheap)
- `gpt-4-turbo`

## Ollama

Runs models locally using Ollama's OpenAI-compatible API.

**Endpoint:** `http://localhost:11434/v1/chat/completions`

**Authentication:** None required

**Default model:** `qwen2.5-coder`

**Setup:**
```bash
# Install Ollama
brew install ollama

# Pull a coding model
ollama pull qwen2.5-coder
ollama pull codellama
ollama pull deepseek-coder

# Start server (if not running)
ollama serve
```

## Azure OpenAI

Azure OpenAI uses the OpenAI-compatible format with a custom endpoint.

**Endpoint format:** `https://<resource-name>.openai.azure.com/openai/deployments/<deployment-name>/chat/completions?api-version=<version>`

```bash
# Set your Azure API key
export OPENAI_API_KEY=your-azure-api-key

# Use with custom endpoint
cmd -e "https://myresource.openai.azure.com/openai/deployments/gpt-4/chat/completions?api-version=2024-02-15-preview" \
    -m gpt-4 \
    "list files"
```

## Custom Endpoints

Use the `-e` flag to specify a custom endpoint:

```bash
# Use a proxy
cmd -e https://my-anthropic-proxy.com/v1/messages "list files"

# Use a different Ollama host
cmd -e http://192.168.1.100:11434/v1/chat/completions "list files"

# Use a self-hosted vLLM server
cmd -e http://localhost:8000/v1/chat/completions -m my-model "list files"

# Use LM Studio
cmd -e http://localhost:1234/v1/chat/completions "list files"
```

## Adding New Providers

Most providers are OpenAI-compatible. To use them:

1. Set the appropriate API key (uses `OPENAI_API_KEY`)
2. Use `-e` to specify their endpoint
3. Use `-m` to specify the model name

```bash
# Groq (OpenAI-compatible, fast inference)
export OPENAI_API_KEY=gsk_...
cmd -e https://api.groq.com/openai/v1/chat/completions -m llama-3.1-70b-versatile "list files"

# Together AI
export OPENAI_API_KEY=...
cmd -e https://api.together.xyz/v1/chat/completions -m mistralai/Mixtral-8x7B-Instruct-v0.1 "list files"

# Fireworks AI
export OPENAI_API_KEY=...
cmd -e https://api.fireworks.ai/inference/v1/chat/completions -m accounts/fireworks/models/llama-v3-70b-instruct "list files"

# OpenRouter (access multiple providers)
export OPENAI_API_KEY=sk-or-...
cmd -e https://openrouter.ai/api/v1/chat/completions -m anthropic/claude-3-sonnet "list files"

# Deepseek
export OPENAI_API_KEY=...
cmd -e https://api.deepseek.com/v1/chat/completions -m deepseek-coder "list files"
```

## Provider Comparison

| Provider | Latency | Cost | Local | Best For |
|----------|---------|------|-------|----------|
| Anthropic | Medium | $$ | No | Accuracy |
| OpenAI | Medium | $$ | No | General use |
| Azure OpenAI | Medium | $$ | No | Enterprise |
| Groq | Fast | $ | No | Speed |
| Ollama | Varies | Free | Yes | Privacy |
| Together | Fast | $ | No | Open models |
