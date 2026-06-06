# MCP Client Setup

RHoiScribe is designed as a local MCP server launched through stdio. Build the release binary first:

```powershell
cargo build --release
```

Default binary paths:

- Windows: `target\release\rhoiscribe.exe`
- Linux: `target/release/rhoiscribe`
- macOS: `target/release/rhoiscribe`

The binary supports:

```powershell
target\release\rhoiscribe.exe --help
target\release\rhoiscribe.exe --version
```

## Roo Code

Add RHoiScribe as a stdio MCP server in Roo Code's MCP settings. Use the absolute path to the release binary.

```json
{
  "mcpServers": {
    "rhoiscribe": {
      "command": "D:\\GitHubProjects\\RHoiScribe\\target\\release\\rhoiscribe.exe",
      "args": []
    }
  }
}
```

## Codex

Add RHoiScribe to the Codex MCP server configuration for the repository or user config, using the local binary as the command.

```toml
[mcp_servers.rhoiscribe]
command = "D:\\GitHubProjects\\RHoiScribe\\target\\release\\rhoiscribe.exe"
args = []
```

The Codex manual fetch was unavailable during this snapshot because the official manual endpoint returned HTTP 403 on 2026-06-06, so verify the exact config location against your installed Codex surface if it differs.

## Claude Code

Claude Code can register local stdio MCP servers from the CLI. Use an absolute path to the release binary.

```powershell
claude mcp add rhoiscribe -- D:\GitHubProjects\RHoiScribe\target\release\rhoiscribe.exe
```

## Generic MCP JSON

Many MCP-compatible clients accept this shape:

```json
{
  "mcpServers": {
    "rhoiscribe": {
      "command": "/absolute/path/to/rhoiscribe",
      "args": []
    }
  }
}
```

## Runtime Behavior

- Transport: stdio.
- Network: no runtime network access is required.
- Resources: served from bundled local snapshots under `knowledge/`.
- Tools: generation tools support `dry_run`; write mode requires `output_root`.
