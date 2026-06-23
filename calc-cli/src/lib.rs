// SPDX-FileCopyrightText: 2026 Marcus Baw and Baw Medical Ltd
// SPDX-License-Identifier: AGPL-3.0-or-later

//! # calc-cli
//!
//! The command-line surface for the open clinical calculators. This crate is
//! the single source of CLI behaviour: the standalone `calc` binary
//! (`src/main.rs`) and any host CLI that embeds it (e.g. GitEHR's `gitehr calc`)
//! both drive [`CalcCommand`] + [`run`], so there is nothing to re-implement
//! when embedding it.
//!
//! ## One regular surface for every calculator
//!
//! There are no per-calculator flags. Every calculator is driven through the
//! same registry-backed shape, so adding a calculator to `calc-core` gives it a
//! working CLI for free, and a human or an LLM learns the interface once:
//!
//! ```text
//! calc list                       # list available calculators
//! calc <name>                     # print a fillable INPUT TEMPLATE
//! calc <name> --schema            # print the JSON Schema (the full contract)
//! calc <name> --input -           # compute, reading JSON from stdin
//! calc <name> --input data.json   # compute, reading JSON from a file
//! calc <name> --input '{...}'     # compute, reading an inline JSON string
//! ```
//!
//! The template printed by `calc <name>` has the same shape as the input
//! `calc <name> --input` expects: fill in the placeholder values and pass it
//! back. Computing always requires an explicit `--input`, so a bare invocation
//! never blocks reading stdin.
//!
//! To embed in a host CLI (e.g. gitehr):
//!
//! ```ignore
//! #[derive(clap::Subcommand)]
//! enum Commands {
//!     // ...
//!     /// Clinical calculators
//!     Calc(calc_cli::CalcCommand),
//! }
//! // ...
//! Commands::Calc(cmd) => calc_cli::run(cmd)?,
//! ```

use std::io::Read;
use std::path::Path;

use anyhow::{Result, anyhow};
use clap::{Args, ValueEnum};

use calc_core::CalculationResponse;

/// How to render computed results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable text.
    #[default]
    Text,
    /// Machine-readable JSON (the `CalculationResponse` schema).
    Json,
}

/// The `calc` command surface. Reused unchanged by host CLIs such as `gitehr calc`.
///
/// A single shape covers discovery, schema, and compute for every calculator;
/// the calculator is selected by `name` and looked up in the `calc-core`
/// registry at runtime.
#[derive(Debug, Args)]
pub struct CalcCommand {
    /// Calculator machine name (e.g. `feverpain`). Omit, or use `list`, to see
    /// all available calculators.
    pub name: Option<String>,

    /// Compute a result from this JSON input: `-` for stdin, a file path, or an
    /// inline JSON string. Without it, a fillable input template is printed.
    #[arg(long, value_name = "JSON|FILE|-")]
    pub input: Option<String>,

    /// Print the calculator's JSON Schema (the full input contract) instead of a
    /// template.
    #[arg(long)]
    pub schema: bool,

    /// Print the calculator's distribution licence and the URL evidencing it.
    #[arg(long)]
    pub license: bool,

