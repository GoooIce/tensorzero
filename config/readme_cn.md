# TensorZero 配置文件说明文档

本文档详细介绍了 TensorZero 配置文件 `tensorzero.toml` 的所有配置选项和使用方法。

## 目录

- [概览](#概览)
- [文件结构](#文件结构)
- [Gateway 网关配置](#gateway-网关配置)
- [Models 模型配置](#models-模型配置)
- [Embedding Models 嵌入模型配置](#embedding-models-嵌入模型配置)
- [Functions 函数配置](#functions-函数配置)
- [Tools 工具配置](#tools-工具配置)
- [Metrics 指标配置](#metrics-指标配置)
- [Evaluations 评估配置](#evaluations-评估配置)
- [环境变量配置](#环境变量配置)
- [完整示例](#完整示例)
- [常见问题](#常见问题)

## 概览

`tensorzero.toml` 是 TensorZero 网关的核心配置文件，采用 TOML 格式。该文件定义了：

- **模型配置**：支持的 AI 模型和提供商
- **函数配置**：业务逻辑的封装和变体管理
- **工具配置**：可供模型调用的外部工具
- **指标配置**：用于评估和优化的指标定义
- **网关配置**：服务器运行参数

## 文件结构

```toml
# ┌────────────────────────────────────────────────────────────────────────────┐
# │                                  GATEWAY                                   │
# └────────────────────────────────────────────────────────────────────────────┘
[gateway]
# 网关全局配置

# ┌────────────────────────────────────────────────────────────────────────────┐
# │                                   MODELS                                   │
# └────────────────────────────────────────────────────────────────────────────┘
[models.model_name]
# 模型配置

# ┌────────────────────────────────────────────────────────────────────────────┐
# │                              EMBEDDING MODELS                              │
# └────────────────────────────────────────────────────────────────────────────┘
[embedding_models.model_name]
# 嵌入模型配置

# ┌────────────────────────────────────────────────────────────────────────────┐
# │                                 FUNCTIONS                                  │
# └────────────────────────────────────────────────────────────────────────────┘
[functions.function_name]
# 函数配置

# ┌────────────────────────────────────────────────────────────────────────────┐
# │                                  METRICS                                   │
# └────────────────────────────────────────────────────────────────────────────┘
[metrics.metric_name]
# 指标配置

# ┌────────────────────────────────────────────────────────────────────────────┐
# │                                  TOOLS                                     │
# └────────────────────────────────────────────────────────────────────────────┘
[tools.tool_name]
# 工具配置
```

## Gateway 网关配置

网关配置控制 TensorZero 服务器的运行参数。

```toml
[gateway]
# 服务器绑定地址（可选，默认：0.0.0.0:3000）
bind_address = "0.0.0.0:3000"

# 调试模式（可选，默认：false）
debug = true

# 允许模板文件系统访问（可选，默认：false）
# 启用后，Jinja2 模板可以使用 {% include %} 指令
enable_template_filesystem_access = false

# 可观测性配置
[gateway.observability]
enabled = true
async_writes = false

# 导出配置
[gateway.export.otlp.traces]
enabled = true
```

### 配置选项说明

- `bind_address`：服务器监听的 IP 地址和端口
- `debug`：启用调试模式，输出更详细的日志
- `enable_template_filesystem_access`：允许模板文件包含其他文件
- `observability.enabled`：启用可观测性功能
- `observability.async_writes`：异步写入观测数据
- `export.otlp.traces.enabled`：启用 OpenTelemetry 追踪导出

## Models 模型配置

模型配置定义了可用的 AI 模型及其提供商设置。

### 基本结构

```toml
[models.model_name]
# 路由列表（按优先级排序，支持故障转移）
routing = ["provider_1", "provider_2"]

# 超时配置（可选）
[models.model_name.timeouts]
[models.model_name.timeouts.non_streaming]
total_ms = 30000  # 非流式请求总超时时间（毫秒）

[models.model_name.timeouts.streaming]
ttft_ms = 5000    # 流式请求首个令牌超时时间（毫秒）

# 提供商配置
[models.model_name.providers.provider_name]
type = "provider_type"
# 其他提供商特定配置...
```

### 支持的提供商

#### 1. OpenAI

```toml
[models.gpt_4o_mini.providers.openai]
type = "openai"
model_name = "gpt-4o-mini-2024-07-18"
api_base = "https://api.openai.com"  # 可选，自定义 API 基础 URL
api_key_location = "env::OPENAI_API_KEY"  # 可选，指定 API 密钥位置
```

#### 2. Anthropic

```toml
[models.claude_haiku.providers.anthropic]
type = "anthropic"
model_name = "claude-3-5-haiku-20241022"
api_key_location = "env::ANTHROPIC_API_KEY"  # 可选

# 额外请求头（可选）
extra_headers = [
    { name = "anthropic-beta", value = "output-128k-2025-02-19" }
]

# 额外请求体参数（可选）
extra_body = [
    { pointer = "/thinking", value = { type = "enabled", budget_tokens = 1024 } }
]
```

#### 3. DeepSeek

```toml
[models.deepseek_chat.providers.deepseek]
type = "deepseek"
model_name = "deepseek-chat"  # 或 "deepseek-reasoner"
api_key_location = "env::DEEPSEEK_API_KEY"  # 可选
```

#### 4. Azure OpenAI

```toml
[models.gpt_4o_azure.providers.azure]
type = "azure"
deployment_id = "gpt-4o-deployment"  # Azure 部署 ID
endpoint = "https://your-endpoint.openai.azure.com"
api_key_location = "env::AZURE_OPENAI_API_KEY"  # 可选
```

#### 5. Groq

```toml
[models.llama_groq.providers.groq]
type = "groq"
model_name = "meta-llama/llama-4-scout-17b-16e-instruct"
api_key_location = "env::GROQ_API_KEY"  # 可选
```

#### 6. Fireworks

```toml
[models.llama_fireworks.providers.fireworks]
type = "fireworks"
model_name = "accounts/fireworks/models/llama-v3p1-8b-instruct"
api_key_location = "env::FIREWORKS_API_KEY"  # 可选
parse_think_blocks = true  # 可选，解析思维块，默认：true
```

#### 7. OpenRouter

```toml
[models.gpt_openrouter.providers.openrouter]
type = "openrouter"
model_name = "openai/gpt-4.1-mini"
api_key_location = "env::OPENROUTER_API_KEY"  # 可选
```

#### 8. XAI (Grok)

```toml
[models.grok_xai.providers.xai]
type = "xai"
model_name = "grok-beta"
api_key_location = "env::XAI_API_KEY"  # 可选
```

#### 9. 自定义 VLLM

```toml
[models.custom_vllm.providers.vllm]
type = "vllm"
model_name = "Qwen/Qwen2.5-0.5B-Instruct"
api_base = "http://localhost:8000/v1"
api_key_location = "env::VLLM_API_KEY"  # 可选
```

#### 10. Rust Proxy

```toml
[models.rust_proxy.providers.rust_proxy]
type = "rust_proxy"
# 无需额外配置，通过环境变量管理
```

### 模型简写形式

TensorZero 支持模型简写，可以在函数变体中直接使用：

```toml
[functions.my_function.variants.my_variant]
type = "chat_completion"
# 使用简写形式
model = "openai::gpt-4o-mini-2024-07-18"
# 或
model = "anthropic::claude-3-5-haiku-20241022" 
# 或
model = "deepseek::deepseek-chat"
```

**支持的简写前缀：**
- `anthropic::`
- `deepseek::`
- `fireworks::`
- `google_ai_studio_gemini::`
- `gcp_vertex_gemini::`
- `gcp_vertex_anthropic::`
- `groq::`
- `hyperbolic::`
- `mistral::`
- `openai::`
- `openrouter::`
- `together::`
- `xai::`

## Embedding Models 嵌入模型配置

嵌入模型用于向量化文本，通常用于检索增强生成 (RAG) 应用。

```toml
[embedding_models.text_embedding_3_small]
routing = ["openai"]

[embedding_models.text_embedding_3_small.providers.openai]
type = "openai"
model_name = "text-embedding-3-small"
dimensions = 1536  # 可选，指定向量维度
```

## Functions 函数配置

函数是 TensorZero 的核心概念，封装了特定的业务逻辑。每个函数可以有多个变体用于 A/B 测试。

### 1. Chat 类型函数

Chat 函数用于对话场景，支持工具调用。

```toml
[functions.my_chat_function]
type = "chat"

# JSON 模式配置（可选）
system_schema = "functions/my_chat_function/system_schema.json"
user_schema = "functions/my_chat_function/user_schema.json"
assistant_schema = "functions/my_chat_function/assistant_schema.json"

# 工具配置
tools = ["tool_1", "tool_2"]  # 可用工具列表
tool_choice = "auto"  # 工具选择策略：auto、none、required 或具体工具
parallel_tool_calls = true  # 是否允许并行工具调用

# 描述（可选）
description = "这个函数用于处理对话场景"

# 变体配置
[functions.my_chat_function.variants.variant_name]
type = "chat_completion"
weight = 1.0  # 权重，用于 A/B 测试分流
model = "gpt-4o-mini-2024-07-18"

# 生成参数
max_tokens = 1000
temperature = 0.7
top_p = 0.9
presence_penalty = 0.0
frequency_penalty = 0.0

# 模板文件路径
system_template = "functions/my_chat_function/variant_name/system.minijinja"
user_template = "functions/my_chat_function/variant_name/user.minijinja"
assistant_template = "functions/my_chat_function/variant_name/assistant.minijinja"

# JSON 模式（可选）
json_mode = "strict"  # 或 "tool"、null

# 变体级别的超时配置（可选）
[functions.my_chat_function.variants.variant_name.timeouts]
[functions.my_chat_function.variants.variant_name.timeouts.non_streaming]
total_ms = 30000
[functions.my_chat_function.variants.variant_name.timeouts.streaming]
ttft_ms = 5000
```

#### 工具选择选项

- `"auto"`：模型自动决定是否使用工具
- `"none"`：不使用任何工具
- `"required"`：必须使用至少一个工具
- `{"type": "function", "function": {"name": "tool_name"}}`：强制使用特定工具

### 2. JSON 类型函数

JSON 函数专门用于结构化数据输出。

```toml
[functions.my_json_function]
type = "json"
output_schema = "functions/my_json_function/output_schema.json"  # 必需
user_schema = "functions/my_json_function/user_schema.json"  # 可选
system_schema = "functions/my_json_function/system_schema.json"  # 可选
assistant_schema = "functions/my_json_function/assistant_schema.json"  # 可选

[functions.my_json_function.variants.variant_name]
type = "chat_completion"
weight = 1.0
model = "gpt-4o-mini-2024-07-18"
system_template = "functions/my_json_function/variant_name/system.minijinja"
json_mode = "strict"  # JSON 函数必须指定
```

### 3. 实验性变体类型

#### Best of N Sampling

生成多个候选结果，使用评判模型选择最佳结果。

```toml
[functions.my_function.variants.best_of_n]
type = "experimental_best_of_n_sampling"
candidates = ["candidate_1", "candidate_2"]  # 引用其他变体名称
judge = "judge_variant_name"  # 引用用于评判的变体
n = 3  # 生成候选数量
```

#### Dynamic In-Context Learning (DICL)

基于相似性检索相关示例进行上下文学习。

```toml
[functions.my_function.variants.dicl]
type = "experimental_dynamic_in_context_learning"
embedding_model = "text-embedding-3-small"  # 用于检索的嵌入模型
model = "gpt-4o-mini-2024-07-18"
k = 10  # 检索的示例数量
system_instructions = "functions/my_function/dicl/system.minijinja"
json_mode = "strict"  # 可选
```

#### Mixture of N

使用路由模型动态选择最适合的变体。

```toml
[functions.my_function.variants.mixture]
type = "experimental_mixture_of_n"
variants = ["variant_1", "variant_2"]  # 可选择的变体列表
router = "router_variant_name"  # 用于路由决策的变体
```

#### Chain of Thought

实现思维链推理。

```toml
[functions.my_function.variants.cot]
type = "experimental_chain_of_thought"
model = "gpt-4o-mini-2024-07-18"
system_template = "functions/my_function/cot/system.minijinja"
json_mode = "strict"  # 可选
```

## Tools 工具配置

工具允许模型调用外部功能，如 API 调用、数据库查询等。

```toml
[tools.my_tool]
description = "工具的描述，告诉模型这个工具的用途"
parameters = "tools/my_tool.json"  # JSON Schema 文件路径
strict = true  # 可选，启用严格模式，默认：false
```

### 工具参数 Schema 示例

对应的 JSON Schema 文件 (`tools/my_tool.json`)：

```json
{
  "type": "object",
  "properties": {
    "location": {
      "type": "string",
      "description": "要查询天气的地点"
    },
    "unit": {
      "type": "string",
      "enum": ["celsius", "fahrenheit"],
      "description": "温度单位"
    }
  },
  "required": ["location"]
}
```

### 工具实现

工具的实际实现通过应用程序代码处理工具调用请求。TensorZero 只负责参数验证和调用协调。

## Metrics 指标配置

指标用于评估和优化模型性能。

```toml
[metrics.my_metric]
type = "boolean"      # 或 "float"
level = "inference"   # 或 "episode"
optimize = "max"      # 或 "min"
```

### 配置选项说明

- **type**: 指标数据类型
  - `boolean`: 布尔值（0/1），适用于成功率等二元指标  
  - `float`: 浮点数，适用于评分、时延等连续指标

- **level**: 指标级别
  - `inference`: 单次推理级别，每次 API 调用产生一个指标值
  - `episode`: 会话级别，多次相关推理的聚合指标

- **optimize**: 优化目标
  - `max`: 最大化该指标（如准确率、满意度）
  - `min`: 最小化该指标（如延迟、错误率）

### 常用指标示例

```toml
# 用户满意度（会话级别，浮点数，最大化）
[metrics.user_satisfaction]
type = "float"
level = "episode"
optimize = "max"

# 响应相关性（推理级别，布尔值，最大化）
[metrics.response_relevance]
type = "boolean"
level = "inference"
optimize = "max"

# 响应时间（推理级别，浮点数，最小化）
[metrics.response_time_ms]
type = "float"
level = "inference"
optimize = "min"
```

## Evaluations 评估配置

评估配置定义了自动化的模型性能评估流程。

```toml
[evaluations.my_evaluation]
type = "llm_judge"                    # 评估类型
function = "my_judge_function"        # 引用已定义的评判函数
dataset = "path/to/evaluation_dataset.jsonl"  # 评估数据集路径
# 其他评估特定配置...
```

## 环境变量配置

TensorZero 使用环境变量管理敏感信息如 API 密钥。在项目根目录创建 `.env` 文件：

```bash
# AI 模型 API 密钥
OPENAI_API_KEY=your-openai-api-key
ANTHROPIC_API_KEY=your-anthropic-api-key
DEEPSEEK_API_KEY=your-deepseek-api-key
GROQ_API_KEY=your-groq-api-key
FIREWORKS_API_KEY=your-fireworks-api-key
OPENROUTER_API_KEY=your-openrouter-api-key
XAI_API_KEY=your-xai-api-key

# Azure OpenAI
AZURE_OPENAI_API_KEY=your-azure-openai-key
AZURE_OPENAI_ENDPOINT=https://your-endpoint.openai.azure.com

# 对象存储
S3_ACCESS_KEY_ID=your-s3-access-key
S3_SECRET_ACCESS_KEY=your-s3-secret-key
AWS_ALLOW_HTTP=false

# 日志级别
RUST_LOG=info
```

## 完整示例

以下是一个完整的配置文件示例：

```toml
# ┌────────────────────────────────────────────────────────────────────────────┐
# │                                  GATEWAY                                   │
# └────────────────────────────────────────────────────────────────────────────┘

[gateway]
bind_address = "0.0.0.0:3000"
debug = false

[gateway.observability]
enabled = true
async_writes = false

# ┌────────────────────────────────────────────────────────────────────────────┐
# │                                   MODELS                                   │
# └────────────────────────────────────────────────────────────────────────────┘

[models.gpt_4o_mini]
routing = ["openai"]

[models.gpt_4o_mini.providers.openai]
type = "openai"
model_name = "gpt-4o-mini-2024-07-18"

[models.claude_haiku]
routing = ["anthropic"]

[models.claude_haiku.providers.anthropic]
type = "anthropic"
model_name = "claude-3-5-haiku-20241022"

[models.deepseek_chat]
routing = ["deepseek"]

[models.deepseek_chat.providers.deepseek]
type = "deepseek"
model_name = "deepseek-chat"

# ┌────────────────────────────────────────────────────────────────────────────┐
# │                              EMBEDDING MODELS                              │
# └────────────────────────────────────────────────────────────────────────────┘

[embedding_models.text_embedding_3_small]
routing = ["openai"]

[embedding_models.text_embedding_3_small.providers.openai]
type = "openai"
model_name = "text-embedding-3-small"

# ┌────────────────────────────────────────────────────────────────────────────┐
# │                                 FUNCTIONS                                  │
# └────────────────────────────────────────────────────────────────────────────┘

[functions.weather_assistant]
type = "chat"
tools = ["get_weather"]
user_schema = "functions/weather_assistant/user_schema.json"
description = "智能天气助手，提供天气查询和建议"

[functions.weather_assistant.variants.gpt_variant]
type = "chat_completion"
weight = 0.5
model = "gpt_4o_mini"
system_template = "functions/weather_assistant/gpt_variant/system.minijinja"
user_template = "functions/weather_assistant/gpt_variant/user.minijinja"
temperature = 0.7
max_tokens = 500

[functions.weather_assistant.variants.claude_variant]
type = "chat_completion"
weight = 0.3
model = "claude_haiku"
system_template = "functions/weather_assistant/claude_variant/system.minijinja"
user_template = "functions/weather_assistant/claude_variant/user.minijinja"
temperature = 0.5
max_tokens = 500

[functions.weather_assistant.variants.deepseek_variant]
type = "chat_completion"
weight = 0.2
model = "deepseek_chat"
system_template = "functions/weather_assistant/deepseek_variant/system.minijinja"
user_template = "functions/weather_assistant/deepseek_variant/user.minijinja"
temperature = 0.6
max_tokens = 500

[functions.extract_weather_info]
type = "json"
output_schema = "functions/extract_weather_info/output_schema.json"
user_schema = "functions/extract_weather_info/user_schema.json"

[functions.extract_weather_info.variants.structured_extraction]
type = "chat_completion"
weight = 1.0
model = "gpt_4o_mini"
system_template = "functions/extract_weather_info/structured_extraction/system.minijinja"
json_mode = "strict"

# ┌────────────────────────────────────────────────────────────────────────────┐
# │                                  METRICS                                   │
# └────────────────────────────────────────────────────────────────────────────┘

[metrics.user_satisfaction]
type = "float"
level = "episode"
optimize = "max"

[metrics.response_relevance]
type = "boolean"
level = "inference"
optimize = "max"

[metrics.response_time_ms]
type = "float"
level = "inference"
optimize = "min"

[metrics.weather_accuracy]
type = "boolean"
level = "inference"
optimize = "max"

# ┌────────────────────────────────────────────────────────────────────────────┐
# │                                  TOOLS                                     │
# └────────────────────────────────────────────────────────────────────────────┘

[tools.get_weather]
description = "获取指定地点的当前天气信息"
parameters = "tools/get_weather.json"
strict = true

[tools.get_weather_forecast]
description = "获取指定地点的天气预报"
parameters = "tools/get_weather_forecast.json"
strict = true
```

## 常见问题

### 1. 如何设置模型故障转移？

在 `routing` 数组中按优先级顺序列出多个提供商：

```toml
[models.my_model]
routing = ["primary_provider", "backup_provider"]

[models.my_model.providers.primary_provider]
type = "openai"
model_name = "gpt-4o-mini"

[models.my_model.providers.backup_provider]
type = "anthropic"
model_name = "claude-3-5-haiku"
```

### 2. 如何进行 A/B 测试？

通过设置不同变体的权重：

```toml
[functions.my_function.variants.variant_a]
weight = 0.7  # 70% 流量
# ...

[functions.my_function.variants.variant_b]
weight = 0.3  # 30% 流量
# ...
```

### 3. 如何配置超时时间？

可以在模型级别或变体级别设置超时：

```toml
# 模型级别超时
[models.my_model.timeouts]
[models.my_model.timeouts.non_streaming]
total_ms = 30000

# 变体级别超时（会覆盖模型级别设置）
[functions.my_function.variants.my_variant.timeouts]
[functions.my_function.variants.my_variant.timeouts.streaming]
ttft_ms = 5000
```

### 4. 如何使用自定义模板？

创建 Jinja2 模板文件并在配置中引用：

```toml
[functions.my_function.variants.my_variant]
system_template = "functions/my_function/my_variant/system.minijinja"
user_template = "functions/my_function/my_variant/user.minijinja"
```

模板文件示例 (`functions/my_function/my_variant/system.minijinja`)：
```jinja2
你是一个专业的{{domain}}助手。
请根据用户的问题提供准确、有用的回答。

当前时间：{{current_time}}
```

### 5. 如何配置 JSON 模式？

对于需要结构化输出的场景：

```toml
[functions.structured_output]
type = "json"
output_schema = "functions/structured_output/output_schema.json"

[functions.structured_output.variants.default]
type = "chat_completion"
model = "gpt-4o-mini"
json_mode = "strict"
```

### 6. 配置文件验证失败怎么办？

TensorZero 会在启动时验证配置文件。常见错误：

- **模型引用不存在**：检查函数变体中引用的模型名称
- **文件路径错误**：确保模板和 Schema 文件存在
- **权重配置错误**：权重必须为非负数
- **命名冲突**：避免使用 `tensorzero::` 前缀

启用调试模式获取详细错误信息：

```toml
[gateway]
debug = true
```

### 7. 如何优化性能？

- 使用合适的超时设置避免长时间等待
- 配置多个提供商实现负载均衡
- 为高频场景使用轻量级模型
- 启用并行工具调用提高效率

---

更多详细信息，请参考 [TensorZero 官方文档](https://docs.tensorzero.com/)。
