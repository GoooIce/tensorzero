# TensorZero 教程

此目录包含 **[TensorZero 教程](https://www.tensorzero.com/docs/gateway/tutorial)** 的代码。

## 运行示例

1. 设置 `OPENAI_API_KEY` 环境变量。
   对于第二个示例（电子邮件助手），您还需要设置 `ANTHROPIC_API_KEY` 环境变量。

2. 启动 TensorZero 网关和 ClickHouse 数据库：

```bash
docker compose up
```

3. 安装依赖项：

```bash
uv venv
uv pip sync requirements.txt
```

或

```bash
# 我们建议使用 Python 3.10+ 和虚拟环境
pip install -r requirements.txt
```

4. 运行示例：

```bash
python run.py # 或 run_async.py 或 run_openai.py
``` 