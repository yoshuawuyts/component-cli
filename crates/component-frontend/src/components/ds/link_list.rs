//! Two-line link list used for "Featured" packages and "Categories" on
//! the landing page.
//!
//! Heavy top rule, single hairline between rows; each row has a primary
//! label on the left and a muted detail on the right.

use html::text_content::Division;

/// Visual treatment for the row's left-hand label.
pub(crate) enum LeftStyle {
    /// Mono, ink-900, bold-ish — used for package names.
    Mono,
    /// Plain ink-900 — used for category names.
    Plain,
}

/// Visual treatment for the row's right-hand detail.
pub(crate) enum RightStyle {
    /// Plain ink-500 — used for descriptions.
    Description,
    /// Mono tabular-nums ink-500 — used for counts.
    Count,
}

/// A single row in the link list.
pub(crate) struct LinkRow<'a> {
    pub left: &'a str,
    pub right: &'a str,
    pub href: &'a str,
}

/// Render the link list.
#[must_use]
pub(crate) fn render(
    kicker: &str,
    rows: &[LinkRow<'_>],
    left_style: &LeftStyle,
    right_style: &RightStyle,
) -> String {
    let kicker = kicker.to_owned();
    let left_class = match left_style {
        LeftStyle::Mono => {
            "mono text-[14px] font-medium text-ink-900 group-hover:underline decoration-1 underline-offset-4"
        }
        LeftStyle::Plain => "text-ink-900 group-hover:underline decoration-1 underline-offset-4",
    };
    let right_class = match right_style {
        RightStyle::Description => "text-[13px] text-ink-500 truncate hidden sm:inline",
        RightStyle::Count => "mono text-[13px] text-ink-500 tabular-nums",
    };
    let row_class = match right_style {
        RightStyle::Description => "group flex items-baseline justify-between gap-4 py-3",
        RightStyle::Count => "group flex items-baseline justify-between gap-4 py-3 text-[14px]",
    };

    let mut list = html::text_content::OrderedList::builder();
    list.class("mt-4 border-t-[1.5px] border-lineSoft");
    for row in rows {
        let left = row.left.to_owned();
        let right = row.right.to_owned();
        let href = row.href.to_owned();
        list.list_item(|li| {
            li.class("border-b border-lineSoft").anchor(|a| {
                a.href(href)
                    .class(format!("{row_class} no-underline"))
                    .span(|s| s.class(left_class).text(left))
                    .span(|s| s.class(right_class).text(right))
            })
        });
    }
    let list = list.build();

    Division::builder()
        .division(|d| {
            d.class("text-[12px] mono uppercase tracking-wider text-ink-500")
                .text(kicker)
        })
        .push(list)
        .build()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_count() {
        let html = render(
            "Categories",
            &[
                LinkRow {
                    left: "WASI",
                    right: "412",
                    href: "/c/wasi",
                },
                LinkRow {
                    left: "HTTP",
                    right: "188",
                    href: "/c/http",
                },
            ],
            &LeftStyle::Plain,
            &RightStyle::Count,
        );
        insta::assert_snapshot!(crate::components::ds::pretty_html(&html));
    }

    #[test]
    fn snapshot_description() {
        let html = render(
            "Featured",
            &[
                LinkRow {
                    left: "wasi:http",
                    right: "Outgoing & incoming HTTP for components.",
                    href: "/p/wasi/http",
                },
                LinkRow {
                    left: "wasi:cli",
                    right: "POSIX-shaped CLI for WebAssembly components.",
                    href: "/p/wasi/cli",
                },
            ],
            &LeftStyle::Mono,
            &RightStyle::Description,
        );
        insta::assert_snapshot!(crate::components::ds::pretty_html(&html));
    }
}
