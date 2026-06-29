# Changelog

All notable changes to this project are documented here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versioning follows [SemVer](https://semver.org/spec/v2.0.0.html).

Once [release-plz](https://release-plz.dev/) is activated (see [`ROADMAP.md`](ROADMAP.md#distribution--release) for the activation steps), each release PR will append the new section automatically from conventional commits, and this file becomes the source of truth for the GitHub Release notes.

## [Unreleased]

Pre-release development. The CLI, engine, docs site, and desktop-GUI MVP all ship under this banner. The first published `0.1.x` release will collapse everything below into a tagged entry.

### Added

- `calc-core` (engine): 42 active calculators + 10 proprietary "unavailable" stubs; `Calculator` trait with mandatory licence; schema-driven input templates; tag taxonomy.
- `calc-cli` (`calc` binary): one regular CLI surface across every calculator (`list`, `--schema`, `--license`, `--input`, `--tag`, `--format`); SIGPIPE-clean stdout.
- Zensical documentation site at <https://pacharanero.github.io/calc/>.
- Tauri 2 desktop GUI with FeverPAIN as the MVP, paste-ready clipboard summary as the headline feature.
- Brand: teal palette (`#0f766e`) + `function-variant` logo; deliberately off NHS Blue.

## [0.1.0] - unreleased

Initial release (placeholder; release-plz will rewrite this header on first publication).
