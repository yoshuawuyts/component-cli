//! OCI-specific types and logic.
//!
//! This module groups all OCI registry and image concepts:
//! client communication, data models, image entries, views, and
//! pure logic for tag classification and layer management.

mod client;
mod image_entry;
mod logic;
mod models;
mod views;

pub(crate) use client::Client;
pub use image_entry::ImageEntry;
pub use logic::{
    TagKind, classify_tag, classify_tags, compute_orphaned_layers, filter_wasm_layers,
};
pub use models::InsertResult;
#[allow(unreachable_pub)]
pub use models::{OciLayer, OciLayerAnnotation, OciManifest, OciReferrer, OciRepository, OciTag};
pub use views::ImageView;
