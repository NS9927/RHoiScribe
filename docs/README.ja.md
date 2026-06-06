# RHoiScribe

[English](../README.md) | [简体中文](README.zh-CN.md) | [Русский](README.ru.md)

RHoiScribe は、Hearts of Iron IV の Modding を行う AI agents 向けのローカル MCP サーバーです。Codex、Claude Code、その他の MCP-compatible clients に、ローカルの HOI4 Modding 参照レイヤーと、ゲームが読めるファイルを生成する tools を提供します。

目的は明確です。繰り返しの Web 検索、古い前提、安全でないパス、localisation のエンコーディング漏れ、そして「Paradox script らしく見えるがゲームでは読み込めない」内容による agent の無駄を減らします。

## 対象ユーザー

- AI agents により良いローカル文脈で HOI4 コンテンツを生成させたい Mod 作者。
- prompts、resources、tools を 1 つの MCP server にまとめたい agent workflows。
- オフラインまたは低検索の開発セッションで、agent がファイルを書く前に内蔵 HOI4 guidance を読む必要があるケース。
- 生成物に予測可能な mod-root path とレビューしやすい出力形式を求めるチーム。

## Agents が得られるもの

### Prompts

内蔵 prompts は次を支援します。

- Mod feature planning
- HOI4 script writing
- localisation writing
- GUI、GFX、scripted GUI work
- generated-content review

現在の prompt 名は `hoi4_mod_planner`、`hoi4_script_writer`、`hoi4_localisation_writer`、`hoi4_gui_assistant`、`hoi4_review` です。

### Resources

Agents は空の prompt から始める代わりに、ローカル resources を読めます。

- `rhoiscribe://hoi4/latest-update`
- `rhoiscribe://hoi4/knowledge/catalog`
- `rhoiscribe://hoi4/knowledge/<topic_id>`

Knowledge catalog は agent 向けに構造化されています。Topics には category、file types、tags、syntax examples、他の HOI4 systems との relationships、validation guidance、source references が含まれます。現在の範囲は script basics、scopes、triggers、effects、modifiers、variables、arrays、localisation、scripted localisation、scripted triggers/effects、GUI、scripted GUI、focuses、events、decisions、ideas、characters、history、map files、technology、equipment、units、AI、diplomacy、game rules、defines、bookmarks、audio、common loading errors です。

### Tools

Agents は反復可能な生成と検証のために tools を呼び出せます。

- `generate_localisation_batch`
- `generate_focus_batch`
- `generate_event_batch`
- `generate_decision_batch`
- `validate_hoi4_paths`
- `format_paradox_script`

Generation tools は dry-run preview をサポートします。write mode では `output_root` が必要で、対象 Mod の root からの相対 path にのみ書き込みます。

## クイックスタート

サーバーをビルドします。

```powershell
cargo build --release
```

MCP クライアントで release binary を指定します。

```text
<ABSOLUTE_PATH_TO_RHOISCRIBE>\target\release\rhoiscribe.exe
```

Linux と macOS では次を使います。

```text
<ABSOLUTE_PATH_TO_RHOISCRIBE>/target/release/rhoiscribe
```

stdio MCP server を手動で起動したい場合だけ直接実行します。

```powershell
.\target\release\rhoiscribe.exe
```

Codex、Claude Code、汎用 MCP 設定例は [client-setup.md](client-setup.md) を参照してください。

## MCP Surface

クライアントが RHoiScribe を起動した後、agent は標準 MCP methods を使えます。

- `prompts/list`
- `prompts/get`
- `resources/list`
- `resources/read`
- `tools/list`
- `tools/call`

Resource read の例:

```text
rhoiscribe://hoi4/knowledge/scripted_gui.dynamic_lists
```

localisation dry-run 用の `tools/call` 引数例:

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

Write mode では Mod output root を追加します。

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

write mode で生成される localisation file は UTF-8 BOM で書き込まれます。

## 出力モデル

Generation tools は構造化された file plan を返します。

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

Paths は Mod 相対です。安全でない path、drive prefix 付き path、directory traversal は書き込み前に拒否されます。

## プロジェクト形態

RHoiScribe は Rust と `rmcp` で構築され、stdio transport を使います。Runtime resources はリポジトリからバンドルされるため、設定済みクライアントは live network access なしで prompts、resources、tools を読めます。

現在のリリース形態は実用性を優先しています。まず agent-facing knowledge base と一般的な HOI4 files の batch generators を提供し、その後 deeper semantic validation と full mod production workflows 向けの richer generators へ拡張します。
