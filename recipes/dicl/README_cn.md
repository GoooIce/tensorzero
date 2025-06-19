# 动态上下文学习 (DICL)

**[DICL 文档 →](https://www.tensorzero.com/docs/gateway/guides/inference-time-optimizations#dynamic-in-context-learning-dicl)**

_动态上下文学习是一种强大的推理时优化技术，您可以通过 TensorZero 轻松地开箱即用。_

LLM 是优秀的少样本学习者。
使用示例进行上下文学习是提高性能的一种方便有效的方法，弥合了零样本提示和微调之间的差距。
对于任何给定的输入，使用成功结果的相似示例作为上下文可以显著提高性能。

<p align="center"><img src="dicl.png" alt="动态上下文学习 (DICL) 图" /></p>

由于 TensorZero 旨在以结构化格式存储推理和反馈，因此很容易查询成功的示例数据帧，然后使用它们对新输入进行上下文学习。

此配方提供了一个示例，说明如何根据正面反馈选择用于 DICL 的推理。
对于浮点指标，我们提供了选择一个截止分数的选项，该分数将推理限定为成功并值得包含在动态上下文学习示例中。
对于演示，我们假设所有都适用。
我们在 `dicl.ipynb` 的开头公开了所有这些设置。
您可能还想修改此 notebook 以使用您自己的策略来选择示例。 