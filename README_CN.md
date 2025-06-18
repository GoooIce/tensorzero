<p><picture><img src="https://github.com/user-attachments/assets/47d67430-386d-4675-82ad-d4734d3262d9" alt="TensorZero Logo" width=128 height=128></picture></p>

# TensorZero

<p><picture><img src="https://www.tensorzero.com/github-trending-badge.svg" alt="#1 Repository Of The Day"></picture></p>

**TensorZero 是一个用于_工业级 LLM 应用_的开源技术栈：**

- [x] **网关**
  - [x] 通过统一的 API 访问所有主要的 LLM 提供商（API 或自托管）
  - [x] 支持流式推理、工具使用、结构化生成（JSON 模式）、批处理、多模态（VLM）、文件输入、缓存等
  - [x] 定义提示模板和模式，确保应用程序与 LLM 之间一致的类型化接口
  - [x] 借助 Rust 满足极端的吞吐量和延迟需求：在 10k+ QPS 下 p99 延迟开销 <1ms
  - [x] 使用我们的 Python 客户端、任何 OpenAI SDK 或 OpenAI 兼容客户端，或我们的 HTTP API 进行集成（支持任何编程语言）
  - [x] 通过路由、重试、故障转移、负载均衡、细粒度超时等确保高可用性
  - [ ] 即将推出：嵌入向量；实时语音  
- [x] **可观测性**
  - [x] 在您自己的数据库中存储推理和反馈（指标、人工编辑等）
  - [x] 使用 TensorZero UI 或程序化方式深入了解单个推理或高级聚合模式
  - [x] 为优化、评估和其他工作流构建数据集
  - [x] 使用新的提示、模型、推理策略等重放历史推理
  - [x] 将 OpenTelemetry (OTLP) 跟踪导出到您喜欢的通用可观测性工具
  - [ ] 即将推出：AI 辅助调试和根因分析；AI 辅助数据标注
- [x] **优化**
  - [x] 使用监督微调、RLHF 和其他技术优化您的模型
  - [x] 使用 MIPROv2 等自动化提示工程算法优化您的提示
  - [x] 使用动态上下文学习、思维链、最佳/混合-N 采样等优化您的推理策略
  - [x] 为您的 LLM 启用反馈循环：将生产数据转化为更智能、更快、更便宜的模型的数据和学习飞轮
  - [ ] 即将推出：程序化优化；合成数据生成
- [x] **评估**
  - [x] 使用由启发式或 LLM 评判器支持的_静态评估_评估个别推理（≈ LLM 的单元测试）
  - [x] 使用完全灵活的_动态评估_评估端到端工作流（≈ LLM 的集成测试）
  - [x] 像优化任何其他 TensorZero 函数一样优化 LLM 评判器，使其与人类偏好保持一致
  - [ ] 即将推出：更多内置评估器；无头评估
- [x] **实验**
  - [x] 通过内置的 A/B 测试，对模型、提示、提供商、超参数等进行自信的发布
  - [x] 在复杂工作流中执行原则性实验（RCT），包括多轮和复合 LLM 系统
  - [ ] 即将推出：多臂老虎机；AI 管理的实验
- [x] **& 更多！**
  - [x] 使用 GitOps 友好的编排构建简单应用程序或大规模部署
  - [x] 通过内置的逃生舱口、程序化优先使用、直接数据库访问等扩展 TensorZero
  - [x] 与第三方工具集成：专业化可观测性和评估、模型提供商、智能体编排框架等
  - [ ] 即将推出：UI 游乐场

按需采用，增量采用，并与其他工具互补。

---

