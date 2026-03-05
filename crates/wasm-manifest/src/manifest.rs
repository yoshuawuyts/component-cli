//! Types for the WASM manifest file (`wasm.toml`).

use crate::permissions::RunPermissions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The type of a WASM package.
///
/// # Example
///
/// ```rust
/// use wasm_manifest::PackageType;
///
/// let component = PackageType::Component;
/// let interface = PackageType::Interface;
/// assert_ne!(component, interface);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[must_use]
pub enum PackageType {
    /// A compiled WebAssembly component.
    Component,
    /// A WIT interface definition.
    Interface,
}

/// The root manifest structure for a WASM package.
///
/// The manifest file (`deps/wasm.toml`) defines dependencies for a WASM package,
/// separated into components and interfaces under a `[dependencies]` section.
///
/// # Example
///
/// ```rust
/// use wasm_manifest::Manifest;
///
/// let toml = r#"
/// [dependencies.components]
/// "root:component" = "ghcr.io/example/component:0.1.0"
///
/// [dependencies.interfaces]
/// "wasi:clocks" = "ghcr.io/webassembly/wasi/clocks:0.2.5"
/// "#;
///
/// let manifest: Manifest = toml::from_str(toml).unwrap();
/// assert_eq!(manifest.dependencies.components.len(), 1);
/// assert_eq!(manifest.dependencies.interfaces.len(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[must_use]
pub struct Manifest {
    /// The dependencies section of the manifest.
    #[serde(default)]
    pub dependencies: Dependencies,
}

impl Manifest {
    /// Iterate over all dependencies with their package type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wasm_manifest::{Manifest, PackageType};
    ///
    /// let toml = r#"
    /// [dependencies.components]
    /// "root:component" = "ghcr.io/example/component:0.1.0"
    ///
    /// [dependencies.interfaces]
    /// "wasi:logging" = "ghcr.io/webassembly/wasi-logging:1.0.0"
    /// "#;
    ///
    /// let manifest: Manifest = toml::from_str(toml).unwrap();
    /// let all: Vec<_> = manifest.all_dependencies().collect();
    /// assert_eq!(all.len(), 2);
    /// assert!(all.iter().any(|(_, _, pt)| *pt == PackageType::Component));
    /// assert!(all.iter().any(|(_, _, pt)| *pt == PackageType::Interface));
    /// ```
    pub fn all_dependencies(&self) -> impl Iterator<Item = (&String, &Dependency, PackageType)> {
        self.dependencies.all()
    }
}

/// The dependency sections of a manifest, grouped by type.
///
/// # Example
///
/// ```rust
/// use wasm_manifest::Dependencies;
///
/// let toml = r#"
/// [components]
/// "root:component" = "ghcr.io/example/component:0.1.0"
///
/// [interfaces]
/// "wasi:logging" = "ghcr.io/webassembly/wasi-logging:1.0.0"
/// "#;
///
/// let deps: Dependencies = toml::from_str(toml).unwrap();
/// assert_eq!(deps.components.len(), 1);
/// assert_eq!(deps.interfaces.len(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[must_use]
pub struct Dependencies {
    /// Component dependencies.
    #[serde(default)]
    pub components: HashMap<String, Dependency>,
    /// Interface dependencies.
    #[serde(default)]
    pub interfaces: HashMap<String, Dependency>,
}

impl Dependencies {
    /// Iterate over all dependencies with their package type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wasm_manifest::{Dependencies, Dependency, PackageType};
    /// use std::collections::HashMap;
    ///
    /// let mut deps = Dependencies::default();
    /// deps.components.insert(
    ///     "root:component".to_string(),
    ///     Dependency::Compact("ghcr.io/example/component:0.1.0".to_string()),
    /// );
    /// let all: Vec<_> = deps.all().collect();
    /// assert_eq!(all.len(), 1);
    /// assert!(all.iter().any(|(_, _, pt)| *pt == PackageType::Component));
    /// ```
    pub fn all(&self) -> impl Iterator<Item = (&String, &Dependency, PackageType)> {
        self.components
            .iter()
            .map(|(k, v)| (k, v, PackageType::Component))
            .chain(
                self.interfaces
                    .iter()
                    .map(|(k, v)| (k, v, PackageType::Interface)),
            )
    }
}

