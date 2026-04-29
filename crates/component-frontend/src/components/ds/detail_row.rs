//! Detail row component.
//!
//! Key-value metadata rows for sidebars and inspector panels.
//! Matches design system section 23 Details.

use html::text_content::Division;

/// Class string for a section label in a detail list.
#[allow(dead_code)]
pub(crate) const SECTION_LABEL_CLASS: &str =
    "text-[11px] uppercase tracking-wider text-ink-500 mb-2";

/// Class string for the rule divider between detail sections.
#[allow(dead_code)]
pub(crate) const SECTION_RULE_CLASS: &str = "my-3 border-t-[1.5px] border-rule";

/// Value type for a detail row.
pub(crate) enum Value {
    /// Plain text value.
    Text(String),
    /// Linked value.
    Link { text: String, href: String },
}

/// Render a key-value detail row.
pub(crate) fn row(label: &str, value: Value) -> Division {
    let mut d = Division::builder();
    d.class("flex items-baseline justify-between gap-4 py-1.5");
    d.span(|s| {
        s.class("text-ink-500 text-[13px] shrink-0")
            .text(label.to_owned())
    });
    match value {
        Value::Text(text) => {
            d.span(|s| s.class("text-ink-900 text-[13px] text-right").text(text));
        }
        Value::Link { text, href } => {
            d.anchor(|a| {
                a.href(href)
                    .class("text-ink-700 underline decoration-line decoration-1 underline-offset-[3px] hover:text-ink-900 text-[13px] text-right truncate")
                    .text(text)
            });
        }
    }
    d.build()
}

/// Render a section label for grouping detail rows.
#[allow(dead_code)]
pub(crate) fn section_label(label: &str) -> Division {
    Division::builder()
        .class(SECTION_LABEL_CLASS)
        .text(label.to_owned())
        .build()
}

/// Render a rule divider between detail sections.
#[allow(dead_code)]
pub(crate) fn section_rule() -> Division {
    Division::builder().class(SECTION_RULE_CLASS).build()
}
