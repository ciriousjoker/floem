# Text editor core patch branch

`text-editor-core` contains the small downstream patch used by Text. The fork's
`main` branch remains an unmodified mirror of upstream.

The baseline is upstream commit `574638db95a4cf898f6156fa1db375de7a1d49c3`,
the published `floem-editor-core` 0.2.0 source. The MIT license is unchanged.

## Changes

- Preserve lone carriage returns when constructing a buffer.
- Preserve inserted LF, CRLF and CR rather than normalizing to the buffer's style.
- Expose `can_undo` and `can_redo`, matching the existing history cursor checks.

These changes preserve Text's exact note contents. They do not alter the other
explicit normalization/reload APIs. Text uses native layout, not Floem's line
navigation for lone CR characters.

## Maintenance

Keep `main` synchronized with `lapce/floem:main`. Evaluate upstream changes on the
patch branch against Text's supported toolchain and tests before integration.
A development-branch upgrade is a separate compatibility change, not a consequence
of extracting the previously vendored code. Never overwrite this patch branch
with a plain fork sync. Prefer a reviewed merge to preserve published commits.

Text pins a complete Git commit in Cargo.toml and Cargo.lock. A patch-branch
update does not silently update Text. Run the regression tests in
`editor-core/tests/text_preservation.rs` and Text's Rust tests before changing
its pin. Preserve line endings, undo/redo, composition and mixed syntax behavior.
If upstream needs an incompatible toolchain or API migration, report that
concrete blocker instead of silently upgrading the application.
