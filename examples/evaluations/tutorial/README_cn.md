# 教程：TensorZero 评估

此目录包含 **[TensorZero 评估指南](https://www.tensorzero.com/docs/evaluations/tutorial)** 的代码。

## 入门

### TensorZero

我们提供一个配置文件 (`./config/tensorzero.toml`)，其中指定：

- 一个生成俳句的 `write_haiku` 函数，具有 `gpt_4o` 和 `gpt_4o_mini` 变体。
- 一个 `haiku_eval` 评估，具有用于精确匹配和各种 LLM 评委的评估器。

### 先决条件

1. 安装 Docker。
2. 安装 Python 3.10+。
3. 使用 `pip install -r requirements.txt` 安装 Python 依赖项。
4. 为 OpenAI 生成一个 API 密钥 (`OPENAI_API_KEY`)。

### 设置

1. 使用 `OPENAI_API_KEY` 环境变量创建一个 `.env` 文件（有关示例，请参阅 `.env.example`）。
2. 运行 `docker compose up` 以启动 TensorZero 网关、TensorZero UI 和开发 ClickHouse 数据库。
3. 运行 `main.py` 脚本以生成 100 个俳句。

### 评估

#### 创建数据集

让我们生成一个由 100 个俳句组成的数据集。

1. 打开 UI，导航到"数据集"，然后选择"构建数据集"(`http://localhost:4000/datasets/builder`)。
2. 创建一个名为 `haiku_dataset` 的新数据集。
   选择您的 `write_haiku` 函数，选择"无"作为指标，选择"推理"作为数据集输出。

#### 运行评估 — CLI

让我们使用 TensorZero 评估 CLI 工具评估我们的 `gpt_4o` 变体。

1. 使用 CLI 启动评估：

```bash
docker compose run --rm evaluations \
    --evaluation-name haiku_eval \
    --dataset-name haiku_dataset \
    --variant-name gpt_4o \
    --concurrency 5
```

#### 评估数据集 — UI

让我们使用 TensorZero 评估 UI 评估我们的 `gpt_4o_mini` 变体，并比较结果。

1. 导航到"评估"(`http://localhost:4000/evaluations`) 并选择"新建运行"。
2. 使用 `gpt_4o_mini` 变体启动评估。
3. 在下拉列表中选择先前的评估运行以比较结果。 