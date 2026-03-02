#![allow(clippy::print_stdout, clippy::print_stderr)]

use anyhow::{Context, Result};
use wasm_package_manager::Reference;
use wasm_package_manager::manager::Manager;

/// Options for the `add` command.
#[derive(clap::Parser)]
pub(crate) struct Opts {
    /// The OCI references to add (e.g., ghcr.io/webassembly/wasi-logging:1.0.0).
    #[arg(value_parser = crate::util::parse_reference, value_name = "REFERENCE", required = true)]
    references: Vec<Reference>,
}

impl Opts {
    pub(crate) async fn run(self, offline: bool) -> Result<()> {
        let deps = std::path::Path::new("deps");
        let manifest_path = deps.join("wasm.toml");

        // Read existing manifest — error if not found, recommend `wasm init`
        let manifest_str = tokio::fs::read_to_string(&manifest_path)
            .await
            .with_context(|| {
                format!(
                    "could not read '{}'. Run `wasm init` first to create the project files",
                    manifest_path.display()
                )
            })?;
        let mut manifest: wasm_manifest::Manifest = toml::from_str(&manifest_str)?;

        // Open manager
        let manager = if offline {
            Manager::open_offline().await?
        } else {
            Manager::open().await?
        };

        // Build the set of existing names once; update it as we add entries.
        let mut existing_names: std::collections::HashSet<String> = manifest
            .components
            .keys()
            .chain(manifest.interfaces.keys())
            .cloned()
            .collect();

        for reference in &self.references {
            let result = manager.add(reference, &existing_names).await?;

            // Add to manifest (compact format) — default to interfaces since
            // we don't inspect the layers to determine the type.
            let reference_str = reference.whole().clone();
            let dep = wasm_manifest::Dependency::Compact(reference_str);
            manifest.interfaces.insert(result.dep_name.clone(), dep);
            existing_names.insert(result.dep_name.clone());

            println!(
                "{:>12} {} as \"{}\"",
                console::style("Added").green().bold(),
                reference.whole(),
                result.dep_name,
            );
        }

        // Write updated manifest
        let manifest_str = toml::to_string_pretty(&manifest)?;
        tokio::fs::write(&manifest_path, manifest_str.as_bytes()).await?;

        Ok(())
    }
}
