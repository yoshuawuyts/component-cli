//! Render `wasmtime::component::Val` results to stdout/stderr.
//!
//! This module is pure (no `process::exit`): the caller in
//! `run/mod.rs` decides whether to terminate based on
//! [`RenderOutcome::exit_code`]. Tests can therefore exercise the
//! rendering rules without touching real stdio.

use std::io::Write;

use serde_json::{Map, Value};
use wasmtime::component::Val;

/// Result of rendering: a process exit code (0 for success, 1 when
/// the guest returned `result::Err`).
#[derive(Debug, Clone, Copy)]
#[must_use]
pub struct RenderOutcome {
    /// Exit code the caller should use. `0` on success, `1` if any
    /// rendered value was a `result::Err`.
    pub exit_code: i32,
}

/// Render every value in `results` to `stdout` / `stderr`.
///
/// Returns the exit code the caller should use.
// r[impl run.library-output-bytes]
// r[impl run.library-output-other]
// r[impl run.library-result-err]
pub fn print_results(
    results: &[Val],
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> std::io::Result<RenderOutcome> {
    let mut exit_code = 0;
    for val in results {
        let outcome = render_one(val, stdout, stderr)?;
        if outcome.exit_code != 0 {
            exit_code = outcome.exit_code;
        }
    }
    Ok(RenderOutcome { exit_code })
}

/// Render a single result value.
fn render_one(
    val: &Val,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> std::io::Result<RenderOutcome> {
    match val {
        // result::Ok(payload) — recurse on the payload (or no-op if None).
        Val::Result(Ok(payload)) => match payload {
            Some(inner) => render_one(inner, stdout, stderr),
            None => Ok(RenderOutcome { exit_code: 0 }),
        },
        // result::Err(payload) — print to stderr, exit non-zero.
        Val::Result(Err(payload)) => {
            if let Some(inner) = payload {
                render_to_stderr(inner, stderr)?;
            }
            Ok(RenderOutcome { exit_code: 1 })
        }
        // option::Some(v) — recurse; option::None — no output.
        Val::Option(Some(inner)) => render_one(inner, stdout, stderr),
        Val::Option(None) => Ok(RenderOutcome { exit_code: 0 }),
        // The headline path: list<u8> → raw bytes.
        Val::List(vals) if all_u8(vals) => {
            let bytes: Vec<u8> = vals
                .iter()
                .map(|v| match v {
                    Val::U8(b) => *b,
                    _ => unreachable!(),
                })
                .collect();
            stdout.write_all(&bytes)?;
            Ok(RenderOutcome { exit_code: 0 })
        }
        // Strings: verbatim, no trailing newline.
        Val::String(s) => {
            stdout.write_all(s.as_bytes())?;
            Ok(RenderOutcome { exit_code: 0 })
        }
        // Scalars: Display + newline.
        v if format_scalar(v).is_some() => {
            let line = format_scalar(v).expect("matched scalar");
            writeln!(stdout, "{line}")?;
            Ok(RenderOutcome { exit_code: 0 })
        }
        // Compounds → JSON.
        Val::List(_)
        | Val::Record(_)
        | Val::Variant(_, _)
        | Val::Enum(_)
        | Val::Flags(_)
        | Val::Tuple(_) => {
            let json = val_to_json(val);
            let s = serde_json::to_string_pretty(&json)
                .unwrap_or_else(|_| "<unrenderable>".to_string());
            writeln!(stdout, "{s}")?;
            Ok(RenderOutcome { exit_code: 0 })
        }
        // Resource handles, futures, streams, error contexts — these
        // shouldn't reach us because extract_library_surface rejects
        // them. Render a debug placeholder so we don't lose data.
        other => {
            writeln!(stdout, "<unsupported result: {other:?}>")?;
            Ok(RenderOutcome { exit_code: 0 })
        }
    }
}

/// Render an error payload to stderr (string for primitives, JSON
/// for compounds), with a trailing newline.
fn render_to_stderr(val: &Val, stderr: &mut impl Write) -> std::io::Result<()> {
    if let Val::String(s) = val {
        return writeln!(stderr, "{s}");
    }
    if let Some(line) = format_scalar(val) {
        return writeln!(stderr, "{line}");
    }
    match val {
        // Recurse through option/result wrappers so a
        // `result<_, option<string>>` errors render cleanly.
        Val::Option(Some(inner)) | Val::Result(Ok(Some(inner)) | Err(Some(inner))) => {
            render_to_stderr(inner, stderr)
        }
        Val::Option(None) | Val::Result(Ok(None) | Err(None)) => Ok(()),
        other => {
            let json = val_to_json(other);
            let s = serde_json::to_string_pretty(&json)
                .unwrap_or_else(|_| "<unrenderable>".to_string());
            writeln!(stderr, "{s}")
        }
    }
}

/// Return the Display representation of a scalar [`Val`], or
/// `None` if `val` is not a scalar.
//
// Each arm intentionally binds a different concrete numeric type;
// `clippy::match_same_arms` would only be silenced by erasing the
// type annotation we rely on for `Display`.
#[allow(clippy::match_same_arms)]
fn format_scalar(val: &Val) -> Option<String> {
    match val {
        Val::Bool(b) => Some(b.to_string()),
        Val::S8(n) => Some(n.to_string()),
        Val::S16(n) => Some(n.to_string()),
        Val::S32(n) => Some(n.to_string()),
        Val::S64(n) => Some(n.to_string()),
        Val::U8(n) => Some(n.to_string()),
        Val::U16(n) => Some(n.to_string()),
        Val::U32(n) => Some(n.to_string()),
        Val::U64(n) => Some(n.to_string()),
        Val::Float32(f) => Some(f.to_string()),
        Val::Float64(f) => Some(f.to_string()),
        Val::Char(c) => Some(c.to_string()),
        _ => None,
    }
}

fn all_u8(vals: &[Val]) -> bool {
    !vals.is_empty() && vals.iter().all(|v| matches!(v, Val::U8(_)))
}

/// Convert a [`Val`] tree into a [`serde_json::Value`] for compound
/// rendering. Best-effort — resources and futures become null.
//
// Each numeric arm binds a different concrete type for `From<T> for
// serde_json::Value`; merging them with `|` is impossible.
#[allow(clippy::match_same_arms)]
fn val_to_json(val: &Val) -> Value {
    match val {
        Val::Bool(b) => Value::Bool(*b),
        Val::S8(n) => Value::from(*n),
        Val::S16(n) => Value::from(*n),
        Val::S32(n) => Value::from(*n),
        Val::S64(n) => Value::from(*n),
        Val::U8(n) => Value::from(*n),
        Val::U16(n) => Value::from(*n),
        Val::U32(n) => Value::from(*n),
        Val::U64(n) => Value::from(*n),
        Val::Float32(f) => {
            serde_json::Number::from_f64((*f).into()).map_or(Value::Null, Value::Number)
        }
        Val::Float64(f) => serde_json::Number::from_f64(*f).map_or(Value::Null, Value::Number),
        Val::Char(c) => Value::String(c.to_string()),
        Val::String(s) => Value::String(s.clone()),
        Val::List(vals) => Value::Array(vals.iter().map(val_to_json).collect()),
        Val::Record(fields) => {
            let mut m = Map::new();
            for (k, v) in fields {
                m.insert(k.clone(), val_to_json(v));
            }
            Value::Object(m)
        }
        Val::Tuple(vals) => Value::Array(vals.iter().map(val_to_json).collect()),
        Val::Variant(case, payload) => {
            let mut m = Map::new();
            m.insert(
                case.clone(),
                payload.as_deref().map_or(Value::Null, val_to_json),
            );
            Value::Object(m)
        }
        Val::Enum(name) => Value::String(name.clone()),
        Val::Flags(names) => Value::Array(names.iter().map(|n| Value::String(n.clone())).collect()),
        Val::Option(Some(inner)) => val_to_json(inner),
        Val::Option(None) => Value::Null,
        Val::Result(Ok(Some(inner))) => {
            let mut m = Map::new();
            m.insert("ok".to_string(), val_to_json(inner));
            Value::Object(m)
        }
        Val::Result(Ok(None)) => {
            let mut m = Map::new();
            m.insert("ok".to_string(), Value::Null);
            Value::Object(m)
        }
        Val::Result(Err(Some(inner))) => {
            let mut m = Map::new();
            m.insert("err".to_string(), val_to_json(inner));
            Value::Object(m)
        }
        Val::Result(Err(None)) => {
            let mut m = Map::new();
            m.insert("err".to_string(), Value::Null);
            Value::Object(m)
        }
        _ => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // r[verify run.library-output-bytes]
    #[test]
    fn list_u8_writes_raw_bytes() {
        let val = Val::List(vec![
            Val::U8(0x44),
            Val::U8(0x4f),
            Val::U8(0x43),
            Val::U8(0x58),
        ]);
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let outcome = print_results(&[val], &mut stdout, &mut stderr).unwrap();
        assert_eq!(outcome.exit_code, 0);
        assert_eq!(stdout, b"DOCX");
        assert!(stderr.is_empty());
    }

    // r[verify run.library-output-other]
    #[test]
    fn string_renders_verbatim_no_newline() {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let outcome = print_results(
            &[Val::String("hello".to_string())],
            &mut stdout,
            &mut stderr,
        )
        .unwrap();
        assert_eq!(outcome.exit_code, 0);
        assert_eq!(stdout, b"hello");
    }

    // r[verify run.library-output-other]
    #[test]
    fn s32_renders_with_newline() {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let outcome = print_results(&[Val::S32(42)], &mut stdout, &mut stderr).unwrap();
        assert_eq!(outcome.exit_code, 0);
        assert_eq!(stdout, b"42\n");
    }

    // r[verify run.library-output-other]
    #[test]
    fn result_ok_recurses() {
        let val = Val::Result(Ok(Some(Box::new(Val::String("ok".to_string())))));
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let outcome = print_results(&[val], &mut stdout, &mut stderr).unwrap();
        assert_eq!(outcome.exit_code, 0);
        assert_eq!(stdout, b"ok");
    }

    // r[verify run.library-result-err]
    #[test]
    fn result_err_renders_to_stderr_with_exit_one() {
        let val = Val::Result(Err(Some(Box::new(Val::String("boom".to_string())))));
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let outcome = print_results(&[val], &mut stdout, &mut stderr).unwrap();
        assert_eq!(outcome.exit_code, 1);
        assert!(stdout.is_empty());
        assert_eq!(stderr, b"boom\n");
    }

    // r[verify run.library-output-other]
    #[test]
    fn record_renders_as_json() {
        let val = Val::Record(vec![
            ("name".to_string(), Val::String("Ada".to_string())),
            ("age".to_string(), Val::U32(37)),
        ]);
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let outcome = print_results(&[val], &mut stdout, &mut stderr).unwrap();
        assert_eq!(outcome.exit_code, 0);
        let s = String::from_utf8(stdout).unwrap();
        assert!(s.contains("\"name\""));
        assert!(s.contains("\"Ada\""));
        assert!(s.contains("\"age\""));
        assert!(s.contains("37"));
    }
}