<p align="center">
  <b><a href="https://www.tensorzero.com/" target="_blank">官网</a></b>
  ·
  <b><a href="https://www.tensorzero.com/docs" target="_blank">文档</a></b>
  ·
  <b><a href="https://www.x.com/tensorzero" target="_blank">Twitter</a></b>
  ·
  <b><a href="https://www.tensorzero.com/slack" target="_blank">Slack</a></b>
  ·
  <b><a href="https://www.tensorzero.com/discord" target="_blank">Discord</a></b>
  <br>
  <br>
  <b><a href="https://www.tensorzero.com/docs/quickstart" target="_blank">快速开始 (5分钟)</a></b>
  ·
  <b><a href="https://www.tensorzero.com/docs/gateway/deployment" target="_blank">部署指南</a></b>
  ·
  <b><a href="https://www.tensorzero.com/docs/gateway/api-reference" target="_blank">API 参考</a></b>
  ·
  <b><a href="https://www.tensorzero.com/docs/gateway/deployment" target="_blank">配置参考</a></b>
</p>

---

<table>
  <tr>
    <td width="30%" valign="top"><b>什么是 TensorZero？</b></td>
    <td width="70%" valign="top">TensorZero 是一个用于工业级 LLM 应用的开源技术栈。它统一了 LLM 网关、可观测性、优化、评估和实验功能。</td>
  </tr>
  <tr>
    <td width="30%" valign="top"><b>TensorZero 与其他 LLM 框架有何不同？</b></td>
    <td width="70%" valign="top">
      1. TensorZero 能够基于生产指标和人工反馈优化复杂的 LLM 应用程序。<br>
      2. TensorZero 支持工业级 LLM 应用的需求：低延迟、高吞吐量、类型安全、自托管、GitOps、可定制性等。<br>
      3. TensorZero 统一了整个 LLMOps 技术栈，创造了复合效益。例如，LLM 评估可以与 AI 评判器一起用于微调模型。
    </td>
  </tr>
  <tr>
    <td width="30%" valign="top"><b>我可以将 TensorZero 与 ___ 一起使用吗？</b></td>
    <td width="70%" valign="top">可以。支持所有主要的编程语言。您可以通过我们的 Python 客户端、任何 OpenAI SDK 或 OpenAI 兼容客户端，或我们的 HTTP API 使用 TensorZero。</td>
  </tr>
  <tr>
    <td width="30%" valign="top"><b>TensorZero 是否已准备好用于生产？</b></td>
    <td width="70%" valign="top">是的。这里有一个案例研究：<b><a href="https://www.tensorzero.com/blog/case-study-automating-code-changelogs-at-a-large-bank-with-llms">在大型银行使用 LLM 自动化代码变更日志</a></b></td>
  </tr>
  <tr>
    <td width="30%" valign="top"><b>TensorZero 的费用是多少？</b></td>
    <td width="70%" valign="top">免费。TensorZero 100% 自托管且开源。没有付费功能。</td>
  </tr>
  <tr>
    <td width="30%" valign="top"><b>谁在构建 TensorZero？</b></td>
    <td width="70%" valign="top">我们的技术团队包括前 Rust 编译器维护者、机器学习研究人员（斯坦福、CMU、牛津、哥伦比亚大学）拥有数千次引用，以及一家独角兽创业公司的首席产品官。我们得到了与领先开源项目（如 ClickHouse、CockroachDB）和 AI 实验室（如 OpenAI、Anthropic）相同投资者的支持。</td>
  </tr>
  <tr>
    <td width="30%" valign="top"><b>如何开始使用？</b></td>
    <td width="70%" valign="top">您可以逐步采用 TensorZero。我们的<b><a href="https://www.tensorzero.com/docs/quickstart">快速开始指南</a></b>仅需 5 分钟就能从普通的 OpenAI 包装器发展为具有可观测性和微调功能的生产就绪 LLM 应用程序。</td>
  </tr>
</table>

---

## 功能特性

### 🌐 LLM 网关

> **与 TensorZero 集成一次，即可访问所有主要的 LLM 提供商。**

