# 推理演示

此演示展示了如何使用 HTTP 网关服务器和嵌入式网关执行推理。

## 用法

1.  在 `<tensorzero_repository>/examples/haiku-hidden-preferences` 目录中运行 `docker compose up`。

接下来的步骤应从仓库的根目录运行。

2.  要对正在运行的网关服务器执行推理，请运行：

```bash
cargo run --example inference_demo -- --gateway-url http://localhost:3000 --function-name 'judge_haiku' --streaming '{"topic": "Rivers", "haiku": "Endless roaring flow. Mountains weep streams for oceans. Carve earth like giants"}'
```

3.  要对嵌入式网关服务器（在示例二进制文件中运行）执行推理，请运行：

```bash
CLICKHOUSE_URL=http://127.0.0.1:8123/tensorzero cargo run --example inference_demo -- --config-path examples/haiku-hidden-preferences/config/tensorzero.toml --function-name judge_haiku --streaming '{"topic": "Rivers", "haiku": "Endless roaring flow. Mountains weep streams for oceans. Carve earth like giants"}'
```

`--streaming` 标志控制输出是在可用时流式传输到控制台，还是仅在完整响应可用时才显示。 