//! WIT interface structures and utilities.
//!
//! This module groups together concepts related to the [WebAssembly Interface
//! Types (WIT)](https://component-model.bytecodealliance.org/design/wit.html)
//! specification, including parsed interface metadata and identifier
//! sanitization.

pub use crate::manager::sanitize_to_wit_identifier;
pub use crate::storage::{WitInterface, WitInterfaceView};
