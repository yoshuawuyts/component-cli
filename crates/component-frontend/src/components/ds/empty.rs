//! 20 — Empty State.

use html::text_content::Division;

const SVG_CHAT: &str = concat!(
    r#"<svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/message-square.svg"),
    "</svg>"
);
const SVG_PLUS: &str = concat!(
    r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/plus.svg"),
    "</svg>"
);

#[allow(dead_code)]
/// Render an empty state card with icon placeholder, title, description, and optional CTA.
pub(crate) fn empty_state(
    icon_svg: &str,
    title: &str,
    description: &str,
    cta: Option<&str>,
) -> Division {
    let icon_svg = icon_svg.to_owned();
    let title = title.to_owned();
    let description = description.to_owned();
    let mut div = Division::builder();
    div.class("border border-line rounded-lg p-12 text-center bg-surface");
    div.division(|icon| {
        icon.class(
            "mx-auto h-12 w-12 grid place-items-center rounded-full bg-surfaceMuted text-ink-500",
        )
        .text(icon_svg)
    });
    div.division(|d| {
        d.class("mt-4 text-[16px] font-semibold tracking-tight")
            .text(title)
    });
    div.paragraph(|p| {
        p.class("mt-1 text-[13px] text-ink-500 max-w-xs mx-auto")
            .text(description)
    });
    if let Some(cta_label) = cta {
        let cta_label = cta_label.to_owned();
        div.division(|d| {
            d.class("mt-6").button(|b| {
                b.type_("button")
                    .class("h-9 px-4 rounded-lg border-[1.5px] border-ink-900 bg-surface text-ink-900 text-[13px] hover:bg-surfaceMuted")
                    .text(cta_label)
            })
        });
    }
    div.build()
}

/// Render this section.
pub(crate) fn render(section_id: &str, num: &str, title: &str, desc: &str) -> String {
    let content = Division::builder()
        .class("border border-line rounded-lg p-12 text-center bg-surface")
        .division(|icon| {
            icon.class("mx-auto h-12 w-12 grid place-items-center rounded-full bg-surfaceMuted text-ink-500")
                .text(SVG_CHAT)
        })
        .division(|d| d.class("mt-4 text-[16px] font-semibold tracking-tight").text("No lorem yet"))
        .paragraph(|p| {
            p.class("mt-1 text-[13px] text-ink-500 max-w-xs mx-auto")
                .text("Pellentesque habitant morbi tristique. Get started by creating your first entry.")
        })
        .button(|btn| {
            btn.class("mt-5 h-9 px-3 inline-flex items-center gap-2 rounded-lg bg-surfaceMuted text-ink-900 text-[13px] hover:bg-ink-300")
                .text(SVG_PLUS)
                .text(" Create entry")
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
            "empty",
            "20",
            "Empty State",
            "Centered illustration glyph, title, body, and primary CTA. Used for empty tables, search misses, and first-run views.",
        )));
    }
}
