//! 21 — Grid.

use html::text_content::Division;

/// Three-column wireframe skeleton.
const THREE_COL_WIREFRAME: &str = r#"<div class="border border-line rounded-lg overflow-hidden bg-canvas">
              <div class="grid grid-cols-[100px_1fr_84px] gap-3 lg:gap-4 p-4 text-[10px]">
                <div class="rounded-md border border-lineSoft bg-surfaceMuted p-2">
                  <div class="mono uppercase tracking-wider text-ink-500">240px</div>
                  <div class="mt-2 space-y-1.5"><div class="h-1.5 w-3/4 rounded-sm bg-ink-300"></div><div class="h-1.5 w-2/3 rounded-sm bg-ink-300"></div><div class="h-1.5 w-4/5 rounded-sm bg-ink-300"></div><div class="h-1.5 w-1/2 rounded-sm bg-ink-300"></div><div class="h-1.5 w-3/5 rounded-sm bg-ink-300"></div><div class="h-1.5 w-2/3 rounded-sm bg-ink-300"></div></div>
                </div>
                <div class="rounded-md border border-lineSoft bg-surface p-2">
                  <div class="mono uppercase tracking-wider text-ink-500">1fr · max-w-[72ch]</div>
                  <div class="mt-2 space-y-1.5"><div class="h-2 w-1/3 rounded-sm bg-ink-700"></div><div class="mt-2 h-1 rounded-sm bg-ink-300"></div><div class="h-1 rounded-sm bg-ink-300"></div><div class="h-1 w-5/6 rounded-sm bg-ink-300"></div><div class="h-1 rounded-sm bg-ink-300"></div><div class="h-1 w-2/3 rounded-sm bg-ink-300"></div></div>
                </div>
                <div class="rounded-md border border-lineSoft bg-surfaceMuted p-2">
                  <div class="mono uppercase tracking-wider text-ink-500">200px</div>
                  <div class="mt-2 space-y-1.5"><div class="h-1.5 w-3/4 rounded-sm bg-ink-300"></div><div class="h-1.5 w-2/3 rounded-sm bg-ink-300"></div><div class="h-1.5 w-1/2 rounded-sm bg-ink-300"></div><div class="h-1.5 w-3/5 rounded-sm bg-ink-300"></div></div>
                </div>
              </div>
            </div>"#;

/// Two-column wireframe skeleton.
const TWO_COL_WIREFRAME: &str = r#"<div class="border border-line rounded-lg overflow-hidden bg-canvas">
              <div class="grid grid-cols-[80px_1fr] gap-3 lg:gap-5 p-4 text-[10px]">
                <div class="rounded-md border border-lineSoft bg-surfaceMuted p-2">
                  <div class="mono uppercase tracking-wider text-ink-500">200px</div>
                  <div class="mt-2 space-y-1.5"><div class="h-1.5 w-2/3 rounded-sm bg-ink-300"></div><div class="h-1.5 w-3/4 rounded-sm bg-ink-300"></div></div>
                </div>
                <div class="rounded-md border border-lineSoft bg-surface p-2">
                  <div class="mono uppercase tracking-wider text-ink-500">1fr</div>
                  <div class="mt-2 space-y-1.5"><div class="h-2 w-1/3 rounded-sm bg-ink-700"></div><div class="mt-2 h-1 rounded-sm bg-ink-300"></div><div class="h-1 w-5/6 rounded-sm bg-ink-300"></div><div class="h-1 rounded-sm bg-ink-300"></div></div>
                </div>
              </div>
            </div>"#;

