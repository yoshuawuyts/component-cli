//! 06 — Tabs & Pills.

use html::text_content::Division;

/// Render the tabs & pills section.
pub(crate) fn render() -> String {
    let content = Division::builder()
        .class("space-y-8")
        .division(|seg_group| {
            seg_group
                .heading_3(|h| {
                    h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
                        .text("Segmented")
                })
                .division(|seg| {
                    seg.class("flex p-1 rounded-lg bg-surfaceMuted w-[200px]")
                        .button(|b| {
                            b.class("flex-1 h-7 rounded-md bg-ink-900 text-canvas text-[13px] font-medium")
                                .text("Lorem")
                        })
                        .button(|b| {
                            b.class("flex-1 h-7 rounded-md text-[13px] text-ink-500")
                                .text("Ipsum")
                        })
                })
        })
        .division(|tab_group| {
            tab_group
                .heading_3(|h| {
                    h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
                        .text("Underline tabs")
                })
                .division(|tabs| {
                    tabs.class("flex items-center gap-6 border-b-[1.5px] border-rule")
                        .button(|b| {
                            b.class("relative pb-3 text-[15px] font-medium")
                                .text("Aenean")
                                .span(|s| {
                                    s.class("absolute left-0 right-0 -bottom-[1.5px] h-[1.5px] bg-ink-900")
                                })
                        })
                        .button(|b| b.class("pb-3 text-[15px] text-ink-500").text("Mauris"))
                        .button(|b| b.class("pb-3 text-[15px] text-ink-500").text("Vivamus"))
                })
        })
        .build()
        .to_string();

    super::section(
        "tabs",
        "06",
        "Tabs & Pills",
        "Segmented controls for binary scoping; underline tabs for sub-views; pills for filterable chips.",
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
