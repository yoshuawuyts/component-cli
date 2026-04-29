//! Rendering helpers for documenting the WIT → CLI mapping.
//!
//! The functions in this module produce **byte-stable** text from a
//! component's WIT and from the generated [`clap::Command`] tree.
//! They are designed to be consumed by `insta` snapshot tests so the
//! committed snapshots become the canonical, human-readable spec for
//! how each WIT type maps onto a CLI argument.

use std::fmt::Write as _;

use wit_component::WitPrinter;
use wit_parser::decoding::{DecodedWasm, decode};

use crate::wit::LibraryExtractError;
use crate::{build_clap, extract_library_surface};

/// Render the component's WIT package as a human-readable WIT
/// document, identical to what `wasm-tools component wit` would
/// print.
///
/// This is the format used by the snapshot tests because it lets
/// readers see exactly the WIT shape — including imports, nested
/// type definitions, and full world syntax — that `wit2cli` is
/// translating into a CLI.
///
/// # Errors
///
/// Returns [`LibraryExtractError::Decode`] if the component's WIT
/// can't be decoded.
pub fn render_wit_text(bytes: &[u8]) -> Result<String, LibraryExtractError> {
    let decoded = decode(bytes).map_err(|e| LibraryExtractError::Decode(e.to_string()))?;
    let (resolve, package_id) = match &decoded {
        DecodedWasm::WitPackage(resolve, package_id) => (resolve, *package_id),
        DecodedWasm::Component(resolve, world_id) => {
            let world = resolve.worlds.get(*world_id).ok_or_else(|| {
                LibraryExtractError::Decode("world id not in resolve".to_string())
            })?;
            let pkg = world.package.ok_or_else(|| {
                LibraryExtractError::Decode("component world has no package".to_string())
            })?;
            (resolve, pkg)
        }
    };
    let nested: Vec<_> = resolve
        .packages
        .iter()
        .filter(|(id, _)| *id != package_id)
        .map(|(id, _)| id)
        .collect();
    let mut printer = WitPrinter::default();
    printer
        .print(resolve, package_id, &nested)
        .map_err(|e| LibraryExtractError::Decode(format!("printing WIT: {e}")))?;
    Ok(printer.output.to_string())
}

/// Render a [`clap::Command`] tree as `--help` text.
///
/// Walks every sub-command depth-first and emits its `--help`
/// output, prefixed by the command path. The output is byte-stable
/// across runs (no ANSI escapes, no terminal-width-dependent
/// wrapping where avoidable).
#[must_use]
pub fn render_clap_tree(cmd: &clap::Command) -> String {
    let mut out = String::new();
    render_clap_node(&mut out, cmd, &[]);
    out
}

/// Render WIT bytes end-to-end: WIT package + generated CLI tree.
///
/// One-stop helper for snapshot tests.
///
/// # Errors
///
/// Forwards any error from [`render_wit_text`],
/// [`extract_library_surface`], or [`build_clap`].
pub fn render_mapping(bytes: &[u8]) -> Result<String, RenderMappingError> {
    let wit_text = render_wit_text(bytes).map_err(RenderMappingError::Extract)?;
    let surface = extract_library_surface(bytes).map_err(RenderMappingError::Extract)?;
    let cmd =
        build_clap(&surface, "<program>").map_err(|e| RenderMappingError::Build(e.to_string()))?;
    let mut out = String::new();
    out.push_str("=== WIT package ===\n");
    out.push_str(wit_text.trim_end());
    out.push_str("\n\n=== Generated CLI ===\n");
    out.push_str(&render_clap_tree(&cmd));
    Ok(out)
}

/// Errors raised by [`render_mapping`].
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RenderMappingError {
    /// Failed to extract the WIT surface.
    #[error(transparent)]
    Extract(LibraryExtractError),
    /// Failed to build the clap CLI from the surface.
    #[error("failed to build clap CLI: {0}")]
    Build(String),
}

fn render_clap_node(out: &mut String, cmd: &clap::Command, path: &[String]) {
    let display_path = if path.is_empty() {
        "<program>".to_string()
    } else {
        format!("<program> {}", path.join(" "))
    };
    let _ = writeln!(out, "$ {display_path} --help");
    let mut cmd = cmd.clone();
    let help = cmd.render_help();
    out.push_str(&help.to_string());
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out.push('\n');
    for sub in cmd.get_subcommands() {
        if sub.get_name() == "help" {
            // Skip clap's auto-generated `help` subcommand: its
            // output is uninformative and identical across crates.
            continue;
        }
        let mut next = path.to_vec();
        next.push(sub.get_name().to_string());
        render_clap_node(out, sub, &next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_clap_tree_walks_subcommands() {
        let cmd = clap::Command::new("test")
            .subcommand(clap::Command::new("foo").about("a foo"))
            .subcommand(clap::Command::new("bar").about("a bar"));
        let out = render_clap_tree(&cmd);
        assert!(out.contains("$ <program> --help"));
        assert!(out.contains("$ <program> foo --help"));
        assert!(out.contains("$ <program> bar --help"));
        assert!(out.contains("a foo"));
        assert!(out.contains("a bar"));
    }
}
