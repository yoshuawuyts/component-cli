//! 23 — Motion.

use html::text_content::Division;

const SVG_STANDARD: &str = r#"<svg class="ease-curve mt-3" viewBox="0 0 200 56" preserveAspectRatio="none"><path class="track" d="M0,56 L200,56 M0,0 L0,56" /><path class="curve" d="M0,56 C40,56 0,0 200,0" /></svg>"#;
const SVG_ENTRANCE: &str = r#"<svg class="ease-curve mt-3" viewBox="0 0 200 56" preserveAspectRatio="none"><path class="track" d="M0,56 L200,56 M0,0 L0,56" /><path class="curve" d="M0,56 C0,56 0,0 200,0" /></svg>"#;
const SVG_EXIT: &str = r#"<svg class="ease-curve mt-3" viewBox="0 0 200 56" preserveAspectRatio="none"><path class="track" d="M0,56 L200,56 M0,0 L0,56" /><path class="curve" d="M0,56 C80,56 200,56 200,0" /></svg>"#;
const SVG_SPRING: &str = r#"<svg class="ease-curve mt-3" viewBox="0 0 200 56" preserveAspectRatio="none"><path class="track" d="M0,56 L200,56 M0,0 L0,56" /><path class="curve" d="M0,56 C68,56 120,-18 200,0" /></svg>"#;

pub(crate) struct Curve {
    pub(crate) name: &'static str,
    pub(crate) value: &'static str,
    pub(crate) svg: &'static str,
    pub(crate) desc: &'static str,
}

pub(crate) const CURVES: &[Curve] = &[
    Curve {
        name: "Standard",
        value: "cubic-bezier(.2,0,0,1)",
        svg: SVG_STANDARD,
        desc: "Default for state changes \u{2014} hover, focus, expand.",
    },
    Curve {
        name: "Entrance",
        value: "cubic-bezier(0,0,0,1)",
        svg: SVG_ENTRANCE,
        desc: "Elements arriving on screen \u{2014} toasts, popovers, modals.",
    },
    Curve {
        name: "Exit",
        value: "cubic-bezier(.4,0,1,1)",
        svg: SVG_EXIT,
        desc: "Elements leaving \u{2014} dismissed alerts, closed sheets.",
    },
    Curve {
        name: "Spring",
        value: "cubic-bezier(.34,1.56,.64,1)",
        svg: SVG_SPRING,
        desc: "Reserved for direct manipulation feedback \u{2014} toggles, drag-snap.",
    },
];

pub(crate) const DURATIONS: &[(&str, &str, &str)] = &[
    ("fast", "120ms", "Color, opacity, focus rings."),
    (
        "base",
        "180ms",
        "Default transition. Hover, expand-collapse.",
    ),
    ("slow", "260ms", "Position changes, panel slides."),
    ("page", "360ms", "Route transitions, modal open."),
];

pub(crate) const PREVIEWS: &[(&str, &str)] = &[
    ("fast \u{00b7} std", "t-fast"),
    ("base \u{00b7} std", "t-base"),
    ("slow \u{00b7} std", "t-slow"),
    ("page \u{00b7} spring", "t-spring"),
];

pub(crate) const RULES: &[(&str, &str)] = &[
    (
        "01",
        r#"Animate <code class="px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]">transform</code> and <code class="px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]">opacity</code> only. Avoid <code class="px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]">width</code>, <code class="px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]">height</code>, <code class="px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]">top</code>."#,
    ),
    (
        "02",
        r#"Default to <code class="px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]">base</code> + <code class="px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]">standard</code>. Pick another token only when intent demands it."#,
    ),
    (
        "03",
        r#"Pair entrance and exit asymmetrically: enter on <code class="px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]">entrance</code>, leave on <code class="px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]">exit</code>, exits 60\u{2013}80% of the entrance duration."#,
    ),
    (
        "04",
        "Spring is for human-initiated, direct manipulation only. Never for system events.",
    ),
    (
        "05",
        r#"Honor <code class="px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]">prefers-reduced-motion</code> \u{2014} collapse to instant or to a 60ms cross-fade."#,
    ),
];

