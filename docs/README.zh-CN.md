# RHoiScribe

[English](../README.md) | [Русский](README.ru.md) | [日本語](README.ja.md)

RHoiScribe 是一个面向 Hearts of Iron IV Modding AI agents 的本地 MCP 服务器。它为 Codex、Claude Code 和其他兼容 MCP 的客户端提供本地 HOI4 Modding 参考层，以及生成游戏可读文件的工具。

它的目标很明确：减少 agent 因重复联网搜索、过时假设、不安全路径、缺少本地化编码、以及“看起来像 Paradox 脚本但游戏无法加载”的内容造成的浪费。

## 适合谁

- 希望 AI agents 生成 HOI4 内容时拥有更好本地上下文的 Mod 作者。
- 需要把 prompts、resources、tools 统一接入一个 MCP server 的 agent 工作流。
- 离线或低搜索开发场景，要求 agent 写文件前先读取内置 HOI4 指引。
- 需要生成内容使用可预测 mod-root 路径和可审查输出结构的团队。

## Agents 能得到什么

### Prompts

内置 prompts 覆盖：

- Mod 功能规划
- HOI4 脚本编写
- 本地化编写
- GUI、GFX、scripted GUI 工作
- 生成内容审查

当前 prompt 名称包括 `hoi4_mod_planner`、`hoi4_script_writer`、`hoi4_localisation_writer`、`hoi4_gui_assistant`、`hoi4_review`。

### Resources

Agents 可以读取本地资源，而不是从空白提示开始：

- `rhoiscribe://hoi4/latest-update`
- `rhoiscribe://hoi4/knowledge/catalog`
- `rhoiscribe://hoi4/knowledge/<topic_id>`

知识目录为 agent 使用而结构化。Topic 包含分类、文件类型、标签、语法示例、与其他 HOI4 系统的关系、验证建议和来源引用。当前覆盖脚本基础、scope、trigger、effect、modifier、变量、数组、本地化、scripted localisation、scripted triggers/effects、GUI、scripted GUI、国策、事件、决议、理念、角色、历史、地图文件、科技、装备、单位、AI、外交、游戏规则、defines、书签、音频和常见加载错误。

### Tools

Agents 可以调用工具进行可重复的生成和验证：

- `generate_localisation_batch`
- `generate_focus_batch`
- `generate_event_batch`
- `generate_decision_batch`
- `validate_hoi4_paths`
- `format_paradox_script`

生成工具支持 dry-run 预览。写入模式需要 `output_root`，并且只按目标 Mod 根目录的相对路径写入。

## 快速开始

构建服务器：

```powershell
cargo build --release
```

在你的 MCP 客户端中使用 release binary：

```text
<ABSOLUTE_PATH_TO_RHOISCRIBE>\target\release\rhoiscribe.exe
```

Linux 和 macOS 使用：

```text
<ABSOLUTE_PATH_TO_RHOISCRIBE>/target/release/rhoiscribe
```

只有当你想手动启动 stdio MCP server 时，才需要直接运行：

```powershell
.\target\release\rhoiscribe.exe
```

Codex、Claude Code 和通用 MCP 配置示例见 [client-setup.md](client-setup.md)。

## MCP Surface

客户端启动 RHoiScribe 后，agent 可以使用标准 MCP 方法：

- `prompts/list`
- `prompts/get`
- `resources/list`
- `resources/read`
- `tools/list`
- `tools/call`

示例 resource read：

```text
rhoiscribe://hoi4/knowledge/scripted_gui.dynamic_lists
```

示例 `tools/call` 本地化 dry-run 参数：

```json
{
  "language": "l_english",
  "file_stem": "my_mod_focuses",
  "key_prefix": "MYMOD",
  "entries": [
    {
      "id": "industrial_recovery",
      "title": "Industrial Recovery",
      "description": "Rebuild the industrial base."
    }
  ],
  "dry_run": true
}
```

写入模式需要增加 Mod 输出根目录：

```json
{
  "language": "l_english",
  "file_stem": "my_mod_focuses",
  "entries": [
    {
      "id": "industrial_recovery",
      "title": "Industrial Recovery"
    }
  ],
  "dry_run": false,
  "output_root": "<MOD_OUTPUT_ROOT>"
}
```

写入模式下生成的本地化文件会使用 UTF-8 BOM。

## 输出模型

生成工具返回结构化文件计划：

```json
{
  "dry_run": true,
  "files": [
    {
      "path": "localisation/english/my_mod_focuses_l_english.yml",
      "encoding": "utf-8-bom",
      "summary": "HOI4 localisation file"
    }
  ],
  "messages": ["dry-run only; no files were written"]
}
```

路径是 Mod 相对路径。不安全路径、带盘符路径和目录穿越会在写入前被拒绝。

## 项目形态

RHoiScribe 使用 Rust 和 `rmcp` 构建，传输方式为 stdio。运行时资源从仓库内置内容提供，因此配置好的客户端无需实时联网即可读取 prompts、resources 和 tools。

当前版本形态偏实用：先提供 agent-facing 知识库和常见 HOI4 文件的批量生成器，再逐步扩展到更深的语义验证和更丰富的完整 Mod 生产工作流。
