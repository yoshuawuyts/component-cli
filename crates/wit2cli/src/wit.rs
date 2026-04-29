//! Extract a [`LibrarySurface`] from a Wasm component's WIT.
//!
//! The surface is a flat IR over the supported subset of WIT types
//! that `component run` can map onto a `clap` CLI. Resources are
//! rejected because they cannot be sensibly represented on the
//! command line.

use wit_parser::decoding::{DecodedWasm, decode};
use wit_parser::{Resolve, Type, TypeDefKind, WorldItem, WorldKey};

/// Logical path to a single exported function on a component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncPath {
    /// `Some(name)` when the function lives inside a nested
    /// interface export; `None` for free world-level exports.
    pub interface: Option<String>,
    /// The function's name as declared in the WIT.
    pub func: String,
}

/// Local IR mirroring the supported subset of WIT types.
///
/// `WitTy::Record` and `WitTy::Variant` preserve WIT declaration
/// order, which is mandatory: wasmtime's runtime checks record fields
/// by position and name (see
/// `wasmtime/src/runtime/component/values.rs`), so we have to emit
/// them in the order they were declared.
// r[impl run.library-args]
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WitTy {
    /// `bool`
    Bool,
    /// `s8`
    S8,
    /// `s16`
    S16,
    /// `s32`
    S32,
    /// `s64`
    S64,
    /// `u8`
    U8,
    /// `u16`
    U16,
    /// `u32`
    U32,
    /// `u64`
    U64,
    /// `f32`
    F32,
    /// `f64`
    F64,
    /// `char`
    Char,
    /// `string`
    String,
    /// `list<T>`
    List(Box<WitTy>),
    /// `option<T>`
    Option(Box<WitTy>),
    /// `result<T, E>` (either side may be absent).
    Result {
        /// The success-payload type, or `None` for `result<_, E>`.
        ok: Option<Box<WitTy>>,
        /// The error-payload type, or `None` for `result<T, _>`.
        err: Option<Box<WitTy>>,
    },
    /// `record { name: type, ... }` — fields preserved in WIT
    /// declaration order.
    Record(Vec<(String, WitTy)>),
    /// `variant { case, case(payload), ... }`.
    Variant(Vec<(String, Option<Box<WitTy>>)>),
    /// `enum { case-a, case-b, ... }`.
    Enum(Vec<String>),
    /// `flags { flag-a, flag-b, ... }`.
    Flags(Vec<String>),
    /// `tuple<T1, T2, ...>`.
    Tuple(Vec<WitTy>),
}

/// A single function parameter.
#[derive(Debug, Clone)]
pub struct ParamDecl {
    /// Parameter name as declared in the WIT.
    pub name: String,
    /// Parameter type.
    pub ty: WitTy,
}

/// A single function result. Currently unnamed.
#[derive(Debug, Clone)]
pub struct ResultDecl {
    /// Type of the result. Used by the wire-up to validate the
    /// number of returned values matches the declared signature
    /// and to drive future type-aware error messages.
    pub ty: WitTy,
}

/// A single exported function.
#[derive(Debug, Clone)]
pub struct FuncDecl {
    /// Function name as declared in the WIT.
    pub name: String,
    /// Doc-comment, used as the clap `about` text.
    pub doc: Option<String>,
    /// Parameters in declaration order.
    pub params: Vec<ParamDecl>,
    /// Function results, used to populate
    /// [`crate::Invocation::expected_results`] for runtime sanity
    /// checks.
    pub results: Vec<ResultDecl>,
}

/// A top-level item in the library surface.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum LibraryItem {
    /// Free function exported at the world level.
    Func(FuncDecl),
    /// An exported interface containing one or more functions.
    Interface {
        /// Short, user-facing name (e.g. `math`).
        name: String,
        /// Fully-qualified export name used by wasmtime
        /// (`namespace:pkg/iface@version`). May equal `name` when the
        /// interface was declared inline at the world level.
        export_name: String,
        /// Doc-comment declared on the interface, if any.
        doc: Option<String>,
        /// Functions exported by the interface, in WIT order.
        funcs: Vec<FuncDecl>,
    },
}

