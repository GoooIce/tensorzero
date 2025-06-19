# Github Actions + 合并队列配置

## 背景

当确定合并队列作业是否通过时，Github 合并队列会查看仓库的必需状态检查。
不幸的是，必需状态检查也用于 PR CI（以确定您是否可以尝试合并 PR）。
如果我们想要求任何特定于合并队列的作业（例如，针对提供商的实时测试），我们需要为 PR CI 添加一个相应的“虚拟”作业，该作业始终成功（这将向 Github 报告所需的状态检查）。

此外，由于必需的状态检查是仓库级别的设置，因此将新的 Github Actions 作业添加为 PR 的一部分很棘手。
在 PR 合并之前将作业添加到必需的状态检查将导致所有其他 PR 卡住（因为永远不会报告检查）。
如果不修改必需的状态检查，即使新添加的作业失败，合并队列也会合并 PR。

## 我们的方法

我们有两个“顶级”作业：
* check-all-general-jobs-passed
* check-all-live-tests-passed

`check-all-general-jobs-passed` 针对 PR CI 和合并队列运行，并依赖于 `general.yml` 中的所有其他作业。
它从其所有依赖项中读取作业状态，如果任何这些作业失败或被取消，则失败。

`check-all-live-tests-passed` 类似，只是我们有两个版本。
在 `merge-queue.yml` 中，它依赖于 `live-tests` 作业。在 `dummy.yml` 中，它是一个没有依赖项且始终成功的虚拟作业（并且仅在 PR CI 中运行）。

此配置可确保 PR CI 和合并队列作业始终报告 `check-all-general-jobs-passed` 和 `check-all-live-tests-passed` 的状态。
我们将必需的状态检查*仅*设置为这两个作业，因为这些作业会从其依赖项传播失败。
这使我们能够从单个 PR 中控制需要哪些作业，而无需触及仓库级别的必需状态检查。

## 添加新的必需 CI 作业

如果您希望作业在 PR CI 和合并队列作业中都运行：
* 将新作业添加到 `general.yml`
* 将作业名称添加到 `general.yml` 中 `check-all-general-jobs-passed` 的 `needs` 数组中

如果您希望作业仅针对合并队列作业运行（而不是 PR CI）：
* 将新作业添加到 `merge-queue.yml`
* 将作业名称添加到 `merge-queue.yml` 中 `check-all-live-tests-passed` 的 `needs` 数组中。您*不*需要修改 `dummy.yml` 或 `general.yml`


参考：
* https://github.com/orgs/community/discussions/103114#discussioncomment-8359045
* https://github.com/orgs/community/discussions/25970
