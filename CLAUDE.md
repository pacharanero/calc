# Agent instructions

This repository is the GitEHR clinical calculators: a pure Rust scoring engine (`calc-core`) and a CLI (`calc-cli`) that drive every surface from one source of truth.

- Read `spec/` for context: `calculators.md` (architecture), `calculator-roadmap.md` (the tool list), and `calculator-input-definitions.md` (the governed input-definition system).
- `calc-core` is a strict leaf crate: it depends only on `serde` and `serde_json`. Do not add other dependencies to it, and never make it depend on an async runtime or on any host application.
- Every calculator must implement the full `Calculator` trait, including `license()` (a registry test enforces this). Verify scoring against primary sources and add literature-vector unit tests.
- Adding a calculator to `calc_core::all()` surfaces it everywhere (the `calc` CLI and any MCP host) with no per-surface code.
- House style: hyphens, not emdashes, in prose. Run `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` before committing; CI enforces all three.