<table>
  <tr></tr> <!-- flip highlight order -->
  <tr>
    <td width="50%" align="center" valign="middle"><b>模型提供商</b></td>
    <td width="50%" align="center" valign="middle"><b>功能特性</b></td>
  </tr>
  <tr>
    <td width="50%" align="left" valign="top">
      <p>
        TensorZero 网关原生支持：
      </p>
      <ul>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/anthropic">Anthropic</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/aws-bedrock">AWS Bedrock</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/aws-sagemaker">AWS SageMaker</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/azure">Azure OpenAI Service</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/deepseek">DeepSeek</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/fireworks">Fireworks</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/gcp-vertex-ai-anthropic">GCP Vertex AI Anthropic</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/gcp-vertex-ai-gemini">GCP Vertex AI Gemini</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/google-ai-studio-gemini">Google AI Studio (Gemini API)</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/hyperbolic">Hyperbolic</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/mistral">Mistral</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/openai">OpenAI</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/together">Together</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/vllm">vLLM</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/xai">xAI</a></b></li>
      </ul>
      <p>
        <em>
          需要其他提供商？
          您的提供商很可能受支持，因为 TensorZero 集成了<b><a href="https://www.tensorzero.com/docs/gateway/guides/providers/openai-compatible">任何 OpenAI 兼容的 API（如 Ollama）</a></b>。
          </em>
      </p>
    </td>
    <td width="50%" align="left" valign="top">
      <p>
        TensorZero 网关支持高级功能，如：
      </p>
      <ul>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/retries-fallbacks">重试和故障转移</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/inference-time-optimizations">推理时优化</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/prompt-templates-schemas">提示模板和模式</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/experimentation/">实验（A/B 测试）</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/configuration-reference">配置即代码（GitOps）</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/batch-inference">批量推理</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/multimodal-inference">多模态推理（VLM）</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/inference-caching">推理缓存</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/metrics-feedback">指标和反馈</a></b></li>
        <li><b><a href="https://www.tensorzero.com/docs/gateway/guides/episodes">多步骤 LLM 工作流（Episode）</a></b></li>
        <li><em>& 更多功能...</em></li>
      </ul>
      <p>
        TensorZero 网关使用 Rust 🦀 编写，专注于<b>性能</b>（在 10k QPS 下 p99 延迟开销 &lt;1ms）。
        查看<b><a href="https://www.tensorzero.com/docs/gateway/benchmarks">基准测试</a></b>。<br>
      </p>
      <p>
        您可以使用 <b>TensorZero 客户端</b>（推荐）、<b>OpenAI 客户端</b>或 <b>HTTP API</b> 运行推理。
      </p>
    </td>
  </tr>
</table>

<br>

<details open>
<summary><b>使用方法：Python &mdash; TensorZero 客户端（推荐）</b></summary>

您可以使用 TensorZero Python 客户端访问任何提供商。

1. `pip install tensorzero`
2. 可选：设置 TensorZero 配置。
3. 运行推理：

```python
from tensorzero import TensorZeroGateway  # or AsyncTensorZeroGateway


with TensorZeroGateway.build_embedded(clickhouse_url="...", config_file="...") as client:
    response = client.inference(
        model_name="openai::gpt-4o-mini",
        # 轻松尝试其他提供商："anthropic::claude-3-7-sonnet-20250219"
        input={
            "messages": [
                {
                    "role": "user",
                    "content": "写一首关于人工智能的俳句。",
                }
            ]
        },
    )
```

