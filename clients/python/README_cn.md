# TensorZero Python 客户端

**[网站](https://www.tensorzero.com/)** ·
**[文档](https://www.tensorzero.com/docs)** ·
**[Twitter](https://www.x.com/tensorzero)** ·
**[Slack](https://www.tensorzero.com/slack)** ·
**[Discord](https://www.tensorzero.com/discord)**

**[快速入门 (5分钟)](https://www.tensorzero.com/docs/quickstart)** ·
**[综合教程](https://www.tensorzero.com/docs/gateway/tutorial)** ·
**[部署指南](https://www.tensorzero.com/docs/gateway/deployment)** ·
**[API 参考](https://www.tensorzero.com/docs/gateway/api-reference/inference)** ·
**[配置参考](https://www.tensorzero.com/docs/gateway/configuration-reference)**

`tensorzero` 包为 TensorZero 网关提供了一个 Python 客户端。
该客户端允许您通过网关轻松发出推理请求并为其分配反馈。

更多信息请参阅我们的 **[API 参考](https.tensorzero.com/docs/gateway/api-reference)**。

## 安装

```bash
pip install tensorzero
```

## 基本用法

### 初始化

TensorZero 客户端提供同步 (`TensorZeroGateway`) 和异步 (`AsyncTensorZeroGateway`) 两种变体。
此外，客户端可以启动一个嵌入式（内存中）网关 (`build_embedded`) 或连接到一个外部 HTTP 网关 (`build_http`) - 这两种方法都返回一个网关实例。

默认情况下，当您调用 `build_http` 或 `build_embedded` 时，异步客户端会返回一个 `Future`，因此您必须 `await` 它。
如果您希望避免 `await`，可以将 `async_setup=False` 设置为以阻塞方式初始化客户端。

#### 同步 HTTP 网关

```python
from tensorzero import TensorZeroGateway

with TensorZeroGateway.build_http(gateway_url="http://localhost:3000") as client:
    # ...
```

#### 异步 HTTP 网关

```python
import asyncio

from tensorzero import AsyncTensorZeroGateway


async def run():
    async with await AsyncTensorZeroGateway.build_http(
        gateway_url="http://localhost:3000",
        # async_setup=False  # 可选：跳过 `await` 并同步（阻塞）运行 `build_http`
    ) as client:
        # ...


if __name__ == "__main__":
    asyncio.run(run())
```

#### 同步嵌入式网关

```python
from tensorzero import TensorZeroGateway

with TensorZeroGateway.build_embedded(
    config_file="/path/to/tensorzero.toml",
    clickhouse_url="http://chuser:chpassword@localhost:8123/tensorzero"
) as client:
    # ...
```

#### 异步嵌入式网关

```python
import asyncio

from tensorzero import AsyncTensorZeroGateway


async def run():
    async with await AsyncTensorZeroGateway.build_embedded(
        config_file="/path/to/tensorzero.toml",
        clickhouse_url="http://chuser:chpassword@localhost:8123/tensorzero"
        # async_setup=False  # 可选：跳过 `await` 并同步（阻塞）运行 `build_embedded`
    ) as client:
        # ...


if __name__ == "__main__":
    asyncio.run(run())
```

### 推理

#### 使用同步客户端进行非流式推理

```python
with TensorZeroGateway.build_http(gateway_url="http://localhost:3000") as client:
    response = client.inference(
        model_name="openai::gpt-4o-mini",
        input={
            "messages": [
                {"role": "user", "content": "日本的首都是哪里？"},
            ],
        },
    )

    print(response)
```

#### 使用异步客户端进行非流式推理

```python
async with await AsyncTensorZeroGateway.build_http(gateway_url="http://localhost:3000") as client:
    response = await client.inference(
        model_name="openai::gpt-4o-mini",
        input={
            "messages": [
                {"role": "user", "content": "日本的首都是哪里？"},
            ],
        },
    )

    print(response)
```

#### 使用同步客户端进行流式推理

```python
with TensorZeroGateway.build_http(gateway_url="http://localhost:3000") as client:
    stream = client.inference(
        model_name="openai::gpt-4o-mini",
        input={
            "messages": [
                {"role": "user", "content": "日本的首都是哪里？"},
            ],
        },
        stream=True,
    )

    for chunk in stream:
        print(chunk)
```

#### 使用异步客户端进行流式推理

```python
async with await AsyncTensorZeroGateway.build_http(gateway_url="http://localhost:3000") as client:
    stream = await client.inference(
        model_name="openai::gpt-4o-mini",
        input={
            "messages": [{"role": "user", "content": "日本的首都是哪里？"}],
        },
        stream=True,
    )

    async for chunk in stream:
        print(chunk)
```

### 反馈

#### 同步

```python
with TensorZeroGateway.build_http(gateway_url="http://localhost:3000") as client:
    response = client.feedback(
        metric_name="thumbs_up",
        inference_id="00000000-0000-0000-0000-000000000000",
        value=True,  # 👍
    )

    print(response)
```

#### 异步

```python
async with await AsyncTensorZeroGateway.build_http(gateway_url="http://localhost:3000") as client:
    response = await client.feedback(
        metric_name="thumbs_up",
        inference_id="00000000-0000-0000-0000-000000000000",
        value=True,  # 👍
    )

    print(response)
``` 