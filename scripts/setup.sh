#!/bin/bash
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
DIM='\033[2m'
NC='\033[0m'
BOLD='\033[1m'

echo -e "\n${BOLD}Vibe CLI Setup${NC}\n"

detect_shell_config() {
    if [[ -n "${ZSH_VERSION:-}" ]]; then
        echo "$HOME/.zshrc"
    elif [[ -n "${BASH_VERSION:-}" ]]; then
        [[ -f "$HOME/.bashrc" ]] && echo "$HOME/.bashrc" || echo "$HOME/.bash_profile"
    else
        echo "$HOME/.profile"
    fi
}

SHELL_CONFIG=$(detect_shell_config)

echo -e "${CYAN}Select your LLM provider:${NC}\n"
echo "  1) Claude (Anthropic)     ${DIM}— key only${NC}"
echo "  2) OpenAI                 ${DIM}— key only${NC}"
echo "  3) Ollama (local)         ${DIM}— no key needed${NC}"
echo "  4) Azure OpenAI           ${DIM}— key + endpoint${NC}"
echo "  5) Groq                   ${DIM}— key + endpoint${NC}"
echo "  6) Other (custom)         ${DIM}— key + endpoint${NC}"
echo ""
read -p "Enter choice [1-6]: " provider_choice

case $provider_choice in
    1)
        PROVIDER="anthropic"
        echo -e "\n${YELLOW}Get your API key at:${NC} https://console.anthropic.com/settings/keys\n"
        read -p "Enter your Anthropic API key: " api_key
        EXPORT_LINE="export ANTHROPIC_API_KEY=\"$api_key\""
        USAGE_TIP="cmd \"list files\""
        ;;
    2)
        PROVIDER="openai"
        echo -e "\n${YELLOW}Get your API key at:${NC} https://platform.openai.com/api-keys\n"
        read -p "Enter your OpenAI API key: " api_key
        EXPORT_LINE="export OPENAI_API_KEY=\"$api_key\""
        USAGE_TIP="cmd \"list files\""
        ;;
    3)
        PROVIDER="ollama"
        echo -e "\n${YELLOW}Ollama Setup${NC}\n"
        
        if command -v ollama &> /dev/null; then
            echo -e "${GREEN}Ollama is installed${NC}\n"
        else
            echo -e "${RED}Ollama not found.${NC} Install with: brew install ollama\n"
        fi
        
        read -p "Enter Ollama host [http://localhost:11434]: " ollama_host
        ollama_host=${ollama_host:-http://localhost:11434}
        EXPORT_LINE="export OLLAMA_HOST=\"$ollama_host\""
        USAGE_TIP="cmd \"list files\""
        
        echo -e "\n${CYAN}Recommended models:${NC}"
        echo "  ollama pull qwen2.5-coder"
        echo "  ollama pull codellama"
        ;;
    4)
        PROVIDER="azure"
        echo -e "\n${YELLOW}Azure OpenAI Setup${NC}"
        echo -e "${DIM}Requires both API key and endpoint${NC}\n"
        
        read -p "Enter your Azure API key: " api_key
        read -p "Enter your Azure resource name: " resource_name
        read -p "Enter your deployment name: " deployment_name
        read -p "Enter API version [2024-02-15-preview]: " api_version
        api_version=${api_version:-2024-02-15-preview}
        
        ENDPOINT="https://${resource_name}.openai.azure.com/openai/deployments/${deployment_name}/chat/completions?api-version=${api_version}"
        EXPORT_LINE="export OPENAI_API_KEY=\"$api_key\"
export CMD_ENDPOINT=\"$ENDPOINT\""
        USAGE_TIP="cmd -e \"\$CMD_ENDPOINT\" \"list files\""
        ;;
    5)
        PROVIDER="groq"
        echo -e "\n${YELLOW}Groq Setup${NC}"
        echo -e "${DIM}Requires both API key and endpoint${NC}\n"
        echo -e "Get your API key at: https://console.groq.com/keys\n"
        
        read -p "Enter your Groq API key: " api_key
        EXPORT_LINE="export OPENAI_API_KEY=\"$api_key\"
export CMD_ENDPOINT=\"https://api.groq.com/openai/v1/chat/completions\""
        USAGE_TIP="cmd -e \"\$CMD_ENDPOINT\" -m llama-3.1-70b-versatile \"list files\""
        ;;
    6)
        PROVIDER="custom"
        echo -e "\n${YELLOW}Custom Provider Setup${NC}"
        echo -e "${DIM}Requires both API key and endpoint${NC}\n"
        
        read -p "Enter your API key (or leave empty if none): " api_key
        read -p "Enter the API endpoint URL: " endpoint
        
        if [[ -n "$api_key" ]]; then
            EXPORT_LINE="export OPENAI_API_KEY=\"$api_key\"
export CMD_ENDPOINT=\"$endpoint\""
        else
            EXPORT_LINE="export CMD_ENDPOINT=\"$endpoint\""
        fi
        
        read -p "Enter model name (optional): " model_name
        if [[ -n "$model_name" ]]; then
            USAGE_TIP="cmd -e \"\$CMD_ENDPOINT\" -m $model_name \"list files\""
        else
            USAGE_TIP="cmd -e \"\$CMD_ENDPOINT\" \"list files\""
        fi
        ;;
    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

echo ""
echo -e "${YELLOW}Add to ${SHELL_CONFIG}:${NC}\n"
echo -e "${GREEN}$EXPORT_LINE${NC}\n"

read -p "Add automatically? [y/N]: " add_to_config

if [[ "$add_to_config" =~ ^[Yy]$ ]]; then
    echo "" >> "$SHELL_CONFIG"
    echo "# Vibe CLI ($PROVIDER)" >> "$SHELL_CONFIG"
    echo "$EXPORT_LINE" >> "$SHELL_CONFIG"
    echo -e "\n${GREEN}Added to $SHELL_CONFIG${NC}"
    echo -e "Run: ${CYAN}source $SHELL_CONFIG${NC}\n"
else
    echo -e "\nAdd manually to your shell config.\n"
fi

echo -e "${GREEN}Setup complete!${NC}"
echo -e "Test with: ${CYAN}$USAGE_TIP${NC}\n"
