# TensorZero 配方：使用 MIPRO 自动进行提示工程

本配方提供了使用历史推理和反馈数据对 TensorZero 函数执行自动提示工程的逐步指南。

## MIPRO（多提示指令建议优化器）

MIPRO 是一个为多阶段 LLM 应用程序设计的优化框架。
它通过系统地搜索指令和少样本演示来增强提示的有效性，以最大化下游任务的性能。
与依赖手动试错方法的传统提示工程不同，MIPRO 引入了算法策略，用于在黑盒模型访问等约束下优化 LM 程序。
有关更多详细信息，请参阅 [MIPRO 论文](https://arxiv.org/abs/2406.11695v1)。
本配方实现了来自 [DSPy](https://github.com/stanfordnlp/dspy) 的此算法的 MIPROv2 变体。

## 高级结构

MIPRO 在一个结构化的优化框架内运行：

- 建议生成：生成候选指令和演示。
- 评估：根据其有效性对生成的提示进行评分。
- 优化：利用代理模型根据观察到的性能优化提示建议。

在我们的实现中，我们使用 LLM 评委对候选提示进行评分。
评委可配置以适应您的问题，方法是描述您要优化的任务和指标。这假设 LLM 评委将输出与您要优化的指标相关的分数。
**我们将展示 TensorZero 如何使用 MIPRO 自动优化 GPT-4o Mini 的提示。**

<p align="center">
  <img src="visualization.svg" alt="按变体划分的指标" />
</p>

虽然——不出所料——它的表现不如 DICL（参见 `recipes/dicl`）和监督微调（参见 `recipes/supervised_fine_tuning`），但它明显优于一个简单的初始提示。

> [!TIP]
>
> 有关使用 MIPRO 进行自动提示工程的更多见解，请参阅我们的文章[从 NER 到代理：自动提示工程能否扩展到复杂任务？](https://tensorzero.com/blog/from-ner-to-agents-does-automated-prompt-engineering-scale-to-complex-tasks)。

## 入门

### 先决条件

1. 安装 Python 3.10+。
2. 使用 `pip install -r requirements.txt` 安装 Python 依赖项。
3. 为 OpenAI 生成一个 API 密钥 (`OPENAI_API_KEY`)。

### 设置

1. 设置 `OPENAI_API_KEY` 和 `TENSORZERO_CLICKHOUSE_URL` 环境变量。
2. 运行 `mipro.ipynb` Jupyter notebook。 