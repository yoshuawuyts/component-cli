//! 15 — Badges.

use html::text_content::Division;

const SVG_CLOSE: &str = concat!(
    r#"<svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/x.svg"),
    "</svg>"
);

/// Status badge entries: (bg, ink, dot_class, label).
pub(crate) const STATUSES: &[(&str, &str, &str)] = &[
    (
        "bg-cat-green text-cat-greenInk",
        "bg-cat-greenInk",
        "Active",
    ),
    (
        "bg-cat-cream text-cat-creamInk",
        "bg-cat-creamInk",
        "Pending",
    ),
    ("bg-cat-pink text-cat-pinkInk", "bg-cat-pinkInk", "Failed"),
    ("bg-cat-blue text-cat-blueInk", "bg-cat-blueInk", "Info"),
];

#[allow(dead_code)]
/// Render a status badge with a colored dot.
pub(crate) fn status_badge(
    badge_class: &str,
    dot_class: &str,
    label: &str,
) -> html::inline_text::Span {
    let badge_cls = format!(
        "inline-flex items-center gap-1.5 px-2 h-6 rounded-pill text-[11px] font-medium {badge_class}"
    );
    let dot_cls = format!("h-1.5 w-1.5 rounded-full {dot_class}");
    let label = label.to_owned();
    html::inline_text::Span::builder()
        .class(badge_cls)
        .span(|s| s.class(dot_cls))
        .text(label)
        .build()
}

#[allow(dead_code)]
/// Render a count badge (small pill with number).
pub(crate) fn count_badge(value: &str) -> html::inline_text::Span {
    html::inline_text::Span::builder()
        .class("inline-flex items-center px-1.5 min-w-[20px] h-5 rounded-pill bg-ink-700 text-canvas justify-center text-[12px] font-medium")
        .text(value.to_owned())
        .build()
}

/// Render this section.
pub(crate) fn render(
    section_id: &str,
    num: &str,
    title: &str,
    desc: &str,
    statuses: &[(&str, &str, &str)],
) -> String {
    let mut status_row = Division::builder();
    status_row.class("flex flex-wrap items-center gap-2 text-[12px] font-medium");
    for (badge_cls, dot_cls, label) in statuses {
        let badge_cls =
            format!("inline-flex items-center gap-1.5 px-2 h-6 rounded-pill {badge_cls}");
        let dot_cls = format!("h-1.5 w-1.5 rounded-full {dot_cls}");
        let label = (*label).to_owned();
        let span = html::inline_text::Span::builder()
            .class(badge_cls)
            .span(|s| s.class(dot_cls))
            .text(label)
            .build();
        status_row.push(span);
    }
    status_row.span(|s| {
        s.class("inline-flex items-center px-2 h-6 rounded-pill bg-surfaceMuted text-ink-700")
            .text("Draft")
    });

    let content = Division::builder()
        .class("space-y-6")
        // Status
        .division(|d| {
            d.heading_3(|h| h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3").text("Status"))
                .push(status_row.build())
        })
        // Counts
        .division(|d| {
            d.heading_3(|h| h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3").text("Counts"))
                .division(|g| {
                    g.class("flex flex-wrap items-center gap-2 text-[12px] font-medium")
                        .span(|s| s.class("inline-flex items-center px-1.5 min-w-[20px] h-5 rounded-pill bg-ink-700 text-canvas justify-center").text("3"))
                        .span(|s| s.class("inline-flex items-center px-1.5 min-w-[20px] h-5 rounded-pill bg-surfaceMuted text-ink-700 border border-line justify-center").text("12"))
                        .span(|s| s.class("inline-flex items-center px-1.5 min-w-[20px] h-5 rounded-pill bg-cat-pink text-cat-pinkInk justify-center").text("99+"))
                })
        })
        // Tags
        .division(|d| {
            d.heading_3(|h| h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3").text("Tag"))
                .division(|g| {
                    g.class("flex flex-wrap items-center gap-2 text-[12px]")
                        .span(|s| {
                            s.class("inline-flex items-center gap-1 px-2 h-6 rounded-md border border-line text-ink-700")
                                .text("Tellus")
                                .button(|b| b.class("text-ink-400 hover:text-ink-900").text(SVG_CLOSE))
                        })
                        .span(|s| {
                            s.class("inline-flex items-center gap-1 px-2 h-6 rounded-md border border-line text-ink-700")
                                .text("Convallis")
                        })
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
            "badges",
            "15",
            "Badges",
            "Compact pill labels. Use categorical pairs for status; ink for counts and metadata.",
            STATUSES,
        )));
    }
}
