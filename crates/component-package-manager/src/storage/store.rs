//! SeaORM-backed implementation of the package-manager metadata store.
//!
//! This is the single place in the crate that talks to the database. It owns
//! a [`sea_orm::DatabaseConnection`] and exposes a method-oriented API used by
//! [`crate::manager::Manager`] and friends.
//!
//! The schema is defined in [`component_package_manager_migration`]; entities
//! used here are re-imported from that crate.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use anyhow::Context;
use chrono::{DateTime, Utc};
use futures_concurrency::prelude::*;
use oci_client::{Reference, client::ImageData, manifest::OciImageManifest};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, ConnectionTrait, Database, DatabaseConnection,
    DbBackend, EntityTrait, FromQueryResult, IntoActiveModel, ModelTrait, NotSet, QueryFilter,
    QueryOrder, QuerySelect, Set, Statement, TransactionTrait,
    sea_query::{Expr, OnConflict, Query, SimpleExpr},
};
use tracing::warn;

use component_package_manager_migration::Migrator;
use component_package_manager_migration::MigratorTrait;
use component_package_manager_migration::entities::{
    component_target, fetch_queue, oci_layer, oci_layer_annotation, oci_manifest,
    oci_manifest_annotation, oci_referrer, oci_repository, oci_tag, sync_meta, wasm_component,
    wit_package, wit_package_dependency, wit_world, wit_world_export, wit_world_import,
};

use super::config::StateInfo;
use super::known_package::KnownPackageParams;
use super::models::Migrations;
use crate::oci::{InsertResult, RawImageEntry};
use crate::types::extract_wit_metadata;

// -- Public types --------------------------------------------------------

/// The kind of work a [`FetchTask`] represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchTaskKind {
    /// Download from the OCI registry and extract metadata.
    Pull,
    /// Re-derive WIT metadata from already-cached layers.
    Reindex,
}

impl From<&str> for FetchTaskKind {
    fn from(s: &str) -> Self {
        match s {
            "reindex" => Self::Reindex,
            _ => Self::Pull,
        }
    }
}

impl From<String> for FetchTaskKind {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<fetch_queue::FetchTask> for FetchTaskKind {
    fn from(t: fetch_queue::FetchTask) -> Self {
        match t {
            fetch_queue::FetchTask::Pull => Self::Pull,
            fetch_queue::FetchTask::Reindex => Self::Reindex,
        }
    }
}

/// A single unit of work dequeued from the fetch queue.
#[derive(Debug)]
pub struct FetchTask {
    /// Row id in the `fetch_queue` table.
    pub id: i64,
    /// OCI registry hostname.
    pub registry: String,
    /// OCI repository path.
    pub repository: String,
    /// Version tag.
    pub tag: String,
    /// What to do with this tag.
    pub kind: FetchTaskKind,
    /// How many times this task has been attempted so far.
    pub attempts: i64,
}

// -- Internal helpers ----------------------------------------------------

/// Calculate the total size of a directory recursively.
async fn dir_size(path: &Path) -> u64 {
    let mut total = 0u64;
    let mut stack = vec![path.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let Ok(mut entries) = tokio::fs::read_dir(&dir).await else {
            continue;
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            let Ok(metadata) = entry.metadata().await else {
                continue;
            };
            if metadata.is_dir() {
                stack.push(entry.path());
            } else {
                total += metadata.len();
            }
        }
    }
    total
}

/// Build a SeaORM connection URL for a SQLite file at `path`.
///
/// Uses `?mode=rwc` so the file is created on first run.
fn sqlite_url(path: &Path) -> String {
    format!("sqlite://{}?mode=rwc", path.display())
}

/// Apply the SQLite-specific PRAGMAs that the legacy rusqlite path used.
async fn apply_sqlite_pragmas(db: &DatabaseConnection) -> anyhow::Result<()> {
    if !matches!(db.get_database_backend(), DbBackend::Sqlite) {
        return Ok(());
    }
    for pragma in [
        "PRAGMA foreign_keys = ON;",
        "PRAGMA journal_mode = WAL;",
        "PRAGMA synchronous = NORMAL;",
        "PRAGMA busy_timeout = 5000;",
    ] {
        db.execute_unprepared(pragma).await?;
    }
    Ok(())
}

// -- Store ---------------------------------------------------------------

/// Handle to the metadata database used by the package manager.
#[derive(Debug)]
pub(crate) struct Store {
    pub(crate) state_info: StateInfo,
    db: DatabaseConnection,
}

impl Store {
    /// Open the store in the platform's default data directory.
    pub(crate) async fn open() -> anyhow::Result<Self> {
        let data_dir = dirs::data_local_dir()
            .context("No local data dir known for the current OS")?
            .join("wasm");
        let config_file = crate::xdg_config_home()
            .context("Could not determine config directory (set $XDG_CONFIG_HOME or $HOME)")?
            .join("wasm")
            .join("config.toml");
        Self::open_inner(data_dir, config_file).await
    }

