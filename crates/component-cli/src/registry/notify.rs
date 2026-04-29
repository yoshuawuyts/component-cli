//! `component registry notify` subcommand.

#![allow(clippy::print_stdout, clippy::print_stderr)]

use anyhow::{Result, bail};
use component_meta_registry_types::NotifyOutcome;
use component_package_manager::Reference;
use component_package_manager::manager::{Manager, SyncPolicy, install};

/// Default meta-registry URL.
const DEFAULT_REGISTRY_URL: &str = Manager::DEFAULT_REGISTRY_URL;

/// Notify a meta-registry that a new version of a package is available.
///
/// Sends a hint to the meta-registry asking it to fetch the given tag as
/// soon as possible, instead of waiting for the next periodic sync.
#[derive(clap::Args)]
pub(crate) struct NotifyOpts {
    /// The newly-published package, given as a WIT-style name
    /// (e.g., `wasi:http@0.2.11`).
    package: String,

    /// URL of the meta-registry to notify.
    #[arg(long, default_value = DEFAULT_REGISTRY_URL)]
    registry_url: String,
}

impl NotifyOpts {
    pub(crate) async fn run(self, offline: bool) -> Result<()> {
        // The notify endpoint requires an HTTP request to the meta-registry,
        // so refuse early in offline mode before opening the store.
        if offline {
            bail!("cannot notify meta-registry in offline mode");
        }

        let manager = Manager::open().await?;
        let reference = resolve_reference(&self.package, &manager).await?;

        let registry = reference.registry();
        let repository = reference.repository();
        let Some(tag) = reference.tag() else {
            bail!(
                "'{}' has no version; specify one (e.g., `wasi:http@0.2.11`) so the registry knows which version to fetch",
                self.package
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
                    reference.whole()
                );
            }
            NotifyOutcome::Skipped { reason } => {
                println!(
                    "Notified {}: '{}' skipped ({reason})",
                    self.registry_url,
                    reference.whole()
                );
            }
        }
        Ok(())
    }
}

/// Resolve a WIT-style name to an OCI [`Reference`].
///
/// Only WIT-style names (`namespace:package@version`) are accepted; full OCI
/// references are rejected. The known-package index is opportunistically
/// refreshed before lookup.
async fn resolve_reference(input: &str, manager: &Manager) -> Result<Reference> {
    if !install::looks_like_wit_name(input) {
        bail!(
            "'{input}' is not a WIT-style package name; expected `namespace:package@version` (e.g., `wasi:http@0.2.11`)"
        );
    }

    // Refresh the known-package index so resolution can find packages
    // that haven't been touched locally yet. Failures here are
    // non-fatal — fall through to the local lookup.
    let _ = manager
        .sync_from_meta_registry(
            Manager::DEFAULT_REGISTRY_URL,
            Manager::DEFAULT_SYNC_INTERVAL,
            SyncPolicy::IfStale,
        )
        .await;
    install::resolve_wit_name(input, manager)
}
