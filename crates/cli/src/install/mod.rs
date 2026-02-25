use anyhow::Result;
use wasm_package_manager::{Manager, Reference};

use crate::util::write_lock_file;

/// Options for the `install` command.
#[derive(clap::Parser)]
pub(crate) struct Opts {
    /// The OCI reference to install (e.g., ghcr.io/webassembly/wasi-logging:1.0.0)
    reference: Reference,
}

impl Opts {
    pub(crate) async fn run(self, offline: bool) -> Result<()> {
        let deps = std::path::Path::new("deps");
        let manifest_path = deps.join("wasm.toml");
        let lockfile_path = deps.join("wasm.lock.toml");
        let vendor_dir = deps.join("vendor/wasm");

        // Read existing manifest
        let manifest_str = tokio::fs::read_to_string(&manifest_path).await?;
        let mut manifest: wasm_manifest::Manifest = toml::from_str(&manifest_str)?;

        // Read existing lockfile
        let lockfile_str = tokio::fs::read_to_string(&lockfile_path).await?;
        let mut lockfile: wasm_manifest::Lockfile = toml::from_str(&lockfile_str)?;

        // Open manager
        let manager = if offline {
            Manager::open_offline().await?
        } else {
            Manager::open().await?
        };

        // Install the package
        let result = manager.install(self.reference.clone(), &vendor_dir).await?;

        // Derive the dependency name from the repository path
        // e.g., "webassembly/wasi-logging" -> "wasi:logging" (use last segment, replace - with :)
        let dep_name = derive_dependency_name(&result.repository);

        // Determine the version from the tag
        let version = result.tag.clone().unwrap_or_default();

        // Add to manifest dependencies (compact format)
        let reference_str = self.reference.whole().to_string();
        manifest.dependencies.insert(
            dep_name.clone(),
            wasm_manifest::Dependency::Compact(reference_str),
        );

        // Add to lockfile packages
        let registry_path = format!("{}/{}", result.registry, result.repository);
        let digest = result.digest.unwrap_or_default();

        // Check if package already exists in lockfile
        let existing = lockfile
            .packages
            .iter()
            .position(|p| p.name == dep_name && p.registry == registry_path);
        let package = wasm_manifest::Package {
            name: dep_name.clone(),
            version,
            registry: registry_path,
            digest,
            dependencies: vec![],
        };
        if let Some(existing_pkg) = existing.and_then(|idx| lockfile.packages.get_mut(idx)) {
            *existing_pkg = package;
        } else {
            lockfile.packages.push(package);
        }

        // Write updated manifest
        let manifest_str = toml::to_string_pretty(&manifest)?;
        tokio::fs::write(&manifest_path, manifest_str.as_bytes()).await?;

        // Write updated lockfile
        write_lock_file(&lockfile_path, &lockfile).await?;

        // Print success message
        let vendored: Vec<_> = result
            .vendored_files
            .iter()
            .map(|p| p.display().to_string())
            .collect();
        if vendored.is_empty() {
            println!("Installed '{}'", self.reference.whole());
        } else {
            println!(
                "Installed '{}' -> {}",
                self.reference.whole(),
                vendored.join(", ")
            );
        }

        Ok(())
    }
}

/// Derive a dependency name from a repository path.
///
/// Takes a repository path like "webassembly/wasi-logging" and converts it
/// to a dependency name like "wasi-logging" (the last path segment).
fn derive_dependency_name(repository: &str) -> String {
    repository
        .rsplit('/')
        .next()
        .unwrap_or(repository)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_dependency_name() {
        assert_eq!(
            derive_dependency_name("webassembly/wasi-logging"),
            "wasi-logging"
        );
        assert_eq!(derive_dependency_name("user/my-component"), "my-component");
        assert_eq!(derive_dependency_name("simple"), "simple");
    }
}
