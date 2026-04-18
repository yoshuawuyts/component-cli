//! Grouped section component.
//!
//! A section with a header showing the title and item count,
//! containing card-like item rows with colored dots and badges.

use html::text_content::Division;

/// Semantic color for a WIT item type.
pub(crate) enum ItemColor {
    /// Records, variants — structural data types.
    Struct,
    /// Enums, flags — enumerable values.
    Enum,
    /// Resources — managed handles.
    Resource,
    /// Functions — callable items.
    Func,
    /// Worlds — component contracts.
    World,
    /// Interfaces — capability groups.
    Iface,
    /// Imports — inbound dependencies.
    Import,
    /// Modules — nested namespaces.
    Module,
    /// Default accent color for aliases/other.
    Accent,
}

impl ItemColor {
    /// CSS text color class for this item type.
    fn text_class(&self) -> &'static str {
        match self {
            Self::Struct => "text-wit-struct",
            Self::Enum => "text-wit-enum",
            Self::Resource => "text-wit-resource",
            Self::Func => "text-wit-func",
            Self::World => "text-wit-world",
            Self::Iface => "text-wit-iface",
            Self::Import => "text-wit-import",
            Self::Module => "text-wit-module",
            Self::Accent => "text-accent",
        }
    }
}

/// Stability badge for a WIT item.
pub(crate) enum Stability {
    /// Stable API — green badge.
    Stable,
    /// Unstable / feature-gated — cream badge.
    Unstable,
    /// No stability annotation — no badge shown.
    Unknown,
}

impl Stability {
    /// Badge classes and text, or `None` if no badge should render.
    fn badge(&self) -> Option<(&'static str, &'static str)> {
        match self {
            Self::Stable => Some((
                "text-[11px] px-1.5 py-0.5 rounded bg-cat-green text-cat-greenInk font-medium",
                "stable",
            )),
            Self::Unstable => Some((
                "text-[11px] px-1.5 py-0.5 rounded bg-cat-cream text-cat-creamInk font-medium",
                "unstable",
            )),
            Self::Unknown => None,
        }
    }
}

/// Render a grouped section header with title and count.
///
/// Produces: `Records — 3 items ∨`
pub(crate) fn header(title: &str, count: usize) -> Division {
    let count_label = match count {
        1 => "1 item".to_owned(),
        n => format!("{n} items"),
    };
    Division::builder()
        .class("flex items-baseline justify-between gap-4 pb-3 border-b border-lineSoft")
        .heading_2(|h2| {
            h2.class("text-[16px] font-semibold tracking-tight")
                .text(title.to_owned())
        })
        .division(|right| {
            right
                .class("flex items-center gap-1.5 shrink-0")
                .span(|s| {
                    s.class("text-[12px] text-ink-400")
                        .text(count_label)
                })
                .text(r#"<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-ink-400"><path d="m6 9 6 6 6-6"/></svg>"#.to_owned())
        })
        .build()
}

/// Render a single item row within a section.
///
/// Shows a colored dot, linked name, stability badge, and description.
pub(crate) fn item_row(
    name: &str,
    url: &str,
    color: ItemColor,
    stability: Stability,
    description: &str,
) -> Division {
    let text_class = color.text_class();

    let mut row = Division::builder();
    row.class(
        "flex items-start gap-3 py-3 px-3 sm:px-4 bg-surface border-b border-lineSoft last:border-b-0",
    );

    // Colored dot
    row.division(|dot| {
        dot.class(format!(
            "mt-[7px] h-2 w-2 rounded-full shrink-0 {text_class}"
        ))
        .style("background: currentColor")
    });

    // Content
    row.division(|content| {
        content.class("min-w-0 flex-1");

        // Name + badge row
        content.division(|name_row| {
            let nr = name_row.class("flex items-baseline gap-2").anchor(|a| {
                a.href(url.to_owned())
                    .class(format!(
                        "text-[14px] font-medium hover:underline truncate {text_class}"
                    ))
                    .text(name.to_owned())
            });
            if let Some((badge_class, badge_text)) = stability.badge() {
                nr.span(|s| s.class(badge_class).text(badge_text.to_owned()));
            }
            nr
        });

        // Description
        if !description.is_empty() {
            content.paragraph(|p| {
                p.class("mt-0.5 text-[13px] text-ink-500 leading-snug line-clamp-1")
                    .text(description.to_owned())
            });
        }

        content
    });

    row.build()
}
