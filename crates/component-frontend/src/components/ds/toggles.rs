//! 14 — Checkbox, Radio, Switch.

use html::text_content::Division;

const SVG_CHECK: &str = concat!(
    r#"<svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/check.svg"),
    "</svg>"
);

#[allow(dead_code)]
/// Render a checkbox input with label.
pub(crate) fn checkbox(label: &str, checked: bool, disabled: bool) -> html::forms::Label {
    let label = label.to_owned();
    let mut input = html::forms::Input::builder();
    input.type_("checkbox");
    input.class("sr-only peer");
    if checked {
        input.checked("");
    }
    if disabled {
        input.disabled("");
    }
    let input = input.build().to_string();
    html::forms::Label::builder()
        .class("inline-flex items-center gap-2 cursor-pointer text-[14px]")
        .text(input)
        .span(|s| s.class("h-4 w-4 rounded border border-line bg-surface peer-checked:bg-ink-900 peer-checked:border-ink-900 grid place-items-center"))
        .span(|s| s.text(label))
        .build()
}

/// Render this section.
pub(crate) fn render(section_id: &str, num: &str, title: &str, desc: &str) -> String {
    let content = Division::builder()
        .class("space-y-8")
        // Checkbox
        .division(|d| {
            d.heading_3(|h| h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3").text("Checkbox"))
                .division(|g| {
                    g.class("space-y-2")
                        .label(|l| {
                            l.class("flex items-center gap-2 text-[14px]")
                                .span(|s| s.class("grid place-items-center h-4 w-4 rounded bg-ink-900 text-canvas").text(SVG_CHECK))
                                .text(" Aenean lectus")
                        })
                        .label(|l| {
                            l.class("flex items-center gap-2 text-[14px]")
                                .span(|s| s.class("h-4 w-4 rounded border border-line bg-surface"))
                                .text(" Vestibulum ante")
                        })
                        .label(|l| {
                            l.class("flex items-center gap-2 text-[14px] text-ink-400")
                                .span(|s| s.class("h-4 w-4 rounded border border-lineSoft bg-surfaceMuted"))
                                .text(" Disabled")
                        })
                })
        })
        // Radio
        .division(|d| {
            d.heading_3(|h| h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3").text("Radio"))
                .division(|g| {
                    g.class("space-y-2")
                        .label(|l| {
                            l.class("flex items-center gap-2 text-[14px]")
                                .span(|s| {
                                    s.class("grid place-items-center h-4 w-4 rounded-full border border-ink-900")
                                        .span(|dot| dot.class("h-2 w-2 rounded-full bg-ink-900"))
                                })
                                .text(" Lorem option")
                        })
                        .label(|l| {
                            l.class("flex items-center gap-2 text-[14px]")
                                .span(|s| s.class("h-4 w-4 rounded-full border border-line bg-surface"))
                                .text(" Ipsum option")
                        })
                })
        })
        // Switch
        .division(|d| {
            d.heading_3(|h| h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3").text("Switch"))
                .division(|g| {
                    g.class("space-y-3")
                        .label(|l| {
                            l.class("flex items-center gap-3 text-[14px]")
                                .span(|s| {
                                    s.class("relative inline-flex h-5 w-9 items-center rounded-full bg-ink-900")
                                        .span(|knob| knob.class("inline-block h-4 w-4 rounded-full bg-surface translate-x-[18px]"))
                                })
                                .text(" Enabled")
                        })
                        .label(|l| {
                            l.class("flex items-center gap-3 text-[14px]")
                                .span(|s| {
                                    s.class("relative inline-flex h-5 w-9 items-center rounded-full bg-ink-300")
                                        .span(|knob| knob.class("inline-block h-4 w-4 rounded-full bg-surface translate-x-[2px]"))
                                })
                                .text(" Disabled")
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
            "toggles",
            "14",
            "Checkbox \u{00b7} Radio \u{00b7} Switch",
            "All controls render in ink-900 when active. 16px hit area minimum on each control; full-row click target via wrapping label.",
        )));
    }
}
