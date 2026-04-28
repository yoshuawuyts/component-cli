//! 06 — Tabs & Pills.

use html::text_content::Division;

#[allow(dead_code)]
/// Render the tabs & pills section.
/// Render a segmented tab control (binary toggle).
pub(crate) fn segmented_tabs(items: &[(&str, bool)]) -> Division {
    let mut div = Division::builder();
    div.class("inline-flex rounded-md border border-line overflow-hidden text-[13px]");
    for (label, active) in items {
        let class = if *active {
            "px-4 h-8 bg-surface text-ink-900 font-medium border-r border-line last:border-r-0"
        } else {
            "px-4 h-8 bg-canvas text-ink-500 hover:text-ink-900 border-r border-line last:border-r-0"
        };
        let label = (*label).to_owned();
        let class = class.to_owned();
        div.button(|b| b.type_("button").class(class).text(label));
    }
    div.build()
}

#[allow(dead_code)]
/// Render underline-style tabs.
pub(crate) fn underline_tabs(items: &[(&str, bool)]) -> Division {
    let mut div = Division::builder();
    div.class("flex gap-4 border-b-[1.5px] border-rule text-[13px]");
    for (label, active) in items {
        let class = if *active {
            "pb-2 border-b-[1.5px] border-ink-900 text-ink-900 font-medium -mb-px"
        } else {
            "pb-2 text-ink-500 hover:text-ink-900"
        };
        let label = (*label).to_owned();
        let class = class.to_owned();
        div.button(|b| b.type_("button").class(class).text(label));
    }
    div.build()
}

pub(crate) fn render(section_id: &str, num: &str, title: &str, desc: &str) -> String {
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

    super::section(section_id, num, title, desc, &content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(crate::components::ds::pretty_html(&render(
            "tabs",
            "06",
            "Tabs & Pills",
            "Segmented controls for binary scoping; underline tabs for sub-views; pills for filterable chips.",
        )));
    }
}
