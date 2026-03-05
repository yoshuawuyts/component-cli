//! Validation functions for manifest and lockfile consistency.

use crate::{Lockfile, Manifest};
use std::collections::HashSet;

/// Error type for validation failures.
///
/// # Example
///
/// ```rust
/// use wasm_manifest::ValidationError;
///
/// let err = ValidationError::MissingDependency {
///     name: "wasi:logging".to_string(),
/// };
/// assert_eq!(
///     err.to_string(),
///     "Package 'wasi:logging' is in the lockfile but not in the manifest"
/// );
///
/// let err = ValidationError::InvalidDependency {
///     package: "wasi:key-value".to_string(),
///     dependency: "wasi:http".to_string(),
/// };
/// assert!(err.to_string().contains("wasi:http"));
///
/// let err = ValidationError::InvalidVersionConstraint {
///     name: "wasi:logging".to_string(),
///     version: "not-a-version".to_string(),
///     reason: "unexpected character 'n' while parsing major version number".to_string(),
/// };
/// assert!(err.to_string().contains("wasi:logging"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub enum ValidationError {
    /// A package in the lockfile is not present in the manifest.
    MissingDependency {
        /// The name of the missing package.
        name: String,
    },
    /// A package dependency references a package that doesn't exist in the lockfile.
    InvalidDependency {
        /// The package that has the invalid dependency.
        package: String,
        /// The name of the dependency that doesn't exist.
        dependency: String,
    },
    /// A dependency has an invalid version constraint string.
    InvalidVersionConstraint {
        /// The name of the dependency with the invalid constraint.
        name: String,
        /// The invalid version string.
        version: String,
        /// The reason the version string is invalid.
        reason: String,
    },
    /// Two dependencies resolve to conflicting versions of the same package.
    VersionConflict {
        /// The name of the package with conflicting versions.
        name: String,
        /// The version constraint from the first source.
        constraint_a: String,
        /// The version constraint from the second source.
        constraint_b: String,
    },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::MissingDependency { name } => {
                write!(
                    f,
                    "Package '{name}' is in the lockfile but not in the manifest",
                )
            }
            ValidationError::InvalidDependency {
                package,
                dependency,
            } => {
                write!(
                    f,
                    "Package '{package}' depends on '{dependency}' which doesn't exist in the lockfile",
                )
            }
            ValidationError::InvalidVersionConstraint {
                name,
                version,
                reason,
            } => {
                write!(
                    f,
                    "Dependency '{name}' has invalid version constraint '{version}': {reason}",
                )
            }
            ValidationError::VersionConflict {
                name,
                constraint_a,
                constraint_b,
            } => {
                write!(
                    f,
                    "Conflicting version constraints for '{name}': '{constraint_a}' vs '{constraint_b}'",
                )
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validates that a lockfile is consistent with its manifest.
///
/// This function checks that:
/// - All packages in the lockfile have corresponding entries in the manifest
/// - All package dependencies reference packages that exist in the lockfile
/// - All version constraints in the manifest are valid semver requirements
/// - No conflicting version constraints exist for the same package
///
/// # Example
///
/// ```rust
/// use wasm_manifest::{Manifest, Lockfile, validate};
///
/// let manifest_toml = r#"
/// [dependencies.interfaces]
/// "wasi:logging" = "ghcr.io/webassembly/wasi-logging:1.0.0"
/// "#;
///
/// let lockfile_toml = r#"
/// lockfile_version = 3
///
/// [[interfaces]]
/// name = "wasi:logging"
/// version = "1.0.0"
/// registry = "ghcr.io/webassembly/wasi-logging"
/// digest = "sha256:abc123"
/// "#;
///
/// let manifest: Manifest = toml::from_str(manifest_toml).unwrap();
/// let lockfile: Lockfile = toml::from_str(lockfile_toml).unwrap();
///
/// assert!(validate(&manifest, &lockfile).is_ok());
/// ```
///
/// # Errors
///
/// Returns a vector of `ValidationError` if validation fails. An empty vector
/// indicates successful validation.
pub fn validate(manifest: &Manifest, lockfile: &Lockfile) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    validate_version_constraints(manifest, &mut errors);
    validate_version_conflicts(manifest, &mut errors);
    validate_lockfile_consistency(manifest, lockfile, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Check that all version constraints in the manifest are valid semver requirements.
fn validate_version_constraints(manifest: &Manifest, errors: &mut Vec<ValidationError>) {
    for (name, dep, _) in manifest.all_dependencies() {
        if let Err(e) = dep.parse_version_req() {
            let version = dep.version().unwrap_or("").to_string();
            errors.push(ValidationError::InvalidVersionConstraint {
                name: name.clone(),
                version,
                reason: e.to_string(),
            });
        }
    }
}

/// Check for conflicting version constraints on the same package name
/// across components and interfaces sections.
fn validate_version_conflicts(manifest: &Manifest, errors: &mut Vec<ValidationError>) {
    let component_keys: HashSet<&str> = manifest
        .dependencies
        .components
        .keys()
        .map(String::as_str)
        .collect();

    for name in manifest.dependencies.interfaces.keys() {
        if component_keys.contains(name.as_str()) {
            let Some(comp_dep) = manifest.dependencies.components.get(name) else {
                continue;
            };
            let Some(iface_dep) = manifest.dependencies.interfaces.get(name) else {
                continue;
            };
            let comp_version = comp_dep.version().unwrap_or("*").to_string();
            let iface_version = iface_dep.version().unwrap_or("*").to_string();
            if comp_version != iface_version {
                errors.push(ValidationError::VersionConflict {
                    name: name.clone(),
                    constraint_a: comp_version,
                    constraint_b: iface_version,
                });
            }
        }
    }
}

/// Validate lockfile consistency with manifest.
fn validate_lockfile_consistency(
    manifest: &Manifest,
    lockfile: &Lockfile,
    errors: &mut Vec<ValidationError>,
) {
    let manifest_deps: HashSet<&str> = manifest
        .all_dependencies()
        .map(|(name, _, _)| name.as_str())
        .collect();

    let lockfile_packages: HashSet<&str> = lockfile
        .all_packages()
        .map(|(p, _)| p.name.as_str())
        .collect();

    for (package, _pkg_type) in lockfile.all_packages() {
        if !manifest_deps.contains(package.name.as_str()) {
            errors.push(ValidationError::MissingDependency {
                name: package.name.clone(),
            });
        }

        for dep in &package.dependencies {
            if !lockfile_packages.contains(dep.name.as_str()) {
                errors.push(ValidationError::InvalidDependency {
                    package: package.name.clone(),
                    dependency: dep.name.clone(),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Dependencies, Dependency, Package, PackageDependency};
    use std::collections::HashMap;

    // r[verify validation.success]
    #[test]
    fn test_validate_success() {
        let mut interfaces = HashMap::new();
        interfaces.insert(
            "wasi:logging".to_string(),
            Dependency::Compact("ghcr.io/webassembly/wasi-logging:1.0.0".to_string()),
        );
        interfaces.insert(
            "wasi:key-value".to_string(),
            Dependency::Compact("ghcr.io/webassembly/wasi-key-value:2.0.0".to_string()),
        );

        let manifest = Manifest {
            dependencies: Dependencies {
                interfaces,
                ..Default::default()
            },
        };

        let lockfile = Lockfile {
            lockfile_version: 3,
            components: vec![],
            interfaces: vec![
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

        assert!(validate(&manifest, &lockfile).is_ok());
    }

    // r[verify validation.missing-dependency]
    #[test]
    fn test_validate_missing_dependency() {
        let mut interfaces = HashMap::new();
        interfaces.insert(
            "wasi:logging".to_string(),
            Dependency::Compact("ghcr.io/webassembly/wasi-logging:1.0.0".to_string()),
        );
        // Missing wasi:key-value in manifest

        let manifest = Manifest {
            dependencies: Dependencies {
                interfaces,
                ..Default::default()
            },
        };

        let lockfile = Lockfile {
            lockfile_version: 3,
            components: vec![],
            interfaces: vec![
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
                    dependencies: vec![],
                },
            ],
        };

        let result = validate(&manifest, &lockfile);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0],
            ValidationError::MissingDependency {
                name: "wasi:key-value".to_string()
            }
        );
    }

    // r[verify validation.invalid-dependency]
    #[test]
    fn test_validate_invalid_dependency() {
        let mut interfaces = HashMap::new();
        interfaces.insert(
            "wasi:logging".to_string(),
            Dependency::Compact("ghcr.io/webassembly/wasi-logging:1.0.0".to_string()),
        );
        interfaces.insert(
            "wasi:key-value".to_string(),
            Dependency::Compact("ghcr.io/webassembly/wasi-key-value:2.0.0".to_string()),
        );

        let manifest = Manifest {
            dependencies: Dependencies {
                interfaces,
                ..Default::default()
            },
        };

        let lockfile = Lockfile {
            lockfile_version: 3,
            components: vec![],
            interfaces: vec![
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
                    dependencies: vec![
                        PackageDependency {
                            name: "wasi:logging".to_string(),
                            version: "1.0.0".to_string(),
                        },
                        PackageDependency {
                            name: "wasi:http".to_string(), // This package doesn't exist
                            version: "1.0.0".to_string(),
                        },
                    ],
                },
            ],
        };

        let result = validate(&manifest, &lockfile);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0],
            ValidationError::InvalidDependency {
                package: "wasi:key-value".to_string(),
                dependency: "wasi:http".to_string()
            }
        );
    }

    // r[verify validation.empty]
    #[test]
    fn test_validate_empty() {
        let manifest = Manifest::default();

        let lockfile = Lockfile {
            lockfile_version: 3,
            components: vec![],
            interfaces: vec![],
        };

        assert!(validate(&manifest, &lockfile).is_ok());
    }

    // r[verify validation.error-display]
    #[test]
    fn test_validation_error_display() {
        let err1 = ValidationError::MissingDependency {
            name: "wasi:logging".to_string(),
        };
        assert_eq!(
            err1.to_string(),
            "Package 'wasi:logging' is in the lockfile but not in the manifest"
        );

        let err2 = ValidationError::InvalidDependency {
            package: "wasi:key-value".to_string(),
            dependency: "wasi:http".to_string(),
        };
        assert_eq!(
            err2.to_string(),
            "Package 'wasi:key-value' depends on 'wasi:http' which doesn't exist in the lockfile"
        );

        let err3 = ValidationError::InvalidVersionConstraint {
            name: "wasi:logging".to_string(),
            version: "bad".to_string(),
            reason: "unexpected".to_string(),
        };
        assert!(err3.to_string().contains("wasi:logging"));
        assert!(err3.to_string().contains("bad"));

        let err4 = ValidationError::VersionConflict {
            name: "wasi:logging".to_string(),
            constraint_a: "1.0.0".to_string(),
            constraint_b: "2.0.0".to_string(),
        };
        assert!(err4.to_string().contains("Conflicting"));
        assert!(err4.to_string().contains("wasi:logging"));
    }

    // r[verify validation.mixed-types]
    #[test]
    fn test_validate_components_and_interfaces() {
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

        let lockfile = Lockfile {
            lockfile_version: 3,
            components: vec![Package {
                name: "root:component".to_string(),
                version: "0.1.0".to_string(),
                registry: "ghcr.io/example/component".to_string(),
                digest: "sha256:comp123".to_string(),
                dependencies: vec![],
            }],
            interfaces: vec![Package {
                name: "wasi:logging".to_string(),
                version: "1.0.0".to_string(),
                registry: "ghcr.io/webassembly/wasi-logging".to_string(),
                digest: "sha256:abc123".to_string(),
                dependencies: vec![],
            }],
        };

        assert!(validate(&manifest, &lockfile).is_ok());
    }

    // r[verify validation.invalid-version-constraint]
    #[test]
    fn test_validate_invalid_version_constraint() {
        let mut interfaces = HashMap::new();
        interfaces.insert(
            "wasi:logging".to_string(),
            Dependency::Explicit {
                registry: "ghcr.io".to_string(),
                namespace: "webassembly".to_string(),
                package: "wasi-logging".to_string(),
                version: "not-a-version".to_string(),
                permissions: None,
            },
        );

        let manifest = Manifest {
            dependencies: Dependencies {
                interfaces,
                ..Default::default()
            },
        };

        let lockfile = Lockfile::default();
        let result = validate(&manifest, &lockfile);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(
            e,
            ValidationError::InvalidVersionConstraint { name, .. } if name == "wasi:logging"
        )));
    }

    // r[verify validation.version-conflict]
    #[test]
    fn test_validate_version_conflict() {
        let mut components = HashMap::new();
        components.insert(
            "shared:pkg".to_string(),
            Dependency::Compact("ghcr.io/example/pkg:1.0.0".to_string()),
        );
        let mut interfaces = HashMap::new();
        interfaces.insert(
            "shared:pkg".to_string(),
            Dependency::Compact("ghcr.io/example/pkg:2.0.0".to_string()),
        );

        let manifest = Manifest {
            dependencies: Dependencies {
                components,
                interfaces,
            },
        };

        let lockfile = Lockfile::default();
        let result = validate(&manifest, &lockfile);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(
            e,
            ValidationError::VersionConflict { name, .. } if name == "shared:pkg"
        )));
    }
}
