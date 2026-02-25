//! Types for the WASM manifest file (`wasm.toml`).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The root manifest structure for a WASM package.
///
/// The manifest file (`deps/wasm.toml`) defines dependencies for a WASM package.
///
/// # Example
///
/// ```toml
/// [dependencies]
/// "wasi:logging" = "ghcr.io/webassembly/wasi-logging:1.0.0"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[must_use]
pub struct Manifest {
    /// The dependencies section of the manifest.
    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,
}

/// A dependency specification in the manifest.
///
/// Dependencies can be specified in two formats:
///
/// 1. Compact format (string):
///    ```toml
///    [dependencies]
///    "wasi:logging" = "ghcr.io/webassembly/wasi-logging:1.0.0"
///    ```
///
/// 2. Explicit format (table):
///    ```toml
///    [dependencies."wasi:logging"]
///    registry = "ghcr.io"
///    namespace = "webassembly"
///    package = "wasi-logging"
///    version = "1.0.0"
///    ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[must_use]
pub enum Dependency {
    /// Compact format: a single string with full registry path and version.
    ///
    /// Format: `registry/namespace/package:version`
    ///
    /// # Example
    /// ```text
    /// "ghcr.io/webassembly/wasi-logging:1.0.0"
    /// ```
    Compact(String),

    /// Explicit format: a table with individual fields.
    Explicit {
        /// The registry host (e.g., "ghcr.io").
        registry: String,
        /// The namespace or organization (e.g., "webassembly").
        namespace: String,
        /// The package name (e.g., "wasi-logging").
        package: String,
        /// The package version (e.g., "1.0.0").
        version: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_compact_format() {
        let toml = r#"
            [dependencies]
            "wasi:logging" = "ghcr.io/webassembly/wasi-logging:1.0.0"
            "wasi:key-value" = "ghcr.io/webassembly/wasi-key-value:2.0.0"
        "#;

        let manifest: Manifest = toml::from_str(toml).expect("Failed to parse manifest");

        assert_eq!(manifest.dependencies.len(), 2);
        assert!(manifest.dependencies.contains_key("wasi:logging"));
        assert!(manifest.dependencies.contains_key("wasi:key-value"));

        match &manifest.dependencies["wasi:logging"] {
            Dependency::Compact(s) => {
                assert_eq!(s, "ghcr.io/webassembly/wasi-logging:1.0.0");
            }
            _ => panic!("Expected compact format"),
        }
    }

    #[test]
    fn test_parse_explicit_format() {
        let toml = r#"
            [dependencies."wasi:logging"]
            registry = "ghcr.io"
            namespace = "webassembly"
            package = "wasi-logging"
            version = "1.0.0"

            [dependencies."wasi:key-value"]
            registry = "ghcr.io"
            namespace = "webassembly"
            package = "wasi-key-value"
            version = "2.0.0"
        "#;

        let manifest: Manifest = toml::from_str(toml).expect("Failed to parse manifest");

        assert_eq!(manifest.dependencies.len(), 2);

        match &manifest.dependencies["wasi:logging"] {
            Dependency::Explicit {
                registry,
                namespace,
                package,
                version,
            } => {
                assert_eq!(registry, "ghcr.io");
                assert_eq!(namespace, "webassembly");
                assert_eq!(package, "wasi-logging");
                assert_eq!(version, "1.0.0");
            }
            _ => panic!("Expected explicit format"),
        }
    }

    #[test]
    fn test_serialize_compact_format() {
        let mut dependencies = HashMap::new();
        dependencies.insert(
            "wasi:logging".to_string(),
            Dependency::Compact("ghcr.io/webassembly/wasi-logging:1.0.0".to_string()),
        );

        let manifest = Manifest { dependencies };
        let toml = toml::to_string(&manifest).expect("Failed to serialize manifest");

        assert!(toml.contains("wasi:logging"));
        assert!(toml.contains("ghcr.io/webassembly/wasi-logging:1.0.0"));
    }

    #[test]
    fn test_serialize_explicit_format() {
        let mut dependencies = HashMap::new();
        dependencies.insert(
            "wasi:logging".to_string(),
            Dependency::Explicit {
                registry: "ghcr.io".to_string(),
                namespace: "webassembly".to_string(),
                package: "wasi-logging".to_string(),
                version: "1.0.0".to_string(),
            },
        );

        let manifest = Manifest { dependencies };
        let toml = toml::to_string(&manifest).expect("Failed to serialize manifest");

        assert!(toml.contains("wasi:logging"));
        assert!(toml.contains("registry"));
        assert!(toml.contains("ghcr.io"));
    }

    #[test]
    fn test_empty_manifest() {
        let toml = r#""#;
        let manifest: Manifest = toml::from_str(toml).expect("Failed to parse empty manifest");
        assert_eq!(manifest.dependencies.len(), 0);
    }
}
