# 极简计时器

一个专注于核心功能的极简计时工具，提供多种单位和可定制 UI 的用户体验

---

## 特性亮点

- **极简 UI** : 纯净界面，无冗余元素
- **高精度** : 基于毫秒时间戳和 64 位浮点数
- **外置配置** : 计时器配置通过配置文件完成，不污染主程序 UI
- **可定制 UI** : 支持修改页眉页脚，目标时间，刷新间隔（支持 1ms 高速刷新）， 显示小数位数，字体大小，窗口大小，窗口标题
- **多单位支持** : 从年到毫秒的完整时间计量单位

## 环境支持

- **Linux**: 主要支持平台，确保功能完整，提供预编译二进制发布
- Windows: 非主动支持，有没有预编译二进制发布取决于开发者心情，不保证最新版本
- 其他系统: 不提供预编译，需要请自行构建

## 快速开始

1. 在 Releases 页面下载最新版本，或手动编译二进制文件
2. 第一次运行自动生成 yaml 格式配置文件
3. 运行主程序

## 构建步骤

### 构建要求
- Rust 工具链最新版本

### 构建命令
```bash

git clone https://github.com/CaSilicate-dev/chronotimer
cd chronotimer
cargo build --release
```

二进制文件位于./target/release/chronotimer

---

## 开源许可证

本项目使用 [GPLv3 LICENSE](https://github.com/CaSilicate-dev/chronotimer/blob/main/LICENSE) 开源协议

**根据GPLv3条款:**
- 你可以自由使用、修改和分发本软件
- 如果你分发修改版本，必须同样使用GPLv3许可证
- 分发二进制文件时必须提供对应的源代码

## 源码

您可以通过以下方式获取本程序的源代码：
- 随本分发的 source-code.zip
- GitHub: https://github.com/CaSilicate-dev/chronotimer
- 通过电子邮件向作者索取

---

## 贡献

欢迎提交 Issue 或 Pull Requests！

## 注意事项

本项目的主要开发与适配环境为 Linux 。对于其他平台上出现的问题，我们通常**无法提供支持**，建议优先在 Linux 环境下进行验证。