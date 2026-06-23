# How it works

`calc` is **one scoring engine behind many surfaces**. A FeverPAIN score computed at the command line, inside a Tauri GUI, or by an LLM tool call is the same code path producing byte-identical output. This page explains the design that makes that true, and how to embed `calc` in your own application.

## One core, many surfaces

```text
              ┌────────────────────────────────┐
              │   calc-core (leaf crate)        │
              │   scoring logic + result schema │
              │   deps: serde, serde_json only  │
              │   NO host, NO async runtime     │
              └─────────────────┬──────────────┘
                                │  (every arrow points in)
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
   ┌─────────┐         ┌────────────────┐       ┌──────────────┐
   │ calc-cli│         │ host MCP server│       │ host GUI     │
   │ (lib+bin)│        │ (e.g. gitehr)  │       │ (e.g. Tauri) │
   │  `calc`  │        │ tools driven   │       │ direct calls │
   │          │        │ from registry  │       │ into core    │
   └─────────┘         └────────────────┘       └──────────────┘
        ▲                       ▲                       ▲
        └─ every surface enumerates calc_core::all() ──┘
```

The dependency arrows all point **into** `calc-core`; the core never depends on anything above it.

### Why this matters

- A calculator added to `calc_core::all()` surfaces **everywhere** with no per-surface code: a new CLI subcommand, a new MCP tool, a new GUI entry.
- Results are **identical by construction** across surfaces - there is no parallel implementation to drift.
- `calc-core` is **embeddable**. It has no opinions about your host, no global state, no clock, and no I/O.

## The crates

### `calc-core` - the leaf engine

Pure, deterministic scoring. The trait every calculator implements:

```rust
pub trait Calculator {
    fn name(&self) -> &'static str;             // stable; CLI subcommand / MCP tool name
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn reference(&self) -> &'static str;
    fn license(&self) -> CalculatorLicense;     // distribution licence + URL
    fn input_schema(&self) -> serde_json::Value;
    fn input_template(&self) -> serde_json::Value;
    fn calculate(&self, input: &serde_json::Value)
        -> Result<CalculationResponse, CalcError>;
}
```

`calc_core::all()` returns every registered calculator; `calc_core::get(name)` resolves one by name. Both are what every surface dispatches through.

**Leaf discipline**: `calc-core` depends only on `serde` and `serde_json`. Not on `tokio`, not on a host, not on a network library. This is non-negotiable and CI-enforced. It is what makes the calculators trivially auditable and trivially embeddable.

### `calc-cli` - CLI surface, library + binary

All CLI behaviour lives in `calc_cli::run` so host CLIs reuse it verbatim. The standalone `calc` binary is a 25-line wrapper around it. The same library is what a host like GitEHR calls for `gitehr calc`:

```rust
#[derive(clap::Subcommand)]
enum Commands {
    /// Clinical calculators
    Calc(calc_cli::CalcCommand),
}

// dispatch:
Commands::Calc(cmd) => calc_cli::run(cmd)?,
```

### `calc-web` - single-file HTML tools (deprioritised)

Self-contained per-calculator HTML files with a shared context-detection bridge. Currently inline JS; the planned end-state is the same `calc-core` compiled to WebAssembly so the browser surface shares the engine. Not on the active work list.

## The result shape

Every computed result, regardless of calculator, is a `CalculationResponse`:

```rust
pub struct CalculationResponse {
    pub calculator: String,        // machine name, e.g. "feverpain"
    pub result: serde_json::Value, // primary value (number or short string)
    pub interpretation: String,    // clinician-facing line(s)
    pub working: serde_json::Map<String, serde_json::Value>,
    pub reference: String,         // primary citation
}
```

- `working` captures every intermediate value, so the result is **re-derivable from the response alone** with no need to re-run the calculation.
- The shape is timestamp-free and id-free. A recording host adds those when it journals the result, so the same response can be deterministically tested and snapshotted.

## Input definitions

Several inputs to clinical scores are not numbers or enums but **clinician-asserted predicates** that are easy to get subtly wrong. CHA2DS2-VASc's "vascular disease", for example, is arterial and explicitly excludes venous thromboembolism.

For these inputs the schema carries a machine-readable definition - **includes**, **excludes**, a cited source, and a draft SNOMED ECL expression. The definition travels in the JSON Schema to every surface, so the CLI, an MCP tool, and a GUI form all show the **same** clinician-validated definition. See `spec/calculator-input-definitions.md` for the design.

## Unavailable on principle

Ten tools widely used in UK clinical practice are licence-locked: FRAX, MMSE, MUST, CAT, ACQ, ELF, CFS, LANSS, OHS, OKS. `calc` lists them, but invoking them returns a structured `unavailable` response rather than a score - with the owner, the reason, and the named open alternative where one exists.

This is deliberate. **Naming the gap is part of the project.** Clinical decision tools that public healthcare relies on should be open, free to use, and auditable; tools that are not should be visible as the licensing problem they are, not silently absent. See the [catalogue](calculators.md#unavailable-on-principle).

## Embedding `calc` in a host

Any application can pull in the engine. The minimum is a Cargo dependency on `calc-core`; CLIs and MCP servers usually also pull in `calc-cli`.

### As a CLI subcommand

`gitehr calc` is the worked example: the GitEHR binary's `Calc` variant flattens [`calc_cli::CalcCommand`] and dispatches to `calc_cli::run`. Adding `gitehr calc` was about a dozen lines and gave it the entire `calc` surface for free.

### As an MCP server

The same `calc_core::all()` registry maps trivially onto MCP tools: each calculator's `name()` becomes the tool name, `input_schema()` becomes the tool's `inputSchema`, and the tool body calls `calculate(value)`. The LLM and the human are working from the same contract.

### In a desktop GUI

A Tauri (or any) GUI calls `calc_core` over a native command. Inputs come from the form, results render directly - no webview-to-engine round trip, no JS reimplementation, no drift.

## Licensing

- **Code** (`calc-core`, `calc-cli`): AGPL-3.0-or-later. Deliberately not available for subsumption into proprietary EHRs; if that service needs to exist, it can be offered as a hosted Calc-API.
- **Algorithms**: most scores are public-domain methods implemented from primary literature. QRISK3 and QFracture are ported from ClinRisk's LGPL-3.0 source and carry the required disclaimer. Each calculator records its own distribution licence via `calc <name> --license`.
- **Clinical content** (source references): CC-BY-SA-4.0.

## Quality bars (CI-enforced)

Every commit, every PR:

- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test` (all calculators; literature-vector tests)
- A registry test asserting every calculator declares a non-empty distribution licence with an `http(s)` evidence URL.

That last test is the structural lock that means **a calculator cannot ship without its basis on record**.