/// The full set of dynamically-dispatchable exports of a component.
#[derive(Debug, Clone)]
#[must_use]
pub struct LibrarySurface {
    /// Top-level items (functions and interfaces).
    pub items: Vec<LibraryItem>,
}

/// Errors raised when we cannot extract a usable surface.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum LibraryExtractError {
    /// The component bytes could not be decoded as a WIT-bearing
    /// component.
    #[error("failed to decode component WIT: {0}")]
    Decode(String),
    /// The component is a WIT package, not a compiled component.
    #[error("input is a WIT package, not a compiled component")]
    NotAComponent,
    // r[impl run.library-resources-rejected]
    /// The component exports a resource type, which cannot be
    /// expressed as a CLI argument.
    #[error("resource type `{name}` is not supported by `component run`")]
    Resource {
        /// Name of the resource type (or interface) that triggered
        /// the rejection.
        name: String,
    },
    /// A WIT type kind we don't support yet (futures, streams,
    /// error-context, owned/borrowed handles).
    #[error("unsupported WIT type kind: {kind}")]
    UnsupportedKind {
        /// Human-readable label for the unsupported kind
        /// (`"future"`, `"stream"`, `"map"`, etc.).
        kind: &'static str,
    },
}

/// Decode `bytes` and walk the world's exports into a
/// [`LibrarySurface`].
pub fn extract_library_surface(bytes: &[u8]) -> Result<LibrarySurface, LibraryExtractError> {
    let decoded = decode(bytes).map_err(|e| LibraryExtractError::Decode(e.to_string()))?;
    let (resolve, world_id) = match decoded {
        DecodedWasm::Component(r, w) => (r, w),
        DecodedWasm::WitPackage(_, _) => return Err(LibraryExtractError::NotAComponent),
    };

    let world = resolve
        .worlds
        .get(world_id)
        .ok_or_else(|| LibraryExtractError::Decode("world id not in resolve".to_string()))?;

    let mut items = Vec::new();
    for (key, item) in &world.exports {
        match item {
            WorldItem::Function(func) => {
                let decl = func_to_decl(&resolve, &func.name, func)?;
                items.push(LibraryItem::Func(decl));
            }
            WorldItem::Interface { id, .. } => {
                let iface = resolve.interfaces.get(*id).ok_or_else(|| {
                    LibraryExtractError::Decode("interface id not in resolve".to_string())
                })?;
                let iface_name = world_key_label(&resolve, key, iface.name.as_deref());
                let export_name = world_key_export_name(&resolve, key, iface);
                let mut funcs = Vec::with_capacity(iface.functions.len());
                for func in iface.functions.values() {
                    funcs.push(func_to_decl(&resolve, &func.name, func)?);
                }
                items.push(LibraryItem::Interface {
                    name: iface_name,
                    export_name,
                    doc: iface.docs.contents.clone(),
                    funcs,
                });
            }
            WorldItem::Type { .. } => {
                // Type aliases at the world level are not invocable.
            }
        }
    }

    Ok(LibrarySurface { items })
}

/// Convert a `wit_parser::Function` into a [`FuncDecl`].
fn func_to_decl(
    resolve: &Resolve,
    name: &str,
    func: &wit_parser::Function,
) -> Result<FuncDecl, LibraryExtractError> {
    let mut params = Vec::with_capacity(func.params.len());
    for p in &func.params {
        params.push(ParamDecl {
            name: p.name.clone(),
            ty: type_to_wit_ty(resolve, &p.ty)?,
        });
    }
    let results = match &func.result {
        Some(ty) => vec![ResultDecl {
            ty: type_to_wit_ty(resolve, ty)?,
        }],
        None => Vec::new(),
    };
    Ok(FuncDecl {
        name: name.to_string(),
        doc: func.docs.contents.clone(),
        params,
        results,
    })
}

