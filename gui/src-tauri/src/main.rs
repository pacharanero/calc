// SPDX-FileCopyrightText: 2026 Marcus Baw and Baw Medical Ltd
// SPDX-License-Identifier: AGPL-3.0-or-later

// Prevents an extra console window on Windows release builds. Tauri
// convention; safe to keep on every target.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    calc_gui_lib::run()
}
