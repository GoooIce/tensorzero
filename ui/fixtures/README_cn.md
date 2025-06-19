# TensorZero fixtures

我们的大多数 fixtures 都存储在此目录中，但一些大型（>100MB）文件除外。

## 从 S3 拉取 fixtures

我们的大型 fixtures 存储在与 S3 兼容的对象存储中（目前是 Cloudflare R2）。
可以使用 `uv run ./download-fixtures.py` 手动下载它们。

## 编写新的大型 fixtures

大型 fixtures 不应提交到仓库中。相反：

1.  将新的 fixtures 添加到 `./s3-fixtures`
2.  运行 `./upload-fixtures.sh`
3.  在 `uv run ./download-fixtures.py` 中列出新的 fixtures 文件 