# Agent instructions

`calc` is an open, standalone library of clinical calculators: a pure Rust scoring engine (`calc-core`) and a CLI (`calc-cli`) that drive every surface from one source of truth. It is reusable by anyone with no knowledge of GitEHR; GitEHR is one downstream consumer.

This file (with the matching `CLAUDE.md`, which points here) is the entry point for any coding agent working on this repository. Read it before changing anything.

## Read first

- `spec/calculators.md` - the architecture (one core, many surfaces).
- `spec/calculator-roadmap.md` - the tool list, build priority, and post-Tier-5 candidates (MedikQuantis-derived).
- `spec/calculator-input-definitions.md` - the governed input-definition system for clinician-asserted predicates.
- `spec/multilingual.md` - design for future multilingual support (English-only today; locale shape agreed).
- `calc-core/src/tags.rs` - the central tag taxonomy used by `calc list --tag` and the docs catalogue. Add new entries here, not per-calculator.
- `docs/how-it-works.md` - the architecture phrased for users.
- `~/code/house-style/AGENTS.md` - the cross-repository engineering standards (CI, distribution, docs, licensing, scripts). Source of truth; this file does not duplicate it.

## The leaf rule (non-negotiable)

`calc-core` is a **strict leaf crate**. It depends only on `serde` and `serde_json`.

- Do not add other dependencies to it.
- Do not make it depend on an async runtime (no `tokio`, no `async-std`).
- Do not make it depend on any host application.
- Do not put I/O, clocks, randomness, or global state in it.

This discipline is what makes the calculators trivially embeddable and trivially auditable. Adding a dependency to `calc-core` requires explicit go-ahead.

## The `Calculator` contract

Every calculator implements the full [`Calculator`](calc-core/src/calculator.rs) trait, including `license()` (a registry test enforces a non-empty licence with an `http(s)` evidence URL - a calculator that omits it does not ship).

Additionally, every calculator exposes a strongly-typed `Input`/`compute` pair and a `build_response` adapter in its own module; the trait is the dynamic JSON surface, `compute` is the typed Rust API.

Scoring must be **verified against primary sources** and unit-tested with literature vectors. Do not reverse-engineer scoring from a competitor's implementation - implement from the cited publication.

## One registry, every surface

Adding a calculator to `calc_core::all()` surfaces it everywhere - the `calc` CLI, any MCP host, any GUI - with **no per-surface code**. There is no per-calculator clap struct, no per-calculator MCP tool definition, no per-calculator GUI form. If you find yourself writing per-calculator dispatch code, stop and rethink.

The CLI surface (one shape for all 42 calculators) is documented in [`docs/cli-reference.md`](docs/cli-reference.md).

## House style

- Hyphens, not emdashes, in prose. Slug-case-with-hyphens for filenames (except recognised conventions like `README.md`, `AGENTS.md`, `Cargo.toml`).
- Markdown is not hard-wrapped: one long line per paragraph, soft wrap.
- Conventional commits (`feat(calc):`, `fix:`, `docs:`, `chore(deps):`, `ci:`).
- Workspace version is single-sourced in the root `Cargo.toml`.

## Before every commit

```bash
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test
```

CI enforces all three. Never commit red.

## GitHub Actions

Every action step is pinned to a **full commit SHA** with a `# vX.Y.Z` comment afterwards (the `pin-github-action` convention). Before adding a new action, fetch its repository to confirm the current latest stable tag - do not pin from memory. See `.github/workflows/` for the living examples; copy from sibling repos (`dsc`, `gitehr`, `sct`) rather than improvising.

## Dependencies

Before writing or bumping any dependency, fetch the package's official repository or registry to confirm the latest stable version. Never rely on memory.

## Adding a calculator

1. Implement it under `calc-core/src/calculators/<name>.rs`:
   - A typed `Input` struct (`#[derive(serde::Deserialize)]`).
   - A pure `compute()` that returns a typed result.
   - A `build_response()` adapter producing a `CalculationResponse`.
   - A `Calculator` impl with `input_schema()` and `license()` (both required).
   - Unit tests against literature vectors.
2. Register it in `calc_core::all()`.
3. That is the only Rust work - the CLI, MCP, and GUI surfaces pick it up automatically.
4. If it should appear in the docs catalogue, add a row to `docs/calculators.md`.

The skill at `.claude/skills/build-calculator/` covers the detail (and may be retired in favour of `spec/` + `examples/` + this file in due course).

## Roadmap snapshot

- `cargo-dist` release pipeline (then publish `calc-cli` to crates.io and the `pacharanero/homebrew-tap`).
- Tauri desktop GUI whose headline is prominent copy-paste ("soft interoperability").
- `calc-web` (single-file HTML) is on the roadmap but deprioritised.

Project lives at <https://github.com/pacharanero/calc>. Docs are deployed to <https://pacharanero.github.io/calc/> from `.github/workflows/deploy-docs-to-ghpages.yml`.
