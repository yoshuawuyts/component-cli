//! Design system reference page — `/design-system`.
//!
//! A living style guide that showcases every token, component, and pattern
//! from the design system. Sections are numbered to match `design-system.html`.

use html::text_content::Division;

use crate::components::{
    badge, button, code_block, copy_button, detail_row, empty_state, icon, link_button, metric,
    nav_list, package_card, package_row, search_bar, section_group, section_heading,
};
use crate::layout;

/// Render the design system reference page.
#[must_use]
pub(crate) fn render() -> String {
    let body = Division::builder()
        .class("pt-8 sm:pt-12 pb-16 max-w-5xl")
        .push(render_header())
        .push(div())
        .push(render_toc())
        .push(div())
        .push(render_colors()) // 01
        .push(div())
        .push(render_typography()) // 02
        .push(div())
        .push(render_spacing()) // 03
        .push(div())
        .push(render_elevation()) // 04
        .push(div())
        .push(render_buttons()) // 05
        .push(div())
        .push(render_tabs()) // 06
        .push(div())
        .push(render_navigation()) // 07
        .push(div())
        .push(render_metrics()) // 08
        .push(div())
        .push(render_labels()) // 09
        .push(div())
        .push(render_tooltip()) // 10
        .push(div())
        .push(render_table()) // 11
        .push(div())
        .push(render_icons()) // 12
        .push(div())
        .push(render_search()) // 13
        .push(div())
        .push(render_toggles()) // 14
        .push(div())
        .push(render_badges()) // 15
        .push(div())
        .push(render_dropdown()) // 16
        .push(div())
        .push(render_modal()) // 17
        .push(div())
        .push(render_breadcrumb()) // 18
        .push(div())
        .push(render_progress()) // 19
        .push(div())
        .push(render_empty_state()) // 20
        .push(div())
        .push(render_regions()) // 21
        .push(div())
        .push(render_motion()) // 22
        .push(div())
        .push(render_details()) // 23
        .push(div())
        .push(render_package_cards()) // P1
        .push(div())
        .push(render_package_rows()) // P2
        .push(div())
        .push(render_section_groups()) // P3
        .push(div())
        .push(render_code_blocks()) // P4
        .push(div())
        .push(render_copy_buttons()) // P5
        .push(div())
        .push(render_nav_lists()) // P6
        .push(div())
        .push(render_section_headings()) // P8
        .push(div())
        .push(render_sidebar_sections()) // P9
        .build();

    layout::document_with_nav("Design System", &body.to_string())
}

// ── Helpers ──────────────────────────────────────────────

fn div() -> Division {
    Division::builder()
        .class("border-t-[1.5px] border-rule mt-12 sm:mt-16")
        .build()
}

fn sec(id: &str, num: &str, title: &str, desc: &str) -> Division {
    Division::builder()
        .class("pt-8 sm:pt-12 mb-6")
        .id(id.to_owned())
        .division(|n| {
            n.class("text-[12px] font-mono uppercase tracking-wider text-ink-500")
                .text(num.to_owned())
        })
        .heading_2(|h| {
            h.class("mt-2 text-[24px] font-semibold tracking-tight")
                .text(title.to_owned())
        })
        .paragraph(|p| {
            p.class("mt-2 text-[13px] text-ink-500 leading-relaxed")
                .text(desc.to_owned())
        })
        .build()
}

fn sub(text: &str) -> Division {
    Division::builder()
        .heading_3(|h| {
            h.class("text-[13px] font-mono uppercase tracking-wider text-ink-500 mb-3")
                .text(text.to_owned())
        })
        .build()
}

fn swatch(label: &str, bg: &str, hex: &str, oklch: &str) -> Division {
    Division::builder()
        .division(|s| s.class(format!("{bg} h-[88px] rounded-lg border border-lineSoft")))
        .division(|n| n.class("mt-2 text-[13px]").text(label.to_owned()))
        .division(|h| {
            h.class("text-[12px] text-ink-500 font-mono")
                .text(hex.to_owned())
        })
        .division(|o| {
            o.class("text-[11px] text-ink-400 font-mono")
                .text(oklch.to_owned())
        })
        .build()
}

fn tsample(label: &str, cls: &str, text: &str, spec: &str) -> Division {
    Division::builder()
        .class("py-5 grid grid-cols-[120px_1fr] gap-6 items-baseline")
        .division(|l| {
            l.class("text-[12px] text-ink-500 font-mono")
                .text(label.to_owned())
        })
        .division(|c| {
            c.division(|d| d.class(cls.to_owned()).text(text.to_owned()))
                .division(|d| {
                    d.class("text-[12px] text-ink-500 mt-1 font-mono")
                        .text(spec.to_owned())
                })
        })
        .build()
}

// ── Header ───────────────────────────────────────────────

fn render_header() -> Division {
    Division::builder()
        .division(|d| {
            d.class("flex items-center gap-2 text-[12px] text-ink-500 font-mono uppercase tracking-wider")
                .span(|s| s.text("v1.0"))
                .span(|s| s.class("h-1 w-1 rounded-full bg-ink-300"))
                .span(|s| s.text("Foundations \u{00b7} Components \u{00b7} Patterns"))
        })
        .heading_1(|h1| h1.class("mt-3 text-[36px] md:text-[44px] leading-[1.05] font-semibold tracking-tight").text("Design System"))
        .paragraph(|p| p.class("mt-3 max-w-2xl text-[15px] text-ink-700 leading-relaxed")
            .text("A quiet, data-forward visual language built around soft rules, neutral ink, and a categorical pastel palette. Optimized for dense dashboards and analytical interfaces."))
        .build()
}

// ── Table of Contents ────────────────────────────────────

fn render_toc() -> Division {
    let links: &[(&str, &str)] = &[
        ("#colors", "01 \u{2014} Color"),
        ("#typography", "02 \u{2014} Typography"),
        ("#spacing", "03 \u{2014} Spacing & Radius"),
        ("#elevation", "04 \u{2014} Elevation"),
        ("#buttons", "05 \u{2014} Buttons"),
        ("#tabs", "06 \u{2014} Tabs & Pills"),
        ("#nav", "07 \u{2014} Navigation"),
        ("#metrics", "08 \u{2014} Metrics"),
        ("#labels", "09 \u{2014} Labels"),
        ("#tooltip", "10 \u{2014} Tooltip"),
        ("#table", "11 \u{2014} Table"),
        ("#icons", "12 \u{2014} Icons"),
        ("#search", "13 \u{2014} Form Fields"),
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
        ("#regions", "21 \u{2014} Regions"),
        ("#motion", "22 \u{2014} Motion"),
        ("#details", "23 \u{2014} Details"),
        ("#cards", "P1 \u{2014} Package Cards"),
        ("#rows", "P2 \u{2014} Package Rows"),
        ("#groups", "P3 \u{2014} Section Groups"),
        ("#codeblocks", "P4 \u{2014} Code Blocks"),
        ("#copybuttons", "P5 \u{2014} Copy Buttons"),
        ("#navlists", "P6 \u{2014} Nav Lists"),
        ("#secheadings", "P7 \u{2014} Section Headings"),
        ("#sidebarsec", "P8 \u{2014} Sidebar Details"),
    ];

    let mut nav = Division::builder();
    nav.class("py-6 grid grid-flow-col grid-rows-16 md:grid-rows-9 gap-y-2 gap-x-6 text-[13px]");
    for (href, label) in links {
        nav.anchor(|a| {
            a.href(href.to_string())
                .class("text-ink-700 hover:text-ink-900")
                .text(label.to_string())
        });
    }
    nav.build()
}

// ── 01 Color ─────────────────────────────────────────────

