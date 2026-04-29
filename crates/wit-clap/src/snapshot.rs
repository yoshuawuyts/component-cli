//! Rendering helpers for documenting the WIT → CLI mapping.
//!
//! The functions in this module produce **byte-stable** text from a
//! component's WIT and from the generated [`clap::Command`] tree.
//! They are designed to be consumed by `insta` snapshot tests so the
//! committed snapshots become the canonical, human-readable spec for
//! how each WIT type maps onto a CLI argument.

use std::fmt::Write as _;

use crate::wit::{FuncDecl, LibraryExtractError, LibraryItem, LibrarySurface, ParamDecl, WitTy};
use crate::{build_clap, extract_library_surface};

/// Render `surface` as multi-line text suitable for a snapshot test.
///
/// The format lists every [`LibraryItem`] (free functions and
/// interfaces), each function's params and results, and any
/// doc-comments declared in the WIT.
#[must_use]
pub fn render_surface(surface: &LibrarySurface) -> String {
    let mut out = String::new();
    let count = surface.items.len();
    let plural = if count == 1 { "item" } else { "items" };
    let _ = writeln!(out, "surface ({count} {plural})");

    let last = count.saturating_sub(1);
    for (i, item) in surface.items.iter().enumerate() {
        let is_last = i == last;
        match item {
            LibraryItem::Func(f) => {
                render_func(&mut out, f, is_last, "");
            }
            LibraryItem::Interface {
                name, doc, funcs, ..
            } => {
                let connector = if is_last { "└─" } else { "├─" };
                let _ = writeln!(out, "{connector} interface {name}");
                let prefix = if is_last { "   " } else { "│  " };
                if let Some(doc) = doc {
                    write_doc(&mut out, doc, prefix);
                }
                let f_last = funcs.len().saturating_sub(1);
                for (j, f) in funcs.iter().enumerate() {
                    render_func(&mut out, f, j == f_last, prefix);
                }
            }
        }
    }
    out
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

/// Render WIT bytes end-to-end: surface + generated CLI tree.
///
/// One-stop helper for snapshot tests.
///
/// # Errors
///
/// Forwards any error from [`extract_library_surface`] or
/// [`build_clap`].
pub fn render_mapping(bytes: &[u8]) -> Result<String, RenderMappingError> {
    let surface = extract_library_surface(bytes).map_err(RenderMappingError::Extract)?;
    let cmd =
        build_clap(&surface, "<program>").map_err(|e| RenderMappingError::Build(e.to_string()))?;
    let mut out = String::new();
    out.push_str("=== WIT surface ===\n");
    out.push_str(&render_surface(&surface));
    out.push_str("\n=== Generated CLI ===\n");
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

fn render_func(out: &mut String, f: &FuncDecl, is_last: bool, parent_prefix: &str) {
    let connector = if is_last { "└─" } else { "├─" };
    let _ = writeln!(out, "{parent_prefix}{connector} func {}", f.name);
    let inner = if is_last { "   " } else { "│  " };
    let prefix = format!("{parent_prefix}{inner}");
    if let Some(doc) = &f.doc {
        write_doc(out, doc, &prefix);
    }
    if f.params.is_empty() {
        let _ = writeln!(out, "{prefix}params: (none)");
    } else {
        let _ = writeln!(out, "{prefix}params:");
        for ParamDecl { name, ty } in &f.params {
            let _ = writeln!(out, "{prefix}  {name}: {}", format_ty(ty));
        }
    }
    if f.results.is_empty() {
        let _ = writeln!(out, "{prefix}results: (none)");
    } else {
        let _ = writeln!(out, "{prefix}results:");
        for r in &f.results {
            let _ = writeln!(out, "{prefix}  {}", format_ty(&r.ty));
        }
    }
}

fn write_doc(out: &mut String, doc: &str, prefix: &str) {
    let trimmed = doc.trim();
    if trimmed.is_empty() {
        return;
    }
    let mut lines = trimmed.lines();
    if let Some(first) = lines.next() {
        let _ = writeln!(out, "{prefix}doc: {first}");
        for rest in lines {
            let _ = writeln!(out, "{prefix}     {rest}");
        }
    }
}

/// Format a [`WitTy`] using a syntax that mirrors WIT itself
/// (`list<u8>`, `result<list<u8>, string>`, etc.).
fn format_ty(ty: &WitTy) -> String {
    match ty {
        WitTy::Bool => "bool".into(),
        WitTy::S8 => "s8".into(),
        WitTy::S16 => "s16".into(),
        WitTy::S32 => "s32".into(),
        WitTy::S64 => "s64".into(),
        WitTy::U8 => "u8".into(),
        WitTy::U16 => "u16".into(),
        WitTy::U32 => "u32".into(),
        WitTy::U64 => "u64".into(),
        WitTy::F32 => "f32".into(),
        WitTy::F64 => "f64".into(),
        WitTy::Char => "char".into(),
        WitTy::String => "string".into(),
        WitTy::List(inner) => format!("list<{}>", format_ty(inner)),
        WitTy::Option(inner) => format!("option<{}>", format_ty(inner)),
        WitTy::Result { ok, err } => {
            let ok = ok.as_deref().map_or("_".to_string(), format_ty);
            let err = err.as_deref().map_or("_".to_string(), format_ty);
            format!("result<{ok}, {err}>")
        }
        WitTy::Record(fields) => {
            let fields: Vec<String> = fields
                .iter()
                .map(|(n, t)| format!("{n}: {}", format_ty(t)))
                .collect();
            format!("record {{ {} }}", fields.join(", "))
        }
        WitTy::Variant(cases) => {
            let cases: Vec<String> = cases
                .iter()
                .map(|(n, t)| match t {
                    Some(payload) => format!("{n}({})", format_ty(payload)),
                    None => n.clone(),
                })
                .collect();
            format!("variant {{ {} }}", cases.join(", "))
        }
        WitTy::Enum(cases) => format!("enum {{ {} }}", cases.join(", ")),
        WitTy::Flags(flags) => format!("flags {{ {} }}", flags.join(", ")),
        WitTy::Tuple(types) => {
            let types: Vec<String> = types.iter().map(format_ty).collect();
            format!("tuple<{}>", types.join(", "))
        }
    }
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
    use crate::wit::{FuncDecl, LibraryItem, LibrarySurface, ParamDecl, ResultDecl};

    #[test]
    fn render_surface_basic() {
        let surface = LibrarySurface {
            items: vec![LibraryItem::Func(FuncDecl {
                name: "to-word".to_string(),
                doc: Some("Convert.".to_string()),
                params: vec![ParamDecl {
                    name: "markdown".to_string(),
                    ty: WitTy::String,
                }],
                results: vec![ResultDecl {
                    ty: WitTy::Result {
                        ok: Some(Box::new(WitTy::List(Box::new(WitTy::U8)))),
                        err: Some(Box::new(WitTy::String)),
                    },
                }],
            })],
        };
        let out = render_surface(&surface);
        assert!(out.contains("surface (1 item)"));
        assert!(out.contains("func to-word"));
        assert!(out.contains("doc: Convert."));
        assert!(out.contains("markdown: string"));
        assert!(out.contains("result<list<u8>, string>"));
    }

    #[test]
    fn format_ty_records_and_variants() {
        let rec = WitTy::Record(vec![
            ("name".to_string(), WitTy::String),
            ("age".to_string(), WitTy::U32),
        ]);
        assert_eq!(format_ty(&rec), "record { name: string, age: u32 }");

        let variant = WitTy::Variant(vec![
            ("red".to_string(), None),
            ("blue".to_string(), Some(Box::new(WitTy::String))),
        ]);
        assert_eq!(format_ty(&variant), "variant { red, blue(string) }");
    }
}