查看**[快速开始](https://www.tensorzero.com/docs/quickstart)**了解更多信息。

</details>

<details>
<summary><b>使用方法：Python &mdash; OpenAI 客户端</b></summary>

您可以通过 TensorZero 使用 OpenAI Python 客户端访问任何提供商。

1. `pip install tensorzero`
2. 可选：设置 TensorZero 配置。
3. 运行推理：

```python
from openai import OpenAI  # or AsyncOpenAI
from tensorzero import patch_openai_client

client = OpenAI()

patch_openai_client(
    client,
    clickhouse_url="http://chuser:chpassword@localhost:8123/tensorzero",
    config_file="config/tensorzero.toml",
    async_setup=False,
)

response = client.chat.completions.create(
    model="tensorzero::model_name::openai::gpt-4o-mini",
    # 轻松尝试其他提供商："tensorzero::model_name::anthropic::claude-3-7-sonnet-20250219"
    messages=[
        {
            "role": "user",
            "content": "写一首关于人工智能的俳句。",
        }
    ],
)
```

查看**[快速开始](https://www.tensorzero.com/docs/quickstart)**了解更多信息。

</details>

<details>
<summary><b>使用方法：JavaScript / TypeScript (Node) &mdash; OpenAI 客户端</b></summary>

您可以通过 TensorZero 使用 OpenAI Node 客户端访问任何提供商。

1. 使用 Docker 部署 `tensorzero/gateway`。
   **[详细说明 →](https://www.tensorzero.com/docs/gateway/deployment)**
2. 设置 TensorZero 配置。
3. 运行推理：

```ts
import OpenAI from "openai";

const client = new OpenAI({
  baseURL: "http://localhost:3000/openai/v1",
});

const response = await client.chat.completions.create({
  model: "tensorzero::model_name::openai::gpt-4o-mini",
  // 轻松尝试其他提供商："tensorzero::model_name::anthropic::claude-3-7-sonnet-20250219"
  messages: [
    {
      role: "user",
      content: "写一首关于人工智能的俳句。",
    },
  ],
});
```

查看**[快速开始](https://www.tensorzero.com/docs/quickstart)**了解更多信息。

</details>

<details>
<summary><b>使用方法：其他语言和平台 &mdash; HTTP API</b></summary>

TensorZero 通过其 HTTP API 支持几乎任何编程语言或平台。

1. 使用 Docker 部署 `tensorzero/gateway`。
   **[详细说明 →](https://www.tensorzero.com/docs/gateway/deployment)**
2. 可选：设置 TensorZero 配置。
3. 运行推理：

```bash
curl -X POST "http://localhost:3000/inference" \
  -H "Content-Type: application/json" \
  -d '{
    "model_name": "openai::gpt-4o-mini",
    "input": {
      "messages": [
        {
          "role": "user",
          "content": "写一首关于人工智能的俳句。"
        }
      ]
    }
  }'
```

查看**[快速开始](https://www.tensorzero.com/docs/quickstart)**了解更多信息。

</details>

<br>

### 📈 LLM 优化

> **发送生产指标和人工反馈，轻松优化您的提示、模型和推理策略 &mdash; 使用 UI 或程序化方式。**

#### 模型优化

使用监督微调（SFT）和偏好微调（DPO）优化闭源和开源模型。

<table>
  <tr></tr> <!-- flip highlight order -->
  <tr>
    <td width="50%" align="center" valign="middle"><b>监督微调 &mdash; UI</b></td>
    <td width="50%" align="center" valign="middle"><b>偏好微调（DPO）&mdash; Jupyter Notebook</b></td>
  </tr>
  <tr>
    <td width="50%" align="center" valign="middle"><img src="https://github.com/user-attachments/assets/cf7acf66-732b-43b3-af2a-5eba1ce40f6f"></td>
    <td width="50%" align="center" valign="middle"><img src="https://github.com/user-attachments/assets/a67a0634-04a7-42b0-b934-9130cb7cdf51"></td>
  </tr>
</table>

#### 推理时优化

通过动态更新带有相关示例的提示、组合多个推理的响应等方式提升性能。

<table>
  <tr></tr> <!-- flip highlight order -->
  <tr>
    <td width="50%" align="center" valign="middle"><b><a href="https://www.tensorzero.com/docs/gateway/guides/inference-time-optimizations#best-of-n-sampling">最佳 N 采样</a></b></td>
    <td width="50%" align="center" valign="middle"><b><a href="https://www.tensorzero.com/docs/gateway/guides/inference-time-optimizations#mixture-of-n-sampling">混合 N 采样</a></b></td>
  </tr>
  <tr>
    <td width="50%" align="center" valign="middle"><img src="https://github.com/user-attachments/assets/c0edfa4c-713c-4996-9964-50c0d26e6970"></td>
    <td width="50%" align="center" valign="middle"><img src="https://github.com/user-attachments/assets/75b5bf05-4c1f-43c4-b158-d69d1b8d05be"></td>
  </tr>
  <tr>
    <td width="50%" align="center" valign="middle"><b><a href="https://www.tensorzero.com/docs/gateway/guides/inference-time-optimizations#dynamic-in-context-learning-dicl">动态上下文学习（DICL）</a></b></td>
    <td width="50%" align="center" valign="middle"><b><a href="https://www.tensorzero.com/docs/gateway/guides/inference-time-optimizations#chain-of-thought-cot">思维链（CoT）</a></b></td>
  </tr>
  <tr>
    <td width="50%" align="center" valign="middle"><img src="https://github.com/user-attachments/assets/d8489e92-ce93-46ac-9aab-289ce19bb67d"></td>
    <td width="50%" align="center" valign="middle"><img src="https://github.com/user-attachments/assets/ea13d73c-76a4-4e0c-a35b-0c648f898311" height="320"></td>
  </tr>
</table>

_更多功能即将推出..._

<br>

#### 提示优化

使用研究驱动的优化技术程序化地优化您的提示。

<table>
  <tr></tr> <!-- flip highlight order -->
  <tr>
    <td width="50%" align="center" valign="middle"><b><a href="https://www.tensorzero.com/docs/gateway/guides/inference-time-optimizations#best-of-n-sampling">MIPROv2</a></b></td>
    <td width="50%" align="center" valign="middle"><b><a href="https://github.com/tensorzero/tensorzero/tree/main/examples/gsm8k-custom-recipe-dspy">DSPy 集成</a></b></td>
  </tr>
  <tr>
    <td width="50%" align="center" valign="middle"><img src="https://github.com/user-attachments/assets/d81a7c37-382f-4c46-840f-e6c2593301db" alt="MIPROv2 diagram"></td>
    <td width="50%" align="center" valign="middle">
      TensorZero 提供了几个优化配方，但您也可以轻松创建自己的配方。
      这个示例展示了如何使用任意工具优化 TensorZero 函数 — 这里使用的是 DSPy，一个流行的自动化提示工程库。
    </td>
  </tr>
</table>

_更多功能即将推出..._

<br>

### 🔍 LLM 可观测性

> **放大调试单个 API 调用，或缩小监控跨模型和提示的指标随时间变化 &mdash; 全部使用开源的 TensorZero UI。**

<table>
  <tr></tr> <!-- flip highlight order -->
  <tr>
    <td width="50%" align="center" valign="middle"><b>可观测性 » 推理</b></td>
    <td width="50%" align="center" valign="middle"><b>可观测性 » 函数</b></td>
  </tr>
  <tr>
    <td width="50%" align="center" valign="middle"><img src="https://github.com/user-attachments/assets/2cc3cc9a-f33f-4e94-b8de-07522326f80a"></td>
    <td width="50%" align="center" valign="middle"><img src="https://github.com/user-attachments/assets/00ae6605-8fa0-4efd-8238-ae8ea589860f"></td>
  </tr>
</table>

<br>

### 📊 LLM 评估

> **使用 TensorZero 评估比较提示、模型和推理策略 &mdash; 支持启发式和 LLM 评判器。**

<table>
  <tr></tr> <!-- flip highlight order -->
  <tr>
    <td width="50%" align="center" valign="middle"><b>评估 » UI</b></td>
    <td width="50%" align="center" valign="middle"><b>评估 » CLI</b></td>
  </tr>
  <tr>
    <td width="50%" align="center" valign="middle"><img src="https://github.com/user-attachments/assets/f4bf54e3-1b63-46c8-be12-2eaabf615699"></td>
    <td width="50%" align="left" valign="middle">
<pre><code class="language-bash">docker compose run --rm evaluations \
  --evaluation-name extract_data \
  --dataset-name hard_test_cases \
  --variant-name gpt_4o \
  --concurrency 5</code></pre>
<pre><code class="language-bash">Run ID: 01961de9-c8a4-7c60-ab8d-15491a9708e4
Number of datapoints: 100
██████████████████████████████████████ 100/100
exact_match: 0.83 ± 0.03
semantic_match: 0.98 ± 0.01
item_count: 7.15 ± 0.39</code></pre>
    </td>
  </tr>
</table>

## 演示

> **观看 LLM 在 TensorZero 的实时数据提取中不断改进！**
>
> **[动态上下文学习（DICL）](https://www.tensorzero.com/docs/gateway/guides/inference-time-optimizations#dynamic-in-context-learning-dicl)** 是 TensorZero 开箱即用的强大推理时优化。
> 它通过自动将相关的历史示例纳入提示中来增强 LLM 性能，无需模型微调。

https://github.com/user-attachments/assets/4df1022e-886e-48c2-8f79-6af3cdad79cb

## 开始使用

**今天就开始构建。**
**[快速开始](https://www.tensorzero.com/docs/quickstart)**展示了使用 TensorZero 设置 LLM 应用程序是多么容易。

**有问题？**
在**[Slack](https://www.tensorzero.com/slack)**或**[Discord](https://www.tensorzero.com/discord)**上询问我们。

**在工作中使用 TensorZero？**
发邮件给我们 **[hello@tensorzero.com](mailto:hello@tensorzero.com)** 与您的团队建立 Slack 或 Teams 频道（免费）。

**与我们一起工作。**
我们正在**[纽约招聘](https://www.tensorzero.com/jobs)**。
我们也欢迎**[开源贡献](https://github.com/tensorzero/tensorzero/blob/main/CONTRIBUTING.md)**！

## 示例

我们正在开发一系列**完整可运行的示例**，展示 TensorZero 的数据和学习飞轮。

> **[使用 TensorZero 优化数据提取（NER）](https://github.com/tensorzero/tensorzero/tree/main/examples/data-extraction-ner)**
>
> 这个示例展示了如何使用 TensorZero 优化数据提取管道。
> 我们演示了微调和动态上下文学习（DICL）等技术。
> 最终，一个优化的 GPT-4o Mini 模型在这个任务上超越了 GPT-4o &mdash; 成本和延迟仅为后者的一小部分 &mdash; 使用少量训练数据。

> **[智能体 RAG — 使用 LLM 进行多跳问答](https://github.com/tensorzero/tensorzero/tree/main/examples/rag-retrieval-augmented-generation/simple-agentic-rag/)**
>
> 这个示例展示了如何使用 TensorZero 构建多跳检索智能体。
> 智能体迭代搜索维基百科收集信息，并决定何时有足够的上下文来回答复杂问题。

> **[写俳句来满足具有隐藏偏好的评判者](https://github.com/tensorzero/tensorzero/tree/main/examples/haiku-hidden-preferences)**
>
> 这个示例微调 GPT-4o Mini 来生成符合特定品味的俳句。
> 您将看到 TensorZero 的"盒装数据飞轮"在实际应用中：更好的变体带来更好的数据，更好的数据带来更好的变体。
> 您将通过多次微调 LLM 看到进展。

> **[通过最佳 N 采样提升 LLM 象棋能力](https://github.com/tensorzero/tensorzero/tree/main/examples/chess-puzzles/)**
>
> 这个示例展示了最佳 N 采样如何通过从多个生成选项中选择最有前景的走法来显著增强 LLM 的象棋能力。

> **[使用自动化提示工程的自定义配方改进数学推理（DSPy）](https://github.com/tensorzero/tensorzero/tree/main/examples/gsm8k-custom-recipe-dspy)**
>
> TensorZero 提供了许多预构建的优化配方，涵盖常见的 LLM 工程工作流。
> 但您也可以轻松创建自己的配方和工作流！
> 这个示例展示了如何使用任意工具优化 TensorZero 函数 — 这里使用 DSPy。

_& 更多示例即将推出！_ 