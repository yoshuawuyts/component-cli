//! Metrics strip — 4-up divided stat row sitting between the hero and the
//! ecosystem region. Each cell shows a mono kicker, a large tabular value,
//! and an optional positive delta.

use html::content::Section;

/// A single metric cell.
pub(crate) struct Metric<'a> {
    /// Mono uppercase label rendered above the value.
    pub label: &'a str,
    /// Headline value (numeric, but rendered as a string so callers can
    /// add suffixes like `%`).
    pub value: &'a str,
    /// Optional small delta string shown next to the value
    /// (e.g. `"+2 this week"`).
    pub delta: Option<&'a str>,
    /// Optional verification dot shown next to the delta (used for the
    /// "index integrity" cell).
    pub verified: bool,
}

/// Render the metrics strip.
#[must_use]
pub(crate) fn render(metrics: &[Metric<'_>]) -> String {
    let mut section = Section::builder();
    section.class("mx-auto max-w-[1280px] w-full px-4 md:px-8 grid grid-cols-2 md:grid-cols-4 divide-x divide-y md:divide-y-0 divide-lineSoft border-y border-lineSoft");
    for metric in metrics {
        push_cell(&mut section, metric);
    }
    section.build().to_string()
}

fn push_cell(parent: &mut html::content::builders::SectionBuilder, metric: &Metric<'_>) {
    let label = metric.label.to_owned();
    let value = metric.value.to_owned();
    let delta = metric.delta.map(str::to_owned);
    let verified = metric.verified;

    parent.division(|cell| {
        let cell = cell.class("p-6").division(|d| {
            d.class("text-[12px] text-ink-500 mono uppercase tracking-wider")
                .text(label)
        });
        cell.division(|row| {
            let row = row.class("mt-2 flex items-baseline gap-2").span(|s| {
                s.class("text-[28px] font-semibold tracking-tight tabular-nums")
                    .text(value)
            });
            match (delta, verified) {
                (Some(d), true) => row.span(|s| {
                    s.class("inline-flex items-center gap-1 text-[12px] font-medium text-positive")
                        .span(|dot| dot.class("h-1.5 w-1.5 rounded-full bg-positive"))
                        .text(d)
                }),
                (Some(d), false) => {
                    row.span(|s| s.class("text-[12px] font-medium text-positive").text(d))
                }
                (None, _) => row,
            }
        })
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        let html = render(&[
            Metric {
                label: "Packages",
                value: "1\u{202f}284",
                delta: Some("+18 this week"),
                verified: false,
            },
            Metric {
                label: "Registries",
                value: "12",
                delta: None,
                verified: false,
            },
            Metric {
                label: "Downloads / mo",
                value: "94k",
                delta: Some("+6%"),
                verified: false,
            },
            Metric {
                label: "Index integrity",
                value: "100%",
                delta: Some("verified"),
                verified: true,
            },
        ]);
        insta::assert_snapshot!(crate::components::ds::pretty_html(&html));
    }
}
