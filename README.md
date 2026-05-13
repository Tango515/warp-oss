# Warp Multilingual Local AI

这是一个基于 [Warp 开源项目](https://github.com/warpdotdev/warp) 修改的社区 fork。上游 Warp 是一个面向开发者的终端和 Agentic Development Environment，本项目不重新复制官方介绍，建议先阅读上游仓库了解原始功能、架构和许可证。

本 fork 的重点是：

- 解锁并改造 AI 能力，支持接入个人或第三方 OpenAI 兼容 API。
- 增加简体中文界面，提供中英文语言切换。
- 保留 Warp 原有终端、智能体、第三方 CLI Agent、设置页等基础能力，并在此基础上做本地化和可配置化改造。

> [!IMPORTANT]
> 本项目不是 Warp 官方版本，也不隶属于 Warp 官方团队。请自行承担使用、构建和发布风险，并遵守上游项目的许可证要求。

## 主要改动

### 本地 AI Provider

新增 `本地 AI Provider` 设置页，可直接配置 OpenAI 兼容接口：

- `Base URL`：例如 `https://api.example.com/v1`
- `模型`：例如 `gpt-4`、`mimo-v2.5` 或你的服务商提供的模型名
- `API 密钥`：用于访问个人或第三方 API

保存后，Warp 智能体相关请求会优先走你配置的本地/第三方 AI Provider，从而减少对 Warp 官方 AI 订阅入口的依赖。

![本地 AI Provider](images/local-ai-provider.png)

### 简体中文界面

新增语言设置页，目前支持：

- English
- 简体中文

切换到简体中文后，设置页导航、部分设置内容、菜单栏和主要 AI 相关页面会显示中文。部分动态内容或上游尚未整理的文案可能仍保留英文，后续可以继续补全。

![多语言设置](images/multilingual.png)

## 使用方式

### 配置第三方 AI

1. 打开 `Settings`。
2. 进入 `智能体` -> `本地 AI Provider`。
3. 填写 `Base URL`、`模型` 和 `API 密钥`。
4. 点击 `保存`。
5. 回到 Agent 页面测试智能体能力。

配置会写入 Warp 的 settings 文件中，结构类似：

```toml
[agents.local_provider]
enabled = true
base_url = "https://api.example.com/v1"
model = "your-model"
api_key = "your-api-key"
```

### 切换语言

1. 打开 `Settings`。
2. 进入 `语言`。
3. 选择 `English` 或 `简体中文`。
4. 部分文案可能需要重新打开窗口后刷新。

## 构建

本项目仍沿用上游 Warp 的 Rust/Cargo 构建体系。首次构建前建议阅读：

- [WARP.md](WARP.md)
- [CONTRIBUTING.md](CONTRIBUTING.md)
- [上游 Warp 仓库](https://github.com/warpdotdev/warp)

常用命令：

```bash
./script/bootstrap
./script/run
./script/presubmit
```

macOS 构建需要完整 Xcode、Metal Toolchain 和 Rust 环境。Windows 便携包构建可参考仓库内的 Windows packaging workflow。

## 与上游的关系

本项目来源于：

<https://github.com/warpdotdev/warp>

当前 fork 的目标不是替代 Warp 官方版本，而是在开源代码基础上探索：

- 更开放的 AI Provider 配置方式
- 面向中文用户的本地化体验
- 可自行构建和分发的社区版本

如果你需要官方支持、官方同步更新或完整商业服务，请使用 Warp 官方版本。

## 许可证

本项目继承上游 Warp 的许可证结构：

- `warpui_core` 和 `warpui` crates 使用 [MIT license](LICENSE-MIT)。
- 仓库其余代码使用 [AGPL v3](LICENSE-AGPL)。

请在分发、修改或二次发布时遵守对应许可证。

## 免责声明

本项目仅用于学习、研究和个人使用场景。项目不提供任何第三方 API Key，不代理任何模型服务，也不保证与 Warp 官方云端服务兼容。
