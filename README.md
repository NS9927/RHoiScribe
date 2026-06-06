# RHoiScribe

RHoiScribe is a local MCP server for HOI4 Modding agents. It provides bundled prompts, resources, and batch generation tools so agents can work from local HOI4 knowledge instead of repeatedly searching the web or producing files that do not match game-readable conventions.

## Current Capabilities

- MCP stdio server implemented in Rust with `rmcp`.
- Built-in prompt catalog:
  - `hoi4_mod_planner`
  - `hoi4_script_writer`
  - `hoi4_localisation_writer`
  - `hoi4_gui_assistant`
  - `hoi4_review`
- Built-in resources:
  - local HOI4 latest-update snapshot
  - HOI4 knowledge catalog
  - per-topic HOI4 syntax and path guidance
- Built-in tools:
  - `generate_localisation_batch`
  - `generate_focus_batch`
  - `generate_event_batch`
  - `generate_decision_batch`
  - `validate_hoi4_paths`
  - `format_paradox_script`

## Build

```powershell
cargo build --release
```

The release binary is:

```text
target/release/rhoiscribe.exe
```

On Linux and macOS the binary is `target/release/rhoiscribe`.

## Run

Run with no arguments to start the MCP server over stdio:

```powershell
target\release\rhoiscribe.exe
```

CLI flags:

```powershell
target\release\rhoiscribe.exe --help
target\release\rhoiscribe.exe --version
```

## Client Setup

See [docs/client-setup.md](docs/client-setup.md) for Roo Code, Codex, Claude Code, and generic MCP configuration examples.

## Knowledge Snapshot Updates

Runtime does not require network access. Bundled knowledge lives under [knowledge/hoi4](knowledge/hoi4). To refresh the latest HOI4 update snapshot, update [knowledge/hoi4/latest-update.md](knowledge/hoi4/latest-update.md) from official Paradox or Steam sources, then run verification and commit the change.

## Verification

Run the full local quality gate:

```powershell
.\scripts\verify.ps1
```

Equivalent commands:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

## Repository Rules

Agents must read [AGENTS.md](AGENTS.md) before development. Local implementation plans live in `plans/` and are intentionally ignored by git.
