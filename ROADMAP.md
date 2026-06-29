# Roadmap

Engineering, infrastructure, and product-level work for the `calc` project. **This file is the home for everything that is not a new calculator.** The clinical-calculator backlog lives in [`spec/calculator-roadmap.md`](spec/calculator-roadmap.md).

## Status legend

- `[x]` **Completed** - shipped and verified.
- `[~]` **In-progress** - actively being worked on or dormant pending a one-off setup.
- `[ ]` **Planned** - committed to build; the next batch.
- `[ ]` **Future** - under consideration; promote to Planned when scheduled.

---

## Distribution & release

### Completed

- [x] **AGPL-3.0-or-later** + SPDX headers on every Rust source.
- [x] **Conventional commits** (`feat(scope):`, `fix:`, `docs:`, ...) used throughout.
- [x] **Single-sourced workspace version** in root `Cargo.toml`.
- [x] **Per-calculator `license()` registry test** enforces a non-empty distribution licence with an `http(s)` evidence URL.
- [x] **`Cargo.lock` committed** for reproducible builds.

### In-progress

- [~] **Publish `calc-core` to crates.io via `release-plz`** - the unblocker for downstream consumers (notably [GitEHR](https://github.com/gitehr/gitehr), whose own release-plz workflow cannot follow git deps). Workflow, config, and CHANGELOG seed are committed and dormant. Activation needs two repo secrets:
    - `RELEASE_PLZ_TOKEN` - fine-grained PAT with `contents:write` + `pull-requests:write` on this repo.
    - `CARGO_REGISTRY_TOKEN` - crates.io API token with publish permissions on `calc-core`.

    Once both are set, the next push to `main` opens a Release PR; merging it bumps the version, regenerates `CHANGELOG.md` from conventional commits, tags `vX.Y.Z`, creates the GitHub Release, and pushes to crates.io. `calc-cli` rides the same workspace version but is held back from crates.io (see Planned, below). See [`release-plz.yml`](.github/workflows/release-plz.yml) and [`release-plz.toml`](release-plz.toml).

### Planned

- [ ] **`cargo-dist` for binary releases of `calc-cli`** - prebuilt installers for macOS / Linux / Windows, then flip `publish = true` in `release-plz.toml` so `cargo install calc-cli` from crates.io also works without `--git`.
- [ ] **`pacharanero/homebrew-tap` formula** - shared tap, same as sibling repos (`sct`, `dsc`, `gitehr`).
- [ ] **Windows code-signing** - EV cert from Sectigo / SSL.com once procured. The cert covers `sct`, `dsc`, **and** `calc` in one purchase; see [`spec/gui.md`](spec/gui.md#windows-code-signing). Until then the GUI installer triggers SmartScreen on first run.

### Future

- [ ] **`cargo binstall` metadata** so `cargo binstall calc-cli` finds the prebuilt archives.
- [ ] **Install one-liners** (`install.sh`, `install.ps1`) hosted at the docs-site root.
- [ ] **deb / rpm / Scoop** packaging - only if user demand surfaces.

---

## Desktop GUI

See the design spec at [`spec/gui.md`](spec/gui.md) and the implementation guide at [`gui/README.md`](gui/README.md).

### Completed

- [x] **MVP scaffold** - Tauri 2 + React 19 + Mantine 8 + Vite 7, matching the GitEHR house-style frontend stack (IBM Plex Sans + Space Grotesk, teal primary).
- [x] **FeverPAIN UI** - first hand-crafted calculator end-to-end, with the headline paste-ready clipboard summary card.
- [x] **AppShell + sidebar** with tag-aware filter and Featured / All split.
- [x] **`ErrorBoundary`** so render errors show a fallback page, not a blank webview.

### Planned

- [ ] **CHA₂DS₂-VASc UI** - first non-trivial calculator (enums, age band); templates the next class of widget.
- [ ] **QRISK3 UI** - the politically-motivated one (still missing from EMIS and SystmOne); 22 mixed-type inputs.
- [ ] **Decide CI build cadence for the GUI** - on every push (slow, ~5 min) vs on release tag only. Probably the latter once cargo-dist lands.

### Future

- [ ] **Updater** - Tauri's built-in updater speaking to a manifest hosted on the docs site.
- [ ] **iOS / Android builds** - Tauri 2 supports them and `calc-core` is pure Rust, so this is mostly a packaging question.
- [ ] **Theme parity with GitEHR** - shared CSS variables or a tiny `@calc/ui-tokens` package, so the two apps stay visually coherent without copy-paste drift.

---

## Authoring workflow & docs

### Completed

- [x] **Zensical docs site** deployed to GitHub Pages via `actions/upload-pages-artifact` + `actions/deploy-pages` (no `gh-pages` branch). See [`.github/workflows/docs.yml`](.github/workflows/docs.yml).
- [x] **`AGENTS.md`** + canonical `s/` script directory (`s/docs`, `s/install`, `s/gui-dev`).
- [x] **Tag taxonomy** in `calc-core/src/tags.rs` + `calc list --tag` CLI filter + tag-aware docs catalogue.
- [x] **`spec/`** structure: `calculators.md` (architecture), `calculator-roadmap.md` (calculator backlog), `gui.md` (desktop), `multilingual.md` (locale design), `calculator-input-definitions.md`.
- [x] **`docs/walkthrough.md`** with copy-paste demos drawn from committed `examples/*.json`.

### Planned

- [ ] **Retire `.claude/skills/build-calculator/`** in favour of `spec/` + `examples/` + `AGENTS.md` as the authoring entry point. Skill is Claude-specific; new authoring path should work in any agent.
- [ ] **Per-crate `README.md` for `calc-cli`** matching the calc-core one, ready for if/when calc-cli publishes.

### Future

- [ ] **`docs/translating.md`** - contribution path for translators once multilingual lands.
- [ ] **API reference for `calc-core`** linked from the docs site (docs.rs handles this automatically once published; just need a link from the Zensical nav).

---

## Engine & embedding

### Completed

- [x] **Leaf rule** for `calc-core` (serde-only, no async, no I/O, no host); CI-enforced by the dependency tree.
- [x] **Schema-driven input templates** so the CLI surface is regular across all 52 calculators with no per-calculator clap struct.
- [x] **Proprietary "unavailable" stub treatment** - 10 named-not-hidden entries with structured `unavailable` responses naming the open alternative.
- [x] **Input-definition system** for clinician-asserted predicates (`spec/calculator-input-definitions.md`).

### Future

- [ ] **Multilingual support** - implement `Locale` enum + `LocalizedString` per [`spec/multilingual.md`](spec/multilingual.md). Validate with one calculator (FeverPAIN) and a native speaker before opening the catalogue for batched translation.
- [ ] **Translation reciprocity with [MedikQuantis](https://medikquantis.me)** - their Catalan/Spanish strings for the 14 overlapping calculators are exactly what we need; agree a shared tag taxonomy and citation shape so either project can ingest the other's metadata.
- [ ] **Reference MCP server** in this repo. Today, embedding hosts roll their own from `calc_core::all()`.
- [ ] **`calc-web`** (single-file HTML calculators) returning, ideally with `calc-core` compiled to WebAssembly so the browser surface shares the engine.
- [ ] **FHIR Observation export** for standardised exchange of results.
- [ ] **Unit conversion** (metric ↔ imperial) at the input boundary; today `--input` is units-explicit per field.
- [ ] **Printable / clipboard-friendly result formatting** beyond the existing text block (rich Markdown with citation links? PDF? RTF?).
- [ ] **Plugin system** for user-defined / third-party calculators loaded at runtime.
- [ ] **Guideline-update registry** - a mechanism to re-verify each calculator's licence and reference URL on a schedule.
- [ ] **High-risk-score alerts** as events embedding hosts can subscribe to (e.g. NEWS2 ≥ 7).

---

## Calculator backlog

See [`spec/calculator-roadmap.md`](spec/calculator-roadmap.md).

At time of writing: 42 active + 10 proprietary stubs shipped; 36+ Future candidates queued (chiefly from MedikQuantis, plus the recently-added [StatinMD](https://www.thelancet.com/journals/landig/article/PIIS2589-7500\(26\)00047-6/fulltext)).