    /// Open the store at a custom data directory.
    pub(crate) async fn open_at(data_dir: impl Into<std::path::PathBuf>) -> anyhow::Result<Self> {
        let data_dir = data_dir.into();
        let config_file = data_dir.join("config.toml");
        Self::open_inner(data_dir, config_file).await
    }

    /// Shared implementation of `open` / `open_at`.
    async fn open_inner(
        data_dir: std::path::PathBuf,
        config_file: std::path::PathBuf,
    ) -> anyhow::Result<Self> {
        let store_dir = data_dir.join("store");
        let db_dir = data_dir.join("db");
        // Bumped from `metadata.db3` to `metadata-v2.db3` as part of the SeaORM
        // port — schema is incompatible with old rusqlite-managed bookkeeping.
        let metadata_file = db_dir.join("metadata-v2.db3");

        let a = tokio::fs::create_dir_all(&data_dir);
        let b = tokio::fs::create_dir_all(&store_dir);
        let c = tokio::fs::create_dir_all(&db_dir);
        let _ = (a, b, c)
            .try_join()
            .await
            .context("Could not create config directories on disk")?;

        let url = sqlite_url(&metadata_file);
        let mut opts = ConnectOptions::new(url);
        opts.sqlx_logging(false);
        let db = Database::connect(opts).await?;
        apply_sqlite_pragmas(&db).await?;
        Migrator::up(&db, None)
            .await
            .context("failed to run database migrations")?;

        let migration_info = Migrations::snapshot(&db).await;
        let store_size = dir_size(&store_dir).await;
        let metadata_size = tokio::fs::metadata(&metadata_file)
            .await
            .map_or(0, |m| m.len());
        let state_info = StateInfo::new_at(
            data_dir,
            config_file,
            &migration_info,
            store_size,
            metadata_size,
        );

        Ok(Self { state_info, db })
    }

    /// Build a Store backed by an in-memory SQLite database with all
    /// migrations applied. Used by tests.
    #[cfg(test)]
    pub(crate) async fn open_in_memory() -> anyhow::Result<Self> {
        let mut opts = ConnectOptions::new("sqlite::memory:");
        opts.sqlx_logging(false);
        let db = Database::connect(opts).await?;
        apply_sqlite_pragmas(&db).await?;
        Migrator::up(&db, None).await?;

        let tmp = tempfile::tempdir()?.keep();
        let migration_info = Migrations::snapshot(&db).await;
        let state_info = StateInfo::new_at(
            tmp.clone(),
            tmp.join("config.toml"),
            &migration_info,
            0,
            0,
        );
        Ok(Self { state_info, db })
    }

    // ---- TODO: methods below are stubbed; will be filled in incrementally.

    pub(crate) async fn insert(
        &self,
        _reference: &Reference,
        _image: ImageData,
    ) -> anyhow::Result<(
        InsertResult,
        Option<String>,
        Option<OciImageManifest>,
        Option<i64>,
    )> {
        todo!("Store::insert")
    }

    pub(crate) async fn insert_metadata(
        &self,
        _reference: &Reference,
        _digest: Option<&str>,
        _manifest: &OciImageManifest,
        _size_on_disk: u64,
    ) -> anyhow::Result<(InsertResult, Option<i64>)> {
        todo!("Store::insert_metadata")
    }

    pub(crate) async fn insert_layer(
        &self,
        _layer_digest: &str,
        _data: &[u8],
        _manifest_id: Option<i64>,
        _media_type: Option<&str>,
        _position: i32,
        _layer_annotations: Option<&BTreeMap<String, String>>,
    ) -> anyhow::Result<()> {
        todo!("Store::insert_layer")
    }

    pub(crate) async fn store_referrer(
        &self,
        _subject_manifest_id: i64,
        _registry: &str,
        _repository: &str,
        _referrer_digest: &str,
        _artifact_type: &str,
    ) -> anyhow::Result<()> {
        todo!("Store::store_referrer")
    }

    pub(crate) async fn reindex_wit_packages(&self) -> anyhow::Result<u64> {
        todo!("Store::reindex_wit_packages")
    }

    pub(crate) async fn list_all(&self) -> anyhow::Result<Vec<RawImageEntry>> {
        todo!("Store::list_all")
    }

