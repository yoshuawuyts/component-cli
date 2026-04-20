//! C01 — Nested Sidebar.

use html::content::Navigation;
use html::interactive::Details;
use html::text_content::Division;

const SVG_CHEV_DOWN: &str = r#"<svg class="h-3 w-3 text-ink-500" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5"><path d="m3 4.5 3 3 3-3" /></svg>"#;
const SVG_CHEV_RIGHT: &str = r#"<svg class="chev" viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.5"><path d="m4 2 3 3-3 3" /></svg>"#;
const SVG_GITHUB: &str = r#"<svg class="h-3.5 w-3.5 text-ink-500 flex-shrink-0" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true"><path d="M8 .2a8 8 0 0 0-2.5 15.6c.4 0 .55-.17.55-.38v-1.4c-2.22.48-2.69-1.07-2.69-1.07-.36-.92-.89-1.17-.89-1.17-.73-.5.05-.49.05-.49.8.06 1.23.83 1.23.83.71 1.23 1.87.87 2.33.66.07-.52.28-.87.5-1.07-1.77-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.83-2.15-.08-.2-.36-1.02.08-2.13 0 0 .67-.22 2.2.82A7.6 7.6 0 0 1 8 4.04c.68 0 1.37.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.11.16 1.93.08 2.13.52.56.83 1.28.83 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.74.54 1.49v2.21c0 .21.15.46.55.38A8 8 0 0 0 8 .2Z" /></svg>"#;
const SVG_CRATE: &str = r#"<svg class="h-3.5 w-3.5 text-ink-500 flex-shrink-0" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" aria-hidden="true"><rect x="2.5" y="3" width="11" height="10" rx="1" /><path d="M2.5 6.5h11M6 3v10" /></svg>"#;