/// A dependency specification in the manifest.
///
/// Dependencies can be specified in two formats:
///
/// 1. Compact format (string):
///    ```toml
///    [dependencies.interfaces]
///    "wasi:logging" = "ghcr.io/webassembly/wasi-logging:1.0.0"
///    ```
///
/// 2. Explicit format (table):
///    ```toml
///    [dependencies.interfaces."wasi:logging"]
///    registry = "ghcr.io"
///    namespace = "webassembly"
///    package = "wasi-logging"
///    version = "1.0.0"
///    ```
///
/// # Example
///
/// ```rust
/// use wasm_manifest::{Manifest, Dependency};
///
/// let toml = r#"
/// [dependencies.interfaces]
/// "wasi:logging" = "ghcr.io/webassembly/wasi-logging:1.0.0"
///
/// [dependencies.interfaces."wasi:key-value"]
/// registry = "ghcr.io"
/// namespace = "webassembly"
/// package = "wasi-key-value"
/// version = "2.0.0"
/// "#;
///
/// let manifest: Manifest = toml::from_str(toml).unwrap();
///
/// assert!(matches!(
///     &manifest.dependencies.interfaces["wasi:logging"],
///     Dependency::Compact(_)
/// ));
/// assert!(matches!(
///     &manifest.dependencies.interfaces["wasi:key-value"],
///     Dependency::Explicit { .. }
/// ));
/// ```
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
        /// The package version or version constraint (e.g., "1.0.0", "^1.2.3", "<2.0.0").
        version: String,
        /// Optional sandbox permissions for running this component.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        permissions: Option<RunPermissions>,
    },
}

