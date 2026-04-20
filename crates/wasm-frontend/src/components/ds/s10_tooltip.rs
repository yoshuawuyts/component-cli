//! 10 — Tooltip.

use html::text_content::Division;

/// Render the tooltip section.
pub(crate) fn render() -> String {
    let content = Division::builder()
        .class("p-12 bg-canvas border border-line rounded-lg flex items-center justify-center")
        .division(|tip| {
            tip.class("shadow-tooltip rounded-md backdrop-blur text-canvas px-3 py-2 text-[12px] leading-tight")
                .style("background: color-mix(in oklab, var(--c-ink-900) 85%, transparent);")
                .division(|lbl| {
                    lbl.class("text-ink-300")
                        .text("Cycle 14 \u{00b7} Aenean")
                })
                .division(|row| {
                    row.class("mt-1 flex items-center justify-between gap-6")
                        .span(|s| s.text("Maxima:"))
                        .span(|s| s.class("font-medium").text("9.42"))
                })
                .division(|row| {
                    row.class("flex items-center justify-between gap-6")
                        .span(|s| s.text("Minima:"))
                        .span(|s| s.class("font-medium").text("3.18"))
                })
        })
        .build()
        .to_string();

    super::section(
        "tooltip",
        "10",
        "Tooltip",
        "Inverted surface with backdrop blur. Caption label above, key/value rows with right-aligned medium values.",
        &content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(render());
    }
}
