//! WIT package and world types.
//!
//! This module groups all WIT-related concepts: package and world
//! data models, the WIT metadata parser, and WIT-package
//! detection logic.

mod detect;
mod parser;
mod raw;
mod wit_package;
mod worlds;

pub use detect::is_wit_package;
pub use parser::DependencyItem;
pub(crate) use parser::extract_wit_metadata;
pub use parser::extract_wit_text;
pub(crate) use raw::RawWitPackage;
pub use wit_package::WitPackage;
pub use worlds::{WitPackageDependency, WitWorld, WitWorldExport, WitWorldImport};
