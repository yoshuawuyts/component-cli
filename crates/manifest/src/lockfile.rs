//! Types for the WASM lockfile (`wasm.lock`).

use serde::{Deserialize, Serialize};

/// The current revision of the lockfile.
pub const LOCKFILE_VERSION: u32 = 1;

/// The root lockfile structure for a WASM package.
///
/// The lockfile (`deps/wasm.lock`) is auto-generated and tracks resolved dependencies
/// with their exact versions and content digests.
///
/// # Example
///
/// ```toml
/// version = 1
///
/// [[package]]
/// name = "wasi:logging"
/// version = "1.0.0"
/// registry = "ghcr.io/webassembly/wasi-logging"
/// digest = "sha256:abc123..."
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct Lockfile {
    /// The lockfile format version.
    pub lockfile_version: u32,

    /// The list of resolved packages.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<Package>,
}

impl Default for Lockfile {
    fn default() -> Self {
        Self {
            lockfile_version: LOCKFILE_VERSION,
            packages: Default::default(),
        }
    }
}

/// A resolved package entry in the lockfile.
///
/// Each package represents a dependency that has been resolved to a specific
/// version with a content digest for integrity verification.
///
/// # Example with dependencies
///
/// ```toml
/// [[package]]
/// name = "wasi:key-value"
/// version = "2.0.0"
/// registry = "ghcr.io/webassembly/wasi-key-value"
/// digest = "sha256:def456..."
///
/// [[package.dependencies]]
/// name = "wasi:logging"
/// version = "1.0.0"
/// ```
///
/// Note: `[[package.dependencies]]` defines dependencies for the last `[[package]]` entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct Package {
    /// The package name (e.g., "wasi:logging").
    pub name: String,

    /// The package version (e.g., "1.0.0").
    pub version: String,

    /// The full registry path (e.g., "ghcr.io/webassembly/wasi-logging").
    pub registry: String,

    /// The content digest for integrity verification (e.g., "sha256:abc123...").
    pub digest: String,

    /// Optional dependencies of this package.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<PackageDependency>,
}

/// A dependency reference within a package.
///
/// This represents a dependency that a package has on another package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct PackageDependency {
    /// The name of the dependency package.
    pub name: String,

    /// The version of the dependency package.
    pub version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lockfile() {
        let toml = r#"
            version = 1

            [[package]]
            name = "wasi:logging"
            version = "1.0.0"
            registry = "ghcr.io/webassembly/wasi-logging"
            digest = "sha256:a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456"

            [[package]]
            name = "wasi:key-value"
            version = "2.0.0"
            registry = "ghcr.io/webassembly/wasi-key-value"
            digest = "sha256:b2c3d4e5f67890123456789012345678901abcdef2345678901abcdef2345678"

            [[package.dependencies]]
            name = "wasi:logging"
            version = "1.0.0"
        "#;

        let lockfile: Lockfile = toml::from_str(toml).expect("Failed to parse lockfile");

        assert_eq!(lockfile.lockfile_version, 1);
        assert_eq!(lockfile.packages.len(), 2);

        let logging = &lockfile.packages[0];
        assert_eq!(logging.name, "wasi:logging");
        assert_eq!(logging.version, "1.0.0");
        assert_eq!(logging.registry, "ghcr.io/webassembly/wasi-logging");
        assert!(logging.digest.starts_with("sha256:"));

        let key_value = &lockfile.packages[1];
        assert_eq!(key_value.name, "wasi:key-value");
        assert_eq!(key_value.version, "2.0.0");
        assert_eq!(key_value.dependencies.len(), 1);
        assert_eq!(key_value.dependencies[0].name, "wasi:logging");
        assert_eq!(key_value.dependencies[0].version, "1.0.0");
    }

    #[test]
    fn test_serialize_lockfile() {
        let lockfile = Lockfile {
            lockfile_version: 1,
            packages: vec![
                Package {
                    name: "wasi:logging".to_string(),
                    version: "1.0.0".to_string(),
                    registry: "ghcr.io/webassembly/wasi-logging".to_string(),
                    digest: "sha256:abc123".to_string(),
                    dependencies: vec![],
                },
                Package {
                    name: "wasi:key-value".to_string(),
                    version: "2.0.0".to_string(),
                    registry: "ghcr.io/webassembly/wasi-key-value".to_string(),
                    digest: "sha256:def456".to_string(),
                    dependencies: vec![PackageDependency {
                        name: "wasi:logging".to_string(),
                        version: "1.0.0".to_string(),
                    }],
                },
            ],
        };

        let toml = toml::to_string(&lockfile).expect("Failed to serialize lockfile");

        assert!(toml.contains("version = 1"));
        assert!(toml.contains("wasi:logging"));
        assert!(toml.contains("wasi:key-value"));
        assert!(toml.contains("sha256:abc123"));
    }

    #[test]
    fn test_package_without_dependencies() {
        let toml = r#"
            version = 1

            [[package]]
            name = "wasi:logging"
            version = "1.0.0"
            registry = "ghcr.io/webassembly/wasi-logging"
            digest = "sha256:abc123"
        "#;

        let lockfile: Lockfile = toml::from_str(toml).expect("Failed to parse lockfile");

        assert_eq!(lockfile.packages.len(), 1);
        assert_eq!(lockfile.packages[0].dependencies.len(), 0);
    }

    #[test]
    fn test_serialize_package_without_dependencies() {
        let package = Package {
            name: "wasi:logging".to_string(),
            version: "1.0.0".to_string(),
            registry: "ghcr.io/webassembly/wasi-logging".to_string(),
            digest: "sha256:abc123".to_string(),
            dependencies: vec![],
        };

        let toml = toml::to_string(&package).expect("Failed to serialize package");

        // Empty dependencies should be skipped
        assert!(!toml.contains("dependencies"));
    }
}
