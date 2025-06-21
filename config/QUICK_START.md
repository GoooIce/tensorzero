# TensorZero 快速启动指南

## 环境变量配置

你已经设置了以下环境变量，这很好！

### DevV (RustProxy) 配置
```bash
export API_ENDPOINT="你的API端点"
export DEVICE_ID="你的设备ID"  
export SID="你的会话ID"
```

### AI 提供商 API 密钥
```bash
export DEEPSEEK_API_KEY="你的DeepSeek API密钥"
export OPENROUTER_API_KEY="你的OpenRouter API密钥"
```

## 启动步骤

### 1. 检查环境变量
```bash
# 运行环境变量检查脚本
chmod +x config/env-check.sh
./config/env-check.sh
```

### 2. 启动服务
```bash
# 启动所有服务
docker-compose up -d

# 查看日志
docker-compose logs -f
```

### 3. 验证服务
```bash
# 检查服务状态
docker-compose ps

# 测试网关
curl http://localhost:3000/health

# 访问UI界面
open http://localhost:4000
```

## 可用的模型

### DevV (RustProxy) 模型
- `claude-3-5-sonnet` - Claude 3.5 Sonnet (通过DevV)
- `claude-3-5-haiku` - Claude 3.5 Haiku (通过DevV)
- `gpt-4o-mini` - GPT-4o Mini (通过DevV)

### DeepSeek 模型
- `deepseek-chat` - DeepSeek Chat
- `deepseek-coder` - DeepSeek Coder

### OpenRouter 模型
- `openrouter-claude` - Claude 3.5 Sonnet (通过OpenRouter)
- `openrouter-gpt4` - GPT-4o (通过OpenRouter)

### OpenAI 模型 (可选)
- `gpt-4o` - GPT-4o (直接通过OpenAI)

## 测试API调用

### 使用curl测试
```bash
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "claude-3-5-sonnet",
    "messages": [
      {"role": "user", "content": "Hello, how are you?"}
    ]
  }'
```

### 使用Python测试
```python
import openai

client = openai.OpenAI(
    base_url="http://localhost:3000/v1",
    api_key="dummy"  # TensorZero不需要API密钥
)

response = client.chat.completions.create(
    model="claude-3-5-sonnet",
    messages=[
        {"role": "user", "content": "Hello, how are you?"}
    ]
)

print(response.choices[0].message.content)
```

## 故障排除

### 常见问题

1. **环境变量未设置**
   - 运行 `./config/env-check.sh` 检查
   - 确保在启动Docker之前设置了所有变量

2. **DevV API调用失败**
   - 检查 `API_ENDPOINT` 是否正确
   - 验证 `DEVICE_ID` 和 `SID` 是否有效
   - 查看gateway日志: `docker-compose logs gateway`

3. **模型不可用**
   - 检查对应的API密钥是否设置正确
   - 查看特定提供商的API配额和限制

4. **Docker服务启动失败**
   - 检查端口是否被占用 (3000, 4000, 8123)
   - 确保Docker有足够的资源

### 查看日志
```bash
# 查看所有服务日志
docker-compose logs

# 查看特定服务日志
docker-compose logs gateway
docker-compose logs ui
docker-compose logs clickhouse
```

## 配置文件说明

- `tensorzero.toml` - 主配置文件，定义模型和提供商
- `docker-compose.yml` - Docker服务配置
- `env-check.sh` - 环境变量检查脚本

## 下一步

1. 访问UI界面: http://localhost:4000
2. 测试不同的模型
3. 查看使用统计和日志
4. 根据需要调整配置 