//! Design system reference page — `/design-system`.
//!
//! A living style guide that showcases every token, component, and pattern
//! from the design system. Sections are numbered to match `design-system.html`.

use crate::components::ds;
use crate::layout;

const RULE: &str = r#"<div class="border-t rule"></div>"#;
const RULE_MT: &str = r#"<div class="border-t rule mt-16"></div>"#;

/// TOC entries: (href, label).
const TOC_ENTRIES: &[(&str, &str)] = &[
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

/// TOC entries for composed components.
const TOC_COMPONENT_ENTRIES: &[(&str, &str)] = &[
    ("#c-sidebar", "C01 \u{2014} Nested Sidebar"),
    ("#c-toc", "C02 \u{2014} On This Page"),
    ("#c-page-header", "C03 \u{2014} Page Header"),
    ("#c-item-list", "C04 \u{2014} Item List"),
    ("#c-item-details", "C05 \u{2014} Item Details"),
    ("#c-navbar", "C06 \u{2014} Navbar"),
];

/// Render the design system reference page.
#[must_use]
pub(crate) fn render() -> String {
    let mut html = String::with_capacity(128 * 1024);

    // Page header + TOC
    html.push_str(&ds::header::render("v1.0",
            "Foundations \u{00b7} Components \u{00b7} Patterns",
            "Design System",
            "A quiet, data-forward visual language built around soft rules, neutral ink, and a categorical pastel palette. Optimized for dense dashboards and analytical interfaces.",));
    html.push_str(RULE);
    html.push_str(&ds::toc::render(TOC_ENTRIES, TOC_COMPONENT_ENTRIES));
    html.push_str(RULE);

    // Foundations (01–24)
    html.push_str(&ds::s01_color::render(
        "colors",
        "01",
        "Color",
        "Neutral surfaces and ink form the structural base. Pastel categoricals encode chart series with paired ink tones for legibility.",
        &[
        ds::s01_color::SwatchGroup {
        title: "Surfaces",
        grid_class: "grid grid-cols-2 md:grid-cols-3 gap-4",
        swatches: ds::s01_color::SURFACES
        },
        ds::s01_color::SwatchGroup {
        title: "Ink",
        grid_class: "grid grid-cols-2 md:grid-cols-5 gap-4",
        swatches: ds::s01_color::INK
        },
        ds::s01_color::SwatchGroup {
        title: "Lines",
        grid_class: "grid grid-cols-2 md:grid-cols-3 gap-4",
        swatches: ds::s01_color::LINES
        },
        ds::s01_color::SwatchGroup {
        title: "Semantic",
        grid_class: "grid grid-cols-2 md:grid-cols-3 gap-4",
        swatches: ds::s01_color::SEMANTIC
        },
        ds::s01_color::SwatchGroup {
        title: "Categorical",
        grid_class: "grid grid-cols-2 md:grid-cols-5 gap-4",
        swatches: ds::s01_color::CATEGORICAL
        },
        ],
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s02_typography::render(
        "typography",
        "02",
        "Typography",
        "System UI stack for native rendering across platforms. Tight tracking on display sizes; relaxed for body.",
        ds::s02_typography::SAMPLES,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s03_spacing::render(
        "spacing",
        "03",
        "Spacing & Radius",
        "4px base scale. Radii stay small for a precise, instrumental feel; pills used for selection chips only.",
        ds::s03_spacing::SPACING,
        ds::s03_spacing::RADII,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s04_elevation::render(
        "elevation",
        "04",
        "Elevation",
        "Soft rules do most of the work. Shadow is reserved for floating overlays.",
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s05_buttons::render(
        "buttons",
        "05",
        "Buttons",
        "Two variants: a soft gray fill or a 1.5px ink outline. The system reserves solid ink for typography only \u{2014} buttons are never pure black. Two heights: 32px (compact toolbars) and 36px (mobile / primary CTAs).",
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s06_tabs::render(
        "tabs",
        "06",
        "Tabs & Pills",
        "Segmented controls for binary scoping; underline tabs for sub-views; pills for filterable chips.",
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s07_navigation::render(
        "nav",
        "07",
        "Navigation",
        "Sidebar list. Active item uses a muted surface fill with full ink weight. Groups separated by a soft rule.",
        ds::s07_navigation::GROUP_1,
        ds::s07_navigation::GROUP_2,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s08_code::render(
        "code",
        "08",
        "Code Samples",
        "One panel \u{2014} <code class=\"mono text-[12px]\">pre.id-code</code> \u{2014} sitting on <code class=\"mono text-[12px]\">--c-surface</code>, with token colours pulled from the theme-aware <code class=\"mono text-[12px]\">--color-wit-*</code> palette so chroma stays balanced on both light and dark pages. Three forms: a plain block, a tabbed multi-language block, and a paired request / response grid.",
        ds::s08_code::TOKENS,
        ds::s08_code::ANATOMY_ITEMS,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s09_labels::render(
        "bars",
        "09",
        "Labels",
        "28px tall, 6px radius, label inset 12px. Pastel fill with paired ink for text \u{2014} 4.5:1 contrast minimum.",
        ds::s09_labels::BARS,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s10_tooltip::render(
        "tooltip",
        "10",
        "Tooltip",
        "Inverted surface with backdrop blur. Caption label above, key/value rows with right-aligned medium values.",
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s11_table::render(
        "table",
        "11",
        "Table",
        r##"Two patterns cover everything: a <strong>definition</strong> table (no <code class="mono text-[12px]">&lt;thead&gt;</code>, identifier on the left, meaning on the right) and a <strong>tabular</strong> table (labeled columns, <code class="mono text-[12px]">tabular-nums</code> for figures). 13px body, 1.5px soft row separators (<code class="mono text-[12px]">border-lineSoft</code>), <code class="mono text-[12px]">py-3</code> rows. When the leading column is a category, use the <a href="#c-item-details" class="text-ink-700 underline decoration-line decoration-1 underline-offset-[3px] hover:text-ink-900">.id-http-status</a> pill family."##,
        ds::s11_table::TAB_ENTRIES,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s12_icons::render(
        "icons",
        "12",
        "Icons",
        r#"<a href="https://lucide.dev" class="text-ink-700 underline decoration-line decoration-1 underline-offset-[3px] hover:text-ink-900">Lucide</a> outline icons, drawn at <code class="mono text-[12px]">stroke-width="1.75"</code> with <code class="mono text-[12px]">stroke-linecap="round"</code> and <code class="mono text-[12px]">stroke-linejoin="round"</code>. Sizes: <strong>14px</strong> inside dense controls (tree links, kbd hints, tabs), <strong>16px</strong> in toolbars and buttons, <strong>18px</strong> on mobile and in empty states. Always <code class="mono text-[12px]">currentColor</code> so they pick up the surrounding ink scale; never coloured directly."#,
        ds::s12_icons::INLINE_ICONS,
        ds::s12_icons::GRID_ICONS,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s13_fields::render(
        "fields",
        "13",
        "Form Fields",
        "Inputs sit on a surface with a 1px line border. Focus darkens the border to ink \u{2014} no thickening, no glow. Two sizes: <strong>md</strong> (default) for primary forms, <strong>sm</strong> for dense contexts like sidebars, metadata strips, and toolbars.",
        ds::s13_fields::STATES,
        ds::s13_fields::COMMANDS,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s14_toggles::render(
        "toggles",
        "14",
        "Checkbox \u{00b7} Radio \u{00b7} Switch",
        "All controls render in ink-900 when active. 16px hit area minimum on each control; full-row click target via wrapping label.",
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s15_badges::render(
        "badges",
        "15",
        "Badges",
        "Compact pill labels. Use categorical pairs for status; ink for counts and metadata.",
        ds::s15_badges::STATUSES,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s16_dropdown::render(
        "dropdown",
        "16",
        "Dropdown",
        "Floating menu on white. 1px gray border + tooltip-grade shadow. Section dividers separate logical groups.",
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s17_modal::render(
        "modal",
        "17",
        "Modal",
        "Centered dialog over a 50% ink scrim. 8px radius, 1px gray border, 24px padding. Header / body / footer rhythm.",
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s18_breadcrumb::render(
        "breadcrumb",
        "19",
        "Breadcrumb &<br />Pagination",
        "Navigation context. Breadcrumb uses chevron separators and dims all but the current item. Pagination is square-buttoned for compact toolbars.",
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s19_progress::render(
        "progress",
        "19",
        "Progress & Spinner",
        "Determinate progress as a 6px ink track. Indeterminate as a 16px spinner (CSS animation). Skeleton shimmer for placeholder content.",
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s20_empty::render(
        "empty",
        "20",
        "Empty State",
        "Centered illustration glyph, title, body, and primary CTA. Used for empty tables, search misses, and first-run views.",
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s21_grid::render(
        "grid",
        "21",
        "Grid",
        r#"Pages live in a <code class="mono text-[12px]">max-w-[1440px]</code> container with <code class="mono text-[12px]">px-4 md:px-6</code> gutters. Inside, a small set of column shapes covers every layout: <strong>three-column</strong> (sidebar · reading · on-this-page) for documentation; <strong>two-column</strong> for narrative pages and this style guide; <strong>single column</strong> bounded by a reading measure for prose. Reading text is always capped at <code class="mono text-[12px]">max-w-[72ch]</code> regardless of the column it sits in."#,
        ds::s21_grid::RULES,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s22_regions::render(
        "regions",
        "22",
        "Regions",
        "Pages are composed of stacked <em>regions</em>. The primary region uses the canvas surface; secondary regions (supporting data, references, appendices) switch to the white surface. The surface swap signals \u{201c}this is additional content\u{201d} \u{2014} no rules or borders are drawn between regions.",
        ds::s22_regions::RULES,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s23_motion::render(
        "motion",
        "23",
        "Motion",
        r#"Motion is functional: it explains state changes, never decorates them. Most transitions sit between 120–260ms on the <code class="px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]">standard</code> curve. Anything longer needs a reason."#,
        ds::s23_motion::CURVES,
        ds::s23_motion::DURATIONS,
        ds::s23_motion::PREVIEWS,
        ds::s23_motion::RULES,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::s24_details::render(
        "details",
        "24",
        "Details",
        "Compact key/value lists for sidebars and inspector panels. Three variants: stacked for spacious layouts, inline for narrow rails, and sectioned when groups need separation.",
        ds::s24_details::STACKED,
        ds::s24_details::INLINE,
        ds::s24_details::SECTIONED_A,
        ds::s24_details::SECTIONED_B,
        ds::s24_details::CARD_DETAILS,
        ds::s24_details::SIDEBAR_PRIMARY,
        ds::s24_details::SIDEBAR_SECONDARY,
    ));

    // Part Two — Components
    html.push_str(&ds::part_two::render("Part Two",
            "Components",
            "Composed patterns built from the foundations above. Each component documents its anchor markup and the variants it supports.",));
    html.push_str(&ds::c01_sidebar::render(
        "c-sidebar",
        "C01",
        "Nested Sidebar",
        r#"Hierarchical navigation for reference docs. Top-level entries collapse with native <code class="mono text-[12px]">&lt;details&gt;</code>; sigils classify each row by kind (command, group, flag, env, etc.)."#,
        ds::c01_sidebar::SIGIL_LEGEND,
        ds::c01_sidebar::ANATOMY_ITEMS,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::c02_toc::render(
        "c-toc",
        "C02",
        "On This Page",
        "Right-rail table of contents for long reference pages. A 1.5px left border lights up on hover and active state \u{2014} the only visual cue, no background fills.",
        ds::c02_toc::TOC_LINKS,
        ds::c02_toc::ANATOMY_ITEMS,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::c03_page_header::render(
        "c-page-header",
        "C03",
        "Page Header",
        "Top-of-page identification block: a kicker, a large title, an optional tagline, and an optional metadata strip. Used to anchor reference and documentation pages.",
        ds::c03_page_header::ANATOMY_ITEMS,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::c04_item_list::render(
        "c-item-list",
        "C04",
        "Item List",
        "Compact index of a group\u{2019}s children \u{2014} subcommands, endpoints, schemas. Each row is a sigil, a name + one-line description, and trailing meta. Rows separate with hairline rules, no card chrome.",
        ds::c04_item_list::CMD_ROWS,
        ds::c04_item_list::ENDPOINT_ROWS,
        ds::c04_item_list::ANATOMY_ITEMS,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::c05_item_details::render(
        "c-item-details",
        "C05",
        "Item Details",
        r##"Reference page for a single endpoint, RPC, schema, or command. A method/kind pill anchors the symbol below the title; a one-sentence tagline explains it; an optional structured request-body table, a responses list, and paired example panels stack below in fixed order. Used as the destination from <a href="#c-item-list" class="text-ink-700 underline decoration-line decoration-1 underline-offset-[3px] hover:text-ink-900">Item List</a> rows."##,
        ds::c05_item_details::ANATOMY_ITEMS,
    ));
    html.push_str(RULE_MT);
    html.push_str(&ds::c06_navbar::render(
        "c-navbar",
        "C06",
        "Navbar",
        "Sticky page chrome: brand mark, command palette trigger, primary nav, theme toggle. Sits above all content with a translucent <code class=\"mono text-[12px]\">bg-canvas/90</code> + <code class=\"mono text-[12px]\">backdrop-blur</code> so scrolling content reads through without losing legibility.",
        ds::c06_navbar::DRAWER_LINKS,
        ds::c06_navbar::ANATOMY_ITEMS,
    ));

    layout::document_design_system("Design System", &html)
}
