//! Button components.
//!
//! Two variants (filled, outline) × two sizes (compact, large).

use html::text_content::Division;

/// Button visual variant.
#[allow(dead_code)]
pub(crate) enum Variant {
    /// Soft gray fill.
    Filled,
    /// 1.5px ink outline on white surface.
    Outline,
}

/// Button size.
#[allow(dead_code)]
pub(crate) enum Size {
    /// 32px — compact toolbars.
    Compact,
    /// 36px — mobile / primary CTAs.
    Large,
}

/// Render a button with the given variant, size, and label.
#[allow(dead_code)]
pub(crate) fn render(variant: &Variant, size: &Size, label: &str) -> Division {
    let height = match *size {
        Size::Compact => "h-8",
        Size::Large => "h-9",
    };
    let style = match *variant {
        Variant::Filled => "bg-surfaceMuted text-ink-900 hover:bg-ink-300",
        Variant::Outline => {
            "border-[1.5px] border-ink-900 bg-surface text-ink-900 hover:bg-surfaceMuted"
        }
    };

    Division::builder()
        .class("inline-block")
        .button(|btn| {
            btn.type_("button")
                .class(format!(
                    "{height} px-3 inline-flex items-center gap-2 rounded-lg text-[13px] {style}"
                ))
                .text(label.to_owned())
        })
        .build()
}
