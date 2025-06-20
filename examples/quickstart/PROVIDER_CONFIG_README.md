# TensorZero Quickstart - 提供商配置指南

这个文档说明如何配置 TensorZero Quickstart 示例使用不同的 AI 提供商。

## 配置选项

### 1. 使用 DeepSeek (推荐)

DeepSeek 提供高性价比的中文友好模型。

**配置文件**: 已修改的 `config/tensorzero.toml`
**模型**: `deepseek::deepseek-chat`

**运行步骤**:
```bash
# 设置 API 密钥
export DEEPSEEK_API_KEY="你的_deepseek_api_密钥"

# 启动服务
docker-compose up -d

# 查看日志
docker-compose logs -f
```

### 2. 使用 OpenRouter

OpenRouter 提供多种模型的统一接口。

**配置文件**: `config/tensorzero-openrouter.toml`
**模型**: `openrouter::openai/gpt-4o-mini`

**运行步骤**:
```bash
# 设置 API 密钥
export OPENROUTER_API_KEY="你的_openrouter_api_密钥"

# 使用 OpenRouter 配置启动
docker-compose -f docker-compose-openrouter.yml up -d
```

### 3. 使用 RustProxy

RustProxy 是 TensorZero 的内置代理服务，用于集成特定的开发者 API。

**配置文件**: `config/tensorzero-rustproxy.toml`
**Docker Compose**: `docker-compose-rustproxy.yml`

**必需的环境变量**:
```bash
export API_ENDPOINT="你的开发者API端点"
export DEVICE_ID="你的设备ID"
export OS_TYPE="3"
export SID="你的会话ID"
```

**运行步骤**:
```bash
cd tensorzero/examples/quickstart
# 设置必需的环境变量（见上方）
docker-compose -f docker-compose-rustproxy.yml up -d
```

**重要说明**: 
- RustProxy 需要真实可用的开发者 API 端点和有效的认证凭据
- 依赖 WASM 签名器进行请求认证
- 仅支持非流式推理（流式推理暂时禁用）
- 详细配置指南请参考：`RUSTPROXY_CONFIG_README.md`

## 测试连接

启动服务后，可以通过以下方式测试：

### 使用 Python 客户端
```python
# 检查 after.py, after_async.py 或 after_openai.py 文件中的示例代码
python after.py
```

### 使用 Web UI
访问 http://localhost:4000 查看 TensorZero UI

### 使用 API
```bash
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "generate_haiku",
    "messages": [{"role": "user", "content": "写一首关于春天的俳句"}]
  }'
```

## 可用的模型

根据 `model.rs` 中的定义，TensorZero 支持以下提供商：

- **DeepSeek**: `deepseek::deepseek-chat`, `deepseek::deepseek-coder`
- **OpenRouter**: `openrouter::<模型名称>` (支持多种模型)
- **Anthropic**: `anthropic::claude-3-haiku`, `anthropic::claude-3-sonnet`
- **Google**: `google_ai_studio_gemini::gemini-pro`
- **Groq**: `groq::llama-3.1-70b-versatile`
- **Mistral**: `mistral::mistral-small-latest`
- **XAI**: `xai::grok-beta`
- **Together**: `together::meta-llama/Llama-3-8b-chat-hf`
- **Hyperbolic**: `hyperbolic::<模型名称>`

## 故障排除

1. **API 密钥错误**: 确保环境变量设置正确
2. **网络连接**: 检查防火墙和网络设置
3. **Docker 问题**: 确保 Docker 和 Docker Compose 已正确安装
4. **端口冲突**: 确保端口 3000、4000、8123 未被占用

## 环境变量参考

- `DEEPSEEK_API_KEY`: DeepSeek API 密钥
- `OPENROUTER_API_KEY`: OpenRouter API 密钥
- `ANTHROPIC_API_KEY`: Anthropic API 密钥
- `GROQ_API_KEY`: Groq API 密钥
- `MISTRAL_API_KEY`: Mistral API 密钥
- `XAI_API_KEY`: XAI API 密钥

根据你选择的提供商设置相应的环境变量即可。 