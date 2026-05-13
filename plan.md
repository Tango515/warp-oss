# Settings 子页面中文化补全计划

日期：2026-05-13

价格/日期核对：Billing and usage 页面涉及 Warp Add-on Credits 和结算日期文案。按 2026-05-13 的核对，当前 Add-on Credits 档位需要按 `$10/400 credits`、`$20/1,000 credits`、`$50/3,000 credits`、`$100/6,500 credits` 处理；页面里的 `billing date`、`Usage resets on ...`、`Resets ...` 等动态日期文案也要一起中文化，不能只翻译静态标题。

## 当前进度审查（2026-05-13）

本轮复查发现上一轮 agent 的翻译仍未完成，并且存在会阻塞编译的问题：

- `app/src/settings_view/environments_page.rs` 删除了 `PAGE_TITLE_TEXT` / `PAGE_DESCRIPTION_TEXT` 常量但仍继续引用，已修正。
- `app/src/settings_view/main_page.rs` 将 `MessageId` 传给 `LoginGatedFeature`，类型不匹配，已修正为静态字符串。
- `README.md` 和 `images/local-ai-provider.png`、`images/multilingual.png` 是用户主动要求保留的改动，不属于本计划的回退范围。
- 静态扫描中的主要 Settings 英文残留已继续补齐，包含 Platform API Key、Shared blocks、Appearance 数值项、AI/Agents、Environments 空状态、动态 `Resets ...`、折扣和镜像 tooltip。

结论：不能标记为“全部完成”。当前状态是“主要阻塞已修正，仍需构建/运行验收”。

## 截图确认仍为英文的页面目录映射（2026-05-13）

用户在运行版截图中确认以下页面仍有大量英文，需要让后续 agent 按“侧边栏目录 -> 对应页面 -> 源码文件”逐项处理：

| 侧边栏目录名称 | 对应页面/截图标题 | 主要英文残留示例 | 需要修改的代码目录和文件 | 翻译状态 |
| --- | --- | --- | --- | --- |
| 智能体 > 知识 | `Knowledge` | `Rules help the Warp Agent follow your conventions...`（L5386）、`Learn more`（L5389）— 通过 `FormattedTextFragment` 渲染，绕过翻译；`Rules`、`Suggested Rules`、`Warp Drive as agent context`、`Manage rules` 已通过 helper → `settings_display_text` 路径 | `app/src/settings_view/ai_page.rs`（L5357-5525，`AIFactWidget`） | 大部分已翻译，仅 FormattedText 内联段落需补 |
| 代码 > 编辑器和代码审查 | `代码编辑器和审查` / `Editor and Code Review` | `Choose an editor to open file links`（external_editor L283）、`Group files into single editor pane`（external_editor L28 const）、`Show code review button`（code_page L2359）、`Project explorer`（L2443）、`Global file search`（L2486）、`Auto open code review panel`（L2283）、`Show diff stats on code review button`（L2401）+ 各描述段落 | `app/src/settings_view/code_page.rs`；共享编辑器控件在 `app/src/settings_view/features/external_editor.rs` | 全部 raw hardcoded，需逐项接入 `settings_display_text` |
| 团队 | `团队` / 创建团队页 | `Domains, comma separated`（L96）、`Emails, comma separated`（L97）、`Contact support`（L2063）、`Manage billing`（L2092）、`Open admin panel`（L2123）、`Reset links`（L2385）、`Manage plan`（L3086）、`update your payment information`（L133）— 大部分 render helper 已翻译，以上绕过 helper 的 raw 字符串需补 | `app/src/settings_view/teams_page.rs` | 主体已翻译，9 处 raw 字符串 + toast 消息需补 |
| 外观 | `外观` / Themes、Icon、Window、Input | Category 名称全部 raw：`Themes`（L1261）、`Icon`（L1270）、`Window`（L1314）、`Input`（L1326）、`Panes`（L1329）、`Blocks`（L1343）、`Text`（L1372）、`Cursor`（L1375）、`Tabs`（L1421）、`Full-screen Apps`（L1424）；toggle/description：`Create your own custom theme`（L2618）、`Current theme`（L2655）、`Automatically switch...`（L2802）、`Customize your app icon`（L2858）、`Window Opacity`（L3053/3080）、`Window Blur Radius`（L3185）、`Input type`（L3382）、`Dim inactive panes`（L3533）、`Focus follows mouse`（L3576）、`Compact mode`（L3624）等 | `app/src/settings_view/appearance_page.rs` | 全部 raw hardcoded（约 40+ 字符串），仅 Text/Font 子节用了 `settings_display_text` |
| 功能 | `功能` / General、Session | `Default mode for new sessions`（features_page L6931，经 `render_dropdown_item_label` 未翻译）；`Grace period (seconds)`（undo_close L140）；`Advanced`（working_directory L324）；`Home directory`/`Previous session's directory`/`Custom directory`（working_directory_config L46-48）；其余 `Show sticky command header`、`Show tooltip on click on links`、`Maximum rows in a block`、`Default shell for new sessions`、`Working directory for new sessions` 已通过 render helper 翻译 | `app/src/settings_view/features_page.rs`；子控件在 `app/src/settings_view/features/working_directory.rs`、`app/src/settings_view/features/undo_close.rs`、`app/src/settings_view/features/working_directory_config.rs` | 大部分已翻译，4 处 raw 需补 |
| Warpify | `Warpify` | `SSH session detection for Warpification`（L62）、`The tmux ssh wrapper works...`（L80 const）、`Controls the installation behavior...`（L82 const）、`Subshells supported: bash, zsh, and fish.`（L181）、`Warpify your interactive SSH sessions.`（L195）、`Configure whether Warp attempts to "Warpify"...`（L542）、`Learn more`（L546）、`Added commands`（L607）、`Denylisted commands`（L619）、`Warpify SSH Sessions`（L692）、`Install SSH extension`（L726）、`Use Tmux Warpification`（L752）、`Denylisted hosts`（L805） | `app/src/settings_view/warpify_page.rs` | 14 处 raw hardcoded（仅 3 个 placeholder 已翻译） |

