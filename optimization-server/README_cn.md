# optimization-server

这是一个基于 Python FastAPI 的服务器，它实现了 UI 使用的优化配方。
当前支持的优化：

- OpenAI 微调
- Fireworks 微调

## 用法

需要以下环境变量：

- `TENSORZERO_UI_CONFIG_PATH`：UI 使用的 TensorZero 网关配置文件的路径。应与 NodeJS `ui` 服务器使用的文件相同。

还可以设置以下可选环境变量：

- `OPENAI_BASE_URL`：覆盖用于微调作业的 OpenAI 服务器
- `FIREWORKS_BASE_URL`：覆盖用于微调作业的 Fireworks 服务器

要启动服务器，请运行 `uv run fastapi run src/optimization_server/main.py --port 7001`

要使用 ui fixtures 配置，请运行 `TENSORZERO_UI_CONFIG_PATH=../ui/fixtures/config/tensorzero.toml uv run fastapi run src/optimization_server/main.py --port 7001` 