//! Component-related structures and utilities.
//!
//! This module groups together concepts related to compiled WebAssembly
//! components, including installation results, component naming, and
//! detection of whether a binary is a WIT package or a compiled component.

pub use crate::manager::{InstallResult, derive_component_name};
pub use crate::utils::is_wit_package;
