//! Link-button component.
//!
//! Anchor (`<a>`) elements styled as buttons. Two variants: a high-contrast
//! primary fill or an ink outline on the surface background.

use html::inline_text::Anchor;

/// Link-button visual variant.
pub(crate) enum Variant {
    /// High-contrast primary: dark fill, light text.
    Primary,
    /// Ink outline on surface background.
    Outline,
}

/// Class string for a primary link-button.
const PRIMARY_CLASS: &str = "h-9 px-4 inline-flex items-center rounded-lg bg-ink-900 text-canvas text-[13px] font-medium hover:bg-ink-700";

/// Class string for an outline link-button.
const OUTLINE_CLASS: &str = "h-9 px-4 inline-flex items-center rounded-lg border-[1.5px] border-ink-900 bg-surface text-ink-900 text-[13px] hover:bg-surfaceMuted";

/// Render an anchor styled as a button.
pub(crate) fn render(variant: &Variant, href: &str, label: &str) -> Anchor {
    let class = match *variant {
        Variant::Primary => PRIMARY_CLASS,
        Variant::Outline => OUTLINE_CLASS,
    };
    Anchor::builder()
        .href(href.to_owned())
        .class(class)
        .text(label.to_owned())
        .build()
}
