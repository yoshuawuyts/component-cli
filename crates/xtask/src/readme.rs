//! `cargo xtask readme` — update or check the README commands section.

use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

const COMMANDS_START: &str = "<!-- commands-start -->";
const COMMANDS_END: &str = "<!-- commands-end -->";

/// Build the wasm binary and return the path to it.
fn build_wasm_bin(workspace_root: &Path) -> Result<std::path::PathBuf> {
    // Clear RUSTFLAGS so that CI-specific flags like `-Dwarnings` do not cause
    // this build to fail with platform-specific warnings unrelated to the
    // README check itself.  Warnings are already checked by `cargo clippy`.
    let status = Command::new("cargo")
        .args(["build", "-p", "wasm"])
        .current_dir(workspace_root)
        .env_remove("RUSTFLAGS")
        .status()
        .context("failed to run `cargo build -p wasm`")?;

    if !status.success() {
        anyhow::bail!("`cargo build -p wasm` failed");
    }

    let bin_name = format!("wasm{}", std::env::consts::EXE_SUFFIX);
    Ok(workspace_root.join("target").join("debug").join(bin_name))
}

/// Run `wasm --help` and return the output, normalized for cross-platform use.
fn wasm_help(workspace_root: &Path) -> Result<String> {
    let bin = build_wasm_bin(workspace_root)?;
    let output = Command::new(&bin)
        .arg("--help")
        // Disable color so ANSI escape codes never appear in the output.
        .env("NO_COLOR", "1")
        // Remove COLUMNS so clap uses its default width on every platform.
        .env_remove("COLUMNS")
        .output()
        .with_context(|| format!("failed to run `{}`", bin.display()))?;

    let help = String::from_utf8_lossy(&output.stdout).into_owned();
    // Normalize trailing whitespace on every line.  clap may emit trailing
    // spaces on blank separator lines (e.g. `          \n`) on some platforms
    // and omit them on others.  Stripping them keeps the output identical
    // regardless of OS or clap version.
    let help = help
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        + if help.ends_with('\n') { "\n" } else { "" };
    // On Windows the binary is named "wasm.exe", which clap uses in the usage
    // line. Normalize to "wasm" so the README is platform-independent.
    Ok(help.replace("wasm.exe", "wasm"))
}

/// Format the help output as the markdown section content (between markers).
fn format_section(help: &str) -> String {
    format!("\n```\n{}\n```\n", help.trim_end())
}

/// Extract the section content currently in the README (between markers).
fn extract_section(readme: &str) -> Result<String> {
    let start = readme
        .find(COMMANDS_START)
        .context("README is missing the `<!-- commands-start -->` marker")?;
    let end = readme
        .find(COMMANDS_END)
        .context("README is missing the `<!-- commands-end -->` marker")?;

    Ok(readme[start + COMMANDS_START.len()..end].to_owned())
}

/// Replace the section between markers with new content.
fn replace_section(readme: &str, help: &str) -> Result<String> {
    let start = readme
        .find(COMMANDS_START)
        .context("README is missing the `<!-- commands-start -->` marker")?;
    let end = readme
        .find(COMMANDS_END)
        .context("README is missing the `<!-- commands-end -->` marker")?;

    let before = &readme[..start + COMMANDS_START.len()];
    let after = &readme[end..];

    Ok(format!("{}{}{}", before, format_section(help), after))
}

/// Update the README commands section from the current `wasm --help` output.
pub(crate) fn update(workspace_root: &Path) -> Result<()> {
    let help = wasm_help(workspace_root)?;
    let readme_path = workspace_root.join("README.md");
    let readme = std::fs::read_to_string(&readme_path).context("failed to read README.md")?;

    let updated = replace_section(&readme, &help)?;
    std::fs::write(&readme_path, updated).context("failed to write README.md")?;

    println!("✓ README commands section updated");
    Ok(())
}

/// Check that the README commands section matches `wasm --help`.
///
/// This is run as part of `cargo xtask test`. It requires the wasm binary to
/// already be built (e.g. via a prior `cargo test` or `cargo build` invocation).
pub(crate) fn check(workspace_root: &Path) -> Result<()> {
    let help = wasm_help(workspace_root)?;
    let readme_path = workspace_root.join("README.md");
    let readme = std::fs::read_to_string(&readme_path).context("failed to read README.md")?;

    let current = extract_section(&readme)?;
    let expected = format_section(&help);

    // Normalize line endings for cross-platform comparison.
    if current.replace("\r\n", "\n") != expected.replace("\r\n", "\n") {
        anyhow::bail!(
            "README commands section is out of date.\n\
             Run `cargo xtask readme update` to regenerate it."
        );
    }

    println!("✓ README commands section is up to date");
    Ok(())
}
