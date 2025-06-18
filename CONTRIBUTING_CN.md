# 为 TensorZero 做贡献

感谢您对 TensorZero 做贡献的兴趣！

TensorZero 旨在为下一代 AI 应用提供动力。我们很乐意与您合作，共同实现这一愿景。

> [!TIP]
>
> 除了社区贡献，我们还在纽约市招聘（仅限现场办公）。查看我们的[职位空缺](https://www.tensorzero.com/jobs)。

## 许可证

TensorZero 采用 [Apache 2.0 许可证](LICENSE)。
通过向此仓库贡献代码，您同意在相同许可证下授权您的贡献。

## 社区与支持

### Slack 和 Discord

加入我们在 [Slack](https://www.tensorzero.com/slack) 或 [Discord](https://www.tensorzero.com/discord) 的社区，与团队和其他贡献者聊天。

### GitHub

我们使用 GitHub Issues 来跟踪 bug 和功能请求。对于一般问题、技术支持和与代码不直接相关的讨论，请使用 GitHub Discussions。

## 贡献

> [!TIP]
>
> 查看 [`good-first-issue`](https://github.com/tensorzero/tensorzero/issues?q=is%3Aopen+is%3Aissue+label%3Agood-first-issue) 标签，寻找适合新贡献者的简单问题。

### 代码

对于小的更改（即几行代码），请直接开启 PR。

对于大的更改，请先与我们沟通，以避免重复工作或浪费精力。
您可以发起讨论（GitHub、Slack 或 Discord）或创建 issue 作为起点。
团队很乐意提供反馈和指导。

> [!TIP]
>
> 查看下面的"技术指南"部分，了解构建和测试 TensorZero 的更多详细信息。

### 文档

我们计划很快开源我们的文档页面。查看 [issue #432](https://github.com/tensorzero/tensorzero/issues/432) 了解更多详情。

与此同时，如果您有建议或在文档中发现任何问题，请创建 issue。

### 内容 — 示例、教程等

我们很乐意在示例、教程和其他展示如何使用 TensorZero 构建 AI 应用的内容上合作。

对于直接贡献到我们仓库的内容，请遵循与代码贡献相同的流程。

对于外部内容（如博客文章、视频、社交媒体内容），我们很兴奋支持和推广您的工作。
在我们的社区频道（Slack 和 Discord）分享您的内容，在社交媒体上标记我们，或者如果您希望在发布前获得技术审查或反馈，请联系我们。

我们很乐意为这两种类型的内容提供指导和支持，帮助您为 TensorZero 社区创建高质量的资源。

### 集成

我们对探索与其他项目和工具（开源和商业）的集成持开放态度。
如果您有兴趣合作，请联系我们。

### 安全

如果您发现安全漏洞，请发送邮件至 [security@tensorzero.com](mailto:security@tensorzero.com)。

### 其他

您有其他想法吗？在 Slack 或 Discord 上联系我们，让我们知道。

---

## 技术指南

### 设置

- 安装 Rust (1.80+) [→](https://www.rust-lang.org/tools/install)
- 安装 `cargo-deny` [→](https://github.com/EmbarkStudios/cargo-deny)
- 安装 `cargo-nextest` [→](https://nexte.st/docs/installation/pre-built-binaries/)
- 安装 `pre-commit` [→](https://pre-commit.com/#install)
- 在您的仓库中启用 `pre-commit`：`pre-commit install`
- 安装 Docker [→](https://docs.docker.com/get-docker/)
- 安装 `uv` [→](https://docs.astral.sh/uv/)
- 安装 Python (3.9+)（例如 `uv python install 3.9` + ）
- 安装 Node.js（我们使用 v22）和 `npm` [→](https://nodejs.org/en)
- 安装 pnpm `npm install -g pnpm@10` [→](https://pnpm.io/installation)

### 优化配方

我们在 `recipes/` 中维护优化配方作为 Jupyter notebooks。
这些 notebooks 作为优化（例如微调）TensorZero 函数的手动工作流程。

Jupyter notebooks 臭名昭著地难以测试、维护和审查。
为了解决这些问题，每个 notebook 都有一个以 `_nb.py` 结尾的相应 Python 脚本，用于相同目的。
我们使用 [Jupytext](https://jupytext.readthedocs.io/en/latest/) 自动保持这两个文件同步。

要将 notebook 转换为脚本，运行 `ci/compile-notebook-to-script.sh path/to/notebook.ipynb`。
要将脚本转换为 notebook，运行 `ci/compile-script-to-notebook.sh path/to/script_nb.py`。

在 `pre-commit` 和 CI 中，我们使用脚本 `ci/compile-check-notebooks.sh` 检查 notebooks 是否与相关脚本匹配。

### 测试

#### Rust

##### 单元测试

```bash
cargo test-unit
```

##### E2E 测试

1. 启动测试 ClickHouse 数据库

   ```bash
   docker compose -f tensorzero-internal/tests/e2e/docker-compose.yml up --wait
   ```

2. 设置相关环境变量。查看 `examples/production-deployment/.env.example` 获取完整列表。

3. 在测试模式下启动网关

   ```bash
   cargo run-e2e
   ```

4. 运行 E2E 测试
   ```bash
   cargo test-e2e
   ```

> [!TIP]
>
> E2E 测试涉及每个支持的模型提供商，因此您需要所有可能的凭据来运行整个测试套件。
>
> 如果您的更改不影响每个提供商，您可以使用 `cargo test-e2e xyz` 运行测试子集，这将只运行名称中包含 `xyz` 的测试。

#### Python

1. 启动 ClickHouse 和 E2E 测试模式下的网关（见上文）。

2. 转到相关目录（例如 `cd clients/python`）

3. 创建虚拟环境并安装依赖项

   ```bash
   uv venv
   uv pip sync requirements.txt
   ```

4. 运行测试

   ```bash
   uv run pytest
   ```

5. 运行类型检查器

   ```bash
   uv pip install pyright
   uv run pyright
   ```

6. 运行格式化程序

   ```bash
   uv pip install ruff
   uv run ruff format --check .
   uv run ruff check --output-format=github --extend-select I .
   ```

#### 仪表板

对于开发，UI 运行在 `ui/fixtures/` 中的硬编码固定数据上。
它依赖于运行中的 ClickHouse 实例，该实例已使用 TensorZero 数据模型初始化。
我们还包含一些固定数据以便测试某些功能。

它还需要从 Rust 源代码一次性构建 WebAssembly 模块，该模块用于确保网关和 UI 之间消息模板的一致性。

以下是运行或测试 UI 的步骤，假设您已安装先决条件并检出此仓库：

1. 安装依赖项：`pnpm install`
2. 按照 `ui/app/utils/minijinja/README.md` 中的说明构建 WebAssembly 模块。
3. 创建 `ui/.env` 文件并为服务器设置以下环境变量：

```bash
OPENAI_API_KEY=<your-key>
FIREWORKS_API_KEY=<your-key>
FIREWORKS_ACCOUNT_ID=<your-account-id>
TENSORZERO_CLICKHOUSE_URL=<your-clickhouse-url> # 对于测试，设置为 http://chuser:chpassword@localhost:8123/tensorzero
TENSORZERO_UI_CONFIG_PATH=<path-to-config-file> # 对于测试，设置为 ./fixtures/config/tensorzero.toml
```

4. 运行依赖项：`docker compose -f ui/fixtures/docker-compose.yml up --build --force-recreate`
   （您可以省略最后 2 个标志来跳过构建步骤，但它们确保您使用最新的网关）

在依赖项运行时，您可以使用 `pnpm ui:test` 运行测试，使用 `pnpm ui:test:e2e` 运行 Playwright 测试。同样，您可以使用 `pnpm ui:dev` 启动开发服务器。

在 `main` 中可能有一些 Playwright 测试需要开启功能标志，所以如果它们因不明显的原因失败，请注意这一点。

---

再次感谢您对 TensorZero 做贡献的兴趣！我们很兴奋看到您构建的内容。
