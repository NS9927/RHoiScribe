<!--
Thanks for contributing to RHoiScribe.
Use a focused pull request scope: one behavior, one knowledge update, one tool, or one documentation improvement.
-->

## Summary

<!-- What problem does this solve, and what changed? -->

## Related Issue

<!-- Link the issue if one exists. Example: Fixes #123 -->

## Change Type

<!-- Check every category that applies. -->

- [ ] Bug fix
- [ ] Knowledge catalog correction or expansion
- [ ] MCP prompt, resource, or tool change
- [ ] Documentation or setup guide update
- [ ] Packaging, release, or build change
- [ ] Other

## MCP Surface

<!-- If this changes prompts, resources, tools, or output fields, list them here. Otherwise write "No MCP surface changes." -->

- Prompts:
- Resources:
- Tools:
- Output fields:

## HOI4 Correctness Notes

<!-- Required for knowledge, generation, validation, or formatting changes. Include version assumptions, source references, and examples. -->

- HOI4 version or compatibility range:
- Source references:
- Relevant file types or code blocks:
- Edge cases considered:

## Documentation

<!-- RHoiScribe keeps user-facing docs synchronized. Check what applies. -->

- [ ] Root `README.md` was updated when user-facing behavior changed.
- [ ] `docs/README.zh-CN.md`, `docs/README.ru.md`, and `docs/README.ja.md` were checked or updated when the root README changed.
- [ ] `docs/client-setup.md` was checked or updated when MCP setup or tool usage changed.
- [ ] Documentation changes avoid hard-coding a single path or naming convention when user workspace conventions should take priority.
- [ ] Not applicable.

## Safety

<!-- Required for generation, scanning, validation, formatting, and write-mode changes. -->

- [ ] Generated paths are mod-root-relative and reject absolute paths, drive-prefixed paths, and traversal.
- [ ] Dry-run behavior is available or the change does not write files.
- [ ] Existing-file overwrite behavior is explicit.
- [ ] Identifier scans or duplicate checks distinguish structured HOI4 fields from plain text matches where relevant.
- [ ] `.mod` and `descriptor.mod` `replace_path` implications were considered where relevant.
- [ ] Localisation output preserves HOI4 encoding requirements where relevant.
- [ ] No secrets, private game files, or unrelated generated artifacts are included.
- [ ] Not applicable.

## Verification

<!-- Paste command output summaries or explain why a command was not run. -->

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test`
- [ ] `cargo build --release`

## Additional Notes

<!-- Add screenshots, generated-output previews, MCP client logs, or reviewer notes if useful. -->
