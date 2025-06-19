# `openai-node` 客户端兼容性测试

此目录包含使用官方 OpenAI Node.js 客户端的 OpenAI 兼容性测试。

## 设置

1.  安装依赖项：`pnpm install`
2.  确保 TensorZero 在本地端口 3000 上运行，并带有 E2E 测试装置。
    *   从仓库的根目录运行 `docker compose -f tensorzero-internal/tests/e2e/docker-compose.yml up --force-recreate --build`
    *   在单独的终端中运行 `cargo run-e2e`

## 测试

```bash
pnpm typecheck
pnpm test
``` 