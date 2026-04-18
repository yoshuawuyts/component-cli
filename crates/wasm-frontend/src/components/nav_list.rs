//! Navigation list component.
//!
//! Sidebar nav items with active/inactive states, matching design system
//! section 07 Navigation.

use html::text_content::{Division, ListItem, UnorderedList};

/// State of a navigation item.
pub(crate) enum NavState {
    /// Currently selected — surfaceMuted background, ink-900, font-medium.
    Active,
    /// Default — ink-700, hover:bg-surfaceMuted.
    Inactive,
}

/// Render a single nav list item (link with active/inactive styling).
pub(crate) fn item(label: &str, href: &str, state: NavState) -> ListItem {
    let cls = match state {
        NavState::Active => {
            "flex items-center px-3 h-9 rounded-md bg-surfaceMuted text-ink-900 font-medium text-[14px] truncate"
        }
        NavState::Inactive => {
            "flex items-center px-3 h-9 rounded-md text-ink-700 hover:bg-surfaceMuted text-[14px] truncate"
        }
    };
    ListItem::builder()
        .anchor(|a| a.href(href.to_owned()).class(cls).text(label.to_owned()))
        .build()
}

/// Render a labelled nav list section (eyebrow label + list of items).
pub(crate) fn section(label: &str) -> Division {
    Division::builder()
        .division(|lbl| {
            lbl.class(super::detail_row::SECTION_LABEL_CLASS)
                .text(label.to_owned())
        })
        .build()
}

/// Render a list wrapper for nav items.
pub(crate) fn list() -> UnorderedList {
    UnorderedList::builder().class("space-y-px").build()
}