执行要求：

- 先把这些页面中仍硬编码的 `.label(...)`、`.paragraph(...)`、`Text::new(...)`、`with_subtitle(...)` 等接入 `settings_display_text` / `settings_display_text_static`。
- 再在 `app/src/settings_view/settings_page.rs::settings_zh_cn_literal` 中补对应中文映射。
- 下拉选项如 `Default App`、`Split Pane`、`Terminal`、`Previous session's directory` 也要一起检查；如果来自枚举 display helper，需要在 helper 或字典层补翻译。
- `Warpify`、`Warp Drive`、`Shell (PS1)`、`SSH`、`MCP`、`API`、`URL` 等产品名/协议名可保留英文，普通说明句和按钮/开关标题必须中文化。

## 翻译机制

- `app/src/i18n/mod.rs`：用于导航标题、部分页面标题和已有 `MessageId`。
- `app/src/settings_view/settings_page.rs::settings_zh_cn_literal`：用于 Settings 页面内大量历史硬编码字符串的临时映射。
- 页面内已有 `settings_display_text(...)` / `settings_display_text_static(...)` 的地方优先补字典；仍然硬编码的 `.label(...)`、`Text::new(...)`、常量 `&str` 需要先接入翻译函数。

## 需要继续复验的页面和文件

### P0：Billing and usage [已补主要残留，需运行复验]

文件：

- `app/src/settings_view/billing_and_usage_page.rs`
- `app/src/settings_view/billing_and_usage/overage_limit_modal.rs`
- `app/src/settings_view/billing_and_usage/usage_history_entry.rs`
- `app/src/settings_view/settings_page.rs`

复验重点：

- Add-on Credits 价格、折扣、自动重新加载、月度限额、用量重置日期是否显示中文。
- 分段富文本里的链接前后缀是否仍有英文残留。

### P0：Agents umbrella 全部子页面 [已补一轮，需逐页运行复验]

文件：