impl Dependency {
    /// Extract the version string from the dependency.
    ///
    /// For compact format, the version is the portion after the last colon.
    /// For explicit format, the version is the `version` field.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wasm_manifest::Dependency;
    ///
    /// let compact = Dependency::Compact("ghcr.io/example/pkg:1.0.0".to_string());
    /// assert_eq!(compact.version(), Some("1.0.0"));
    ///
    /// let explicit = Dependency::Explicit {
    ///     registry: "ghcr.io".into(),
    ///     namespace: "example".into(),
    ///     package: "pkg".into(),
    ///     version: "^2.0.0".into(),
    ///     permissions: None,
    /// };
    /// assert_eq!(explicit.version(), Some("^2.0.0"));
    /// ```
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        match self {
            Dependency::Compact(s) => s.rsplit_once(':').map(|(_, v)| v),
            Dependency::Explicit { version, .. } => Some(version.as_str()),
        }
    }

    /// Parse the version string as a semver version requirement.
    ///
    /// Returns `Ok` if the version is a valid semver version or version
    /// requirement (e.g. `1.0.0`, `^1.2.3`, `>=1.0.0, <2.0.0`).
    /// Returns `Err` if the version string is not valid semver.
    /// Returns `Ok(None)` if no version is present.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wasm_manifest::Dependency;
    ///
    /// let dep = Dependency::Explicit {
    ///     registry: "ghcr.io".into(),
    ///     namespace: "example".into(),
    ///     package: "pkg".into(),
    ///     version: "^1.2.3".into(),
    ///     permissions: None,
    /// };
    /// assert!(dep.parse_version_req().unwrap().is_some());
    ///
    /// let dep_bad = Dependency::Explicit {
    ///     registry: "ghcr.io".into(),
    ///     namespace: "example".into(),
    ///     package: "pkg".into(),
    ///     version: "not-a-version".into(),
    ///     permissions: None,
    /// };
    /// assert!(dep_bad.parse_version_req().is_err());
    /// ```
    pub fn parse_version_req(&self) -> Result<Option<semver::VersionReq>, semver::Error> {
        match self.version() {
            Some("" | "latest") | None => Ok(None),
            Some(v) => v.parse::<semver::VersionReq>().map(Some),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // r[verify manifest.parse.compact]
    #[test]
    fn test_parse_compact_format() {
        let toml = r#"
            [dependencies.interfaces]
            "wasi:logging" = "ghcr.io/webassembly/wasi-logging:1.0.0"
            "wasi:key-value" = "ghcr.io/webassembly/wasi-key-value:2.0.0"
        "#;

        let manifest: Manifest = toml::from_str(toml).expect("Failed to parse manifest");

        assert_eq!(manifest.dependencies.interfaces.len(), 2);
        assert!(
            manifest
                .dependencies
                .interfaces
                .contains_key("wasi:logging")
        );
        assert!(
            manifest
                .dependencies
                .interfaces
                .contains_key("wasi:key-value")
        );

        match &manifest.dependencies.interfaces["wasi:logging"] {
            Dependency::Compact(s) => {
                assert_eq!(s, "ghcr.io/webassembly/wasi-logging:1.0.0");
            }
            _ => panic!("Expected compact format"),
        }
    }

    // r[verify manifest.parse.explicit]
    #[test]
    fn test_parse_explicit_format() {
        let toml = r#"
            [dependencies.interfaces."wasi:logging"]
            registry = "ghcr.io"
            namespace = "webassembly"
            package = "wasi-logging"
            version = "1.0.0"

            [dependencies.interfaces."wasi:key-value"]
            registry = "ghcr.io"
            namespace = "webassembly"
            package = "wasi-key-value"
            version = "2.0.0"
        "#;

        let manifest: Manifest = toml::from_str(toml).expect("Failed to parse manifest");

        assert_eq!(manifest.dependencies.interfaces.len(), 2);

        match &manifest.dependencies.interfaces["wasi:logging"] {
            Dependency::Explicit {
                registry,
                namespace,
                package,
                version,
                ..
            } => {
                assert_eq!(registry, "ghcr.io");
                assert_eq!(namespace, "webassembly");
                assert_eq!(package, "wasi-logging");
                assert_eq!(version, "1.0.0");
            }
            _ => panic!("Expected explicit format"),
        }
    }

    // r[verify manifest.serialize.compact]
    #[test]
    fn test_serialize_compact_format() {
        let mut interfaces = HashMap::new();
        interfaces.insert(
            "wasi:logging".to_string(),
            Dependency::Compact("ghcr.io/webassembly/wasi-logging:1.0.0".to_string()),
        );

        let manifest = Manifest {
            dependencies: Dependencies {
                interfaces,
                ..Default::default()
            },
        };
        let toml = toml::to_string(&manifest).expect("Failed to serialize manifest");

        assert!(toml.contains("wasi:logging"));
        assert!(toml.contains("ghcr.io/webassembly/wasi-logging:1.0.0"));
    }

    // r[verify manifest.serialize.explicit]
    #[test]
    fn test_serialize_explicit_format() {
        let mut interfaces = HashMap::new();
        interfaces.insert(
            "wasi:logging".to_string(),
            Dependency::Explicit {
                registry: "ghcr.io".to_string(),
                namespace: "webassembly".to_string(),
                package: "wasi-logging".to_string(),
                version: "1.0.0".to_string(),
                permissions: None,
            },
        );

        let manifest = Manifest {
            dependencies: Dependencies {
                interfaces,
                ..Default::default()
            },
        };
        let toml = toml::to_string(&manifest).expect("Failed to serialize manifest");

        assert!(toml.contains("wasi:logging"));
        assert!(toml.contains("registry"));
        assert!(toml.contains("ghcr.io"));
    }

    // r[verify manifest.parse.empty]
    #[test]
    fn test_empty_manifest() {
        let toml = r#""#;
        let manifest: Manifest = toml::from_str(toml).expect("Failed to parse empty manifest");
        assert_eq!(manifest.dependencies.components.len(), 0);
        assert_eq!(manifest.dependencies.interfaces.len(), 0);
    }

    // r[verify manifest.parse.mixed]
    #[test]
    fn test_parse_components_and_interfaces() {
        let toml = r#"
            [dependencies.components]
            "root:component" = "ghcr.io/example/component:0.1.0"

            [dependencies.interfaces]
            "wasi:clocks" = "ghcr.io/webassembly/wasi/clocks:0.2.5"
        "#;

        let manifest: Manifest = toml::from_str(toml).expect("Failed to parse manifest");

        assert_eq!(manifest.dependencies.components.len(), 1);
        assert_eq!(manifest.dependencies.interfaces.len(), 1);
        assert!(
            manifest
                .dependencies
                .components
                .contains_key("root:component")
        );
        assert!(manifest.dependencies.interfaces.contains_key("wasi:clocks"));
    }

    // r[verify manifest.parse.all-dependencies]
    #[test]
    fn test_all_dependencies() {
        let mut components = HashMap::new();
        components.insert(
            "root:component".to_string(),
            Dependency::Compact("ghcr.io/example/component:0.1.0".to_string()),
        );
        let mut interfaces = HashMap::new();
        interfaces.insert(
            "wasi:logging".to_string(),
            Dependency::Compact("ghcr.io/webassembly/wasi-logging:1.0.0".to_string()),
        );

        let manifest = Manifest {
            dependencies: Dependencies {
                components,
                interfaces,
            },
        };

        let all: Vec<_> = manifest.all_dependencies().collect();
        assert_eq!(all.len(), 2);

        let has_component = all.iter().any(|(_, _, pt)| *pt == PackageType::Component);
        let has_interface = all.iter().any(|(_, _, pt)| *pt == PackageType::Interface);
        assert!(has_component);
        assert!(has_interface);
    }

    // r[verify manifest.parse.permissions]
    #[test]
    fn test_parse_explicit_with_permissions() {
        let toml = r#"
            [dependencies.components."root:component"]
            registry = "ghcr.io"
            namespace = "yoshuawuyts"
            package = "fetch"
            version = "latest"
            permissions.inherit-env = true
            permissions.allow-dirs = ["/data", "./output"]
        "#;

        let manifest: Manifest = toml::from_str(toml).expect("Failed to parse manifest");

        match &manifest.dependencies.components["root:component"] {
            Dependency::Explicit {
                registry,
                permissions,
                ..
            } => {
                assert_eq!(registry, "ghcr.io");
                let perms = permissions.as_ref().expect("Expected permissions");
                assert_eq!(perms.inherit_env, Some(true));
                assert_eq!(
                    perms.allow_dirs,
                    Some(vec![
                        std::path::PathBuf::from("/data"),
                        std::path::PathBuf::from("./output"),
                    ])
                );
            }
            _ => panic!("Expected explicit format"),
        }
    }

    // r[verify manifest.parse.no-permissions]
    #[test]
    fn test_explicit_without_permissions_still_works() {
        let toml = r#"
            [dependencies.components."root:component"]
            registry = "ghcr.io"
            namespace = "yoshuawuyts"
            package = "fetch"
            version = "latest"
        "#;

        let manifest: Manifest = toml::from_str(toml).expect("Failed to parse manifest");

        match &manifest.dependencies.components["root:component"] {
            Dependency::Explicit { permissions, .. } => {
                assert!(permissions.is_none());
            }
            _ => panic!("Expected explicit format"),
        }
    }

    // r[verify manifest.parse.version-constraint]
    #[test]
    fn test_parse_version_constraints() {
        let toml = r#"
            [dependencies.interfaces."wasi:logging"]
            registry = "ghcr.io"
            namespace = "webassembly"
            package = "wasi-logging"
            version = "^1.2.3"

            [dependencies.interfaces."wasi:clocks"]
            registry = "ghcr.io"
            namespace = "webassembly"
            package = "wasi-clocks"
            version = ">=1.0.0, <2.0.0"
        "#;

        let manifest: Manifest = toml::from_str(toml).expect("Failed to parse manifest");

        let logging = &manifest.dependencies.interfaces["wasi:logging"];
        assert_eq!(logging.version(), Some("^1.2.3"));
        assert!(logging.parse_version_req().unwrap().is_some());

        let clocks = &manifest.dependencies.interfaces["wasi:clocks"];
        assert_eq!(clocks.version(), Some(">=1.0.0, <2.0.0"));
        assert!(clocks.parse_version_req().unwrap().is_some());
    }

    // r[verify manifest.parse.version-constraint-invalid]
    #[test]
    fn test_invalid_version_constraint() {
        let dep = Dependency::Explicit {
            registry: "ghcr.io".to_string(),
            namespace: "example".to_string(),
            package: "pkg".to_string(),
            version: "not-a-version".to_string(),
            permissions: None,
        };
        assert!(dep.parse_version_req().is_err());
    }

    // r[verify manifest.parse.version-extract]
    #[test]
    fn test_version_extraction() {
        let compact = Dependency::Compact("ghcr.io/example/pkg:1.0.0".to_string());
        assert_eq!(compact.version(), Some("1.0.0"));

        let explicit = Dependency::Explicit {
            registry: "ghcr.io".to_string(),
            namespace: "example".to_string(),
            package: "pkg".to_string(),
            version: "^2.0.0".to_string(),
            permissions: None,
        };
        assert_eq!(explicit.version(), Some("^2.0.0"));
    }

    // r[verify manifest.parse.version-latest]
    #[test]
    fn test_version_latest_skips_validation() {
        let dep = Dependency::Explicit {
            registry: "ghcr.io".to_string(),
            namespace: "example".to_string(),
            package: "pkg".to_string(),
            version: "latest".to_string(),
            permissions: None,
        };
        assert!(dep.parse_version_req().unwrap().is_none());
    }
}
