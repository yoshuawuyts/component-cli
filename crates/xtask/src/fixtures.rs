//! `cargo xtask fixtures` — build / verify the `.wasm` test fixtures
//! used by `component-cli`'s integration tests.
//!
//! The committed fixtures under
//! `crates/component-cli/tests/fixtures/library_*.wasm` are produced
//! from the `cargo-component` source crates under
//! `crates/component-cli/tests/fixtures/sources/`. Tests load the
//! prebuilt artifacts directly so contributors who don't have
//! `cargo-component` installed can still run `cargo test`.
//!
//! - `cargo xtask fixtures rebuild` — recompile each source crate and
//!   copy the resulting `.wasm` over the committed artifact.
//! - `cargo xtask fixtures check` — recompile into a temp dir and
//!   compare the printed WIT text of the rebuilt artifact against the
//!   committed one. We compare WIT (via `wit-component::WitPrinter`)
//!   rather than raw bytes because `cargo-component` output is not
//!   byte-stable across toolchain versions.

#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::workspace_root;

/// Per-fixture metadata: source crate dir name and the resulting
/// committed `.wasm` filename.
struct Fixture {
    /// Directory name under `tests/fixtures/sources/`.
    source: &'static str,
    /// Filename of the cargo-component output (under
    /// `target/wasm32-wasip1/release/`).
    artifact: &'static str,
    /// Filename of the committed fixture under `tests/fixtures/`.
    committed: &'static str,
}

const FIXTURES: &[Fixture] = &[
    Fixture {
        source: "library-wordmark",
        artifact: "library_wordmark.wasm",
        committed: "library_wordmark.wasm",
    },
    Fixture {
        source: "library-kitchen-sink",
        artifact: "library_kitchen_sink.wasm",
        committed: "library_kitchen_sink.wasm",
    },
    Fixture {
        source: "library-resources",
        artifact: "library_resources.wasm",
        committed: "library_resources.wasm",
    },
    Fixture {
        source: "library-needs-import",
        artifact: "library_needs_import.wasm",
        committed: "library_needs_import.wasm",
    },
];

/// Path from the workspace root to the directory holding the source
/// crates.
const SOURCES_REL: &str = "crates/component-cli/tests/fixtures/sources";
/// Path from the workspace root to the directory holding the committed
/// `.wasm` fixtures.
const FIXTURES_REL: &str = "crates/component-cli/tests/fixtures";

/// Check that `cargo-component` is on PATH; print a friendly hint and
/// return `false` if it isn't, so callers can soft-skip in CI.
fn require_cargo_component() -> bool {
    let status = Command::new("cargo")
        .args(["component", "--version"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    match status {
        Ok(s) if s.success() => true,
        _ => {
            println!(
                "warning: `cargo-component` is not installed; skipping fixture build/check.\n\
                 Install it with: cargo install cargo-component"
            );
            false
        }
    }
}

/// Build a single fixture in `source_dir`, returning the path to the
/// produced `.wasm` artifact.
fn build_fixture(source_dir: &Path, fixture: &Fixture) -> Result<PathBuf> {
    println!("building fixture: {}", fixture.source);
    let status = Command::new("cargo")
        .args(["component", "build", "--release"])
        .current_dir(source_dir)
        .status()
        .with_context(|| format!("running cargo-component for {}", fixture.source))?;
    if !status.success() {
        bail!("cargo component build failed for {}", fixture.source);
    }
    let artifact = source_dir
        .join("target")
        .join("wasm32-wasip1")
        .join("release")
        .join(fixture.artifact);
    if !artifact.exists() {
        bail!(
            "expected build artifact {} does not exist",
            artifact.display()
        );
    }
    Ok(artifact)
}

/// `cargo xtask fixtures rebuild` — recompile each source crate and
/// overwrite the committed fixture with the fresh artifact.
pub(crate) fn rebuild() -> Result<()> {
    if !require_cargo_component() {
        return Ok(());
    }
    let root = workspace_root()?;
    let sources = root.join(SOURCES_REL);
    let dest = root.join(FIXTURES_REL);

    for fixture in FIXTURES {
        let source_dir = sources.join(fixture.source);
        let artifact = build_fixture(&source_dir, fixture)?;
        let target = dest.join(fixture.committed);
        fs::copy(&artifact, &target)
            .with_context(|| format!("copying {} → {}", artifact.display(), target.display()))?;
        println!("  → {}", target.display());
    }
    println!("All fixtures rebuilt.");
    Ok(())
}

/// Decode a `.wasm` binary and return its printable WIT text.
fn wit_text(bytes: &[u8]) -> Result<String> {
    let decoded = wit_parser::decoding::decode(bytes).context("decoding WIT from wasm bytes")?;
    let resolve = decoded.resolve();
    let pkg_id = decoded.package();
    let nested: Vec<_> = resolve
        .packages
        .iter()
        .filter(|(id, _)| *id != pkg_id)
        .map(|(id, _)| id)
        .collect();
    let mut printer = wit_component::WitPrinter::default();
    printer
        .print(resolve, pkg_id, &nested)
        .context("printing WIT")?;
    Ok(printer.output.to_string())
}

/// `cargo xtask fixtures check` — verify each committed fixture's
/// WIT matches what its source crate produces today.
pub(crate) fn check() -> Result<()> {
    if !require_cargo_component() {
        return Ok(());
    }
    let root = workspace_root()?;
    let sources = root.join(SOURCES_REL);
    let dest = root.join(FIXTURES_REL);

    let mut drift = Vec::new();
    for fixture in FIXTURES {
        let source_dir = sources.join(fixture.source);
        let rebuilt_path = build_fixture(&source_dir, fixture)?;
        let committed_path = dest.join(fixture.committed);
        if !committed_path.exists() {
            bail!(
                "committed fixture {} is missing; run `cargo xtask fixtures rebuild`",
                committed_path.display()
            );
        }
        let rebuilt = fs::read(&rebuilt_path)
            .with_context(|| format!("reading {}", rebuilt_path.display()))?;
        let committed = fs::read(&committed_path)
            .with_context(|| format!("reading {}", committed_path.display()))?;

        let rebuilt_wit = wit_text(&rebuilt)
            .with_context(|| format!("decoding rebuilt {}", rebuilt_path.display()))?;
        let committed_wit = wit_text(&committed)
            .with_context(|| format!("decoding committed {}", committed_path.display()))?;

        if rebuilt_wit == committed_wit {
            println!("  ✓ {}", fixture.committed);
        } else {
            drift.push((fixture.source, committed_wit, rebuilt_wit));
        }
    }

    if !drift.is_empty() {
        for (source, committed, rebuilt) in &drift {
            println!("\n--- {source}: WIT drift ---");
            println!("--- committed ---\n{committed}");
            println!("--- rebuilt ---\n{rebuilt}");
        }
        bail!(
            "{} fixture(s) have drifted from their sources; run `cargo xtask fixtures rebuild`",
            drift.len()
        );
    }
    println!("All fixtures match their sources.");
    Ok(())
}
