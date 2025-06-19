# TensorZero UI

TensorZero UI 提供了一个 Web 界面来帮助管理您的 TensorZero 部署。
UI 提供了可观察性、优化等功能。

## 运行 UI

运行 UI 最简单的方法是使用 `tensorzero/ui` Docker 镜像。
有关更多信息，请参阅[快速入门](https://www.tensorzero.com/docs/quickstart/)和 [TensorZero UI 部署指南](https://www.tensorzero.com/docs/ui/deployment/)。

## 开发设置

> [!NOTE]
>
> **_以下说明适用于构建 TensorZero 本身的人员，而不适用于使用 TensorZero 进行应用程序开发的人员。_**

我们为开发目的提供固定数据，但您也可以将 UI 与任何相关配置一起使用。
以下说明假定您使用的是带有固定数据的预设。

1.  构建 `evaluations` 二进制文件。运行：`cargo build -p evaluations`
2.  构建 MiniJinja WASM 模块。有关参考，请参阅 `./app/utils/minijinja/README.md`。
3.  为网关设置环境变量。在 `fixtures/` 中创建一个包含凭据的 `.env` 文件。有关参考，请参阅 `fixtures/.env.example`。
4.  使用 `docker compose -f fixtures/docker-compose.yml up` 启动 TensorZero 网关和 ClickHouse。
5.  在 shell（而不是 `.env`）中设置 UI 环境变量。有关参考，请参阅 `./.env.example`。
6.  从仓库的根目录运行 `pnpm` 脚本以启动应用程序：

    *   运行 `pnpm install` 安装依赖项。
    *   运行 `pnpm ui:dev` 启动开发服务器。或者，启用功能标志以试用新功能：

        ```sh
        # 用于委托给基于 Python 的优化服务器
        TENSORZERO_UI_FF_ENABLE_PYTHON=1
        ```

    *   您还可以使用 `pnpm ui:test` 运行测试，并使用 `pnpm ui:storybook` 运行 Storybook。 