/// Single-column wireframe skeleton.
const SINGLE_COL_WIREFRAME: &str = r#"<div class="border border-line rounded-lg overflow-hidden bg-canvas">
              <div class="p-4 text-[10px] flex justify-center">
                <div class="rounded-md border border-lineSoft bg-surface p-2 w-[60%]">
                  <div class="mono uppercase tracking-wider text-ink-500">max-w-[72ch]</div>
                  <div class="mt-2 space-y-1.5"><div class="h-2 w-1/3 rounded-sm bg-ink-700"></div><div class="mt-2 h-1 rounded-sm bg-ink-300"></div><div class="h-1 rounded-sm bg-ink-300"></div><div class="h-1 w-5/6 rounded-sm bg-ink-300"></div><div class="h-1 rounded-sm bg-ink-300"></div><div class="h-1 w-2/3 rounded-sm bg-ink-300"></div></div>
                </div>
              </div>
            </div>"#;

/// Pre blocks showing the grid markup.
const PRE_THREE: &str = r#"<pre class="mt-3 p-3 rounded-md bg-surfaceMuted text-[12px] mono text-ink-700 overflow-x-auto">&lt;div class=&quot;max-w-[1440px] px-4 md:px-6 grid grid-cols-1
            lg:grid-cols-[240px_1fr_200px] gap-8 lg:gap-10 pt-8 pb-24&quot;&gt;</pre>"#;
const PRE_TWO: &str = r#"<pre class="mt-3 p-3 rounded-md bg-surfaceMuted text-[12px] mono text-ink-700 overflow-x-auto">&lt;div class=&quot;grid md:grid-cols-[200px_1fr] gap-6 md:gap-12&quot;&gt;</pre>"#;
const PRE_SINGLE: &str = r#"<pre class="mt-3 p-3 rounded-md bg-surfaceMuted text-[12px] mono text-ink-700 overflow-x-auto">&lt;article class=&quot;prose max-w-[72ch] mx-auto&quot;&gt;</pre>"#;

/// Description paragraphs.
const DESC_THREE: &str = r#"Reference layout in <a href="docs.html" class="underline decoration-line decoration-1 underline-offset-[3px] hover:text-ink-900">docs.html</a>. Three concrete column widths (<code class="mono text-[12px]">240px / 1fr / 200px</code>) — the sidebar carries the full nav tree, the centre column reads, the right column lists in-page anchors. Side columns use <code class="mono text-[12px]">lg:sticky lg:top-16 lg:self-start lg:max-h-[calc(100vh-5rem)] lg:overflow-y-auto</code> so they pin once the page scrolls past the header. Below <code class="mono text-[12px]">lg</code> the layout collapses to <code class="mono text-[12px]">grid-cols-1</code>; the right column is hidden (<code class="mono text-[12px]">hidden lg:block</code>), the left becomes a standard top-of-page nav. Gutters: <code class="mono text-[12px]">gap-8 lg:gap-10</code> (32 → 40px) — generous because three regions need air to read as separate concerns."#;
const DESC_TWO: &str = r#"The pattern this style guide uses for every section. <code class="mono text-[12px]">200px</code> for the section label / synopsis; <code class="mono text-[12px]">1fr</code> for the demos. Wider gutter (<code class="mono text-[12px]">md:gap-12</code> = 48px) than the three-column layout because there are only two columns to space."#;
const DESC_SINGLE: &str = r#"Long-form prose, blog posts, README-style pages. <code class="mono text-[12px]">72ch</code> is the body-copy reading measure (~640px at 14px); larger and the eye loses the line. Even inside the three-column grid, the reading column constrains its content to <code class="mono text-[12px]">max-w-[72ch]</code> — the column gets the available space, the prose doesn't fill it."#;