fn render_colors() -> Division {
    Division::builder()
        .push(sec("colors", "01", "Color", "Neutral surfaces and ink form the structural base. Pastel categoricals encode chart series with paired ink tones for legibility."))
        .push(sub("Surfaces"))
        .division(|g| g.class("grid grid-cols-2 sm:grid-cols-3 gap-4")
            .push(swatch("Canvas", "bg-canvas", "#F4F4F5", "oklch(.967 .001 286)"))
            .push(swatch("Surface", "bg-surface", "#FFFFFF", "oklch(1 0 0)"))
            .push(swatch("Surface Muted", "bg-surfaceMuted", "#E8E8EA", "oklch(.928 .003 286)")))
        .division(|d| d.class("mt-8").push(sub("Ink"))
            .division(|g| g.class("grid grid-cols-2 sm:grid-cols-5 gap-4")
                .push(swatch("Ink 900", "bg-ink-900", "#18181B", "oklch(.210 .006 286)"))
                .push(swatch("Ink 700", "bg-ink-700", "#3F3F46", "oklch(.370 .013 286)"))
                .push(swatch("Ink 500", "bg-ink-500", "#71717A", "oklch(.552 .016 286)"))
                .push(swatch("Ink 400", "bg-ink-400", "#A1A1AA", "oklch(.705 .015 286)"))
                .push(swatch("Ink 300", "bg-ink-300", "#D4D4D8", "oklch(.871 .006 286)"))))
        .division(|d| d.class("mt-8").push(sub("Lines"))
            .division(|g| g.class("grid grid-cols-2 sm:grid-cols-3 gap-4")
                .push(swatch("Line", "bg-line", "#D4D4D8", "oklch(.871 .006 286)"))
                .push(swatch("Line Soft", "bg-lineSoft", "#E4E4E7", "oklch(.920 .004 286)"))))
        .division(|d| d.class("mt-8").push(sub("Semantic"))
            .division(|g| g.class("grid grid-cols-2 sm:grid-cols-3 gap-4")
                .push(swatch("Positive", "bg-positive", "#1F8A4C", "oklch(.561 .149 149)"))
                .push(swatch("Negative", "bg-negative", "#9B4F5E", "oklch(.490 .080 13)"))))
        .division(|d| d.class("mt-8").push(sub("Categorical"))
            .division(|g| g.class("grid grid-cols-2 sm:grid-cols-5 gap-4")
                .push(swatch("Blue", "bg-cat-blue", "#D6E4FF", "oklch(.910 .046 264)"))
                .push(swatch("Pink", "bg-cat-pink", "#FBD9DF", "oklch(.910 .037 9)"))
                .push(swatch("Green", "bg-cat-green", "#D2ECD8", "oklch(.918 .039 148)"))
                .push(swatch("Peach", "bg-cat-peach", "#F8E2C2", "oklch(.911 .049 79)"))
                .push(swatch("Lilac", "bg-cat-lilac", "#E4DAF1", "oklch(.852 .051 287)"))
                .push(swatch("Cream", "bg-cat-cream", "#F4ECC2", "oklch(.937 .055 100)"))
                .push(swatch("Teal", "bg-cat-teal", "#BFE3EE", "oklch(.890 .037 215)"))
                .push(swatch("Rust", "bg-cat-rust", "#F4D2C0", "oklch(.880 .045 50)"))
                .push(swatch("Plum", "bg-cat-plum", "#E8C5E8", "oklch(.851 .065 322)"))
                .push(swatch("Slate", "bg-cat-slate", "#DADCE0", "oklch(.882 .005 264)"))))
        .build()
}

// ── 02 Typography ────────────────────────────────────────

fn render_typography() -> Division {
    Division::builder()
        .push(sec("typography", "02", "Typography", "System UI stack for native rendering across platforms. Tight tracking on display sizes; relaxed for body."))
        .division(|s| s.class("divide-y divide-lineSoft")
            .push(tsample("Display", "text-[44px] leading-[1.05] font-semibold tracking-tight", "Aa Display", "44 / 1.05 / -0.01em / 600"))
            .push(tsample("H1", "text-[28px] leading-[1.15] font-semibold tracking-tight", "Lorem ipsum dolor", "28 / 1.15 / 600"))
            .push(tsample("H2", "text-[22px] font-semibold tracking-tight", "Sit amet consectetur", "22 / 600"))
            .push(tsample("Lead", "text-[20px] font-semibold tracking-tight leading-tight", "42.7 k", "20 / tight / 600 \u{2014} metric value"))
            .push(tsample("Body", "text-[15px] leading-relaxed text-ink-700", "The quick brown fox jumps over the lazy dog.", "15 / 1.6 / 400"))
            .push(tsample("UI", "text-[14px]", "Navigation item \u{00b7} Table cell", "14 / 400 \u{2014} 13 / 500 (medium)"))
            .push(tsample("Caption", "text-[12px] text-ink-500", "Aenean lectus \u{00b7} Vivamus aliquet", "12 / 400 / ink-500"))
            .push(tsample("Micro", "text-[11px] text-ink-500", "Tempor incididunt \u{00b7} ut labore", "11 / 400")))
        // Inline formatting — uses raw HTML since it contains styled elements
        .division(|row| {
            row.class("py-5 grid grid-cols-[120px_1fr] gap-6 items-baseline border-t border-lineSoft")
                .division(|l| l.class("text-[12px] text-ink-500 font-mono").text("Inline"))
                .division(|c| {
                    c.division(|d| d.class("text-[15px] leading-relaxed text-ink-700")
                        .text("Read the <a href=\"#\" class=\"text-ink-900 underline decoration-line decoration-1 underline-offset-[3px]\">guide</a>, then run <code class=\"px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 font-mono text-[0.875em]\">wasm install</code>. Press <kbd class=\"inline-flex items-center px-1.5 h-5 rounded-sm border border-line bg-surface text-ink-700 font-mono text-[11px]\">\u{2318}K</kbd> to search.".to_owned()))
                    .division(|d| d.class("text-[12px] text-ink-500 mt-1 font-mono")
                        .text(format!("link \u{00b7} code \u{00b7} kbd")))
                })
        })
        // Markdown rendering — full block-level content
        .division(|row| {
            row.class("py-5 grid grid-cols-[120px_1fr] gap-6 items-start border-t border-lineSoft")
                .division(|l| l.class("text-[12px] text-ink-500 font-mono pt-1").text("Markdown"))
                .division(|c| {
                    c.division(|article| article.class("text-[15px] leading-relaxed text-ink-700 space-y-4")
                        .text("<h3 class=\"text-[20px] font-semibold tracking-tight text-ink-900 leading-tight\">Configuring the registry</h3>".to_owned())
                        .text("<p>The <code class=\"px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 font-mono text-[0.875em]\">wasm.toml</code> manifest lives at the root of every package.</p>".to_owned())
                        .text("<h4 class=\"text-[16px] font-semibold tracking-tight text-ink-900 leading-snug pt-2\">Manifest fields</h4>".to_owned())
                        .text("<ul class=\"list-disc pl-5 space-y-1 marker:text-ink-400\"><li><code class=\"px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 font-mono text-[0.875em]\">name</code> \u{2014} reverse-DNS package identifier</li><li><code class=\"px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 font-mono text-[0.875em]\">version</code> \u{2014} semantic version</li></ul>".to_owned())
                        .text("<blockquote class=\"border-l-2 border-ink-900 pl-4 text-ink-700 italic\">Registries are append-only. A version, once published, cannot be overwritten.</blockquote>".to_owned())
                        .text("<pre class=\"overflow-x-auto rounded-md border border-line bg-surfaceMuted text-ink-900 font-mono text-[13px] leading-relaxed p-4\"><code>[package]\nname    = \"example.com/hello-world\"\nversion = \"0.1.0\"</code></pre>".to_owned()))
                    .division(|d| d.class("text-[12px] text-ink-500 mt-3 font-mono")
                        .text(format!("h3 / h4 \u{00b7} p \u{00b7} ul \u{00b7} blockquote \u{00b7} pre")))
                })
        })
        .build()
}

// ── 03 Spacing & Radius ─────────────────────────────────

