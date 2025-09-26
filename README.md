# 🦥 Vibe CLI Commands

> Sometimes we know what we want but just forgot the command

A natural language CLI tool that translates your intentions into terminal commands, powered by Simon Willison's [llm](https://github.com/simonw/llm).

## Why Vibe CLI?

Ever found yourself thinking "I want to find all files larger than 100MB" but can't remember the exact `find` syntax? Vibe CLI bridges that gap by letting you describe what you want in plain language.

## Prerequisites

- **uv** - Python package installer
- **ollama** (optional) - For local LLM hosting
- **Rust/Cargo** - For building the CLI tool

## Quick Start

### 1. Install and Configure LLM

```bash
# Install the llm tool
uv tool install llm

# Option A: Use local models with Ollama
llm install llm-ollama
ollama pull qwen2.5-coder
llm models default qwen2.5-coder:latest

# Option B: Use Anthropic's Claude (requires API key)
llm install llm-anthropic
llm keys set anthropic
llm models default claude-3-5-sonnet-latest
```

### 2. Create Command Template

```bash
# Save a
