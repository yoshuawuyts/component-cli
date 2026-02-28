//! `cargo xtask test` — run tests, clippy, and formatting checks.

use anyhow::Result;

use crate::{run_command, sql};

pub(crate) fn run_tests() -> Result<()> {
    println!("Running cargo test...");
    run_command("cargo", &["test", "--all"])?;

    println!("\nRunning cargo clippy...");
    run_command("cargo", &["clippy", "--all", "--", "-D", "warnings"])?;

    println!("\nRunning cargo fmt check...");
    run_command("cargo", &["fmt", "--all", "--", "--check"])?;

    println!("\nRunning sql check...");
    sql::check()?;

    println!("\n✓ All checks passed!");
    Ok(())
}
