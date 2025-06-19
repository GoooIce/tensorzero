# TensorZero 配方：使用 Fireworks 进行监督微调

`fireworks.ipynb` notebook 提供了一个分步配方，可根据 TensorZero 网关收集的演示数据，使用 Fireworks 进行监督微调。
在 notebook 将运行的 shell 中设置 `TENSORZERO_CLICKHOUSE_URL=http://chuser:chpassword@localhost:8123/tensorzero`，并按照 notebook 中的说明安装 `firectl`。

## 设置

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