fn render_spacing() -> Division {
    let bar = |width: u32, value: &str, label: &str| -> Division {
        Division::builder()
            .class("flex items-center gap-4")
            .division(|b| b.class("h-3 bg-ink-900").style(format!("width:{width}px")))
            .span(|s| s.class("text-[13px] font-mono w-12").text(value.to_owned()))
            .span(|s| s.class("text-[12px] text-ink-500").text(label.to_owned()))
            .build()
    };
    Division::builder()
        .push(sec("spacing", "03", "Spacing & Radius", "4px base scale. Radii stay small for a precise, instrumental feel; pills used for selection chips only."))
        .push(sub("Spacing scale"))
        .division(|d| d.class("space-y-2")
            .push(bar(4, "4", "xs"))
            .push(bar(8, "8", "sm"))
            .push(bar(12, "12", "md"))
            .push(bar(16, "16", "lg"))
            .push(bar(24, "24", "xl"))
            .push(bar(32, "32", "2xl"))
            .push(bar(48, "48", "3xl")))
        .division(|d| d.class("mt-10")
            .push(sub("Radius"))
            .division(|g| g.class("grid grid-cols-2 sm:grid-cols-4 gap-4")
                .division(|d| d.division(|s| s.class("h-16 bg-surfaceMuted").style("border-radius:2px")).division(|l| l.class("mt-2 text-[13px]").text("sm \u{2014} 2px")))
                .division(|d| d.division(|s| s.class("h-16 bg-surfaceMuted").style("border-radius:4px")).division(|l| l.class("mt-2 text-[13px]").text("md \u{2014} 4px (inputs, bars)")))
                .division(|d| d.division(|s| s.class("h-16 bg-surfaceMuted").style("border-radius:5px")).division(|l| l.class("mt-2 text-[13px]").text("lg \u{2014} 5px (buttons, cards)")))
                .division(|d| d.division(|s| s.class("h-16 bg-surfaceMuted rounded-pill")).division(|l| l.class("mt-2 text-[13px]").text("pill \u{2014} 9999px")))))
        .build()
}

// ── 04 Elevation ─────────────────────────────────────────

fn render_elevation() -> Division {
    Division::builder()
        .push(sec(
            "elevation",
            "04",
            "Elevation",
            "Soft rules do most of the work. Shadow is reserved for floating overlays.",
        ))
        .division(|g| {
            g.class("grid grid-cols-1 sm:grid-cols-3 gap-6")
                .division(|d| {
                    d.class("p-5 bg-surface border border-lineSoft rounded-lg")
                        .division(|t| t.class("text-[13px] font-medium").text("Rule"))
                        .division(|t| {
                            t.class("mt-1 text-[12px] text-ink-500 font-mono")
                                .text("border 1px #E4E4E7")
                        })
                })
                .division(|d| {
                    d.class("p-5 bg-surface rounded-lg shadow-card")
                        .division(|t| t.class("text-[13px] font-medium").text("Card"))
                        .division(|t| {
                            t.class("mt-1 text-[12px] text-ink-500 font-mono")
                                .text("0 1px 0 rgba(20,22,28,.04)")
                        })
                })
                .division(|d| {
                    d.class("p-5 backdrop-blur text-canvas rounded-md shadow-tooltip")
                        .style("background: var(--c-ink-900)")
                        .division(|t| t.class("text-[13px] font-medium").text("Tooltip"))
                        .division(|t| {
                            t.class("mt-1 text-[12px] text-ink-300 font-mono")
                                .text("0 8px 24px -8px rgba(20,22,28,.35)")
                        })
                })
        })
        .build()
}

// ── 05 Buttons ───────────────────────────────────────────

fn render_buttons() -> Division {
    Division::builder()
        .push(sec("buttons", "05", "Buttons", "Two variants: a soft gray fill or a 1.5px ink outline. Two heights: 32px (compact) and 36px (primary CTAs)."))
        .push(sub("Filled"))
        .division(|r| r.class("flex flex-wrap items-center gap-3")
            .push(button::render(button::Variant::Filled, button::Size::Compact, "Compact"))
            .push(button::render(button::Variant::Filled, button::Size::Large, "Larger")))
        .division(|d| d.class("mt-8").push(sub("Outline"))
            .division(|r| r.class("flex flex-wrap items-center gap-3")
                .push(button::render(button::Variant::Outline, button::Size::Compact, "Outline"))
                .push(button::render(button::Variant::Outline, button::Size::Large, "Dismiss"))))
        .division(|d| d.class("mt-8").push(sub("Link Buttons"))
            .paragraph(|p| p.class("text-ink-500 text-[13px] mb-3").text("Anchor elements styled as buttons. Primary (high-contrast fill) and outline variants."))
            .division(|r| r.class("flex flex-wrap items-center gap-3")
                .push(link_button::render(link_button::Variant::Primary, "#", "Primary"))
                .push(link_button::render(link_button::Variant::Outline, "#", "Outline"))))
        .division(|d| d.class("mt-8").push(sub("Icon"))
            .division(|r| r.class("flex items-center gap-1")
                .division(|b| b.class("h-8 w-8 grid place-items-center rounded-md hover:bg-surfaceMuted text-ink-700")
                    .text(icon::svg(icon::Icon::Copy, icon::IconSize::Md).to_owned()))
                .division(|b| b.class("h-8 w-8 grid place-items-center rounded-md hover:bg-surfaceMuted text-ink-700")
                    .text(icon::svg(icon::Icon::Check, icon::IconSize::Md).to_owned()))
                .division(|b| b.class("h-8 w-8 grid place-items-center rounded-md hover:bg-surfaceMuted text-ink-700")
                    .text(icon::svg(icon::Icon::Search, icon::IconSize::Md).to_owned()))))
        .build()
}

// ── 06 Tabs & Pills ──────────────────────────────────────

fn render_tabs() -> Division {
    Division::builder()
        .push(sec("tabs", "06", "Tabs & Pills", "Segmented controls for binary scoping; underline tabs for sub-views; pills for filterable chips."))
        .push(sub("Segmented"))
        .division(|seg| seg.class("flex p-1 rounded-lg bg-surfaceMuted w-[200px]")
            .button(|b| b.type_("button").class("flex-1 h-7 rounded-md bg-ink-900 text-canvas text-[13px] font-medium").text("Lorem"))
            .button(|b| b.type_("button").class("flex-1 h-7 rounded-md text-[13px] text-ink-500").text("Ipsum")))
        .division(|d| d.class("mt-8").push(sub("Underline tabs"))
            .division(|tabs| tabs.class("flex items-center gap-6 border-b-[1.5px] border-rule")
                .division(|t| t.class("relative pb-3 text-[15px] font-medium")
                    .text("Aenean".to_owned())
                    .span(|s| s.class("absolute left-0 right-0 -bottom-[1.5px] h-[1.5px] bg-ink-900")))
                .division(|t| t.class("pb-3 text-[15px] text-ink-500").text("Mauris"))
                .division(|t| t.class("pb-3 text-[15px] text-ink-500").text("Vivamus"))))
        .division(|d| d.class("mt-8").push(sub("Pills"))
            .division(|r| r.class("flex flex-wrap items-center gap-2 text-[13px]")
                .span(|s| s.class("inline-flex items-center px-3 h-8 rounded-pill bg-ink-900 text-canvas font-medium").text("Tellus"))
                .span(|s| s.class("inline-flex items-center px-3 h-8 rounded-pill bg-surfaceMuted text-ink-700").text("Pellentesque"))
                .span(|s| s.class("inline-flex items-center px-3 h-8 rounded-pill bg-surfaceMuted text-ink-700").text("Vestibulum"))
                .span(|s| s.class("inline-flex items-center px-3 h-8 rounded-pill bg-surfaceMuted text-ink-700").text("Convallis"))))
        .division(|d| d.class("mt-8").push(sub("Range selector"))
            .division(|r| r.class("inline-flex items-center gap-1 text-[13px] text-ink-500")
                .division(|b| b.class("h-7 w-7 grid place-items-center rounded").text("D"))
                .division(|b| b.class("h-7 w-7 grid place-items-center rounded").text("W"))
                .division(|b| b.class("h-7 w-7 grid place-items-center rounded text-ink-900 font-medium").text("M"))
                .division(|b| b.class("h-7 pl-1 pr-3 inline-flex items-center gap-1.5 rounded-pill bg-surfaceMuted text-ink-900")
                    .span(|s| s.class("h-5 w-5 rounded-full bg-ink-300"))
                    .text("All".to_owned()))))
        .build()
}

// ── 07 Navigation ────────────────────────────────────────