    pub(crate) async fn delete(&self, _reference: &Reference) -> anyhow::Result<bool> {
        todo!("Store::delete")
    }

    pub(crate) async fn search_known_packages(
        &self,
        _query: &str,
        _offset: u32,
        _limit: u32,
    ) -> anyhow::Result<Vec<super::known_package::KnownPackage>> {
        todo!("Store::search_known_packages")
    }

    pub(crate) async fn search_known_packages_by_import(
        &self,
        _interface: &str,
        _offset: u32,
        _limit: u32,
    ) -> anyhow::Result<Vec<super::known_package::KnownPackage>> {
        todo!("Store::search_known_packages_by_import")
    }

    pub(crate) async fn search_known_packages_by_export(
        &self,
        _interface: &str,
        _offset: u32,
        _limit: u32,
    ) -> anyhow::Result<Vec<super::known_package::KnownPackage>> {
        todo!("Store::search_known_packages_by_export")
    }

    pub(crate) async fn list_known_packages(
        &self,
        _offset: u32,
        _limit: u32,
    ) -> anyhow::Result<Vec<super::known_package::KnownPackage>> {
        todo!("Store::list_known_packages")
    }

    pub(crate) async fn list_recent_known_packages(
        &self,
        _offset: u32,
        _limit: u32,
    ) -> anyhow::Result<Vec<super::known_package::KnownPackage>> {
        todo!("Store::list_recent_known_packages")
    }

    pub(crate) async fn get_known_package(
        &self,
        _registry: &str,
        _repository: &str,
    ) -> anyhow::Result<Option<super::known_package::KnownPackage>> {
        todo!("Store::get_known_package")
    }

    pub(crate) async fn add_known_package(
        &self,
        _registry: &str,
        _repository: &str,
        _tag: Option<&str>,
        _description: Option<&str>,
    ) -> anyhow::Result<()> {
        todo!("Store::add_known_package")
    }

    pub(crate) async fn add_known_package_with_params(
        &self,
        _params: &KnownPackageParams<'_>,
    ) -> anyhow::Result<()> {
        todo!("Store::add_known_package_with_params")
    }

    pub(crate) async fn is_tag_fresh(
        &self,
        _registry: &str,
        _repository: &str,
        _tag: &str,
        _max_age_secs: u64,
    ) -> bool {
        false
    }

    // ---- Fetch queue --------------------------------------------------

    pub(crate) async fn enqueue_pull(
        &self,
        _registry: &str,
        _repository: &str,
        _tag: &str,
        _priority: i32,
    ) -> anyhow::Result<()> {
        todo!("Store::enqueue_pull")
    }

    pub(crate) async fn record_completed(
        &self,
        _registry: &str,
        _repository: &str,
        _tag: &str,
    ) -> anyhow::Result<()> {
        todo!("Store::record_completed")
    }

    #[allow(dead_code)]
    pub(crate) async fn enqueue_reindex(
        &self,
        _registry: &str,
        _repository: &str,
        _tag: &str,
    ) -> anyhow::Result<()> {
        todo!("Store::enqueue_reindex")
    }

    pub(crate) async fn enqueue_reindex_all(&self) -> anyhow::Result<u64> {
        todo!("Store::enqueue_reindex_all")
    }

    pub(crate) async fn seed_completed_from_tags(&self) -> anyhow::Result<u64> {
        todo!("Store::seed_completed_from_tags")
    }

    pub(crate) async fn enqueue_refetch(
        &self,
        _registry: &str,
        _repository: &str,
        _tag: &str,
        _priority: i32,
    ) -> anyhow::Result<()> {
        todo!("Store::enqueue_refetch")
    }

    pub(crate) async fn dequeue_next(&self) -> anyhow::Result<Option<FetchTask>> {
        todo!("Store::dequeue_next")
    }

    pub(crate) async fn complete_task(&self, _task_id: i64) -> anyhow::Result<()> {
        todo!("Store::complete_task")
    }

    pub(crate) async fn fail_task(&self, _task_id: i64, _error: &str) -> anyhow::Result<()> {
        todo!("Store::fail_task")
    }

    pub(crate) async fn pending_count(&self) -> anyhow::Result<u64> {
        todo!("Store::pending_count")
    }

    pub(crate) async fn get_queue_status(
        &self,
    ) -> anyhow::Result<component_meta_registry_types::QueueStatus> {
        todo!("Store::get_queue_status")
    }

    pub(crate) async fn reindex_tag(
        &self,
        _registry: &str,
        _repository: &str,
        _tag: &str,
    ) -> anyhow::Result<()> {
        todo!("Store::reindex_tag")
    }