/// Convert a `wit_parser::Type` into a [`WitTy`].
fn type_to_wit_ty(resolve: &Resolve, ty: &Type) -> Result<WitTy, LibraryExtractError> {
    match ty {
        Type::Bool => Ok(WitTy::Bool),
        Type::S8 => Ok(WitTy::S8),
        Type::S16 => Ok(WitTy::S16),
        Type::S32 => Ok(WitTy::S32),
        Type::S64 => Ok(WitTy::S64),
        Type::U8 => Ok(WitTy::U8),
        Type::U16 => Ok(WitTy::U16),
        Type::U32 => Ok(WitTy::U32),
        Type::U64 => Ok(WitTy::U64),
        Type::F32 => Ok(WitTy::F32),
        Type::F64 => Ok(WitTy::F64),
        Type::Char => Ok(WitTy::Char),
        Type::String => Ok(WitTy::String),
        Type::ErrorContext => Err(LibraryExtractError::UnsupportedKind {
            kind: "error-context",
        }),
        Type::Id(id) => {
            let td = resolve
                .types
                .get(*id)
                .ok_or_else(|| LibraryExtractError::Decode("type id not in resolve".to_string()))?;
            type_def_to_wit_ty(resolve, td)
        }
    }
}

/// Convert a `wit_parser::TypeDef` into a [`WitTy`].
fn type_def_to_wit_ty(
    resolve: &Resolve,
    td: &wit_parser::TypeDef,
) -> Result<WitTy, LibraryExtractError> {
    let resource_name = || td.name.clone().unwrap_or_else(|| "<anonymous>".to_string());
    match &td.kind {
        TypeDefKind::List(inner) => Ok(WitTy::List(Box::new(type_to_wit_ty(resolve, inner)?))),
        TypeDefKind::Option(inner) => Ok(WitTy::Option(Box::new(type_to_wit_ty(resolve, inner)?))),
        TypeDefKind::Result(r) => {
            let ok = match &r.ok {
                Some(t) => Some(Box::new(type_to_wit_ty(resolve, t)?)),
                None => None,
            };
            let err = match &r.err {
                Some(t) => Some(Box::new(type_to_wit_ty(resolve, t)?)),
                None => None,
            };
            Ok(WitTy::Result { ok, err })
        }
        TypeDefKind::Record(rec) => {
            let mut fields = Vec::with_capacity(rec.fields.len());
            for f in &rec.fields {
                fields.push((f.name.clone(), type_to_wit_ty(resolve, &f.ty)?));
            }
            Ok(WitTy::Record(fields))
        }
        TypeDefKind::Variant(v) => {
            let mut cases = Vec::with_capacity(v.cases.len());
            for c in &v.cases {
                let payload = match &c.ty {
                    Some(t) => Some(Box::new(type_to_wit_ty(resolve, t)?)),
                    None => None,
                };
                cases.push((c.name.clone(), payload));
            }
            Ok(WitTy::Variant(cases))
        }
        TypeDefKind::Enum(e) => Ok(WitTy::Enum(
            e.cases.iter().map(|c| c.name.clone()).collect(),
        )),
        TypeDefKind::Flags(f) => Ok(WitTy::Flags(
            f.flags.iter().map(|fl| fl.name.clone()).collect(),
        )),
        TypeDefKind::Tuple(t) => {
            let mut tys = Vec::with_capacity(t.types.len());
            for inner in &t.types {
                tys.push(type_to_wit_ty(resolve, inner)?);
            }
            Ok(WitTy::Tuple(tys))
        }
        TypeDefKind::Type(inner) => type_to_wit_ty(resolve, inner),
        TypeDefKind::Resource | TypeDefKind::Handle(_) => Err(LibraryExtractError::Resource {
            name: resource_name(),
        }),
        TypeDefKind::Future(_) => Err(LibraryExtractError::UnsupportedKind { kind: "future" }),
        TypeDefKind::Stream(_) => Err(LibraryExtractError::UnsupportedKind { kind: "stream" }),
        TypeDefKind::Map(_, _) => Err(LibraryExtractError::UnsupportedKind { kind: "map" }),
        TypeDefKind::FixedLengthList(_, _) => Err(LibraryExtractError::UnsupportedKind {
            kind: "fixed-length-list",
        }),
        TypeDefKind::Unknown => Err(LibraryExtractError::UnsupportedKind { kind: "unknown" }),
    }
}

