//! `cargo xtask sql` — database schema and migration management.
//!
//! TODO(seaorm-port-phase6): rewrite this module on top of
//! [`component_package_manager_migration`]. The legacy `sqlite3def`-driven
//! flow has been removed because the schema is now defined in Rust under
//! `crates/component-package-manager-migration/`. Until Phase 6 lands, the
//! commands surface a clear error so callers know to use `sea-orm-cli`
//! directly (or `Migrator::up` from a test).

#![allow(clippy::print_stdout, clippy::print_stderr)]

use anyhow::Result;

const PHASE6_NOTE: &str = "`cargo xtask sql ...` is being rewritten as part of \
    the SeaORM port (Phase 6). Migrations now live in \
    `crates/component-package-manager-migration/`. \
    Apply them programmatically via \
    `component_package_manager_migration::Migrator::up`, or via `sea-orm-cli`.";

/// `cargo xtask sql install` — placeholder.
pub(crate) fn install() -> Result<()> {
    anyhow::bail!("{PHASE6_NOTE}");
}

/// `cargo xtask sql migrate` — placeholder.
pub(crate) fn migrate(_name: &str) -> Result<()> {
    anyhow::bail!("{PHASE6_NOTE}");
}

/// `cargo xtask sql check` — placeholder.
pub(crate) fn check() -> Result<()> {
    anyhow::bail!("{PHASE6_NOTE}");
}
