//! C04 — Item List.

use html::inline_text::Anchor;
use html::text_content::Division;

pub(crate) struct ItemRow {
    pub(crate) sigil_bg: &'static str,
    pub(crate) sigil_color: &'static str,
    pub(crate) sigil_text: &'static str,
    pub(crate) name: &'static str,
    pub(crate) desc: &'static str,
    pub(crate) meta: &'static str,
    pub(crate) deprecated: bool,
}

/// Dynamic item row with owned strings, for use with runtime data.
pub(crate) struct DynItemRow {
    pub(crate) sigil_bg: String,
    pub(crate) sigil_color: String,
    pub(crate) sigil_text: String,
    pub(crate) name: String,
    pub(crate) href: String,
    pub(crate) desc: String,
    pub(crate) meta: String,
    pub(crate) deprecated: bool,
}

#[allow(dead_code)]
pub(crate) fn render_item_row(item: &ItemRow) -> Anchor {
    let row_class = if item.deprecated {
        "item-row deprecated"
    } else {
        "item-row"
    };
    let sigil_style = format!("background:{};color:{};", item.sigil_bg, item.sigil_color);
    Anchor::builder()
        .href("#c-item-list".to_owned())
        .class(row_class)
        .span(|s| s.class("sigil").style(sigil_style).text(item.sigil_text))
        .division(|d| {
            d.span(|s| s.class("name").text(item.name))
                .division(|desc| desc.class("desc").text(item.desc))
        })
        .span(|s| s.class("meta").text(item.meta))
        .build()
}

#[allow(dead_code)]
pub(crate) fn render_item_list(items: &[ItemRow]) -> Division {
    let mut list = Division::builder();
    list.class("item-list");
    for item in items {
        list.push(render_item_row(item));
    }
    list.build()
}

/// Render a dynamic item row (owned strings, real href).
pub(crate) fn render_dyn_item_row(item: &DynItemRow) -> Anchor {
    let row_class = if item.deprecated {
        "item-row deprecated"
    } else {
        "item-row"
    };
    let _sigil_style = format!("background:{};color:{};", item.sigil_bg, item.sigil_color);
    let sigil_text = item.sigil_text.clone();
    let name = item.name.clone();
    let desc = item.desc.clone();
    let meta = item.meta.clone();
    let href = item.href.clone();
    // Raw HTML: Span::style() creates a <style> child, not an inline style attribute.
    let sigil_html = format!(
        r#"<span class="sigil" style="background:{};color:{};">{}</span>"#,
        item.sigil_bg, item.sigil_color, sigil_text
    );
    Anchor::builder()
        .href(href)
        .class(row_class)
        .text(sigil_html)
        .division(|d| {
            d.span(|s| s.class("name").text(name))
                .division(|dd| dd.class("desc").text(desc))
        })
        .span(|s| s.class("meta").text(meta))
        .build()
}

/// Render a list of dynamic item rows.
pub(crate) fn render_dyn_item_list(title: &str, items: &[DynItemRow]) -> Division {
    let title = title.to_owned();
    let count = items.len();
    let mut wrapper = Division::builder();
    wrapper.class("space-y-3");
    wrapper.division(|d| {
        d.class("flex items-baseline justify-between")
            .heading_2(|h| {
                h.class("text-[16px] font-semibold tracking-tight")
                    .text(title)
            })
            .span(|s| {
                s.class("text-[12px] text-ink-500 mono")
                    .text(format!("{count}"))
            })
    });
    let mut list = Division::builder();
    list.class("item-list");
    for item in items {
        list.push(render_dyn_item_row(item));
    }
    wrapper.push(list.build());
    wrapper.build()
}

pub(crate) const CMD_ROWS: &[ItemRow] = &[
    ItemRow {
        sigil_bg: "var(--c-cat-green)",
        sigil_color: "var(--c-cat-greenInk)",
        sigil_text: "c",
        name: "add",
        desc: "Register a new namespace and point it at an OCI registry URL.",
        meta: "1.2.0",
        deprecated: false,
    },
    ItemRow {
        sigil_bg: "var(--c-cat-green)",
        sigil_color: "var(--c-cat-greenInk)",
        sigil_text: "c",
        name: "remove",
        desc: "Forget a registered namespace. Locally cached artifacts are kept.",
        meta: "1.2.0",
        deprecated: false,
    },
    ItemRow {
        sigil_bg: "var(--c-cat-green)",
        sigil_color: "var(--c-cat-greenInk)",
        sigil_text: "c",
        name: "list",
        desc: "Print the effective set of registries, with the source of each entry.",
        meta: "1.2.0",
        deprecated: false,
    },
    ItemRow {
        sigil_bg: "var(--c-cat-green)",
        sigil_color: "var(--c-cat-greenInk)",
        sigil_text: "c",
        name: "login",
        desc: "Store credentials for a registry in the OS keychain.",
        meta: "1.4.0",
        deprecated: false,
    },
    ItemRow {
        sigil_bg: "var(--c-cat-green)",
        sigil_color: "var(--c-cat-greenInk)",
        sigil_text: "c",
        name: "publish",
        desc: "Build and upload the current package to a registry.",
        meta: "1.3.0",
        deprecated: false,
    },
    ItemRow {
        sigil_bg: "var(--c-cat-green)",
        sigil_color: "var(--c-cat-greenInk)",
        sigil_text: "c",
        name: "login-token",
        desc: "Legacy token-only login. Removed in 2.0 \u{2014} use <span class=\"mono text-[12px]\">login --password-stdin</span> instead.",
        meta: "deprecated",
        deprecated: true,
    },
];

