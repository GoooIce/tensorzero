# RustProxy 配置详细指南

RustProxy 是 TensorZero 的内置代理提供商，集成了 DevV AI 服务，支持动态模型发现、智能过滤和完整的配置管理。

## 🚀 新版本配置（推荐）

### TensorZero 原生配置

现在可以直接在 TensorZero 配置文件中完整配置 RustProxy，无需依赖环境变量：

```toml
[models.rust-proxy.providers.rust_proxy_provider]
type = "rust-proxy"
model_name = "claude-3.5-sonnet"

# API 配置（可选 - 未指定时使用默认值）
api_endpoint = "https://api.devv.ai/api/v1/stream/chat"
device_id = "your-device-id"
session_id = "your-session-id"
os_type = "3"
accept_language = "en"

# 模型过滤选项（可选）
[models.rust-proxy.providers.rust_proxy_provider.model_filter]
include_types = ["base", "freeTrial"]  # 只包含免费模型
exclude_types = ["premium"]            # 排除付费模型
min_usage_left = 10                    # 最少使用次数
only_new = false                       # 是否只显示新模型
```

### 🎯 配置选项详解

#### 必需配置
- **`model_name`**: 要使用的模型名称，支持友好名称映射：
  - `"claude-3.5-sonnet"` → `"us.anthropic.claude-3-7-sonnet-20250219-v1:0"`
  - `"gpt-4.1"` → `"gpt-4.1"`
  - `"gemini-2.0-flash"` → `"gemini-2.0-flash-001"`
  - 或直接使用 DevV API 模型标识符

#### API 配置（可选）
- **`api_endpoint`**: API 端点 URL（默认：DevV API 端点）
- **`device_id`**: 设备标识符（默认：auto-generated）
- **`session_id`**: 会话标识符（默认：auto-generated）
- **`os_type`**: 操作系统类型（默认：`"3"`）
- **`accept_language`**: 语言偏好（默认：`"en"`）

#### 模型过滤器（可选）
- **`include_types`**: 包含的模型类型列表 `["base", "freeTrial", "premium"]`
- **`exclude_types`**: 排除的模型类型列表
- **`min_usage_left`**: 最小剩余使用次数
- **`only_new`**: 是否只显示新模型（`true`/`false`）

## 🔧 高级功能

### 1. 动态模型发现
RustProxy 会自动从 DevV API 获取可用模型列表：
- 实时模型信息
- 使用限制检查
- 智能缓存机制

### 2. 模型验证
- 推理前自动验证模型可用性
- 基于模型类型的差异化验证
- 缓存验证结果提升性能

### 3. 智能模型映射
- 友好的 TensorZero 模型名称
- 自动映射到 DevV API 标识符
- 支持多种模型别名

## 📝 Docker Compose 配置

### 新版本配置（推荐）

```yaml
services:
  clickhouse:
    image: clickhouse/clickhouse-server:24.12-alpine
    environment:
      CLICKHOUSE_USER: chuser
      CLICKHOUSE_DEFAULT_ACCESS_MANAGEMENT: 1
      CLICKHOUSE_PASSWORD: chpassword
    ports:
      - "8123:8123"
    healthcheck:
      test: wget --spider --tries 1 http://chuser:chpassword@clickhouse:8123/ping
      start_period: 30s
      start_interval: 1s
      timeout: 1s

  gateway:
    image: tensorzero/gateway
    volumes:
      - ./config:/app/config:ro
    command: --config-file /app/config/tensorzero-rustproxy.toml
    environment:
      TENSORZERO_CLICKHOUSE_URL: http://chuser:chpassword@clickhouse:8123/tensorzero
      # 无需额外的 RustProxy 环境变量 - 全部在配置文件中
    ports:
      - "3000:3000"
    extra_hosts:
      - "host.docker.internal:host-gateway"
    healthcheck:
      test: ["CMD", "wget", "--no-verbose", "--tries=1", "--spider", "http://localhost:3000/health"]
      start_period: 1s
      start_interval: 1s
      timeout: 1s
    depends_on:
      clickhouse:
        condition: service_healthy

  ui:
    image: tensorzero/ui
    volumes:
      - ./config:/app/config:ro
    environment:
      TENSORZERO_CLICKHOUSE_URL: http://chuser:chpassword@clickhouse:8123/tensorzero
      TENSORZERO_GATEWAY_URL: http://gateway:3000
    ports:
      - "4000:4000"
    depends_on:
      clickhouse:
        condition: service_healthy
      gateway:
        condition: service_healthy
```

