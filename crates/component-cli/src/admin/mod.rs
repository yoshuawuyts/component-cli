//! `component admin` — administrative subcommands.
//!
//! These are operator-facing commands that don't fit the day-to-day
//! workflows of `install`, `run`, etc. Today the only subcommand is
//! `migrate`, which applies pending database migrations.
//!
//! # Migration semantics
//!
//! * **SQLite** (default): migrations are applied automatically on every
//!   `Manager::open`. Running `component admin migrate` is still safe, but
//!   redundant.
//! * **PostgreSQL**: migrations are NOT applied automatically because two
//!   replicas booting concurrently would race. Operators must run
//!   `component admin migrate` (or `sea-orm-cli`) once during deploy
//!   before starting the service. `Manager::open` refuses to start when
//!   the database has pending migrations.

#![allow(clippy::print_stdout)]

use anyhow::{Context, Result};
use component_package_manager::storage::DbConfig;
use component_package_manager_migration::{Migrator, MigratorTrait};
use sea_orm::Database;

/// Administrative commands.
#[derive(clap::Parser)]
pub(crate) enum Opts {
    /// Apply any pending database migrations.
    Migrate,
}

impl Opts {
    pub(crate) async fn run(&self) -> Result<()> {
        match self {
            Opts::Migrate => migrate().await,
        }
    }
}

async fn migrate() -> Result<()> {
    // Use the same default SQLite path as `Manager::open` so this command
    // targets the user's local database when no env var is set.
    let default_path = default_sqlite_path()?;
    let cfg = DbConfig::from_env(&default_path)?;
    println!("Connecting to {} ...", cfg.redacted_url());
    let db = Database::connect(cfg.to_connect_options())
        .await
        .with_context(|| format!("failed to connect to {}", cfg.redacted_url()))?;
    println!("Applying pending migrations ...");
    Migrator::up(&db, None)
        .await
        .context("failed to apply migrations")?;
    println!("Migrations applied successfully.");
    Ok(())
}

/// Compute the default SQLite metadata path used by `Manager::open`.
fn default_sqlite_path() -> Result<std::path::PathBuf> {
    let data_dir = dirs::data_local_dir()
        .context("No local data dir known for the current OS")?
        .join("wasm");
    Ok(data_dir.join("db").join("metadata-v2.db3"))
}
