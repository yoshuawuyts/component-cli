//! 04 — Elevation.

use html::text_content::Division;

#[allow(dead_code)]
/// Render the elevation section.
/// Render a card with the given elevation style.
pub(crate) fn elevation_card(title: &str, spec: &str, class: &str) -> Division {
    let title = title.to_owned();
    let spec = spec.to_owned();
    let class = format!("p-5 {class}");
    Division::builder()
        .class(class)
        .division(|t| t.class("text-[13px] font-medium").text(title))
        .division(|s| s.class("mt-1 text-[12px] text-ink-500 mono").text(spec))
        .build()
}

pub(crate) fn render(section_id: &str, num: &str, title: &str, desc: &str) -> String {
    let content = Division::builder()
        .class("grid grid-cols-1 md:grid-cols-3 gap-6")
        .division(|d| {
            d.class("p-5 bg-surface border border-lineSoft rounded-lg")
                .division(|t| t.class("text-[13px] font-medium").text("Rule"))
                .division(|t| {
                    t.class("mt-1 text-[12px] text-ink-500 mono")
                        .text("border 1px #E4E4E7")
                })
        })
        .division(|d| {
            d.class("p-5 bg-surface rounded-lg shadow-card")
                .division(|t| t.class("text-[13px] font-medium").text("Card"))
                .division(|t| {
                    t.class("mt-1 text-[12px] text-ink-500 mono")
                        .text("0 1px 0 rgba(20,22,28,.04)")
                })
        })
        .division(|d| {
            d.class("p-5 backdrop-blur text-canvas rounded-md shadow-tooltip")
                .style("background: var(--c-ink-900);")
                .division(|t| t.class("text-[13px] font-medium").text("Tooltip"))
                .division(|t| {
                    t.class("mt-1 text-[12px] text-ink-300 mono")
                        .text("0 8px 24px -8px rgba(20,22,28,.35)")
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
            "elevation",
            "04",
            "Elevation",
            "Soft rules do most of the work. Shadow is reserved for floating overlays.",
        )));
    }
}
