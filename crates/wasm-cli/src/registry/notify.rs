//! `wasm registry notify` subcommand.

#![allow(clippy::print_stdout, clippy::print_stderr)]

use anyhow::{Result, bail};
use wasm_meta_registry_types::NotifyOutcome;
use wasm_package_manager::Reference;
use wasm_package_manager::manager::Manager;

/// Default meta-registry URL.
const DEFAULT_REGISTRY_URL: &str = Manager::DEFAULT_REGISTRY_URL;

/// Notify a meta-registry that a new version of a package is available.
///
/// Sends a hint to the meta-registry asking it to fetch the given tag as
/// soon as possible, instead of waiting for the next periodic sync.
#[derive(clap::Args)]
pub(crate) struct NotifyOpts {
    /// The reference of the newly-published version
    /// (e.g., `ghcr.io/example/component:1.2.3`).
    #[arg(value_parser = crate::util::parse_reference)]
    reference: Reference,

    /// URL of the meta-registry to notify.
    #[arg(long, default_value = DEFAULT_REGISTRY_URL)]
    registry_url: String,
}

impl NotifyOpts {
    pub(crate) async fn run(self, offline: bool) -> Result<()> {
        let manager = if offline {
            Manager::open_offline().await?
        } else {
            Manager::open().await?
        };

        let registry = self.reference.registry();
        let repository = self.reference.repository();
        let Some(tag) = self.reference.tag() else {
            bail!(
                "reference '{}' has no tag; specify a tag (e.g., `:1.2.3`) so the registry knows which version to fetch",
                self.reference.whole()
            );
        };

        let outcome = manager
            .notify_meta_registry(&self.registry_url, registry, repository, tag)
            .await?;

        match outcome {
            NotifyOutcome::Enqueued => {
                println!(
                    "Notified {}: '{}' enqueued for fetch",
                    self.registry_url,
                    self.reference.whole()
                );
            }
            NotifyOutcome::Skipped { reason } => {
                println!(
                    "Notified {}: '{}' skipped ({reason})",
                    self.registry_url,
                    self.reference.whole()
                );
            }
        }
        Ok(())
    }
}