- `app/src/settings_view/ai_page.rs`
- `app/src/settings_view/execution_profile_view.rs`
- `app/src/settings_view/local_ai_provider_page.rs`
- `app/src/settings_view/mcp_servers_page.rs`
- `app/src/settings_view/mcp_servers/list_page.rs`
- `app/src/settings_view/mcp_servers/edit_page.rs`
- `app/src/settings_view/mcp_servers/server_card.rs`
- `app/src/settings_view/mcp_servers/installation_modal.rs`
- `app/src/settings_view/mcp_servers/update_modal.rs`
- `app/src/settings_view/mcp_servers/destructive_mcp_confirmation_dialog.rs`
- `app/src/settings_view/settings_page.rs`

复验重点：

- Warp Agent、Profiles、MCP Servers、Knowledge、Local AI Provider、Third party CLI agents 的标题、说明、按钮、空状态、更新弹窗。
- 代码里保留的产品名、协议名、模型名、Provider 名称不强制翻译。

### P0：Cloud platform 子页面 [已修编译阻塞，需运行复验]

文件：

- `app/src/settings_view/environments_page.rs`
- `app/src/settings_view/environments_page/new_environment_button.rs`
- `app/src/settings_view/update_environment_form.rs`
- `app/src/settings_view/delete_environment_confirmation_dialog.rs`
- `app/src/settings_view/agent_assisted_environment_modal.rs`
- `app/src/settings_view/platform_page.rs`
- `app/src/settings_view/platform/create_api_key_modal.rs`
- `app/src/settings_view/platform/expire_api_key_button.rs`
- `app/src/settings_view/settings_page.rs`

复验重点：

- Environments 空状态、搜索、创建表单、仓库输入 placeholder、镜像 tooltip。
- Oz Cloud API Keys 的创建弹窗、类型选择、过期时间、创建成功/删除提示。

### P1：Appearance [已补扫描残留，需运行复验]

文件：

- `app/src/settings_view/appearance_page.rs`
- `app/src/settings_view/directory_color_add_picker.rs`
- `app/src/settings_view/settings_page.rs`

复验重点：`Line height`、`Font weight`、`Font size (px)` 等可见项中文化，`px` 单位保留。

### P1：Features / Teams / Privacy / Shared blocks / Warp Drive / Referrals / Warpify [需运行复验]

文件：

- `app/src/settings_view/features_page.rs`
- `app/src/settings_view/features/external_editor.rs`
- `app/src/settings_view/features/working_directory.rs`
- `app/src/settings_view/features/startup_shell.rs`
- `app/src/settings_view/teams_page.rs`
- `app/src/settings_view/privacy_page.rs`
- `app/src/settings_view/show_blocks_view.rs`
- `app/src/settings_view/warp_drive_page.rs`
- `app/src/settings_view/referrals_page.rs`
- `app/src/settings_view/warpify_page.rs`
- `app/src/settings_view/settings_page.rs`

复验重点：常见状态、空状态、错误状态、弹窗按钮和说明段落无主要英文 UI 文案。

### P2：Account / Keybindings / Language / About / 通用 Settings 框架

文件：

- `app/src/settings_view/main_page.rs`
- `app/src/settings_view/keybindings.rs`
- `app/src/settings_view/language_page.rs`
- `app/src/settings_view/about_page.rs`
- `app/src/settings_view/settings_page.rs`
- `app/src/settings_view/settings_file_footer.rs`
- `app/src/settings_view/mod.rs`
- `app/src/i18n/mod.rs`

复验重点：顶层导航、搜索结果、设置同步、登录/升级拦截、设置文件页脚。

## 验收步骤

1. 运行 `git diff --check`。
2. 运行可用的 Rust 检查；当前本机若仍缺 Metal Toolchain，`cargo check -p warp --lib` 会被 `xcodebuild -downloadComponent MetalToolchain` 前置环境问题阻塞，需要记录为环境阻塞而非翻译代码通过。
3. 启动应用后逐页打开 Settings，按上面的 P0/P1/P2 页面人工确认中文显示。
4. 若出现英文残留，优先判断是品牌/协议/模型名还是普通 UI 文案；普通 UI 文案继续接入 `settings_display_text` 并补 `settings_zh_cn_literal`。