pub(crate) const ENDPOINT_ROWS: &[ItemRow] = &[
    ItemRow {
        sigil_bg: "var(--c-cat-blue)",
        sigil_color: "var(--c-cat-blueInk)",
        sigil_text: "G",
        name: "/v1/packages/{name}",
        desc: "Resolve a package by canonical name. Returns the latest version metadata.",
        meta: "v1",
        deprecated: false,
    },
    ItemRow {
        sigil_bg: "var(--c-cat-blue)",
        sigil_color: "var(--c-cat-blueInk)",
        sigil_text: "G",
        name: "/v1/packages/{name}/versions",
        desc: "List every published version of a package, newest first.",
        meta: "v1",
        deprecated: false,
    },
    ItemRow {
        sigil_bg: "var(--c-cat-green)",
        sigil_color: "var(--c-cat-greenInk)",
        sigil_text: "P",
        name: "/v1/packages/{name}",
        desc: "Publish a new version. Body is a streaming OCI image manifest.",
        meta: "v1",
        deprecated: false,
    },
    ItemRow {
        sigil_bg: "var(--c-cat-pink)",
        sigil_color: "var(--c-cat-pinkInk)",
        sigil_text: "D",
        name: "/v1/packages/{name}/versions/{ver}",
        desc: "Yank a version. The artifact remains, but it stops resolving by default.",
        meta: "v1",
        deprecated: false,
    },
];

pub(crate) const ANATOMY_ITEMS: &[&str] = &[
    r#"The whole row is the link — <code class="mono text-[12px]">&lt;a class="item-row"&gt;</code> — so the entire chip is the click target. The inner <code class="mono text-[12px]">.name</code> stays a <code class="mono text-[12px]">&lt;span&gt;</code>."#,
    r#"Three-column grid: <code class="mono text-[12px]">sigil · name+desc · meta</code>; the middle column is the only flexible one."#,
    r#"The whole list sits in a <code class="mono text-[12px]">rounded-lg border border-line bg-canvas</code> card — same surface treatment as the search trigger and Item Details. Rows separate with a 1px <code class="mono text-[12px]">--c-line-soft</code> hairline inside the card; the first row drops it."#,
    "Name is 13.5px mono ink-900 medium; underlines on hover only. Description is 13px ink-700, one line, no wrapping pressure.",
    r#"Trailing <code class="mono text-[12px]">.meta</code> stays mono ink-500 — version, status, or count. Drop it entirely if there's nothing to show."#,
    "Sigil colour follows the C01 convention so the same kind reads consistently across sidebar and item list.",
    r#"<strong>Hover</strong> tints the entire row to <code class="mono text-[12px]">--c-surface-muted</code> — the only affordance that the row is interactive. No border or accent change."#,
    r#"<strong>Deprecated</strong> rows fade name + description to ink-400, strike through the name with a 1px line, and drop the sigil opacity to 50%. Set the trailing <code class="mono text-[12px]">.meta</code> to a one-word state ("deprecated", "removed") instead of a version."#,
];

/// Render this section.
pub(crate) fn render(
    section_id: &str,
    num: &str,
    title: &str,
    desc: &str,
    cmd_rows: &[ItemRow],
    endpoint_rows: &[ItemRow],
    anatomy_items: &[&str],
) -> String {
    let mut anatomy_ul = html::text_content::UnorderedList::builder();
    anatomy_ul.class(
        "text-[13px] text-ink-700 leading-relaxed space-y-1.5 pl-5 list-disc marker:text-ink-400",
    );
    for item in anatomy_items {
        let item = (*item).to_owned();
        anatomy_ul.list_item(|li| li.paragraph(|p| p.text(item)));
    }

    let content = Division::builder()
        .class("space-y-8")
        // Subcommands demo
        .division(|d| {
            d.division(|l| {
                l.class("text-[12px] text-ink-500 mb-3")
                    .text("Subcommands of ")
                    .span(|s| s.class("mono").text("wasm registry"))
            })
            .push(render_item_list(cmd_rows))
        })
        // Endpoints demo
        .division(|d| {
            d.division(|l| {
                l.class("text-[12px] text-ink-500 mb-3")
                    .text("Endpoints under ")
                    .span(|s| s.class("mono").text("/v1/packages"))
            })
            .push(render_item_list(endpoint_rows))
        })
        // Anatomy
        .division(|d| {
            d.division(|l| l.class("text-[12px] text-ink-500 mb-3").text("Anatomy"))
                .push(anatomy_ul.build())
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
            "c-item-list",
            "C04",
            "Item List",
            "Compact index of a group\u{2019}s children \u{2014} subcommands, endpoints, schemas. Each row is a sigil, a name + one-line description, and trailing meta. Rows separate with hairline rules, no card chrome.",
            CMD_ROWS,
            ENDPOINT_ROWS,
            ANATOMY_ITEMS,
        )));
    }
}