    /// Output format for computed results and `list`.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}

/// Dispatch a parsed [`CalcCommand`].
pub fn run(cmd: CalcCommand) -> Result<()> {
    // No name (or `list`) means: show the catalogue.
    let name = match cmd.name.as_deref() {
        None | Some("list") => return print_list(cmd.format),
        Some(n) => n,
    };

    let calc = calc_core::get(name)
        .ok_or_else(|| anyhow!("unknown calculator: {name} (try `calc list`)"))?;

    // `--schema` prints the formal contract, regardless of everything else.
    if cmd.schema {
        println!("{}", serde_json::to_string_pretty(&calc.input_schema())?);
        return Ok(());
    }

    // `--license` prints the algorithm's distribution licence and evidence URL.
    if cmd.license {
        println!("{}", serde_json::to_string_pretty(&calc.license())?);
        return Ok(());
    }

    match cmd.input.as_deref() {
        // No input: print a fillable template and explain how to pass it back.
        None => {
            let schema = calc.input_schema();
            println!("{}", serde_json::to_string_pretty(&calc.input_template())?);
            // If the schema has `oneOf` alternatives, the template shows only
            // the first - flag the others so they're discoverable without
            // having to read the full schema.
            if let Some(note) = oneof_alternatives_note(&schema) {
                eprintln!("\n{note}");
            }
            eprintln!(
                "\nReplace each placeholder with a value, then compute with one of:\n  \
                 calc {name} --input <file.json>\n  \
                 calc {name} --input '<json>'\n  \
                 calc {name} --input -        # read JSON from stdin\n\
                 See the full input contract with: calc {name} --schema"
            );
            Ok(())
        }
        // Input supplied: validate (via the calculator's typed deserialization)
        // and compute.
        Some(src) => {
            let input = read_input(src)?;
            let response = calc.calculate(&input).map_err(|e| anyhow!("{e}"))?;
            emit(&response, cmd.format)
        }
    }
}

/// Resolve an `--input` argument to a JSON value.
///
/// `-` reads stdin; an existing file path is read from disk; anything else is
/// treated as an inline JSON string.
fn read_input(src: &str) -> Result<serde_json::Value> {
    let raw = if src == "-" {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else if Path::new(src).is_file() {
        std::fs::read_to_string(src)?
    } else {
        src.to_string()
    };

    serde_json::from_str(&raw)
        .map_err(|e| anyhow!("invalid JSON input: {e}\nSee the expected shape with: calc <name>"))
}

fn print_list(format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => {
            let items: Vec<_> = calc_core::all()
                .iter()
                .map(|c| {
                    let lic = c.license();
                    serde_json::json!({
                        "name": c.name(),
                        "title": c.title(),
                        "description": c.description(),
                        "license": lic.license,
                        "license_source": lic.source_url,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&items)?);
        }
        OutputFormat::Text => {
            for c in calc_core::all() {
                println!("{:<12}  {}", c.name(), c.title());
            }
        }
    }
    Ok(())
}

fn emit(response: &CalculationResponse, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(response)?),
        OutputFormat::Text => println!("{}", render_text(response)),
    }
    Ok(())
}

/// Render a result as a clinician-facing text block.
fn render_text(r: &CalculationResponse) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{} = {}\n\n",
        r.calculator,
        value_to_string(&r.result)
    ));
    out.push_str(&r.interpretation);
    if !r.working.is_empty() {
        out.push_str("\n\nWorking:");
        for (k, v) in &r.working {
            out.push_str(&format!("\n  {k}: {}", value_to_string(v)));
        }
    }
    out.push_str(&format!("\n\nReference: {}", r.reference));
    out
}

fn value_to_string(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// If the schema declares top-level `oneOf` alternative input shapes (each
/// with its own `required` array), build a one-paragraph note listing them.
///
/// The template printed by `calc <name>` shows only the first alternative;
/// this note tells the reader what else is permitted, so they don't have to
/// read the full schema to discover the other shapes.
fn oneof_alternatives_note(schema: &serde_json::Value) -> Option<String> {
    let alts = schema.get("oneOf")?.as_array()?;
    let groups: Vec<Vec<String>> = alts
        .iter()
        .filter_map(|alt| {
            alt.get("required").and_then(|r| r.as_array()).map(|r| {
                r.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
        })
        .filter(|g: &Vec<String>| !g.is_empty())
        .collect();

    if groups.len() < 2 {
        return None;
    }

    let mut out =
        String::from("This calculator accepts more than one input shape. Pick exactly one:\n");
    for (i, g) in groups.iter().enumerate() {
        let marker = if i == 0 { "shown above" } else { "alternative" };
        out.push_str(&format!("  {}: {}    ({marker})\n", i + 1, g.join(" + ")));
    }
    Some(out.trim_end().to_string())
}

#[cfg(test)]
mod tests {
    use super::oneof_alternatives_note;
    use serde_json::json;

    #[test]
    fn oneof_note_lists_each_alternative() {
        let schema = json!({
            "type": "object",
            "oneOf": [
                { "required": ["acr", "acr_unit"] },
                { "required": ["albumin", "creatinine"] }
            ]
        });
        let note = oneof_alternatives_note(&schema).unwrap();
        assert!(note.contains("acr + acr_unit"));
        assert!(note.contains("albumin + creatinine"));
        assert!(note.contains("shown above"));
    }

    #[test]
    fn no_oneof_yields_no_note() {
        assert!(oneof_alternatives_note(&json!({"type": "object"})).is_none());
    }
}