/// Render this section.
#[allow(clippy::too_many_arguments)]
pub(crate) fn render(
    section_id: &str,
    num: &str,
    title: &str,
    desc: &str,
    curves: &[Curve],
    durations: &[(&str, &str, &str)],
    previews: &[(&str, &str)],
    rules: &[(&str, &str)],
) -> String {
    // Curves grid
    let mut curves_grid = Division::builder();
    curves_grid.class("grid grid-cols-1 md:grid-cols-2 gap-4");
    for c in curves {
        let card = Division::builder()
            .class("p-4 rounded-md border border-lineSoft")
            .division(|hdr| {
                hdr.class("flex items-baseline justify-between")
                    .division(|n| n.class("text-[13px] font-medium").text(c.name))
                    .division(|v| v.class("text-[11px] mono text-ink-500").text(c.value))
            })
            .text(c.svg)
            .division(|d| d.class("mt-2 text-[12px] text-ink-500").text(c.desc))
            .build();
        curves_grid.push(card);
    }

    // Durations
    let mut dur_rows = Division::builder();
    dur_rows.class("divide-y divide-lineSoft border-t border-lineSoft");
    for (name, ms, desc) in durations {
        let name = (*name).to_owned();
        let ms = (*ms).to_owned();
        let desc = (*desc).to_owned();
        let row = Division::builder()
            .class("py-3 grid grid-cols-[80px_80px_1fr] gap-4 items-center text-[13px]")
            .span(|s| s.class("mono").text(name))
            .span(|s| s.class("mono text-ink-500").text(ms))
            .span(|s| s.class("text-ink-700").text(desc))
            .build();
        dur_rows.push(row);
    }

    // Preview tracks
    let mut preview_rows = Division::builder();
    preview_rows.class("space-y-2");
    for (label, target_class) in previews {
        let label = (*label).to_owned();
        let target_cls = format!("motion-target {target_class}");
        let row = Division::builder()
            .class("motion-track group flex items-center gap-4 p-2 rounded-md border border-lineSoft hover:bg-canvas")
            .span(|s| s.class("mono text-[12px] text-ink-500 w-20").text(label))
            .division(|track| {
                track.class("relative flex-1 h-8")
                    .division(|target| target.class(target_cls.clone()))
            })
            .build();
        preview_rows.push(row);
    }

    // Rules
    let mut rules_ul = html::text_content::UnorderedList::builder();
    rules_ul.class("space-y-2 text-[13px] text-ink-700 leading-relaxed");
    for (num, text) in rules {
        let num = (*num).to_owned();
        let text = (*text).to_owned();
        let li = html::text_content::ListItem::builder()
            .class("flex gap-3")
            .span(|s| s.class("mono text-ink-500 w-12 shrink-0").text(num))
            .paragraph(|p| p.text(text))
            .build();
        rules_ul.push(li);
    }

    let content = Division::builder()
        .class("space-y-10")
        .division(|d| {
            d.heading_3(|h| {
                h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
                    .text("Easing curves")
            })
            .push(curves_grid.build())
        })
        .division(|d| {
            d.heading_3(|h| {
                h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
                    .text("Durations")
            })
            .push(dur_rows.build())
        })
        .division(|d| {
            d.heading_3(|h| {
                h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
                    .text("Preview ")
                    .span(|s| {
                        s.class("font-normal normal-case text-ink-400")
                            .text("(hover row)")
                    })
            })
            .push(preview_rows.build())
        })
        .division(|d| {
            d.heading_3(|h| {
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
            "motion",
            "23",
            "Motion",
            r#"Motion is functional: it explains state changes, never decorates them. Most transitions sit between 120–260ms on the <code class="px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]">standard</code> curve. Anything longer needs a reason."#,
            CURVES,
            DURATIONS,
            PREVIEWS,
            RULES,
        )));
    }
}