fn render_navigation() -> Division {
    Division::builder()
        .push(sec("nav", "07", "Navigation", "Sidebar list. Active item uses a muted surface fill with full ink weight. Groups separated by a soft rule."))
        .division(|nav| {
            nav.class("max-w-[260px]")
                .division(|ul| ul.class("space-y-px text-[14px]")
                    .division(|li| li.anchor(|a| a.href("#").class("flex items-center px-3 h-9 rounded-md bg-surfaceMuted text-ink-900 font-medium").text("Tellus")))
                    .division(|li| li.anchor(|a| a.href("#").class("flex items-center px-3 h-9 rounded-md hover:bg-surfaceMuted text-ink-700").text("Pellentesque Habitant")))
                    .division(|li| li.anchor(|a| a.href("#").class("flex items-center px-3 h-9 rounded-md hover:bg-surfaceMuted text-ink-700").text("Vestibulum Ante")))
                    .division(|li| li.anchor(|a| a.href("#").class("flex items-center px-3 h-9 rounded-md hover:bg-surfaceMuted text-ink-700").text("Convallis Dolor"))))
                .division(|rule| rule.class("my-4 border-t-[1.5px] border-rule"))
                .division(|ul| ul.class("space-y-px text-[14px]")
                    .division(|li| li.anchor(|a| a.href("#").class("flex items-center px-3 h-9 rounded-md hover:bg-surfaceMuted text-ink-700").text("Faucibus")))
                    .division(|li| li.anchor(|a| a.href("#").class("flex items-center px-3 h-9 rounded-md hover:bg-surfaceMuted text-ink-700").text("Suspendisse")))
                    .division(|li| li.anchor(|a| a.href("#").class("flex items-center px-3 h-9 rounded-md hover:bg-surfaceMuted text-ink-700").text("Aliquam Erat"))))
        })
        .build()
}

// ── 08 Metrics ───────────────────────────────────────────

fn render_metrics() -> Division {
    Division::builder()
        .push(sec(
            "metrics",
            "08",
            "Metrics",
            "Caption label, large value, optional delta. Stacked vertically on desktop; cards on mobile.",
        ))
        .division(|d| {
            d.class("max-w-[180px]")
                .division(|m| {
                    m.class("pb-4")
                        .division(|l| l.class("text-[12px] text-ink-500").text("Current Index"))
                        .division(|v| v.class("text-[20px] font-semibold tracking-tight leading-tight mt-1").text("42.7 k"))
                })
                .division(|m| {
                    m.class("py-4 border-t-[1.5px] border-rule")
                        .division(|l| l.class("text-[12px] text-ink-500").text("Last cycle"))
                        .division(|row| row.class("mt-1 flex items-baseline gap-1.5")
                            .span(|s| s.class("text-[20px] font-semibold tracking-tight leading-tight").text("38.1 k"))
                            .span(|s| s.class("text-[11px] font-medium text-positive").text("+12.1%")))
                })
                .division(|m| {
                    m.class("pt-4 border-t-[1.5px] border-rule")
                        .division(|l| l.class("text-[12px] text-ink-500").text("Two cycles ago"))
                        .division(|row| row.class("mt-1 flex items-baseline gap-1.5")
                            .span(|s| s.class("text-[20px] font-semibold tracking-tight leading-tight").text("31.4 k"))
                            .span(|s| s.class("text-[11px] font-medium text-positive").text("+36.0%")))
                })
        })
        .division(|d| {
            d.class("mt-8").push(sub("Card variant")).division(|c| {
                c.class("w-[160px] p-3 rounded-lg border border-line bg-surface")
                    .division(|l| l.class("text-[12px] text-ink-500").text("Baseline"))
                    .division(|row| row.class("mt-1 flex items-baseline gap-1.5")
                        .span(|s| s.class("text-[18px] font-semibold tracking-tight leading-tight").text("22.9 k"))
                        .span(|s| s.class("text-[11px] font-medium text-positive").text("+86.4%")))
            })
        })
        .build()
}

// ── 09 Labels ────────────────────────────────────────────

fn render_labels() -> Division {
    let bar = |bg: &str, ink: &str, text: &str| -> Division {
        Division::builder()
            .class(format!("h-7 rounded inline-flex items-center px-3 text-[12px] font-medium whitespace-nowrap {bg} {ink}"))
            .text(text.to_owned())
            .build()
    };
    Division::builder()
        .push(sec("labels", "09", "Labels", "28px tall, 4px radius, label inset 12px. Pastel fill with paired ink for text \u{2014} 4.5:1 contrast minimum."))
        .division(|col| col.class("flex flex-col items-start gap-2")
            .push(bar("bg-cat-blue", "text-cat-blueInk", "Lorem ipsum dolor"))
            .push(bar("bg-cat-pink", "text-cat-pinkInk", "Sit amet"))
            .push(bar("bg-cat-cream", "text-cat-creamInk", "Consectetur"))
            .push(bar("bg-cat-green", "text-cat-greenInk", "Adipiscing elit"))
            .push(bar("bg-cat-peach", "text-cat-peachInk", "Sed do eiusmod"))
            .push(bar("bg-cat-lilac", "text-cat-lilacInk", "Tempor incididunt"))
            .push(bar("bg-cat-teal", "text-cat-tealInk", "Ut labore"))
            .push(bar("bg-cat-rust", "text-cat-rustInk", "Et dolore magna"))
            .push(bar("bg-cat-plum", "text-cat-plumInk", "Aliqua enim"))
            .push(bar("bg-cat-slate", "text-cat-slateInk", "Ad minim veniam")))
        .build()
}

// ── 10 Tooltip ───────────────────────────────────────────

fn render_tooltip() -> Division {
    Division::builder()
        .push(sec("tooltip", "10", "Tooltip", "Inverted surface with backdrop blur. Caption label above, key/value rows with right-aligned medium values."))
        .division(|d| {
            d.class("p-12 bg-canvas border border-line rounded-lg flex items-center justify-center")
                .division(|tip| {
                    tip.class("shadow-tooltip rounded-md backdrop-blur text-canvas px-3 py-2 text-[12px] leading-tight")
                        .style("background: color-mix(in oklab, var(--c-ink-900) 85%, transparent)")
                        .division(|lbl| lbl.class("text-ink-300").text("Cycle 14 \u{00b7} Aenean"))
                        .division(|row| row.class("mt-1 flex items-center justify-between gap-6")
                            .span(|s| s.text("Maxima:"))
                            .span(|s| s.class("font-medium").text("9.42")))
                        .division(|row| row.class("flex items-center justify-between gap-6")
                            .span(|s| s.text("Minima:"))
                            .span(|s| s.class("font-medium").text("3.18")))
                })
        })
        .build()
}

// ── 11 Table ─────────────────────────────────────────────

