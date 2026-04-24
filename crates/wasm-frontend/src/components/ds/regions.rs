//! 22 — Regions.

use html::text_content::Division;

pub(crate) const RULES: &[(&str, &str)] = &[
    (
        "01",
        "The primary region (top of page) sits on <code class=\"px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]\">canvas</code>. Use it for the main subject.",
    ),
    (
        "02",
        "Secondary regions sit on <code class=\"px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]\">surface</code>. The boundary is the surface swap itself \u{2014} no rule, no border.",
    ),
    (
        "03",
        "Use full-bleed background swap on wide screens so the boundary reads as a true section break, not a card.",
    ),
    (
        "04",
        "Maximum two surface swaps per page. Beyond that, switch to a new page or tabs.",
    ),
    (
        "05",
        "Within a region, use <code class=\"px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]\">lineSoft</code> for internal subdivisions (table rows, list separators).",
    ),
];

#[allow(dead_code)]
/// Render a page region with the given surface class and content.
pub(crate) fn region(surface_class: &str, content: &str) -> Division {
    let surface_class = format!("py-8 md:py-12 {surface_class}");
    let content = content.to_owned();
    Division::builder()
        .class(surface_class)
        .text(content)
        .build()
}

/// Render this section.
pub(crate) fn render(
    section_id: &str,
    num: &str,
    title: &str,
    desc: &str,
    rules: &[(&str, &str)],
) -> String {
    let mut rules_ul = html::text_content::UnorderedList::builder();
    rules_ul.class("space-y-2 text-[13px] text-ink-700 leading-relaxed");
    for (num, text) in rules {
        let num = (*num).to_owned();
        let text = (*text).to_owned();
        rules_ul.list_item(|li| {
            li.class("flex gap-3")
                .span(|s| s.class("mono text-ink-500 w-12 shrink-0").text(num))
                .paragraph(|p| p.text(text))
        });
    }

    let content = Division::builder()
        .class("space-y-8")
        .division(|demo| {
            demo.class("border border-line rounded-lg overflow-hidden")
                .division(|primary| {
                    primary
                        .class("bg-canvas p-6")
                        .division(|d| {
                            d.class("text-[11px] mono uppercase tracking-wider text-ink-500")
                                .text("Primary region \u{00b7} canvas")
                        })
                        .division(|d| {
                            d.class("mt-3 text-[18px] font-semibold tracking-tight")
                                .text("Lorem ipsum dolor sit")
                        })
                        .division(|grid| {
                            grid.class("mt-4 grid grid-cols-3 gap-3")
                                .division(|d| d.class("h-12 rounded bg-surfaceMuted"))
                                .division(|d| d.class("h-12 rounded bg-surfaceMuted"))
                                .division(|d| d.class("h-12 rounded bg-surfaceMuted"))
                        })
                })
                .division(|secondary| {
                    secondary
                        .class("bg-surface p-6")
                        .division(|d| {
                            d.class("text-[11px] mono uppercase tracking-wider text-ink-500")
                                .text("Secondary region \u{00b7} surface")
                        })
                        .division(|d| {
                            d.class("mt-3 text-[18px] font-semibold tracking-tight")
                                .text("Aenean lectus pellentesque")
                        })
                        .division(|d| d.class("mt-4 h-px bg-lineSoft"))
                        .division(|grid| {
                            grid.class("mt-4 grid grid-cols-4 gap-3 text-[12px] text-ink-500")
                                .division(|d| d.text("Vestibulum"))
                                .division(|d| d.text("Convallis"))
                                .division(|d| d.text("Tempor"))
                                .division(|d| d.text("Faucibus"))
                        })
                })
        })
        .division(|rules| {
            rules
                .heading_3(|h| {
                    h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
                        .text("Rules")
                })
                .push(rules_ul.build())
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
            "regions",
            "22",
            "Regions",
            "Pages are composed of stacked <em>regions</em>. The primary region uses the canvas surface; secondary regions (supporting data, references, appendices) switch to the white surface. The surface swap signals \u{201c}this is additional content\u{201d} \u{2014} no rules or borders are drawn between regions.",
            RULES,
        )));
    }
}
