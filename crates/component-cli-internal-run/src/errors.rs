//! Error types for component validation and execution.

use miette::Diagnostic;

/// Error type for component validation and execution failures.
///
/// Each variant carries a stable [diagnostic error code][miette::Diagnostic::code]
/// that uniquely identifies the failure.
#[derive(Debug, Clone, PartialEq, Eq, Diagnostic)]
#[must_use]
pub enum RunError {
    /// The binary is a core WebAssembly module, not a component.
    #[diagnostic(
        code(component::run::core_module),
        help("use a tool like `wasm-tools component new` to wrap the module as a component")
    )]
    CoreModule,

    /// The binary could not be parsed as valid WebAssembly.
    #[diagnostic(
        code(component::run::invalid_binary),
        help("{reason}; ensure the file is a valid WebAssembly binary")
    )]
    InvalidBinary {
        /// The parser error message.
        reason: String,
    },

    /// The binary has no version header.
    #[diagnostic(
        code(component::run::no_version_header),
        help("ensure the file is a valid WebAssembly binary")
    )]
    NoVersionHeader,

    /// The component does not export the requested function.
    #[diagnostic(
        code(component::run::library_export_missing),
        help("the component does not export `{path}`; check the component's WIT")
    )]
    LibraryExportMissing {
        /// The export path that was looked up (`func` or `iface#func`).
        path: String,
    },

    /// The component imports a WIT package the runner does not
    /// provide (e.g. a custom interface unknown to wasmtime-wasi).
    #[diagnostic(
        code(component::run::library_instantiation_failed),
        help(
            "{cause}; the component imports a WIT package the runner does not provide; \
             check the component's WIT for unsupported imports"
        )
    )]
    LibraryInstantiationFailed {
        /// The underlying wasmtime error message.
        cause: String,
    },
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::CoreModule => {
                write!(
                    f,
                    "only Wasm Components can be executed; this appears to be a core module",
                )
            }
            RunError::InvalidBinary { reason } => {
                write!(f, "invalid Wasm binary: {reason}")
            }
            RunError::NoVersionHeader => {
                write!(f, "invalid Wasm binary: no version header found")
            }
            RunError::LibraryExportMissing { path } => {
                write!(f, "component has no export named `{path}`")
            }
            RunError::LibraryInstantiationFailed { cause } => {
                write!(
                    f,
                    "failed to instantiate component (missing or unsupported import): {cause}"
                )
            }
        }
    }
}

impl std::error::Error for RunError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_variants_have_error_codes() {
        use miette::Diagnostic;

        let variants: Vec<Box<dyn Diagnostic>> = vec![
            Box::new(RunError::CoreModule),
            Box::new(RunError::InvalidBinary {
                reason: "test".to_string(),
            }),
            Box::new(RunError::NoVersionHeader),
            Box::new(RunError::LibraryExportMissing {
                path: "foo".to_string(),
            }),
            Box::new(RunError::LibraryInstantiationFailed {
                cause: "missing import".to_string(),
            }),
        ];

        let expected_codes = [
            "component::run::core_module",
            "component::run::invalid_binary",
            "component::run::no_version_header",
            "component::run::library_export_missing",
            "component::run::library_instantiation_failed",
        ];

        for (variant, expected_code) in variants.iter().zip(expected_codes.iter()) {
            assert_eq!(
                variant
                    .code()
                    .unwrap_or_else(|| panic!("{expected_code} must have a diagnostic code"))
                    .to_string(),
                *expected_code,
            );
            assert!(
                variant.help().is_some(),
                "{expected_code} must have a help message"
            );
        }
    }
}
