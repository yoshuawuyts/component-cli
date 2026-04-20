//! Reusable UI components encoding the design system.
//!
//! Each submodule provides builder functions that return `html` crate types
//! with design-system Tailwind classes baked in. Pages call these instead of
//! writing raw class strings.

pub(crate) mod badge;
pub(crate) mod button;
pub(crate) mod code_block;
pub(crate) mod copy_button;
pub(crate) mod detail_row;
pub(crate) mod ds;
pub(crate) mod empty_state;
pub(crate) mod home_link;
pub(crate) mod icon;
pub(crate) mod link_button;
pub(crate) mod metric;
pub(crate) mod nav_link;
pub(crate) mod nav_list;
pub(crate) mod package_card;
pub(crate) mod package_row;
pub(crate) mod page_heading;
pub(crate) mod pagination;
pub(crate) mod search_bar;
pub(crate) mod section_group;
pub(crate) mod section_heading;
