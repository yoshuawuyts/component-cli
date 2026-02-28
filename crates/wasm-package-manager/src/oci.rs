//! OCI-specific types and utilities for interacting with OCI registries.
//!
//! This module groups together concepts related to the [Open Container
//! Initiative (OCI)](https://opencontainers.org/) image and distribution
//! specifications, including image metadata, tag classification, layer
//! management, and pull results.

pub use crate::manager::{
    PullResult, TagKind, classify_tag, classify_tags, compute_orphaned_layers, filter_wasm_layers,
    vendor_filename,
};
pub use crate::storage::{ImageEntry, ImageView, InsertResult};
pub use oci_client::Reference;