## 🚀 运行步骤

### 1. 配置文件设置
更新 `config/tensorzero-rustproxy.toml` 中的认证信息：

```toml
[models.rust-proxy.providers.rust_proxy_provider]
type = "rust-proxy"
model_name = "claude-3.5-sonnet"
device_id = "你的真实设备ID"
session_id = "你的真实会话ID"
```

### 2. 启动服务
```bash
cd tensorzero/examples/quickstart
docker-compose -f docker-compose-rustproxy.yml up -d
```

### 3. 验证服务
```bash
# 检查日志
docker-compose -f docker-compose-rustproxy.yml logs -f gateway

# 测试 API
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "rust-proxy",
    "messages": [{"role": "user", "content": "生成一首俳句"}]
  }'
```

## 📊 功能特性

### ✅ 完全支持的功能
- **非流式推理**: 完全支持，包含错误处理
- **动态模型发现**: 自动获取可用模型列表
- **模型验证**: 推理前验证模型可用性
- **智能过滤**: 基于类型、使用限制等过滤模型
- **消息转换**: 自动格式转换
- **配置管理**: 原生 TensorZero 配置支持

### 🚧 开发中的功能
- **流式推理**: 基础设施就绪，正在完善中
- **批量推理**: 计划中的功能

## 💡 配置示例

### 基础配置
```toml
[models.rust-proxy.providers.rust_proxy_provider]
type = "rust-proxy"
model_name = "claude-3.5-sonnet"
device_id = "device-123"
session_id = "session-456"
```

### 高级配置（包含过滤）
```toml
[models.rust-proxy.providers.rust_proxy_provider]
type = "rust-proxy"
model_name = "claude-3.5-sonnet"
api_endpoint = "https://api.devv.ai/api/v1/stream/chat"
device_id = "device-123"
session_id = "session-456"
os_type = "3"
accept_language = "zh-CN"

[models.rust-proxy.providers.rust_proxy_provider.model_filter]
include_types = ["base", "freeTrial"]
min_usage_left = 5
only_new = false
```

## 🔄 向后兼容性

### 环境变量支持（已弃用，但仍可用）

如果需要使用旧的环境变量方式，仍然支持：

```bash
export API_ENDPOINT="https://api.devv.ai/api/v1/stream/chat"
export DEVICE_ID="your-device-identifier"
export OS_TYPE="3"
export SID="your-session-id"
```

**注意**: 推荐使用新的 TensorZero 配置方式，环境变量方式将在未来版本中移除。

## ⚠️ 重要说明

1. **认证信息**: `device_id` 和 `session_id` 必须是有效的 DevV AI 认证凭据
2. **网络连接**: 确保可以访问 DevV AI API 端点
3. **模型可用性**: 会自动验证模型可用性和使用限制
4. **缓存机制**: 模型信息会被缓存以提升性能

## 🔍 故障排除

### 常见错误
1. **"Failed to create DevApiClient"**: 检查配置文件中的认证信息
2. **"Model not available"**: 模型可能无使用次数或不存在
3. **"Request failed"**: 验证网络连接和 API 端点

### 调试方法
```bash
# 查看详细日志
docker-compose -f docker-compose-rustproxy.yml logs --tail=50 gateway

# 检查模型发现
# 模型信息会在日志中显示
```

## 🎯 最佳实践

1. **使用友好的模型名称**: 如 `"claude-3.5-sonnet"` 而不是完整的 API 标识符
2. **配置模型过滤**: 使用 `model_filter` 只显示相关模型
3. **监控使用限制**: 定期检查模型使用次数
4. **缓存管理**: 系统会自动管理缓存，无需手动干预

RustProxy 现在提供了完整的 TensorZero 原生配置体验，同时保持了强大的 DevV AI 服务集成能力！ 