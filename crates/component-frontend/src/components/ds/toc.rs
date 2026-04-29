//! Table of contents.

use html::content::Navigation;

/// Render the table of contents.
pub(crate) fn render(entries: &[(&str, &str)], component_entries: &[(&str, &str)]) -> String {
    let mut nav = Navigation::builder();
    nav.class("py-6 grid grid-flow-col grid-rows-12 md:grid-rows-6 gap-y-2 gap-x-6 text-[13px]");
    for (href, label) in entries {
        let href = (*href).to_owned();
        let label = (*label).to_owned();
        nav.anchor(|a| {
            a.href(href)
                .class("text-ink-700 hover:text-ink-900")
                .text(label)
        });
    }
    nav.span(|s| {
        s.class("text-ink-400 mono uppercase tracking-wider text-[11px] mt-2")
            .text("Components")
    });
    for (href, label) in component_entries {
        let href = (*href).to_owned();
        let label = (*label).to_owned();
        nav.anchor(|a| {
            a.href(href)
                .class("text-ink-700 hover:text-ink-900")
                .text(label)
        });
    }
    nav.build().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTRIES: &[(&str, &str)] = &[
        ("#colors", "01 \u{2014} Color"),
        ("#typography", "02 \u{2014} Typography"),
        ("#spacing", "03 \u{2014} Spacing & Radius"),
        ("#elevation", "04 \u{2014} Elevation"),
        ("#buttons", "05 \u{2014} Buttons"),
        ("#tabs", "06 \u{2014} Tabs & Pills"),
        ("#nav", "07 \u{2014} Navigation"),
        ("#code", "08 \u{2014} Code Samples"),
        ("#bars", "09 \u{2014} Labels"),
        ("#tooltip", "10 \u{2014} Tooltip"),
        ("#table", "11 \u{2014} Table"),
        ("#icons", "12 \u{2014} Icons"),
        ("#fields", "13 \u{2014} Form Fields"),
        (
            "#toggles",
            "14 \u{2014} Checkbox \u{00b7} Radio \u{00b7} Switch",
        ),
        ("#badges", "15 \u{2014} Badges"),
        ("#dropdown", "16 \u{2014} Dropdown"),
        ("#modal", "17 \u{2014} Modal"),
        ("#breadcrumb", "18 \u{2014} Breadcrumb & Pagination"),
        ("#progress", "19 \u{2014} Progress & Spinner"),
        ("#empty", "20 \u{2014} Empty State"),
        ("#grid", "21 \u{2014} Grid"),
        ("#regions", "22 \u{2014} Regions"),
        ("#motion", "23 \u{2014} Motion"),
        ("#details", "24 \u{2014} Details"),
    ];

    const COMPONENT_ENTRIES: &[(&str, &str)] = &[
        ("#c-sidebar", "C01 \u{2014} Nested Sidebar"),
        ("#c-toc", "C02 \u{2014} On This Page"),
        ("#c-page-header", "C03 \u{2014} Page Header"),
        ("#c-item-list", "C04 \u{2014} Item List"),
        ("#c-item-details", "C05 \u{2014} Item Details"),
        ("#c-navbar", "C06 \u{2014} Navbar"),
    ];

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(crate::components::ds::pretty_html(&render(
            ENTRIES,
            COMPONENT_ENTRIES,
        )));
    }
}
