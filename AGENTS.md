# RHoiScribe Agent 工作规则

本仓库用于开发面向 AI Agents 的本地 MCP 服务：以 Rust 实现 HOI4 Modding 辅助能力，提供 prompts、resources、最新更新信息整理，以及批量生成游戏可读内容的 tools。

## 必须遵守的计划

- 开发前必须先读取 `plans/` 下的当前计划文件，并按计划顺序推进。
- `plans/` 是本地工作计划目录，不提交到 git；如果当前工作区缺少计划文件，先向维护者确认或重新生成本地计划，再继续实现。
- 未经明确确认，不得绕过计划添加大范围新功能或重构。
- 每完成一个计划步骤，必须运行对应验证命令，再使用 Conventional Commits 提交一次。

## 提交规范

- 使用 Conventional Commits，例如：
  - `chore: initialize rust mcp project`
  - `feat: add hoi4 resource catalog`
  - `test: cover localisation generator`
  - `docs: document mcp client setup`
- 提交必须只包含当前步骤相关文件。
- `plans/`、`target/`、临时输出和发行产物目录不得提交。

## 工程边界

- 优先保持 MCP 协议兼容，默认支持本地 stdio 启动，后续再扩展 Streamable HTTP。
- 面向 Roo Code、Codex、Claude Code 等可配置本地 MCP server 的客户端，避免依赖单一平台私有能力。
- HOI4 知识库内容必须可追溯、可版本化，并与生成工具解耦。
- 生成 HOI4 文件时要优先保证游戏可读性、路径规范、编码规范和可批量验证。
