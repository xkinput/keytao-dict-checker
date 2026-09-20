# 「RIME 键道」（KeyTao）码表检查器

![Windows x64](https://img.shields.io/badge/Windows-x64-0078D4)
![macOS arm64](https://img.shields.io/badge/macOS-arm64-000?logo=macos)
![Linux x64](https://img.shields.io/badge/Linux-x64-F4BC00?logo=linux)

[![GitHub repo](https://img.shields.io/badge/GitHub-repo-0FBF3E?logo=github)](https://github.com/xkinput/keytao-dict-checker)
[![Latest Release](https://img.shields.io/github/v/release/xkinput/keytao-dict-checker?color=0FBF3E&label=Latest&logo=github)](https://github.com/xkinput/keytao-dict-checker/releases/latest)
![Downloads](https://img.shields.io/github/downloads/xkinput/keytao-dict-checker/total?color=0FBF3E&label=Downloads&logo=github)

[![Rust 1.98](https://img.shields.io/badge/Rust-1.98-D34516?logo=rust)](https://rust-lang.org/)
[![MIT License](https://img.shields.io/badge/License-MIT-750014)](https://mit-license.org)

一个 CLI 程序，用于检查 [RIME 键道](https://github.com/xkinput/KeyTao) 词库中的几项特定问题。

## 🧭 用法

```text
keytao-dict-checker -s <单字码表>
keytao-dict-checker -p <词组码表>
keytao-dict-checker <单字码表> <词组码表>
```

| 单字检查项   | 说明                                   | 输出 |
|--------------|----------------------------------------|------|
| 编码形式异常 | 编码不是 `<2个音码><2-4个形码>` 的前缀 | 词条 |
| 形码段冲突   | 单字不同词条的形码不是单一形码段的前缀 | 词条 |
| 词条冗余     | 单字在某编码链上存在超过两个词条       | 词条 |
| 孤立飞键     | 该词条编码包含飞键，但未伴生           | 词条 |
| 孤立全码     | 该全码词条缺失对应的简码词条           | 词条 |
| 简码重码     | 该编码被多个现有简码词条使用           | 编码 |
| 简码空码     | 该编码被现有简码词条跳过               | 编码 |

| 词组检查项   | 说明                                                         | 输出 | 依赖单字 |
|--------------|--------------------------------------------------------------|------|----------|
| 编码形式异常 | 编码不是 `<4个音码><2个形码>` 或 `<3个音码><3个形码>` 的前缀 | 词条 | 否       |
| 词条冗余     | 词组在某编码链上存在超过一个词条                             | 词条 | 否       |
| 空码         | 该编码被现有词条跳过                                         | 编码 | 否       |
| 无理码       | 无法按规则推导该词条的编码                                   | 词条 | 是       |
| 残缺飞键     | 该词条编码包含飞键，但伴生不完整                             | 词条 | 是       |

按输入自动启用条件允许的全部检查项：仅提供单字码表时 7 项；仅提供词组码表时 3 项；齐备时 12 项。

## 📤 输出与退出

问题报告输出到标准输出；运行错误输出到标准错误。
退出码只表示运行状态：正常完成为 0，运行错误非 0；检查结果以输出内容为准。
