// SPDX-FileCopyrightText: 2026 Marcus Baw and Baw Medical Ltd
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The standalone `calc` binary.
//!
//! A thin wrapper over [`calc_cli`]: all behaviour lives in the library so host
//! CLIs (e.g. GitEHR's `gitehr calc`) can reuse it without repetition.

use clap::Parser;

use calc_cli::CalcCommand;

/// Open clinical calculators - scoring at the command line.
#[derive(Debug, Parser)]
#[command(name = "calc", version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    command: CalcCommand,
}

fn main() -> anyhow::Result<()> {
    // Reset SIGPIPE on Unix so output pipes cleanly into `head`/`less` instead
    // of producing a "Broken pipe" panic when the reader closes early. The
    // house style requires this for CLIs whose stdout is meant to be piped.
    #[cfg(unix)]
    // SAFETY: setting the default disposition for SIGPIPE in a single-threaded
    // start-up context is the canonical pattern; no signal handlers run yet.
    unsafe {
        libc_signal_default_sigpipe();
    }

    let cli = Cli::parse();
    calc_cli::run(cli.command)
}

#[cfg(unix)]
unsafe fn libc_signal_default_sigpipe() {
    // Avoid pulling in the `libc` crate just for this: use the raw syscall via
    // the std-exposed signal number constants. `signal(SIGPIPE, SIG_DFL)`.
    unsafe extern "C" {
        fn signal(signum: i32, handler: usize) -> usize;
    }
    const SIGPIPE: i32 = 13;
    const SIG_DFL: usize = 0;
    unsafe {
        signal(SIGPIPE, SIG_DFL);
    }
}
