# RustProxy 配置详细指南

RustProxy 是 TensorZero 的内置代理提供商，它实际上是一个开发者 API 客户端的代理。根据代码分析，RustProxy 需要以下额外配置：

## 🔧 必需的环境变量

RustProxy 依赖于 `DevApiClient`，需要以下环境变量：

### 1. API 端点配置
```bash
export API_ENDPOINT="https://your-dev-api-endpoint.com"
```
- **说明**: 指向实际的开发者 API 服务端点
- **默认值**: `"https://xxx"` (需要替换为真实端点)

### 2. 设备 ID
```bash
export DEVICE_ID="your-device-identifier"
```
- **说明**: 设备唯一标识符，用于 API 认证
- **默认值**: `"xxxx"` (需要替换为真实设备 ID)

### 3. 操作系统类型
```bash
export OS_TYPE="3"
```
- **说明**: 操作系统类型标识符
- **默认值**: `"3"`

### 4. 会话 ID
```bash
export SID="your-session-id"
```
- **说明**: 会话标识符
- **默认值**: `"sid"` (需要替换为真实会话 ID)

## 📝 Docker Compose 配置

创建一个使用 RustProxy 的 `docker-compose-rustproxy.yml`：

```yaml
# This is a simplified example for learning purposes. Do not use this in production.
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
      # RustProxy 所需的环境变量
      API_ENDPOINT: ${API_ENDPOINT:?Environment variable API_ENDPOINT must be set.}
      DEVICE_ID: ${DEVICE_ID:?Environment variable DEVICE_ID must be set.}
      OS_TYPE: ${OS_TYPE:-3}
      SID: ${SID:?Environment variable SID must be set.}
    ports:
      - "3000:3000"
    extra_hosts:
      - "host.docker.internal:host-gateway"
    healthcheck:
      test:
        [
          "CMD",
          "wget",
          "--no-verbose",
          "--tries=1",
          "--spider",
          "http://localhost:3000/health",
        ]
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
      # UI 也需要相同的环境变量
      API_ENDPOINT: ${API_ENDPOINT:?Environment variable API_ENDPOINT must be set.}
      DEVICE_ID: ${DEVICE_ID:?Environment variable DEVICE_ID must be set.}
      OS_TYPE: ${OS_TYPE:-3}
      SID: ${SID:?Environment variable SID must be set.}
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

## 🔐 WASM 签名器依赖

RustProxy 使用 WASM 签名器进行请求认证：

### 1. WASM 文件
- **位置**: `rust_proxy/sign_bg.wasm`
- **作用**: 生成请求签名，用于 API 认证

### 2. 签名参数
RustProxy 会自动生成以下参数用于签名：
- `nonce`: UUID v4 格式的随机数
- `timestamp`: Unix 时间戳
- `device_id`: 来自环境变量
- `content`: 请求内容

## 📊 功能特性

### ✅ 支持的功能
- **非流式推理**: 完全支持
- **消息转换**: 自动将 TensorZero 消息格式转换为开发者 API 格式
- **错误处理**: 完整的错误处理和日志记录

### ❌ 暂不支持的功能
- **流式推理**: 由于 Rust 生命周期复杂性暂时禁用（基础设施已就绪）
- **批量推理**: 不支持批量推理操作

## 🚀 运行步骤

1. **设置环境变量**:
```bash
export API_ENDPOINT="你的真实API端点"
export DEVICE_ID="你的设备ID"
export OS_TYPE="3"
export SID="你的会话ID"
```

2. **启动服务**:
```bash
cd tensorzero/examples/quickstart
docker-compose -f docker-compose-rustproxy.yml up -d
```

3. **验证服务**:
```bash
# 检查日志
docker-compose -f docker-compose-rustproxy.yml logs -f

# 测试 API
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "rust-proxy",
    "messages": [{"role": "user", "content": "Hello, world!"}]
  }'
```

## ⚠️ 注意事项

1. **API 端点**: 必须提供真实可用的开发者 API 端点
2. **认证信息**: 设备 ID 和会话 ID 必须是有效的认证凭据
3. **网络连接**: 确保容器可以访问配置的 API 端点
4. **WASM 依赖**: 确保 `sign_bg.wasm` 文件存在且可访问

## 🔍 故障排除

### 常见错误
1. **"Failed to create DevApiClient"**: 检查环境变量是否正确设置
2. **"Request failed"**: 验证 API 端点是否可访问和认证信息是否有效
3. **"Failed to get WasmSigner instance"**: 确保 WASM 文件存在且加载正常

### 日志查看
```bash
# 查看 gateway 日志
docker-compose -f docker-compose-rustproxy.yml logs gateway

# 查看详细错误信息
docker-compose -f docker-compose-rustproxy.yml logs --tail=50 gateway
```

RustProxy 主要用于特定的开发者 API 集成场景，需要对应的后端服务支持。如果你没有相应的开发者 API 服务，建议使用 DeepSeek 或 OpenRouter 等更通用的提供商。 