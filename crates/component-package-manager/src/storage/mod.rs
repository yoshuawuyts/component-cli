//! Cross-cutting persistence types and database storage.

mod config;
mod known_package;
mod models;
mod store;

pub use component_meta_registry_types::PackageDependencyRef;
pub use config::StateInfo;
pub use known_package::{KnownPackage, KnownPackageParams};
pub use models::Migrations;
pub(crate) use store::Store;
pub use store::{FetchTask, FetchTaskKind};
