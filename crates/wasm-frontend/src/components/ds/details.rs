//! 24 — Details.

use html::text_content::{DescriptionDetails, DescriptionList, DescriptionTerm, Division};

/// An inline detail row: (label, value, value_class).
pub(crate) struct DetailEntry {
    pub(crate) label: &'static str,
    pub(crate) value: &'static str,
    pub(crate) value_class: &'static str,
}

impl DetailEntry {
    const fn new(label: &'static str, value: &'static str) -> Self {
        Self {
            label,
            value,
            value_class: "",
        }
    }
    const fn mono(label: &'static str, value: &'static str) -> Self {
        Self {
            label,
            value,
            value_class: "mono tabular-nums",
        }
    }
}

#[allow(dead_code)]
pub(crate) fn inline_dl(class: &'static str, entries: &[DetailEntry]) -> DescriptionList {
    let mut dl = DescriptionList::builder();
    dl.class(class);
    for e in entries {
        let label = e.label.to_owned();
        let value = e.value.to_owned();
        let vcls = e.value_class.to_owned();
        let dt = DescriptionTerm::builder()
            .class("text-ink-500")
            .text(label)
            .build();
        let dd = if vcls.is_empty() {
            DescriptionDetails::builder().text(value).build()
        } else {
            DescriptionDetails::builder()
                .class(vcls)
                .text(value)
                .build()
        };
        let row = Division::builder()
            .class("flex items-baseline justify-between gap-4 py-1.5")
            .push(dt)
            .push(dd)
            .build();
        dl.push(row);
    }
    dl.build()
}

#[allow(dead_code)]
pub(crate) fn stacked_dl(entries: &[DetailEntry]) -> DescriptionList {
    let mut dl = DescriptionList::builder();
    dl.class("space-y-3 max-w-[220px]");
    for e in entries {
        let label = e.label.to_owned();
        let value = e.value.to_owned();
        let vcls = if e.value_class.is_empty() {
            "text-[14px] mt-0.5".to_owned()
        } else {
            format!("text-[14px] mt-0.5 {}", e.value_class)
        };
        let dt = DescriptionTerm::builder()
            .class("text-[11px] text-ink-500 uppercase tracking-wider")
            .text(label)
            .build();
        let dd = DescriptionDetails::builder()
            .class(vcls)
            .text(value)
            .build();
        let row = Division::builder().push(dt).push(dd).build();
        dl.push(row);
    }
    dl.build()
}

pub(crate) const STACKED: &[DetailEntry] = &[
    DetailEntry::new("Status", "Active"),
    DetailEntry::new("Owner", "Aenean Lectus"),
    DetailEntry::mono("Created", "2026-04-02"),
    DetailEntry::new("Region", "eu-west-1"),
];

pub(crate) const INLINE: &[DetailEntry] = &[
    DetailEntry::new("Status", "Active"),
    DetailEntry::new("Owner", "Aenean Lectus"),
    DetailEntry::mono("Created", "2026-04-02"),
    DetailEntry::new("Region", "eu-west-1"),
    DetailEntry::mono("Replicas", "3"),
];

pub(crate) const SECTIONED_A: &[DetailEntry] = &[
    DetailEntry::new("Status", "Active"),
    DetailEntry::new("Owner", "Aenean Lectus"),
];
pub(crate) const SECTIONED_B: &[DetailEntry] = &[
    DetailEntry::new("Region", "eu-west-1"),
    DetailEntry::mono("Replicas", "3"),
    DetailEntry::mono("Uptime", "99.94%"),
];

pub(crate) const CARD_DETAILS: &[DetailEntry] = &[
    DetailEntry::new("Region", "eu-west-1"),
    DetailEntry::mono("Replicas", "3"),
    DetailEntry::mono("Created", "2026-04-02"),
    DetailEntry::mono("Uptime", "99.94%"),
];

pub(crate) const SIDEBAR_PRIMARY: &[DetailEntry] = &[
    DetailEntry::new("Status", "Active"),
    DetailEntry::new("Owner", "A. Lectus"),
    DetailEntry::new("Region", "eu-west-1"),
    DetailEntry::mono("Replicas", "3"),
];

pub(crate) const SIDEBAR_SECONDARY: &[DetailEntry] = &[
    DetailEntry::mono("Created", "2026-04-02"),
    DetailEntry::mono("Uptime", "99.94%"),
    DetailEntry {
        label: "Build",
        value: "v2.18.3",
        value_class: "mono",
    },
    DetailEntry::new("Tier", "Standard"),
];

fn sub(text: &'static str) -> html::content::Heading3 {
    html::content::Heading3::builder()
        .class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
        .text(text)
        .build()
}

