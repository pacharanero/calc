<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# `calc-core`

Pure clinical-calculator scoring engine. The library every surface in the [`calc`](https://github.com/pacharanero/calc) project shares: the [`calc`](https://github.com/pacharanero/calc) CLI, the desktop GUI, and any host (e.g. [GitEHR](https://github.com/gitehr/gitehr)) that embeds clinical scoring.

```toml
[dependencies]
calc-core = "0.1"
```

## What it is

A leaf crate that knows how to compute a clinical score and nothing else. It deliberately depends only on `serde` and `serde_json` - no async runtime, no host application, no I/O, no clocks, no global state.

That discipline is what makes the calculators trivially embeddable and trivially auditable: pull in `calc-core`, get the same 42 active calculators and 10 named-but-unavailable proprietary stubs as the standalone `calc` CLI.

## Quick start

```rust
use calc_core::{Calculator, get};
use serde_json::json;

let feverpain = get("feverpain").expect("known calculator");
let response = feverpain.calculate(&json!({
    "fever": true,
    "purulence": true,
    "attend_rapidly": true,
    "inflamed_tonsils": false,
    "absence_of_cough": false,
})).expect("valid input");

println!("{} = {}", response.calculator, response.result);
// feverpain = 3
println!("{}", response.interpretation);
// A score of 3 is associated with 34-40% isolation of streptococcus...
```

## Enumerating the registry

```rust
for calc in calc_core::all() {
    println!("{:<14} {}", calc.name(), calc.title());
}
```

`calc_core::all()` returns every registered calculator (boxed trait objects). `calc_core::get(name)` resolves one by machine name (`"feverpain"`, `"news2"`, `"qrisk3"`, ...).

## The Calculator trait

```rust
pub trait Calculator {
    fn name(&self) -> &'static str;           // stable machine name
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn reference(&self) -> &'static str;      // primary citation
    fn license(&self) -> CalculatorLicense;   // distribution licence + URL
    fn input_schema(&self) -> serde_json::Value;
    fn input_template(&self) -> serde_json::Value;
    fn calculate(&self, input: &serde_json::Value)
        -> Result<CalculationResponse, CalcError>;
    fn tags(&self) -> &'static [&'static str];
}
```

Every calculator implements the trait dynamically (via `dyn Calculator` in the registry) **and** exposes a strongly-typed `Input`/`compute` pair in its own module for ergonomic compile-time-checked use directly from Rust.

## Tags

Each calculator carries specialty + status tags (`cardiology`, `nephrology`, `screening`, `nhs-mandated`, `proprietary`, `unavailable`, ...). Use them to filter or group:

```rust
let nephro: Vec<_> = calc_core::all()
    .into_iter()
    .filter(|c| c.tags().contains(&"nephrology"))
    .collect();
```

The full vocabulary lives in [`tags.rs`](https://github.com/pacharanero/calc/blob/main/calc-core/src/tags.rs).

## Result shape

Every computed result is a `CalculationResponse`:

```rust
pub struct CalculationResponse {
    pub calculator: String,        // machine name
    pub result: serde_json::Value, // number for most scores, string for categoricals
    pub interpretation: String,    // clinician-facing prose
    pub working: serde_json::Map<String, serde_json::Value>,
    pub reference: String,         // primary citation
}
```

`working` captures every intermediate value, so the response is re-derivable without re-running the calculation. The shape is **timestamp-free and id-free** - a host adds those when journaling.

## Licensing

- Code: **AGPL-3.0-or-later**. Deliberately not available for subsumption into proprietary EHRs.
- Each calculator additionally records the distribution licence of its **clinical algorithm**, surfaced via `calc.license()` - either an SPDX identifier (e.g. `LGPL-3.0` for QRISK3/QFracture) or a short description (e.g. `Public-domain method`), with a reverifiable source URL.

## Calculators

42 active calculators ranging from primary-care screens (PHQ-9, GAD-7, AUDIT, FeverPAIN) through acute-care (NEWS2, qSOFA, CURB-65, Wells) to specialist scores (GRACE, EuroSCORE II, QRISK3, QFracture, MELD, UKELD). 10 proprietary scores (FRAX, MMSE, MUST, CAT, ACQ, ELF, CFS, LANSS, OHS, OKS) are deliberately listed as `unavailable` stubs, naming the gap as a first-class object and pointing at the open alternative where one exists. See the [catalogue](https://pacharanero.github.io/calc/calculators/).

## Related crates

- [`calc-cli`](https://github.com/pacharanero/calc/tree/main/calc-cli) - the standalone `calc` binary and a reusable library for host CLIs (e.g. `gitehr calc`).
- [`calc-gui`](https://github.com/pacharanero/calc/tree/main/gui) - Tauri 2 desktop GUI (not published).

## Links

- [Repository](https://github.com/pacharanero/calc)
- [Documentation site](https://pacharanero.github.io/calc/)
- [Architecture spec](https://github.com/pacharanero/calc/blob/main/spec/calculators.md)
- [API docs](https://docs.rs/calc-core)
