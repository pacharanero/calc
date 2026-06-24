// SPDX-FileCopyrightText: 2026 Marcus Baw and Baw Medical Ltd
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * Typed wrappers around the Tauri `invoke` channel.
 *
 * The shapes here match the Rust types in `src-tauri/src/lib.rs` exactly -
 * if you change the Rust side, change these in the same commit. Both sides
 * ultimately defer to `calc-core`, so the canonical contract is the Rust
 * struct, not the TypeScript interface.
 */

import { invoke } from "@tauri-apps/api/core";

/** One catalogue entry (mirrors `CalcSummary` in lib.rs). */
export interface CalcSummary {
  name: string;
  title: string;
  description: string;
  tags: string[];
  /** True for the 10 proprietary "unavailable" stubs. */
  proprietary: boolean;
}

/** A computed result (mirrors `calc_core::CalculationResponse`). */
export interface CalculationResponse {
  calculator: string;
  /** Number for most scores; a short string for categorical results. */
  result: number | string;
  interpretation: string;
  /** Every intermediate value the score depended on, snake_case keys. */
  working: Record<string, unknown>;
  reference: string;
}

export async function listCalculators(): Promise<CalcSummary[]> {
  return invoke<CalcSummary[]>("list_calculators");
}

export async function calculate(
  name: string,
  input: Record<string, unknown>,
): Promise<CalculationResponse> {
  return invoke<CalculationResponse>("calculate", { name, input });
}
