# `provider-proxy`

> [!NOTE]
>
> ***此包适用于构建 TensorZero 本身的人员，而不适用于使用 TensorZero 进行应用程序开发的人员。***

这是一个缓存 MITM 代理，用于为我们的 E2E 测试缓存（出了名的不稳定）模型提供商请求。
这是一个缓存 MITM 代理，用于在 tensorzero e2e 测试期间缓存模型提供商请求。

## 用法

可以使用 `cargo run` 启动代理。默认情况下，它在端口 `3003` 上运行，并将缓存条目写入 `./request_cache`。
使用 `cargo run -- --help` 获取更多信息。

要将此代理与 e2e 测试一起使用，请在运行 e2e 或批处理测试时设置 `TENSORZERO_E2E_PROXY="http://localhost:3003"`
（例如 `TENSORZERO_E2E_PROXY="http://localhost:3003" cargo run-e2e`） 