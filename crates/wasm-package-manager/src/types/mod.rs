//! WIT type and world types.
//!
//! This module groups all WIT-related concepts: type and world
//! data models, views, the WIT metadata parser, and WIT-package
//! detection logic.

mod detect;
mod models;
mod parser;
mod views;
mod worlds;

pub use detect::is_wit_package;
pub use models::WitType;
pub(crate) use parser::extract_wit_metadata;
pub use views::WitTypeView;
pub use worlds::{WitTypeDependency, WitWorld, WitWorldExport, WitWorldImport};