    // ---- WIT helpers --------------------------------------------------

    #[allow(dead_code)]
    pub(crate) async fn list_wit_packages(&self) -> anyhow::Result<Vec<wit_package::Model>> {
        Ok(wit_package::Entity::find().all(&self.db).await?)
    }

    pub(crate) async fn list_wit_packages_with_components(
        &self,
    ) -> anyhow::Result<Vec<(wit_package::Model, String)>> {
        todo!("Store::list_wit_packages_with_components")
    }

    pub(crate) async fn find_oci_reference_by_wit_name(
        &self,
        _package_name: &str,
        _version: Option<&str>,
    ) -> anyhow::Result<Option<(String, String)>> {
        todo!("Store::find_oci_reference_by_wit_name")
    }

    pub(crate) async fn search_known_package_by_wit_name(
        &self,
        _wit_name: &str,
    ) -> anyhow::Result<Option<super::known_package::KnownPackage>> {
        todo!("Store::search_known_package_by_wit_name")
    }

    // ---- _sync_meta ---------------------------------------------------

    #[allow(dead_code)]
    pub(crate) async fn get_sync_meta(&self, key: &str) -> anyhow::Result<Option<String>> {
        let row = sync_meta::Entity::find_by_id(key.to_owned())
            .one(&self.db)
            .await?;
        Ok(row.map(|r| r.value))
    }

    #[allow(dead_code)]
    pub(crate) async fn set_sync_meta(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let am = sync_meta::ActiveModel {
            key: Set(key.to_owned()),
            value: Set(value.to_owned()),
        };
        sync_meta::Entity::insert(am)
            .on_conflict(
                OnConflict::column(sync_meta::Column::Key)
                    .update_column(sync_meta::Column::Value)
                    .to_owned(),
            )
            .exec(&self.db)
            .await?;
        Ok(())
    }

    pub(crate) async fn get_package_dependencies(
        &self,
        _registry: &str,
        _repository: &str,
    ) -> anyhow::Result<Vec<component_meta_registry_types::PackageDependencyRef>> {
        todo!("Store::get_package_dependencies")
    }

    pub(crate) async fn get_package_dependencies_by_name(
        &self,
        _package_name: &str,
        _version: Option<&str>,
    ) -> anyhow::Result<Vec<component_meta_registry_types::PackageDependencyRef>> {
        todo!("Store::get_package_dependencies_by_name")
    }

    pub(crate) async fn list_wit_package_versions(
        &self,
        _package_name: &str,
    ) -> anyhow::Result<Vec<String>> {
        todo!("Store::list_wit_package_versions")
    }

    #[cfg(feature = "http-sync")]
    pub(crate) async fn upsert_package_dependencies_from_sync(
        &self,
        _package_name: &str,
        _version: Option<&str>,
        _dependencies: &[component_meta_registry_types::PackageDependencyRef],
    ) -> anyhow::Result<()> {
        todo!("Store::upsert_package_dependencies_from_sync")
    }

    pub(crate) async fn get_package_versions(
        &self,
        _registry: &str,
        _repository: &str,
    ) -> anyhow::Result<Vec<component_meta_registry_types::PackageVersion>> {
        todo!("Store::get_package_versions")
    }

    pub(crate) async fn get_package_version(
        &self,
        _registry: &str,
        _repository: &str,
        _version_tag: &str,
    ) -> anyhow::Result<Option<component_meta_registry_types::PackageVersion>> {
        todo!("Store::get_package_version")
    }

    pub(crate) async fn get_package_detail(
        &self,
        _registry: &str,
        _repository: &str,
    ) -> anyhow::Result<Option<component_meta_registry_types::PackageDetail>> {
        todo!("Store::get_package_detail")
    }
}

// Suppress dead-code warnings on imports that will be used as methods
// fill in.
#[allow(dead_code)]
mod _imports_used_by_todos {
    #[allow(unused_imports)]
    use super::{
        ActiveModelTrait, ColumnTrait, EntityTrait, Expr, FromQueryResult, IntoActiveModel,
        ModelTrait, NotSet, OnConflict, Query, QueryFilter, QueryOrder, QuerySelect, Set,
        SimpleExpr, Statement, TransactionTrait, component_target, extract_wit_metadata,
        fetch_queue, oci_layer, oci_layer_annotation, oci_manifest, oci_manifest_annotation,
        oci_referrer, oci_repository, oci_tag, wasm_component, warn, wit_package,
        wit_package_dependency, wit_world, wit_world_export, wit_world_import, DateTime, Utc,
        HashMap,
    };
}