fn sigil(bg: &str, color: &str, text: &str) -> String {
    format!(r#"<span class="sigil" style="background:{bg};color:{color};">{text}</span>"#)
}

fn tree_link(sigil_html: &str, name: &str, meta: &str) -> html::inline_text::Anchor {
    let mut a = html::inline_text::Anchor::builder();
    a.href("#".to_owned()).class("tree-link");
    a.text(sigil_html.to_owned());
    a.span(|s| s.class("mono").text(name.to_owned()));
    if !meta.is_empty() {
        let meta = meta.to_owned();
        a.span(|s| {
            s.class("ml-auto mono text-[10.5px] text-ink-400")
                .text(meta)
        });
    }
    a.build()
}

const SIGIL_CMD: &str = "var(--c-cat-green)";
const SIGIL_CMD_INK: &str = "var(--c-cat-greenInk)";
const SIGIL_GRP: &str = "var(--c-cat-lilac)";
const SIGIL_GRP_INK: &str = "var(--c-cat-lilacInk)";

const SIGIL_LEGEND: &[(&str, &str, &str, &str)] = &[
    (
        "var(--c-cat-green)",
        "var(--c-cat-greenInk)",
        "c",
        "Command",
    ),
    ("var(--c-cat-lilac)", "var(--c-cat-lilacInk)", "G", "Group"),
    ("var(--c-cat-blue)", "var(--c-cat-blueInk)", "F", "Flag"),
    ("var(--c-cat-peach)", "var(--c-cat-peachInk)", "E", "Env"),
    (
        "var(--c-cat-pink)",
        "var(--c-cat-pinkInk)",
        "X",
        "Exit code",
    ),
    (
        "var(--c-cat-slate)",
        "var(--c-cat-slateInk)",
        "\u{00b7}",
        "Root / misc",
    ),
];

const ANATOMY_ITEMS: &[&str] = &[
    r#"One <code class="mono text-[12px]">&lt;details&gt;</code> per group; rotates the chevron via <code class="mono text-[12px]">details[open]</code>."#,
    r#"Children indent 14px with a 1px <code class="mono text-[12px]">--c-line-soft</code> guide on the left."#,
    r#"Active state uses <code class="mono text-[12px]">--c-surface-muted</code> + medium weight — no border, no accent."#,
    r#"Trailing meta (counts, tags) sits in a <code class="mono text-[12px]">ml-auto</code> slot in 10.5px ink-400 mono."#,
    "Sigil colour signals <em>kind</em>, never status. Use the categorical palette and pair consistently across the surface.",
    "Bottom <strong>Project</strong> section uses lucide-style 14px outline icons (ink-500) instead of sigils \u{2014} reserved for external links (repo, package registry, issues).",
];

/// Render this section.
pub(crate) fn render() -> String {
    let cmd_sigil = sigil(SIGIL_CMD, SIGIL_CMD_INK, "c");
    let grp_sigil = sigil(SIGIL_GRP, SIGIL_GRP_INK, "G");
    let root_sigil = sigil("var(--c-cat-slate)", "var(--c-cat-slateInk)", "\u{00b7}");

    // Build the sidebar nav
    let mut nav = Navigation::builder();
    nav.class("space-y-0.5 text-[13px]");
    nav.push(tree_link(&root_sigil, "wasm", "root"));
    nav.push(tree_link(&cmd_sigil, "init", ""));
    nav.push(tree_link(&cmd_sigil, "build", ""));
    nav.push(tree_link(&cmd_sigil, "run", ""));

    // Registry group (open, active)
    let mut registry = Details::builder();
    registry.open(true);
    registry.text(format!(
        r#"<summary class="tree-link active">{SVG_CHEV_RIGHT}{grp_sigil}<span class="mono">registry</span><span class="ml-auto mono text-[10.5px] text-ink-400">7</span></summary><div class="tree-children space-y-0.5">{}{}{}{}</div>"#,
        tree_link(&cmd_sigil, "add", ""),
        tree_link(&cmd_sigil, "remove", ""),
        tree_link(&cmd_sigil, "list", ""),
        tree_link(&cmd_sigil, "publish", ""),
    ));
    nav.push(registry.build());

    // Component group (closed)
    let mut component = Details::builder();
    component.text(format!(
        r#"<summary class="tree-link">{SVG_CHEV_RIGHT}{grp_sigil}<span class="mono">component</span><span class="ml-auto mono text-[10.5px] text-ink-400">5</span></summary>"#
    ));
    nav.push(component.build());

    // Wit group (closed)
    let mut wit = Details::builder();
    wit.text(format!(
        r#"<summary class="tree-link">{SVG_CHEV_RIGHT}{grp_sigil}<span class="mono">wit</span><span class="ml-auto mono text-[10.5px] text-ink-400">4</span></summary>"#
    ));
    nav.push(wit.build());

    nav.push(tree_link(&cmd_sigil, "help", ""));

    // Sigil legend
    let mut legend_grid = Division::builder();
    legend_grid.class("flex flex-wrap gap-x-5 gap-y-2 text-[12px]");
    for (bg, color, text, label) in SIGIL_LEGEND {
        let sigil_html = sigil(bg, color, text);
        let label = (*label).to_owned();
        let entry = Division::builder()
            .class("flex items-center gap-2")
            .text(sigil_html)
            .span(|s| s.class("text-ink-700").text(label))
            .build();
        legend_grid.push(entry);
    }

    // Anatomy UL
    let mut anatomy_ul = html::text_content::UnorderedList::builder();
    anatomy_ul.class(
        "text-[13px] text-ink-700 leading-relaxed space-y-1.5 pl-5 list-disc marker:text-ink-400",
    );
    for item in ANATOMY_ITEMS {
        let item = (*item).to_owned();
        anatomy_ul.list_item(|li| li.paragraph(|p| p.text(item)));
    }

    let content = Division::builder()
        .class("space-y-6")
        // Live demo
        .division(|d| {
            d.class("border border-line rounded-lg bg-canvas p-4 max-w-[300px]")
                // Version dropdown
                .division(|v| {
                    v.class("pb-4 border-b hairline")
                        .division(|l| l.class("mono uppercase tracking-wider text-[10px] text-ink-500 mb-1").text("Version"))
                        .button(|b| {
                            b.class("w-full h-7 px-2.5 rounded-md border border-line bg-surface flex items-center justify-between text-ink-900 hover:bg-surfaceMuted text-[12px]")
                                .span(|s| s.class("mono").text("v2.4.0 (latest)"))
                                .text(SVG_CHEV_DOWN)
                        })
                })
                // Commands label
                .division(|l| l.class("mono uppercase tracking-wider text-[10px] text-ink-500 mb-2 mt-4").text("Commands"))
                // Nav tree
                .push(nav.build())
                // Project links
                .division(|proj| {
                    proj.class("mt-5 pt-4 border-t hairline")
                        .division(|l| l.class("mono uppercase tracking-wider text-[10px] text-ink-500 mb-2").text("Project"))
                        .push(Navigation::builder()
                            .class("space-y-px")
                            .anchor(|a| a.href("#".to_owned()).class("tree-link").text(format!("{SVG_GITHUB} Repository")))
                            .anchor(|a| a.href("#".to_owned()).class("tree-link").text(format!("{SVG_CRATE} Crates.io")))
                            .build())
                })
        })
        // Sigil legend
        .division(|d| {
            d.division(|l| l.class("text-[12px] text-ink-500 mb-3").text("Sigil kinds"))
                .push(legend_grid.build())
        })
        // Anatomy
        .division(|d| {
            d.division(|l| l.class("text-[12px] text-ink-500 mb-3").text("Anatomy"))
                .push(anatomy_ul.build())
        })
        .build()
        .to_string();

    super::section(
        "c-sidebar",
        "C01",
        "Nested Sidebar",
        r#"Hierarchical navigation for reference docs. Top-level entries collapse with native <code class="mono text-[12px]">&lt;details&gt;</code>; sigils classify each row by kind (command, group, flag, env, etc.)."#,
        &content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(render());
    }
}