fn render_table() -> Division {
    Division::builder()
        .push(sec("table", "11", "Table", "Soft 1px row separators. Tabular numerals; right-aligned values; negatives in pinkInk."))
        .division(|d| {
            d.class("overflow-x-auto border-t-[1.5px] border-lineSoft")
                .text(r#"<table class="w-full min-w-[400px] text-[13px]"><thead><tr class="text-ink-400"><th class="text-left font-normal py-4 pr-6 w-[160px]"></th><th class="text-right font-normal py-4 px-4">Cycle 13</th><th class="text-right font-normal py-4 px-4">Cycle 14</th></tr></thead><tbody class="text-ink-900"><tr class="border-t-[1.5px] border-lineSoft"><td class="py-5 pr-6 font-medium">Lorem inflow</td><td class="text-right px-4 tabular-nums">10 246</td><td class="text-right px-4 tabular-nums">5 642</td></tr><tr class="border-t-[1.5px] border-lineSoft"><td class="py-5 pr-6 font-medium">Dolor outflow</td><td class="text-right px-4 tabular-nums text-negative">\u{2212}984</td><td class="text-right px-4 tabular-nums text-negative">\u{2212}1 889</td></tr><tr class="border-t-[1.5px] border-lineSoft"><td class="py-5 pr-6 font-medium">Net amet</td><td class="text-right px-4 tabular-nums font-medium">9 262</td><td class="text-right px-4 tabular-nums font-medium">3 753</td></tr></tbody></table>"#.to_owned())
        })
        .build()
}

// ── 12 Icons ─────────────────────────────────────────────

fn render_icons() -> Division {
    Division::builder()
        .push(sec("icons", "12", "Icons", "Stroke icons at 1.75 weight. Default size 16px in toolbars, 14px inside buttons."))
        .push(sub("Available icons"))
        .division(|g| g.class("flex flex-wrap items-center gap-4")
            .division(|d| d.class("flex flex-col items-center gap-2 p-3 border border-lineSoft rounded-md text-ink-700")
                .text(format!("{} Copy", icon::svg(icon::Icon::Copy, icon::IconSize::Lg))))
            .division(|d| d.class("flex flex-col items-center gap-2 p-3 border border-lineSoft rounded-md text-ink-700")
                .text(format!("{} Check", icon::svg(icon::Icon::Check, icon::IconSize::Lg))))
            .division(|d| d.class("flex flex-col items-center gap-2 p-3 border border-lineSoft rounded-md text-ink-700")
                .text(format!("{} Search", icon::svg(icon::Icon::Search, icon::IconSize::Md))))
            .division(|d| d.class("flex flex-col items-center gap-2 p-3 border border-lineSoft rounded-md text-ink-700")
                .text(format!("{} Chevron", icon::svg(icon::Icon::ChevronRight, icon::IconSize::Sm)))))
        .build()
}

// ── 13 Search / Form Fields ──────────────────────────────

fn render_search() -> Division {
    Division::builder()
        .push(sec("search", "13", "Form Fields", "Inputs sit on white surface with a 1px gray border. Three search bar variants: compact, hero, and inline."))
        .push(sub("Compact"))
        .push(search_bar::compact("ds-search-compact"))
        .division(|d| d.class("mt-8").push(sub("Hero"))
            .division(|c| c.class("max-w-lg").push(search_bar::hero(&search_bar::SearchBar {
                input_id: "ds-search-hero",
                ..search_bar::SearchBar::default()
            }))))
        .division(|d| d.class("mt-8").push(sub("Inline"))
            .division(|c| c.class("max-w-lg").push(search_bar::inline("example query"))))
        .build()
}

// ── 14 Checkbox · Radio · Switch ─────────────────────────

fn render_toggles() -> Division {
    Division::builder()
        .push(sec("toggles", "14", "Checkbox \u{00b7} Radio \u{00b7} Switch", "All controls render in ink-900 when active. 16px hit area minimum."))
        .push(sub("Checkbox"))
        .division(|d| {
            d.class("space-y-2")
                .division(|row| row.class("flex items-center gap-2 text-[14px]")
                    .span(|s| s.class("grid place-items-center h-4 w-4 rounded bg-ink-900 text-canvas")
                        .text("\u{2713}".to_owned()))
                    .text("Aenean lectus".to_owned()))
                .division(|row| row.class("flex items-center gap-2 text-[14px]")
                    .span(|s| s.class("h-4 w-4 rounded border border-line bg-surface"))
                    .text("Vestibulum ante".to_owned()))
        })
        .division(|d| {
            d.class("mt-6")
                .push(sub("Radio"))
                .division(|inner| inner.class("space-y-2")
                    .division(|row| row.class("flex items-center gap-2 text-[14px]")
                        .span(|s| s.class("grid place-items-center h-4 w-4 rounded-full border border-ink-900")
                            .span(|dot| dot.class("h-2 w-2 rounded-full bg-ink-900")))
                        .text("Lorem option".to_owned()))
                    .division(|row| row.class("flex items-center gap-2 text-[14px]")
                        .span(|s| s.class("h-4 w-4 rounded-full border border-line bg-surface"))
                        .text("Ipsum option".to_owned())))
        })
        .division(|d| {
            d.class("mt-6")
                .push(sub("Switch"))
                .division(|inner| inner.class("space-y-3")
                    .division(|row| row.class("flex items-center gap-3 text-[14px]")
                        .span(|s| s.class("relative inline-flex h-5 w-9 items-center rounded-full bg-ink-900")
                            .span(|knob| knob.class("inline-block h-4 w-4 rounded-full bg-surface translate-x-[18px]")))
                        .text("Enabled".to_owned()))
                    .division(|row| row.class("flex items-center gap-3 text-[14px]")
                        .span(|s| s.class("relative inline-flex h-5 w-9 items-center rounded-full bg-ink-300")
                            .span(|knob| knob.class("inline-block h-4 w-4 rounded-full bg-surface translate-x-[2px]")))
                        .text("Disabled".to_owned())))
        })
        .build()
}

// ── 15 Badges ────────────────────────────────────────────

fn render_badges() -> Division {
    Division::builder()
        .push(sec(
            "badges",
            "15",
            "Badges",
            "Compact pill labels. Use categorical pairs for status; ink for counts and metadata.",
        ))
        .push(sub("Status"))
        .division(|r| {
            r.class("flex flex-wrap items-center gap-2 text-[12px] font-medium")
                .push(badge::status("Active", badge::BadgeColor::Green))
                .push(badge::status("Pending", badge::BadgeColor::Cream))
                .push(badge::status("Failed", badge::BadgeColor::Pink))
                .push(badge::status("Info", badge::BadgeColor::Blue))
                .push(badge::status("Draft", badge::BadgeColor::Muted))
        })
        .division(|d| {
            d.class("mt-6").push(sub("Counts")).division(|r| {
                r.class("flex flex-wrap items-center gap-2")
                    .push(badge::count("3"))
                    .push(badge::count("12"))
                    .push(badge::count("99+"))
            })
        })
        .division(|d| {
            d.class("mt-6").push(sub("Tag")).division(|r| {
                r.class("flex flex-wrap items-center gap-2 text-[12px]")
                    .span(|s| s.class("inline-flex items-center gap-1 px-2 h-6 rounded-md border border-line text-ink-700").text("Tellus"))
                    .span(|s| s.class("inline-flex items-center gap-1 px-2 h-6 rounded-md border border-line text-ink-700").text("Convallis"))
            })
        })
        .build()
}

// ── 16 Dropdown ──────────────────────────────────────────

fn render_dropdown() -> Division {
    Division::builder()
        .push(sec("dropdown", "16", "Dropdown", "Floating menu on surface. 1px gray border + tooltip-grade shadow. Section dividers separate logical groups."))
        .division(|d| {
            d.class("p-12 bg-canvas border border-line rounded-lg flex items-start justify-center")
                .division(|menu| {
                    menu.class("w-56 rounded-md bg-surface border border-line shadow-tooltip py-1 text-[13px]")
                        .division(|lbl| lbl.class("px-3 py-1.5 text-[11px] font-mono uppercase tracking-wider text-ink-400").text("Actions"))
                        .division(|item| item.class("px-3 h-8 flex items-center gap-2 text-ink-900").text("Edit lorem"))
                        .division(|item| item.class("px-3 h-8 flex items-center gap-2 text-ink-900").text("Duplicate"))
                        .division(|sep| sep.class("my-1 border-t border-lineSoft"))
                        .division(|item| item.class("px-3 h-8 flex items-center gap-2 text-ink-900")
                            .text("Share")
                            .span(|s| s.class("ml-auto text-[11px] font-mono text-ink-400").text("\u{2318}S")))
                        .division(|sep| sep.class("my-1 border-t border-lineSoft"))
                        .division(|item| item.class("px-3 h-8 flex items-center gap-2 text-negative").text("Delete"))
                })
        })
        .build()
}

// ── 17 Modal ─────────────────────────────────────────────

fn render_modal() -> Division {
    Division::builder()
        .push(sec("modal", "17", "Modal", "Centered dialog over a 50% ink scrim. 8px radius, 1px gray border, 24px padding. Header / body / footer rhythm."))
        .division(|d| {
            d.class("relative rounded-lg p-8 overflow-hidden bg-canvas")
                // Skeleton page beneath
                .division(|skel| skel.class("absolute inset-0 p-6 select-none pointer-events-none").aria_hidden(true)
                    .division(|b| b.class("h-3 w-40 rounded mb-3 bg-ink-300"))
                    .division(|b| b.class("h-2 w-72 rounded mb-2 bg-line"))
                    .division(|b| b.class("h-2 w-64 rounded bg-line")))
                // Scrim
                .division(|scrim| scrim.class("absolute inset-0").style("background:rgba(15,15,17,0.55)"))
                // Dialog
                .division(|dialog| {
                    dialog.class("relative mx-auto max-w-md bg-surface border border-line rounded-lg shadow-tooltip")
                        .division(|hdr| hdr.class("flex items-start justify-between p-5 border-b border-lineSoft")
                            .division(|t| t
                                .division(|n| n.class("text-[15px] font-semibold tracking-tight").text("Confirm action"))
                                .division(|s| s.class("text-[12px] text-ink-500 mt-1").text("Lorem ipsum dolor sit amet"))))
                        .division(|body| body.class("p-5 text-[14px] text-ink-700 leading-relaxed")
                            .text("Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas."))
                        .division(|footer| footer.class("flex items-center justify-end gap-2 p-4 border-t border-lineSoft bg-canvas rounded-b-lg")
                            .push(button::render(button::Variant::Outline, button::Size::Compact, "Cancel"))
                            .push(button::render(button::Variant::Filled, button::Size::Compact, "Confirm")))
                })
        })
        .build()
}

// ── 18 Breadcrumb & Pagination ───────────────────────────

fn render_breadcrumb() -> Division {
    let chevron = icon::svg(icon::Icon::ChevronRight, icon::IconSize::Sm);
    Division::builder()
        .push(sec("breadcrumb", "18", "Breadcrumb & Pagination", "Breadcrumb uses chevron separators. Pagination is square-buttoned for compact toolbars."))
        .push(sub("Breadcrumb"))
        .division(|nav| {
            nav.class("flex items-center gap-1.5 text-[13px] text-ink-500")
                .anchor(|a| a.href("#").class("hover:text-ink-900").text("Tellus"))
                .span(|s| s.class("text-ink-300").text(chevron.to_owned()))
                .anchor(|a| a.href("#").class("hover:text-ink-900").text("Pellentesque"))
                .span(|s| s.class("text-ink-300").text(chevron.to_owned()))
                .span(|s| s.class("text-ink-900 font-medium").text("Vestibulum ante"))
        })
        .division(|d| {
            d.class("mt-8")
                .push(sub("Pagination"))
                .division(|row| row.class("inline-flex items-center gap-1 text-[13px]")
                    .division(|b| b.class("h-8 w-8 grid place-items-center rounded-md border border-line bg-surface text-ink-500 hover:bg-surfaceMuted")
                        .text(icon::svg(icon::Icon::ChevronRight, icon::IconSize::Sm).replace("m9 18 6-6-6-6", "m15 18-6-6 6-6").to_owned()))
                    .division(|b| b.class("h-8 w-8 grid place-items-center rounded-md border border-line bg-surface hover:bg-surfaceMuted").text("1"))
                    .division(|b| b.class("h-8 w-8 grid place-items-center rounded-md bg-ink-900 text-canvas font-medium").text("2"))
                    .division(|b| b.class("h-8 w-8 grid place-items-center rounded-md border border-line bg-surface hover:bg-surfaceMuted").text("3"))
                    .span(|s| s.class("px-1 text-ink-400").text("\u{2026}"))
                    .division(|b| b.class("h-8 w-8 grid place-items-center rounded-md border border-line bg-surface hover:bg-surfaceMuted").text("12"))
                    .division(|b| b.class("h-8 w-8 grid place-items-center rounded-md border border-line bg-surface text-ink-500 hover:bg-surfaceMuted")
                        .text(icon::svg(icon::Icon::ChevronRight, icon::IconSize::Sm).to_owned())))
        })
        .build()
}

// ── 19 Progress & Spinner ────────────────────────────────

fn render_progress() -> Division {
    Division::builder()
        .push(sec(
            "progress",
            "19",
            "Progress & Spinner",
            "Determinate progress bar and skeleton shimmer for placeholder content.",
        ))
        .push(sub("Progress bar"))
        .division(|d| {
            d.class("space-y-2 max-w-md").division(|bar| {
                bar.division(|labels| {
                    labels
                        .class("flex justify-between text-[12px] text-ink-500 mb-1")
                        .span(|s| s.text("Aenean lectus"))
                        .span(|s| s.class("font-mono").text("68%"))
                })
                .division(|track| {
                    track
                        .class("h-1.5 w-full rounded-pill bg-surfaceMuted overflow-hidden")
                        .division(|fill| {
                            fill.class("h-full bg-ink-900 rounded-pill")
                                .style("width:68%")
                        })
                })
            })
        })
        .division(|d| {
            d.class("mt-8").push(sub("Spinner"))
                .division(|r| r.class("flex items-center gap-4")
                    .text(r#"<svg class="animate-spin text-ink-900" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><path d="M21 12a9 9 0 1 1-6.2-8.55"/></svg>"#.to_owned())
                    .text(r#"<svg class="animate-spin text-ink-500" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M21 12a9 9 0 1 1-6.2-8.55"/></svg>"#.to_owned())
                    .text(r#"<svg class="animate-spin text-ink-300" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round"><path d="M21 12a9 9 0 1 1-6.2-8.55"/></svg>"#.to_owned()))
        })
        .division(|d| {
            d.class("mt-8").push(sub("Skeleton")).division(|inner| {
                inner
                    .class("max-w-md space-y-2")
                    .division(|s| s.class("h-4 w-2/3 rounded bg-surfaceMuted"))
                    .division(|s| s.class("h-3 w-full rounded bg-surfaceMuted"))
                    .division(|s| s.class("h-3 w-5/6 rounded bg-surfaceMuted"))
            })
        })
        .build()
}

// ── 20 Empty State ───────────────────────────────────────

fn render_empty_state() -> Division {
    Division::builder()
        .push(sec("empty", "20", "Empty State", "Centered illustration, title, body, and CTA for empty tables, search misses, and first-run views."))
        .push(empty_state::render(
            "No lorem yet",
            "Pellentesque habitant morbi tristique. Get started by creating your first entry.",
        ))
        .build()
}

// ── 21 Regions ───────────────────────────────────────────

fn render_regions() -> Division {
    Division::builder()
        .push(sec("regions", "21", "Regions", "Pages use stacked regions. Primary on canvas, secondary on surface. The surface swap is the boundary \u{2014} no rules needed."))
        .division(|demo| {
            demo.class("border border-line rounded-lg overflow-hidden")
                .division(|primary| {
                    primary.class("bg-canvas p-6")
                        .division(|lbl| lbl.class("text-[11px] font-mono uppercase tracking-wider text-ink-500").text("Primary region \u{00b7} canvas"))
                        .division(|h| h.class("mt-3 text-[18px] font-semibold tracking-tight").text("Lorem ipsum dolor sit"))
                        .division(|grid| grid.class("mt-4 grid grid-cols-3 gap-3")
                            .division(|b| b.class("h-12 rounded bg-surfaceMuted"))
                            .division(|b| b.class("h-12 rounded bg-surfaceMuted"))
                            .division(|b| b.class("h-12 rounded bg-surfaceMuted")))
                })
                .division(|secondary| {
                    secondary.class("bg-surface p-6")
                        .division(|lbl| lbl.class("text-[11px] font-mono uppercase tracking-wider text-ink-500").text("Secondary region \u{00b7} surface"))
                        .division(|h| h.class("mt-3 text-[18px] font-semibold tracking-tight").text("Aenean lectus pellentesque"))
                        .division(|line| line.class("mt-4 h-px bg-lineSoft"))
                        .division(|grid| grid.class("mt-4 grid grid-cols-4 gap-3 text-[12px] text-ink-500")
                            .division(|c| c.text("Vestibulum"))
                            .division(|c| c.text("Convallis"))
                            .division(|c| c.text("Tempor"))
                            .division(|c| c.text("Faucibus")))
                })
        })
        .division(|d| {
            d.class("mt-8").push(sub("Rules"))
                .division(|ul| ul.class("space-y-2 text-[13px] text-ink-700 leading-relaxed")
                    .division(|li| li.class("flex gap-3")
                        .span(|s| s.class("font-mono text-ink-500 w-12 shrink-0").text("01"))
                        .text("The primary region sits on canvas. Use it for the main subject.".to_owned()))
                    .division(|li| li.class("flex gap-3")
                        .span(|s| s.class("font-mono text-ink-500 w-12 shrink-0").text("02"))
                        .text("Secondary regions sit on surface. The boundary is the surface swap itself.".to_owned()))
                    .division(|li| li.class("flex gap-3")
                        .span(|s| s.class("font-mono text-ink-500 w-12 shrink-0").text("03"))
                        .text("Use full-bleed background swap so the boundary reads as a true section break.".to_owned()))
                    .division(|li| li.class("flex gap-3")
                        .span(|s| s.class("font-mono text-ink-500 w-12 shrink-0").text("04"))
                        .text("Maximum two surface swaps per page.".to_owned()))
                    .division(|li| li.class("flex gap-3")
                        .span(|s| s.class("font-mono text-ink-500 w-12 shrink-0").text("05"))
                        .text("Within a region, use lineSoft for internal subdivisions.".to_owned())))
        })
        .build()
}

// ── 22 Motion ────────────────────────────────────────────

fn render_motion() -> Division {
    Division::builder()
        .push(sec("motion", "22", "Motion", "Motion is functional: it explains state changes, never decorates them. Most transitions sit between 120\u{2013}260ms on the standard curve."))
        .push(sub("Durations"))
        .division(|d| d.class("divide-y divide-lineSoft border-t border-lineSoft")
            .division(|row| row.class("py-3 grid grid-cols-[80px_80px_1fr] gap-4 items-center text-[13px]")
                .span(|s| s.class("font-mono").text("fast"))
                .span(|s| s.class("font-mono text-ink-500").text("120ms"))
                .span(|s| s.class("text-ink-700").text("Color, opacity, focus rings.")))
            .division(|row| row.class("py-3 grid grid-cols-[80px_80px_1fr] gap-4 items-center text-[13px]")
                .span(|s| s.class("font-mono").text("base"))
                .span(|s| s.class("font-mono text-ink-500").text("180ms"))
                .span(|s| s.class("text-ink-700").text("Default transition. Hover, expand-collapse.")))
            .division(|row| row.class("py-3 grid grid-cols-[80px_80px_1fr] gap-4 items-center text-[13px]")
                .span(|s| s.class("font-mono").text("slow"))
                .span(|s| s.class("font-mono text-ink-500").text("260ms"))
                .span(|s| s.class("text-ink-700").text("Position changes, panel slides.")))
            .division(|row| row.class("py-3 grid grid-cols-[80px_80px_1fr] gap-4 items-center text-[13px]")
                .span(|s| s.class("font-mono").text("page"))
                .span(|s| s.class("font-mono text-ink-500").text("360ms"))
                .span(|s| s.class("text-ink-700").text("Route transitions, modal open."))))
        .division(|d| d.class("mt-8").push(sub("Easing curves"))
            .division(|g| g.class("grid grid-cols-1 sm:grid-cols-2 gap-4")
                .division(|c| c.class("p-4 rounded-md border border-lineSoft")
                    .division(|h| h.class("flex items-baseline justify-between")
                        .division(|n| n.class("text-[13px] font-medium").text("Standard"))
                        .division(|v| v.class("text-[11px] font-mono text-ink-500").text("cubic-bezier(.2,0,0,1)")))
                    .division(|d2| d2.class("mt-2 text-[12px] text-ink-500").text("Default for state changes \u{2014} hover, focus, expand.")))
                .division(|c| c.class("p-4 rounded-md border border-lineSoft")
                    .division(|h| h.class("flex items-baseline justify-between")
                        .division(|n| n.class("text-[13px] font-medium").text("Entrance"))
                        .division(|v| v.class("text-[11px] font-mono text-ink-500").text("cubic-bezier(0,0,0,1)")))
                    .division(|d2| d2.class("mt-2 text-[12px] text-ink-500").text("Elements arriving on screen \u{2014} toasts, popovers, modals.")))
                .division(|c| c.class("p-4 rounded-md border border-lineSoft")
                    .division(|h| h.class("flex items-baseline justify-between")
                        .division(|n| n.class("text-[13px] font-medium").text("Exit"))
                        .division(|v| v.class("text-[11px] font-mono text-ink-500").text("cubic-bezier(.4,0,1,1)")))
                    .division(|d2| d2.class("mt-2 text-[12px] text-ink-500").text("Elements leaving \u{2014} dismissed alerts, closed sheets.")))
                .division(|c| c.class("p-4 rounded-md border border-lineSoft")
                    .division(|h| h.class("flex items-baseline justify-between")
                        .division(|n| n.class("text-[13px] font-medium").text("Spring"))
                        .division(|v| v.class("text-[11px] font-mono text-ink-500").text("cubic-bezier(.34,1.56,.64,1)")))
                    .division(|d2| d2.class("mt-2 text-[12px] text-ink-500").text("Reserved for direct manipulation feedback \u{2014} toggles, drag-snap.")))))
        .division(|d| d.class("mt-8").push(sub("Rules"))
            .division(|ul| ul.class("space-y-2 text-[13px] text-ink-700 leading-relaxed")
                .division(|li| li.class("flex gap-3")
                    .span(|s| s.class("font-mono text-ink-500 w-12 shrink-0").text("01"))
                    .text("Animate transform and opacity only.".to_owned()))
                .division(|li| li.class("flex gap-3")
                    .span(|s| s.class("font-mono text-ink-500 w-12 shrink-0").text("02"))
                    .text("Default to base + standard. Pick another token only when intent demands it.".to_owned()))
                .division(|li| li.class("flex gap-3")
                    .span(|s| s.class("font-mono text-ink-500 w-12 shrink-0").text("03"))
                    .text("Pair entrance and exit asymmetrically: exits 60\u{2013}80% of the entrance duration.".to_owned()))
                .division(|li| li.class("flex gap-3")
                    .span(|s| s.class("font-mono text-ink-500 w-12 shrink-0").text("04"))
                    .text("Spring is for human-initiated, direct manipulation only.".to_owned()))
                .division(|li| li.class("flex gap-3")
                    .span(|s| s.class("font-mono text-ink-500 w-12 shrink-0").text("05"))
                    .text("Honor prefers-reduced-motion \u{2014} collapse to instant or a 60ms cross-fade.".to_owned()))))
        .build()
}

// ── 23 Details ───────────────────────────────────────────

fn render_details() -> Division {
    use crate::components::detail_row;

    Division::builder()
        .push(sec("details", "23", "Details", "Compact key/value lists for sidebars and inspector panels. Three variants: stacked, inline, and sectioned."))
        .push(sub("Stacked"))
        .division(|dl| {
            dl.class("max-w-[220px] space-y-3")
                .division(|item| item
                    .division(|dt| dt.class("text-[11px] text-ink-500 uppercase tracking-wider").text("Status"))
                    .division(|dd| dd.class("text-[14px] mt-0.5").text("Active")))
                .division(|item| item
                    .division(|dt| dt.class("text-[11px] text-ink-500 uppercase tracking-wider").text("Owner"))
                    .division(|dd| dd.class("text-[14px] mt-0.5").text("Aenean Lectus")))
                .division(|item| item
                    .division(|dt| dt.class("text-[11px] text-ink-500 uppercase tracking-wider").text("Created"))
                    .division(|dd| dd.class("text-[14px] mt-0.5 font-mono").text("2026-04-02")))
        })
        .division(|d| d.class("mt-8").push(sub("Inline")))
        .division(|dl| {
            dl.class("max-w-[260px]")
                .push(detail_row::row("Status", detail_row::Value::Text("Active".to_owned())))
                .push(detail_row::row("Owner", detail_row::Value::Text("Aenean Lectus".to_owned())))
                .push(detail_row::row("Created", detail_row::Value::Text("2026-04-02".to_owned())))
                .push(detail_row::row("Region", detail_row::Value::Text("eu-west-1".to_owned())))
                .push(detail_row::row("Repository", detail_row::Value::Link {
                    text: "rust-lang/rust".to_owned(),
                    href: "#".to_owned(),
                }))
        })
        .division(|d| {
            d.class("mt-8").push(sub("In a card"))
                .division(|card| {
                    card.class("p-5 bg-surface rounded-lg shadow-card max-w-[320px]")
                        .division(|hdr| hdr.class("flex items-baseline justify-between gap-3")
                            .division(|n| n.class("text-[14px] font-medium tracking-tight").text("Aenean Lectus"))
                            .push(badge::status("Active", badge::BadgeColor::Green)))
                        .division(|id| id.class("text-[11px] text-ink-500 font-mono mt-0.5").text("id_8a4f29c1"))
                        .division(|rule| rule.class("my-4 border-t-[1.5px] border-rule"))
                        .push(detail_row::row("Region", detail_row::Value::Text("eu-west-1".to_owned())))
                        .push(detail_row::row("Replicas", detail_row::Value::Text("3".to_owned())))
                        .push(detail_row::row("Uptime", detail_row::Value::Text("99.94%".to_owned())))
                })
        })
        .division(|d| {
            d.class("mt-8").push(sub("Sectioned"))
                .division(|s| {
                    s.class("max-w-[260px] text-[12px]")
                        .push(detail_row::row("Status", detail_row::Value::Text("Active".to_owned())))
                        .push(detail_row::row("Owner", detail_row::Value::Text("Aenean Lectus".to_owned())))
                        .push(detail_row::section_rule())
                        .push(detail_row::section_label("Infrastructure"))
                        .push(detail_row::row("Region", detail_row::Value::Text("eu-west-1".to_owned())))
                        .push(detail_row::row("Replicas", detail_row::Value::Text("3".to_owned())))
                        .push(detail_row::row("Uptime", detail_row::Value::Text("99.94%".to_owned())))
                })
        })
        .build()
}

// ── Package Cards (custom) ───────────────────────────────

fn render_package_cards() -> Division {
    let demo_pkg = demo_package(
        "wasi",
        "http",
        "HTTP request and response types for WebAssembly components.",
    );

    Division::builder()
        .push(sec("cards", "P1", "Package Cards", "Clickable cards used in home and namespace grids. Shows namespace, name, version, and description."))
        .division(|g| g.class("grid grid-cols-1 sm:grid-cols-2 gap-4 max-w-lg")
            .push(package_card::render(&demo_pkg))
            .push(package_card::render(&demo_package("wasi", "cli", "Command-line interface types and streams."))))
        .build()
}

// ── Package Rows (custom) ────────────────────────────────

fn render_package_rows() -> Division {
    Division::builder()
        .push(sec("rows", "P2", "Package Rows", "List-style rows for search results and all-packages pages. Name, version, description."))
        .division(|list| {
            list.class("divide-y divide-lineSoft max-w-xl")
                .push(package_row::render(&demo_package("wasi", "http", "HTTP request and response types.")))
                .push(package_row::render(&demo_package("wasi", "cli", "Command-line interface types.")))
                .push(package_row::render(&demo_package("wasi", "io", "I/O stream primitives.")))
        })
        .build()
}

// ── Section Groups (custom) ──────────────────────────────

fn render_section_groups() -> Division {
    Division::builder()
        .push(sec("groups", "P3", "Section Groups", "Grouped sections with header counts and item rows with colored dots and stability badges. Used on interface detail pages."))
        .push(section_group::header("Traits", 3))
        .push(section_group::item_row("Read", "#", section_group::ItemColor::Resource, section_group::Stability::Stable, "Read bytes from a source."))
        .push(section_group::item_row("Write", "#", section_group::ItemColor::Resource, section_group::Stability::Stable, "Write bytes to a sink."))
        .push(section_group::item_row("Seek", "#", section_group::ItemColor::Resource, section_group::Stability::Unstable, "Reposition the cursor within a stream."))
        .division(|d| d.class("mt-8")
            .push(section_group::header("Functions", 2))
            .push(section_group::item_row("copy", "#", section_group::ItemColor::Func, section_group::Stability::Stable, "Copy bytes from a reader to a writer."))
            .push(section_group::item_row("read-all", "#", section_group::ItemColor::Func, section_group::Stability::Unknown, "Read all bytes from a stream into a buffer.")))
        .build()
}

// ── P4 Code Blocks ───────────────────────────────────────

fn render_code_blocks() -> Division {
    Division::builder()
        .push(sec(
            "codeblocks",
            "P4",
            "Code Blocks",
            "Preformatted code containers with monospace font, border, and horizontal scroll. Used for WIT definitions and install commands.",
        ))
        .push(sub("Standard"))
        .push(
            html::text_content::PreformattedText::builder()
                .class(code_block::CLASS)
                .code(|c| c.text("wasm install wasi:http@0.2.0".to_owned()))
                .build(),
        )
        .division(|d| {
            d.class("mt-6").push(sub("Multi-line")).push(
                html::text_content::PreformattedText::builder()
                    .class(code_block::CLASS)
                    .code(|c| {
                        c.text(
                            "interface poll {\n  resource pollable;\n  poll: func(in: list<borrow<pollable>>) -> list<u32>;\n}"
                                .to_owned(),
                        )
                    })
                    .build(),
            )
        })
        .build()
}

// ── P5 Copy Buttons ──────────────────────────────────────

fn render_copy_buttons() -> Division {
    Division::builder()
        .push(sec(
            "copybuttons",
            "P5",
            "Copy Buttons",
            "Page headings with integrated copy-to-clipboard and optional version badge. Used on package, interface, and item detail pages.",
        ))
        .push(sub("With version badge"))
        .text(copy_button::heading_with_copy_and_version(
            "poll",
            "Interface",
            "wasi:io/poll",
            "text-wit-iface",
            "",
            Some("0.2.11"),
        ))
        .division(|d| {
            d.class("mt-8").push(sub("Without version")).text(
                copy_button::heading_with_copy(
                    "error-code",
                    "Enum",
                    "wasi:http/types/error-code",
                    "text-wit-enum",
                    "",
                ),
            )
        })
        .build()
}

// ── P6 Nav Lists ─────────────────────────────────────────

fn render_nav_lists() -> Division {
    use html::text_content::UnorderedList;

    let mut ifaces = UnorderedList::builder();
    ifaces.class("space-y-px");
    ifaces.push(nav_list::item("poll", "#", nav_list::NavState::Active));
    ifaces.push(nav_list::item("streams", "#", nav_list::NavState::Inactive));
    ifaces.push(nav_list::item("error", "#", nav_list::NavState::Inactive));

    let mut worlds = UnorderedList::builder();
    worlds.class("space-y-px");
    worlds.push(nav_list::item("imports", "#", nav_list::NavState::Inactive));

    Division::builder()
        .push(sec(
            "navlists",
            "P7",
            "Nav Lists",
            "Sidebar navigation items with active/inactive states and left border accent. Used for interface and world navigation.",
        ))
        .division(|d| {
            d.class("max-w-[260px] bg-surface border border-line p-4")
                .push(nav_list::section("Interfaces"))
                .push(ifaces.build())
                .division(|rule| rule.class("my-3 border-t-[1.5px] border-rule"))
                .push(nav_list::section("Worlds"))
                .push(worlds.build())
        })
        .build()
}

// ── P8 Section Headings ──────────────────────────────────

fn render_section_headings() -> Division {
    Division::builder()
        .push(sec(
            "secheadings",
            "P8",
            "Section Headings",
            "Muted h2 headings used to label content sections. Two variants: plain and bordered (with a bottom rule).",
        ))
        .push(sub("Plain"))
        .heading_2(|h2| h2.class(section_heading::CLASS).text("WIT Definition"))
        .division(|d| {
            d.class("mt-8")
                .push(sub("Bordered"))
                .heading_2(|h2| h2.class(section_heading::BORDERED_CLASS).text("Producers"))
        })
        .build()
}

// ── P9 Sidebar Sections ──────────────────────────────────

fn render_sidebar_sections() -> Division {
    Division::builder()
        .push(sec(
            "sidebarsec",
            "P9",
            "Sidebar Details",
            "Section labels, rule dividers, and detail rows for the sidebar. Uses detail_row primitives.",
        ))
        .division(|d| {
            d.class("max-w-[260px]")
                .push(detail_row::section_label("Dependencies"))
                .division(|list| {
                    list.class("space-y-1")
                        .division(|link| {
                            link.class("text-[12px]").anchor(|a| {
                                a.href("/wasi/io/0.2.11")
                                    .class("text-accent hover:underline font-mono")
                                    .text("wasi:io")
                            })
                        })
                        .division(|link| {
                            link.class("text-[12px]").anchor(|a| {
                                a.href("/wasi/cli/0.2.11")
                                    .class("text-accent hover:underline font-mono")
                                    .text("wasi:cli")
                            })
                        })
                        .division(|link| {
                            link.class("text-[12px]").anchor(|a| {
                                a.href("/wasi/http/0.2.11")
                                    .class("text-accent hover:underline font-mono")
                                    .text("wasi:http")
                            })
                        })
                })
                .push(detail_row::section_rule())
                .push(detail_row::section_label("Metadata"))
                .push(detail_row::row(
                    "Version",
                    detail_row::Value::Text("0.2.11".to_owned()),
                ))
                .push(detail_row::row(
                    "License",
                    detail_row::Value::Text("Apache-2.0".to_owned()),
                ))
        })
        .build()
}

// ── Demo data ────────────────────────────────────────────

fn demo_package(ns: &str, name: &str, desc: &str) -> wasm_meta_registry_client::KnownPackage {
    wasm_meta_registry_client::KnownPackage {
        registry: "ghcr.io".to_string(),
        repository: format!("{ns}/{name}"),
        kind: Some(wasm_meta_registry_client::PackageKind::Interface),
        description: Some(desc.to_string()),
        tags: vec!["0.2.0".to_string()],
        signature_tags: vec![],
        attestation_tags: vec![],
        last_seen_at: "2026-01-01T00:00:00Z".to_string(),
        created_at: "2026-01-01T00:00:00Z".to_string(),
        wit_namespace: Some(ns.to_string()),
        wit_name: Some(name.to_string()),
        dependencies: vec![],
    }
}
