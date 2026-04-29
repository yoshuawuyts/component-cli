//! Translate a WebAssembly component's WIT exports into a [`clap::Command`].
//!
//! Given a compiled component (a `.wasm` file), `wit2cli` extracts a
//! [`LibrarySurface`] describing every exported function, then builds
//! a [`clap::Command`] that mirrors the WIT shape. Parsed
//! [`clap::ArgMatches`] become a `Vec<`[`Val`]`>` ready to hand off
//! to wasmtime for invocation.
//!
//! The mapping rules — how each WIT type translates into a CLI
//! argument, how compound types are flattened into flags, how
//! results render — are documented end-to-end by the snapshot tests
//! under `crates/wit2cli/tests/snapshots/`.
//!
//! # Quick start
//!
//! ```no_run
//! use wit2cli::{build_clap, extract_library_surface, parse_invocation};
//!
//! # fn _example() -> Result<(), Box<dyn std::error::Error>> {
//! let bytes = std::fs::read("my-component.wasm")?;
//! let surface = extract_library_surface(&bytes)?;
//! let cmd = build_clap(&surface, "my-tool")?;
//! let matches = cmd.get_matches();
//! let invocation = parse_invocation(&matches, &surface)?;
//! // hand `invocation.path` + `invocation.args` to wasmtime ...
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]

mod cli;
mod render;
mod wit;

pub mod snapshot;

pub use cli::{CliError, Invocation, build_clap, parse_invocation};
pub use render::{RenderOutcome, print_results};
pub use wit::{
    FuncDecl, FuncPath, LibraryExtractError, LibraryItem, LibrarySurface, ParamDecl, ResultDecl,
    WitTy, extract_library_surface,
};

/// Re-export of [`wasmtime::component::Val`] so callers don't have to
/// pin the wasmtime version themselves.
pub use wasmtime::component::Val;
