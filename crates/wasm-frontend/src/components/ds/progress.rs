//! 19 — Progress & Spinner.

use html::text_content::Division;

const SVG_SPIN_SM: &str = concat!(
    r#"<svg class="ds-spinner text-ink-900" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">"#,
    include_str!("../../../../../vendor/lucide/loader.svg"),
    "</svg>"
);
const SVG_SPIN_MD: &str = concat!(
    r#"<svg class="ds-spinner text-ink-500" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">"#,
    include_str!("../../../../../vendor/lucide/loader.svg"),
    "</svg>"
);
const SVG_SPIN_LG: &str = concat!(
    r#"<svg class="ds-spinner text-ink-300" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round">"#,
    include_str!("../../../../../vendor/lucide/loader.svg"),
    "</svg>"
);

#[allow(dead_code)]
pub(crate) fn progress_bar(
    label: &'static str,
    pct: &'static str,
    fill_class: &'static str,
) -> Division {
    Division::builder()
        .division(|labels| {
            labels
                .class("flex justify-between text-[12px] text-ink-500 mb-1")
                .span(|s| s.text(label))
                .span(|s| s.class("mono").text(pct))
        })
        .division(|track| {
            track
                .class("h-1.5 w-full rounded-pill bg-surfaceMuted overflow-hidden")
                .division(|fill| {
                    fill.class(format!("h-full {fill_class} rounded-pill"))
                        .style(format!("width:{pct}"))
                })
        })
        .build()
}

/// Render this section.
pub(crate) fn render(section_id: &str, num: &str, title: &str, desc: &str) -> String {
    let content = Division::builder()
        .class("space-y-8")
        // Progress bar
        .division(|d| {
            d.heading_3(|h| {
                h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
                    .text("Progress bar")
            })
            .division(|g| {
                g.class("space-y-2 max-w-md")
                    .push(progress_bar("Aenean lectus", "68%", "bg-ink-900"))
                    .push(progress_bar("Pellentesque", "24%", "bg-cat-greenInk"))
            })
        })
        // Spinner
        .division(|d| {
            d.heading_3(|h| {
                h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
                    .text("Spinner")
            })
            .division(|g| {
                g.class("flex items-center gap-4")
                    .text(SVG_SPIN_SM)
                    .text(SVG_SPIN_MD)
                    .text(SVG_SPIN_LG)
            })
        })
        // Skeleton
        .division(|d| {
            d.heading_3(|h| {
                h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
                    .text("Skeleton")
            })
            .division(|g| {
                g.class("max-w-md space-y-2")
                    .division(|s| s.class("ds-skel h-4 w-2/3 rounded bg-surfaceMuted"))
                    .division(|s| s.class("ds-skel h-3 w-full rounded bg-surfaceMuted"))
                    .division(|s| s.class("ds-skel h-3 w-5/6 rounded bg-surfaceMuted"))
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
            "progress",
            "19",
            "Progress & Spinner",
            "Determinate progress as a 6px ink track. Indeterminate as a 16px spinner (CSS animation). Skeleton shimmer for placeholder content.",
        )));
    }
}
