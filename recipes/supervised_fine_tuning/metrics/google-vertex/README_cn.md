# TensorZero 配方：使用 Google Vertex AI 进行监督微调

`google_vertex.ipynb` notebook 提供了一个分步配方，可根据 TensorZero 网关收集的数据对 Google Gemini 模型进行监督微调。
在 notebook 将运行的 shell 中设置 `TENSORZERO_CLICKHOUSE_URL=http://chuser:chpassword@localhost:8123/tensorzero`。

## 设置

### 先决条件

- [gcloud CLI](https://cloud.google.com/sdk/docs/install)
- [Google Cloud 本地身份验证凭据](https://cloud.google.com/docs/authentication/set-up-adc-local-dev-environment)
- [Google Cloud Storage 存储桶](https://cloud.google.com/storage/docs/creating-buckets)。

### 使用 [`uv`](https://github.com/astral-sh/uv) (推荐)

```bash
uv venv  # 创建一个新的虚拟环境
uv pip sync requirements.txt  # 安装依赖项
```

### 使用 `pip`

我们建议使用 Python 3.10+ 和虚拟环境。

```bash
pip install -r requirements.txt
``` 