/// Rules items.
pub(crate) const RULES: &[(&str, &str)] = &[
    (
        "01",
        r#"Outer container is always <code class="mono text-[12px] text-ink-900 px-1 py-0.5 rounded-sm bg-surfaceMuted">max-w-[1440px] px-4 md:px-6</code>. Use full bleed (no <code class="mono text-[12px] text-ink-900 px-1 py-0.5 rounded-sm bg-surfaceMuted">max-w</code>) only for region surface swaps."#,
    ),
    (
        "02",
        "Pick the widest layout that fits the content, not the page. A docs page with no in-page anchors uses the two-column layout, not three-column with an empty column.",
    ),
    (
        "03",
        r#"Side columns are <strong>fixed-width</strong> (<code class="mono text-[12px] text-ink-900 px-1 py-0.5 rounded-sm bg-surfaceMuted">240px</code>, <code class="mono text-[12px] text-ink-900 px-1 py-0.5 rounded-sm bg-surfaceMuted">200px</code>); only the centre column is <code class="mono text-[12px] text-ink-900 px-1 py-0.5 rounded-sm bg-surfaceMuted">1fr</code>. This keeps the reading column stable as the viewport grows."#,
    ),
    (
        "04",
        r#"Gutter scales with column count: two columns get <code class="mono text-[12px] text-ink-900 px-1 py-0.5 rounded-sm bg-surfaceMuted">md:gap-12</code> (48px), three columns get <code class="mono text-[12px] text-ink-900 px-1 py-0.5 rounded-sm bg-surfaceMuted">lg:gap-10</code> (40px). More columns means more boundaries, so each gets less air."#,
    ),
    (
        "05",
        r#"Reading text is <strong>always</strong> <code class="mono text-[12px] text-ink-900 px-1 py-0.5 rounded-sm bg-surfaceMuted">max-w-[72ch]</code> inside its column. Demos, code, and tables can fill the column."#,
    ),
    (
        "06",
        r#"Side columns become sticky at <code class="mono text-[12px] text-ink-900 px-1 py-0.5 rounded-sm bg-surfaceMuted box-decoration-clone">lg:</code> and above with <code class="mono text-[12px] text-ink-900 px-1 py-0.5 rounded-sm bg-surfaceMuted box-decoration-clone">lg:sticky lg:top-16 lg:self-start lg:max-h-[calc(100vh-5rem)] lg:overflow-y-auto</code>. Below that breakpoint they collapse into the document flow."#,
    ),
    (
        "07",
        r#"The right column is the first to drop on narrow viewports (<code class="mono text-[12px] text-ink-900 px-1 py-0.5 rounded-sm bg-surfaceMuted box-decoration-clone">hidden lg:block</code>). On-this-page is a navigation aid, not load-bearing."#,
    ),
];

fn grid_example(
    label: &'static str,
    wireframe: &'static str,
    pre: &'static str,
    desc: &'static str,
) -> Division {
    Division::builder()
        .division(|l| l.class("text-[12px] text-ink-500 mb-3").text(label))
        .text(wireframe.to_owned())
        .text(pre.to_owned())
        .paragraph(|p| p.class("mt-3 text-[12px] text-ink-500").text(desc))
        .build()
}

/// Render this section.
pub(crate) fn render(
    section_id: &str,
    num: &str,
    title: &str,
    desc: &str,
    rules: &[(&str, &str)],
) -> String {
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
        .class("space-y-12")
        .push(grid_example(
            "Three-column \u{00b7} sidebar \u{00b7} reading \u{00b7} on this page",
            THREE_COL_WIREFRAME,
            PRE_THREE,
            DESC_THREE,
        ))
        .push(grid_example(
            "Two-column \u{00b7} section label \u{00b7} content",
            TWO_COL_WIREFRAME,
            PRE_TWO,
            DESC_TWO,
        ))
        .push(grid_example(
            "Single column \u{00b7} reading measure",
            SINGLE_COL_WIREFRAME,
            PRE_SINGLE,
            DESC_SINGLE,
        ))
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
            "grid",
            "21",
            "Grid",
            r#"Pages live in a <code class="mono text-[12px]">max-w-[1440px]</code> container with <code class="mono text-[12px]">px-4 md:px-6</code> gutters. Inside, a small set of column shapes covers every layout: <strong>three-column</strong> (sidebar · reading · on-this-page) for documentation; <strong>two-column</strong> for narrative pages and this style guide; <strong>single column</strong> bounded by a reading measure for prose. Reading text is always capped at <code class="mono text-[12px]">max-w-[72ch]</code> regardless of the column it sits in."#,
            RULES,
        )));
    }
}
