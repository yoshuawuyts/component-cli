//! Design system reference page — `/design-system`.
//!
//! A living style guide that showcases every token, component, and pattern
//! from the design system. Sections are numbered to match `design-system.html`.

use crate::components::ds;
use crate::layout;

const RULE: &str = r#"<div class="border-t rule"></div>"#;
const RULE_MT: &str = r#"<div class="border-t rule mt-16"></div>"#;

/// Render the design system reference page.
#[must_use]
pub(crate) fn render() -> String {
    let mut html = String::with_capacity(128 * 1024);

    // Page header + TOC
    html.push_str(ds::header::render());
    html.push_str(RULE);
    html.push_str(ds::toc::render());
    html.push_str(RULE);

    // Foundations (01–24)
    html.push_str(ds::s01_color::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s02_typography::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s03_spacing::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s04_elevation::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s05_buttons::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s06_tabs::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s07_navigation::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s08_code::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s09_labels::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s10_tooltip::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s11_table::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s12_icons::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s13_fields::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s14_toggles::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s15_badges::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s16_dropdown::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s17_modal::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s18_breadcrumb::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s19_progress::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s20_empty::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s21_grid::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s22_regions::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s23_motion::render());
    html.push_str(RULE_MT);
    html.push_str(ds::s24_details::render());

    // Part Two — Components
    html.push_str(ds::part_two::render());
    html.push_str(ds::c01_sidebar::render());
    html.push_str(RULE_MT);
    html.push_str(ds::c02_toc::render());
    html.push_str(RULE_MT);
    html.push_str(ds::c03_page_header::render());
    html.push_str(RULE_MT);
    html.push_str(ds::c04_item_list::render());
    html.push_str(RULE_MT);
    html.push_str(ds::c05_item_details::render());
    html.push_str(RULE_MT);
    html.push_str(ds::c06_navbar::render());

    layout::document_design_system("Design System", &html)
}
