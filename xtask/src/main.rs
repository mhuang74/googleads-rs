//! xtask: canonical validation entry points for googleads-rs.
//!
//! Zero external dependencies; shells out to cargo via `std::process::Command`.
//! Run as `cargo xtask <subcommand>` (see `.cargo/config.toml` alias).
//!
//! Subcommands: `fmt`, `check`, `test`, `clippy`, and the aggregate `validate`
//! that runs all four, fail-fast.

use std::env;
use std::process::{Command, ExitCode};

/// Repo root: the parent of the xtask crate directory.
fn repo_root() -> &'static str {
    env!("CARGO_MANIFEST_DIR").trim_end_matches("/xtask")
}

const USAGE: &str = "usage: cargo xtask <fmt|check|test|clippy|validate>";

fn main() -> ExitCode {
    let subcommand = match env::args().nth(1) {
        Some(arg) => arg,
        None => return usage_failure(),
    };

    let subcommand: &str = &subcommand;
    let checks: &[&str] = match subcommand {
        "fmt" | "check" | "test" | "clippy" => std::slice::from_ref(&subcommand),
        "validate" => &["fmt", "check", "test", "clippy"],
        _ => return usage_failure(),
    };

    // Fail-fast: the first failing check stops with cargo's own exit code.
    for check in checks {
        let code = run_check(check);
        if code != 0 {
            return ExitCode::from(u8::try_from(code).unwrap_or(1));
        }
    }
    ExitCode::SUCCESS
}

fn usage_failure() -> ExitCode {
    eprintln!("{USAGE}");
    ExitCode::FAILURE
}

/// Shell out to cargo for one check, propagating its exit code.
fn run_check(check: &str) -> i32 {
    let mut command = Command::new("cargo");
    command.current_dir(repo_root());
    match check {
        "fmt" => {
            println!("Running: fmt");
            command.args(["fmt", "--", "--check"]);
        }
        "check" => {
            println!("Running: check");
            command.args(["check", "--all-targets", "--all-features"]);
        }
        "test" => {
            println!("Running: test");
            command.args(["test", "--all-targets", "--all-features"]);
        }
        "clippy" => {
            println!("Running: clippy");
            command.args(["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"]);
        }
        other => {
            eprintln!("{USAGE}");
            eprintln!("unknown subcommand: {other}");
            return 1;
        }
    }

    match command.status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(error) => {
            eprintln!("error: failed to spawn cargo: {error}");
            1
        }
    }
}