/// Best-effort name for an interface export, used as the clap
/// sub-command name.
fn world_key_label(resolve: &Resolve, key: &WorldKey, iface_name: Option<&str>) -> String {
    match key {
        WorldKey::Name(name) => name.clone(),
        WorldKey::Interface(id) => {
            if let Some(iface) = resolve.interfaces.get(*id)
                && let Some(name) = iface.name.as_deref()
            {
                return name.to_string();
            }
            iface_name.unwrap_or("interface").to_string()
        }
    }
}

/// Compute the fully-qualified export name wasmtime uses for an
/// interface export. For named world keys (declared inline) it is
/// just the bare name; for `WorldKey::Interface(id)` it's
/// `namespace:pkg/iface@version`.
fn world_key_export_name(
    resolve: &Resolve,
    key: &WorldKey,
    iface: &wit_parser::Interface,
) -> String {
    match key {
        WorldKey::Name(name) => name.clone(),
        WorldKey::Interface(_) => {
            let name = iface.name.as_deref().unwrap_or("interface");
            let Some(pkg_id) = iface.package else {
                return name.to_string();
            };
            let Some(pkg) = resolve.packages.get(pkg_id) else {
                return name.to_string();
            };
            let pname = &pkg.name;
            match &pname.version {
                Some(v) => format!("{}:{}/{name}@{v}", pname.namespace, pname.name),
                None => format!("{}:{}/{name}", pname.namespace, pname.name),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_path(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    fn read_fixture(name: &str) -> Vec<u8> {
        std::fs::read(fixture_path(name)).expect("read fixture")
    }

    // r[verify run.library-detection]
    #[test]
    fn extract_wordmark_surface() {
        let bytes = read_fixture("library_wordmark.wasm");
        let surface = extract_library_surface(&bytes).expect("extract");
        assert_eq!(surface.items.len(), 1);
        let LibraryItem::Func(decl) = &surface.items[0] else {
            panic!("expected free function, got {:?}", surface.items[0]);
        };
        assert_eq!(decl.name, "to-word");
        assert_eq!(decl.params.len(), 1);
        assert_eq!(decl.params[0].name, "markdown");
        assert!(matches!(decl.params[0].ty, WitTy::String));
        assert_eq!(decl.results.len(), 1);
        assert!(matches!(
            decl.results[0].ty,
            WitTy::Result {
                ok: Some(_),
                err: Some(_)
            }
        ));
    }

    // r[verify run.library-dispatch]
    #[test]
    fn extract_kitchen_sink_surface() {
        let bytes = read_fixture("library_kitchen_sink.wasm");
        let surface = extract_library_surface(&bytes).expect("extract");

        // Must contain at least one interface (math) plus the free
        // functions.
        let has_iface = surface
            .items
            .iter()
            .any(|i| matches!(i, LibraryItem::Interface { .. }));
        assert!(has_iface, "expected math interface in surface");

        let names: Vec<&str> = surface
            .items
            .iter()
            .map(|i| match i {
                LibraryItem::Func(f) => f.name.as_str(),
                LibraryItem::Interface { name, .. } => name.as_str(),
            })
            .collect();
        for expected in &["shout", "greet", "pick", "fail"] {
            assert!(
                names.iter().any(|n| *n == *expected),
                "missing export {expected}; got {names:?}"
            );
        }
    }

    // r[verify run.library-resources-rejected]
    #[test]
    fn extract_resources_fixture_is_rejected() {
        let bytes = read_fixture("library_resources.wasm");
        let err = extract_library_surface(&bytes).expect_err("must reject resource");
        assert!(
            matches!(err, LibraryExtractError::Resource { .. }),
            "expected Resource error, got {err:?}"
        );
    }
}
