//! 10 — Tooltip.

use html::text_content::Division;

#[allow(dead_code)]
/// Render the tooltip section.
/// Render a tooltip card with a label and key-value rows.
pub(crate) fn tooltip(label: &str, rows: &[(&str, &str)]) -> Division {
    let label = label.to_owned();
    let mut div = Division::builder();
    div.class("w-48 bg-ink-900 text-canvas rounded-lg px-3 py-2.5 shadow-tooltip text-[12px]");
    div.division(|d| d.class("text-ink-400 mb-1.5").text(label));
    for (key, value) in rows {
        let key = (*key).to_owned();
        let value = (*value).to_owned();
        div.division(|d| {
            d.class("flex items-baseline justify-between gap-4 py-0.5")
                .span(|s| s.class("text-ink-400").text(key))
                .span(|s| s.class("font-medium").text(value))
        });
    }
    div.build()
}

pub(crate) fn render(section_id: &str, num: &str, title: &str, desc: &str) -> String {
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

    super::section(section_id, num, title, desc, &content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(crate::components::ds::pretty_html(&render(
            "tooltip",
            "10",
            "Tooltip",
            "Inverted surface with backdrop blur. Caption label above, key/value rows with right-aligned medium values.",
        )));
    }
}
