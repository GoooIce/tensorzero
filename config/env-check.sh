#!/bin/bash

# TensorZero Environment Variables Check Script
# This script helps verify that all required environment variables are set

echo "🔍 Checking TensorZero Environment Variables..."
echo "================================================"

# Function to check if environment variable is set
check_env_var() {
    local var_name=$1
    local is_required=$2
    
    if [ -n "${!var_name}" ]; then
        echo "✅ $var_name: SET"
        # Don't print actual values for security
    else
        if [ "$is_required" = "true" ]; then
            echo "❌ $var_name: NOT SET (REQUIRED)"
            return 1
        else
            echo "⚠️  $var_name: NOT SET (OPTIONAL)"
        fi
    fi
    return 0
}

# Check DevV (RustProxy) variables
echo ""
echo "📡 DevV (RustProxy) Configuration:"
check_env_var "API_ENDPOINT" true
check_env_var "DEVICE_ID" true  
check_env_var "SID" true

# Check AI Provider API Keys
echo ""
echo "🤖 AI Provider API Keys:"
check_env_var "DEEPSEEK_API_KEY" false
check_env_var "OPENROUTER_API_KEY" false
check_env_var "OPENAI_API_KEY" false

echo ""
echo "================================================"

# Check if at least one AI provider is configured
if [ -n "$DEEPSEEK_API_KEY" ] || [ -n "$OPENROUTER_API_KEY" ] || [ -n "$OPENAI_API_KEY" ]; then
    echo "✅ At least one AI provider is configured"
else
    echo "❌ No AI provider API keys are set!"
    echo "   Please set at least one of: DEEPSEEK_API_KEY, OPENROUTER_API_KEY, or OPENAI_API_KEY"
fi

# Check if all required DevV variables are set
if [ -n "$API_ENDPOINT" ] && [ -n "$DEVICE_ID" ] && [ -n "$SID" ]; then
    echo "✅ DevV (RustProxy) configuration is complete"
    echo ""
    echo "🚀 You can now run: docker-compose up -d"
else
    echo "❌ DevV (RustProxy) configuration is incomplete!"
    echo "   Please set: API_ENDPOINT, DEVICE_ID, and SID"
    echo ""
    echo "💡 Example:"
    echo "   export API_ENDPOINT=\"https://api.devv.ai/api/v1/stream/chat\""
    echo "   export DEVICE_ID=\"your-device-id\""
    echo "   export SID=\"your-session-id\""
fi

echo "" 