//! Snapshot tests documenting the WIT → CLI mapping for each
//! committed `.wasm` fixture.
//!
//! These snapshots are the canonical, end-user-facing spec for how
//! `wit-clap` translates WIT exports into a `clap` sub-CLI. If you
//! change the mapping rules, the snapshot diff will surface every
//! visible consequence — review carefully before accepting.
//!
//! Update workflow:
//! ```text
//! INSTA_UPDATE=always cargo test -p wit-clap --test snapshots
//! cargo insta review
//! ```

use wit_clap::LibraryExtractError;
use wit_clap::snapshot::{RenderMappingError, render_mapping};

fn fixture_bytes(name: &str) -> Vec<u8> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);
    std::fs::read(&path).unwrap_or_else(|e| {
        panic!(
            "failed to read fixture `{}`: {e}\n\
             Hint: snapshots reference fixtures via symlinks under \
             crates/wit-clap/tests/fixtures/. If you see this on a fresh \
             checkout, ensure git symlinks are enabled (`git config \
             core.symlinks true`).",
            path.display()
        )
    })
}

/// Snapshot the full WIT → CLI mapping for the wordmark fixture
/// (`to-word: func(string) -> result<list<u8>, string>`).
///
/// This is the smallest, most readable example; treat it as the
/// "minimal working example" of the mapping rules.
#[test]
fn wordmark() {
    let bytes = fixture_bytes("library_wordmark.wasm");
    let rendered = render_mapping(&bytes).expect("render mapping");
    insta::assert_snapshot!(rendered);
}

/// Snapshot the full WIT → CLI mapping for the kitchen-sink fixture
/// — the canonical reference covering every supported WIT type:
/// primitives, strings, records (→ flag groups), variants with
/// payloads (→ `name=value`), lists (→ variadic positional or
/// repeated `--flag`), enums, plus an exported interface that
/// becomes a nested sub-command.
#[test]
fn kitchen_sink() {
    let bytes = fixture_bytes("library_kitchen_sink.wasm");
    let rendered = render_mapping(&bytes).expect("render mapping");
    insta::assert_snapshot!(rendered);
}

/// Snapshot the mapping for a component that imports a custom WIT
/// package the runner doesn't satisfy. The mapping itself succeeds
/// (the component still has a usable export surface); only
/// invocation fails — that's verified separately in the
/// `component-cli` integration tests.
#[test]
fn needs_import() {
    let bytes = fixture_bytes("library_needs_import.wasm");
    let rendered = render_mapping(&bytes).expect("render mapping");
    insta::assert_snapshot!(rendered);
}

/// Snapshot the rejection diagnostic for a component that exports
/// a resource type. Resources cannot be expressed as CLI arguments,
/// so `extract_library_surface` returns
/// [`LibraryExtractError::Resource`] which propagates through
/// [`render_mapping`].
#[test]
fn resources_are_rejected() {
    let bytes = fixture_bytes("library_resources.wasm");
    let err = render_mapping(&bytes).expect_err("must reject resource");
    let RenderMappingError::Extract(LibraryExtractError::Resource { name }) = err else {
        panic!("expected Resource error, got {err:?}");
    };
    insta::assert_snapshot!(format!("rejected: resource `{name}`"));
}