/// Render this section.
#[allow(clippy::too_many_arguments)]
pub(crate) fn render(
    section_id: &str,
    num: &str,
    title: &str,
    desc: &str,
    stacked: &[DetailEntry],
    inline: &[DetailEntry],
    sectioned_a: &[DetailEntry],
    sectioned_b: &[DetailEntry],
    card_details: &[DetailEntry],
    sidebar_primary: &[DetailEntry],
    sidebar_secondary: &[DetailEntry],
) -> String {
    let content = Division::builder()
        .class("grid grid-cols-1 md:grid-cols-3 gap-8")
        // Stacked
        .division(|d| d.push(sub("Stacked")).push(stacked_dl(stacked)))
        // Inline
        .division(|d| {
            d.push(sub("Inline"))
                .push(inline_dl("text-[13px] max-w-[260px]", inline))
        })
        // Sectioned
        .division(|d| {
            d.push(sub("Sectioned")).division(|s| {
                s.class("max-w-[260px] text-[13px]")
                    .push(inline_dl("", sectioned_a))
                    .division(|rule| rule.class("my-3 border-t-[1.5px] border-rule"))
                    .division(|lbl| {
                        lbl.class("text-[11px] mono uppercase tracking-wider text-ink-500 mb-1.5")
                            .text("Infrastructure")
                    })
                    .push(inline_dl("", sectioned_b))
            })
        })
        .build()
        .to_string();

    // Combined examples (second row)
    let combined = Division::builder()
        .class("mt-10 space-y-8")
        // In a card
        .division(|d| {
            d.push(sub("In a card"))
                .division(|card| {
                    card.class("p-5 bg-surface rounded-lg shadow-card max-w-[320px]")
                        .division(|hdr| {
                            hdr.class("flex items-baseline justify-between gap-3")
                                .division(|n| {
                                    n.class("text-[14px] font-medium tracking-tight")
                                        .text("Aenean Lectus")
                                })
                                .span(|s| {
                                    s.class("inline-flex items-center px-2 h-5 rounded-pill bg-cat-green text-cat-greenInk text-[11px] font-medium")
                                        .text("Active")
                                })
                        })
                        .division(|id| id.class("text-[11px] text-ink-500 mono mt-0.5").text("id_8a4f29c1"))
                        .division(|rule| rule.class("my-4 border-t-[1.5px] border-rule"))
                        .push(inline_dl("text-[13px]", card_details))
                })
        })
        // Sidebar across regions
        .division(|d| {
            d.push(sub("As a sidebar across regions"))
                .division(|demo| {
                    demo.class("border border-line rounded-lg overflow-hidden")
                        // Primary region
                        .division(|primary| {
                            primary.class("bg-canvas p-5 space-y-5")
                                .division(|content| {
                                    content
                                        .division(|lbl| lbl.class("text-[11px] mono uppercase tracking-wider text-ink-500").text("Primary \u{00b7} canvas"))
                                        .division(|h| h.class("mt-2 text-[16px] font-semibold tracking-tight").text("Lorem ipsum dolor"))
                                        .division(|grid| {
                                            grid.class("mt-3 grid grid-cols-3 gap-2 max-w-[320px]")
                                                .division(|b| b.class("h-10 rounded bg-surfaceMuted"))
                                                .division(|b| b.class("h-10 rounded bg-surfaceMuted"))
                                                .division(|b| b.class("h-10 rounded bg-surfaceMuted"))
                                        })
                                })
                                .text({
                                    let mut aside = String::from(r#"<aside class="max-w-[260px]"><div class="text-[11px] mono uppercase tracking-wider text-ink-500 mb-2">Details</div>"#);
                                    aside.push_str(&inline_dl("text-[12px]", sidebar_primary).to_string());
                                    aside.push_str("</aside>");
                                    aside
                                })
                        })
                        // Secondary region
                        .division(|secondary| {
                            secondary.class("bg-surface p-5")
                                .division(|lbl| lbl.class("text-[11px] mono uppercase tracking-wider text-ink-500 mb-2").text("Secondary \u{00b7} surface"))
                                .push(inline_dl("text-[12px] max-w-[320px]", sidebar_secondary))
                        })
                })
        })
        .build()
        .to_string();

    let full_content = format!("{content}{combined}");

    super::section(section_id, num, title, desc, &full_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(crate::components::ds::pretty_html(&render(
            "details",
            "24",
            "Details",
            "Compact key/value lists for sidebars and inspector panels. Three variants: stacked for spacious layouts, inline for narrow rails, and sectioned when groups need separation.",
            STACKED,
            INLINE,
            SECTIONED_A,
            SECTIONED_B,
            CARD_DETAILS,
            SIDEBAR_PRIMARY,
            SIDEBAR_SECONDARY,
        )));
    }
}
