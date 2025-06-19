# `openai-go` 客户端兼容性测试

此目录包含使用官方 OpenAI Go 客户端对 TensorZero 的 OpenAI API 兼容性进行的测试。

## 设置

1.  确保您的系统上已安装 Go。
2.  确保 TensorZero 在本地端口 3000 上运行，并带有 E2E 测试装置。
    *   从仓库的根目录运行 `docker compose -f tensorzero-internal/tests/e2e/docker-compose.yml up --force-recreate --build`
    *   在单独的终端中运行 `cargo run-e2e`

## 测试

```bash
cd tests
go test -v
``` 