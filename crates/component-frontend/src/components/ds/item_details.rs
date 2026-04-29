//! C05 — Item Details.

use html::content::{Article, Header};
use html::text_content::Division;

/// An auxiliary link shown next to the title (e.g. "source", "spec").
pub(crate) struct AuxLink {
    /// Link text.
    pub label: String,
    /// Link href.
    pub href: String,
}

/// Configuration for rendering a single item detail entry.
pub(crate) struct ItemDetailEntry {
    /// Sigil background color CSS value.
    pub sigil_bg: String,
    /// Sigil text color CSS value.
    pub sigil_color: String,
    /// Sigil character.
    pub sigil_text: String,
    /// Item name (displayed as heading).
    pub name: String,
    /// Optional anchor href for the § link.
    pub anchor_href: Option<String>,
    /// Optional "since" version tag (e.g. "v1.4.0").
    pub since: Option<String>,
    /// Auxiliary links (source, spec, etc.).
    pub aux_links: Vec<AuxLink>,
    /// Optional header/signature bar HTML (method pill + path, or code sig).
    pub header_html: Option<String>,
    /// Optional tagline / description text.
    pub tagline: Option<String>,
    /// Optional body content HTML (code block, docs, etc.).
    pub body_html: Option<String>,
}

/// Render a single item detail entry using the DS C05 pattern.
///
/// Produces an `<article>` with:
/// - Title heading (sigil + name) with optional aux row
/// - Optional header/signature bar
/// - Optional tagline paragraph
/// - Optional body content
///
/// When `in_list` is true, adds `py-5 border-b border-lineSoft` for stacking
/// in a list of entries. When false, uses `space-y-5` for standalone display.
pub(crate) fn item_detail_entry(entry: &ItemDetailEntry, in_list: bool) -> Article {
    let sigil_prefix = if entry.sigil_text.is_empty() {
        String::new()
    } else {
        format!(
            r#"<span class="sigil" style="background:{};color:{};">{}</span> "#,
            entry.sigil_bg, entry.sigil_color, entry.sigil_text,
        )
    };

    let cls = if in_list {
        "py-5 border-b border-lineSoft"
    } else {
        "space-y-5"
    };

    let mut article = Article::builder();
    article.class(cls);

    // Title row with optional aux
    let mut header = Header::builder();
    header.division(|d| {
        d.class("id-title-head");
        d.heading_2(|h| {
            h.text(sigil_prefix);
            h.span(|s| s.text(entry.name.clone()));
            if let Some(href) = &entry.anchor_href {
                h.anchor(|a| a.href(href.clone()).class("id-anchor").text("\u{00a7}"));
            }
            h
        });
        if entry.since.is_some() || !entry.aux_links.is_empty() {
            d.division(|aux| {
                aux.class("id-aux");
                if let Some(since) = &entry.since {
                    aux.span(|s| s.class("id-since-tag").text(since.clone()));
                }
                for link in &entry.aux_links {
                    let href = link.href.clone();
                    let label = link.label.clone();
                    aux.anchor(|a| a.href(href).class("id-src-link").text(label));
                }
                aux
            });
        }
        d
    });
    article.push(header.build());

    // Header / signature bar
    if let Some(header_html) = &entry.header_html {
        article.text(header_html.clone());
    }

    // Tagline
    if let Some(tagline) = &entry.tagline {
        article.paragraph(|p| p.class("id-page-tagline").text(tagline.clone()));
    }

    // Body content
    if let Some(body) = &entry.body_html {
        article.text(body.clone());
    }

    article.build()
}

/// Anatomy list items — each contains rich inline HTML with `<strong>`,
/// `<code>`, and `<em>` mixed into prose. Kept as raw strings since
/// converting every inline element would make the code unreadable.
pub(crate) const ANATOMY_ITEMS: &[&str] = &[
    r#"<strong>Container</strong> — the whole article sits inside a <code class="mono text-[12px]">rounded-lg border border-line bg-canvas</code> card with <code class="mono text-[12px]">p-5 md:p-6</code> padding, mirroring the bordered surfaces used by the search trigger and the navbar mockups. The card frames the symbol so it reads as a discrete reference unit, not loose page chrome."#,
    r#"<strong>Title</strong> — page-level <code class="mono text-[12px]">&lt;h2&gt;</code> in <code class="mono text-[12px]">.id-title-head</code> with an aux row on the right (<code class="mono text-[12px]">.id-since-tag</code> for the version a method/endpoint was introduced, plus <code class="mono text-[12px]">.id-src-link</code> entries for source / spec / man). Always the first element on the page."#,
    r#"<strong>Header (signature)</strong> — sits directly below the title so readers see what the symbol <em>is</em> before reading what it <em>does</em>. Pill (<code class="mono text-[12px]">.id-method</code> for HTTP, <code class="mono text-[12px]">.id-kind</code> for RPC) on the left, mono path (<code class="mono text-[12px]">.id-path</code>) center, optional <code class="mono text-[12px]">.id-auth-tag</code> pushed right via <code class="mono text-[12px]">margin-left: auto</code>. Hairline-bordered band on <code class="mono text-[12px]">canvas</code> — distinct from cards, anchored to the article."#,
    r#"<strong>Tagline</strong> — a single-paragraph <code class="mono text-[12px]">.id-page-tagline</code> below the signature, capped at <code class="mono text-[12px]">72ch</code>. What this thing does, in one sentence. Acts as the bridge from the signature to the structured sections below."#,
    r#"<strong>Path coloring</strong> — static segments stay <code class="mono text-[12px] text-ink-900">ink-900</code>, parameters (<code class="mono text-[12px] text-ink-500">{registry}</code>) take <code class="mono text-[12px]" style="color:var(--c-cat-plum-ink);font-weight:600;">plum-ink</code> so the eye can scan placeholders without reading every character. Slashes drop to <code class="mono text-[12px] text-ink-500">ink-400</code> as low-contrast punctuation."#,
    r#"<strong>Request body</strong> (optional) — structured table, not a code block. Each row: a name + type/required line on the left (<code class="mono text-[12px]">w-[180px]</code>, mono name + 11.5px ink-500 mono meta) and a description on the right. Hairline rules between rows. Use this whenever the body has more than two fields or any optional fields — code blocks become hard to scan past ~3 keys."#,
    r#"<strong>Responses</strong> — divided list. HTTP uses <code class="mono text-[12px]">.id-http-status</code> pills (2xx green / 3xx blue / 4xx peach / 5xx pink). RPC uses <code class="mono text-[12px]">.id-status-dot</code> + fixed-width mono code (<code class="mono text-[12px]">w-20</code> so descriptions align)."#,
    r#"<strong>Example</strong> — for HTTP, paired request/response code panels in a 2-col grid. For RPC, language tabs above a single panel; inactive tabs are <code class="mono text-[12px]">is-soon</code> placeholders so users see where future bindings will land."#,
    r"Sections always appear in this order: <strong>title → header → tagline → request body → responses → example</strong>. Optional sections (request body, auth tag, example, since-tag, source/spec links) drop cleanly without reordering or backfilling — predictability across pages matters more than local optimization.",
];

/// Pill variant description (prose with inline `<code>`).
const PILL_DESC: &str = r#"Method pill (<code class="mono text-[12px]">.id-method-*</code>) and kind pill (<code class="mono text-[12px]">.id-kind-*</code>) share shape and slot — swap by surface, not by stacking. The auth tag is the only header element that uses <code class="mono text-[12px]">cream</code> + the rounded-pill shape, reserving cream for "you need credentials" semantics across the design system."#;

/// Build the live demo card: article with title, method pill, path, and tagline.
pub(crate) fn build_demo() -> Division {
    // Build the header bar HTML (method pill + path + auth tag)
    let header_html = Division::builder()
        .class("id-header")
        .span(|s| s.class("id-method id-method-post").text("POST"))
        .span(|s| {
            s.class("id-path")
                .span(|seg| seg.class("seg").text("/v1/packages"))
                .span(|sl| sl.class("sl").text("/"))
                .span(|par| par.class("par").text("{registry}"))
                .span(|sl| sl.class("sl").text("/"))
                .span(|par| par.class("par").text("{*repository}"))
        })
        .span(|s| s.class("id-auth-tag").text("Auth required"))
        .build()
        .to_string();

    let entry = ItemDetailEntry {
        sigil_bg: String::new(),
        sigil_color: String::new(),
        sigil_text: String::new(),
        name: "publish a package version".to_owned(),
        anchor_href: Some("#c-item-details".to_owned()),
        since: Some("v1.4.0".to_owned()),
        aux_links: vec![
            AuxLink {
                label: "source".to_owned(),
                href: "#".to_owned(),
            },
            AuxLink {
                label: "spec".to_owned(),
                href: "#".to_owned(),
            },
        ],
        header_html: Some(header_html),
        tagline: Some(
            "Push a new version of a package to a registry. The body \
             references an OCI manifest already uploaded via the standard \
             distribution endpoints."
                .to_owned(),
        ),
        body_html: None,
    };

    let detail = item_detail_entry(&entry, false);

    Division::builder()
        .division(|l| {
            l.class("text-[12px] text-ink-500 mb-3")
                .text("Cornerstone \u{00b7} POST with auth and request body")
        })
        .division(|card| {
            card.class("rounded-lg border border-line bg-canvas p-5 md:p-6")
                .push(detail)
        })
        .build()
}

/// Build the anatomy rules list.
pub(crate) fn build_anatomy(items: &[&str]) -> Division {
    let mut ul = html::text_content::UnorderedList::builder();
    ul.class(
        "text-[13px] text-ink-700 leading-relaxed space-y-1.5 pl-5 list-disc marker:text-ink-400",
    );
    for item_html in items {
        let item_html = (*item_html).to_owned();
        ul.list_item(|li| li.paragraph(|p| p.text(item_html)));
    }

    Division::builder()
        .division(|l| l.class("text-[12px] text-ink-500 mb-3").text("Anatomy"))
        .push(ul.build())
        .build()
}

/// Pill row: label + list of pills.
pub(crate) fn pill_row(label: &'static str, pills: &[(&'static str, &'static str)]) -> Division {
    let mut row = Division::builder();
    row.class("flex flex-wrap items-center gap-3");
    row.span(|s| {
        s.class("text-[11px] mono uppercase tracking-wider text-ink-500 w-16")
            .text(label)
    });
    for (cls, text) in pills {
        let cls = (*cls).to_owned();
        let text = (*text).to_owned();
        let span = html::inline_text::Span::builder()
            .class(cls)
            .text(text)
            .build();
        row.push(span);
    }
    row.build()
}

/// Build the pill variants grid.
pub(crate) fn build_pills() -> Division {
    Division::builder()
        .division(|l| {
            l.class("text-[12px] text-ink-500 mb-3")
                .text("Header pill variants")
        })
        .division(|pills| {
            pills
                .class("space-y-3")
                .push(pill_row(
                    "HTTP",
                    &[
                        ("id-method id-method-get", "GET"),
                        ("id-method id-method-post", "POST"),
                        ("id-method id-method-put", "PUT"),
                        ("id-method id-method-patch", "PATCH"),
                        ("id-method id-method-delete", "DELETE"),
                    ],
                ))
                .push(pill_row(
                    "gRPC",
                    &[
                        ("id-kind id-kind-unary", "UNARY"),
                        ("id-kind id-kind-server", "SERVER"),
                        ("id-kind id-kind-client", "CLIENT"),
                        ("id-kind id-kind-bidi", "BIDI"),
                    ],
                ))
                .division(|row| {
                    row.class("flex flex-wrap items-center gap-3")
                        .span(|s| {
                            s.class("text-[11px] mono uppercase tracking-wider text-ink-500 w-16")
                                .text("Tag")
                        })
                        .span(|s| {
                            s.class("id-auth-tag")
                                .style("margin-left:0;")
                                .text("Auth required")
                        })
                })
        })
        .paragraph(|p| p.class("mt-3 text-[12px] text-ink-500").text(PILL_DESC))
        .build()
}

/// Render this section.
pub(crate) fn render(
    section_id: &str,
    num: &str,
    title: &str,
    desc: &str,
    anatomy_items: &[&str],
) -> String {
    let content = Division::builder()
        .class("space-y-8")
        .push(build_demo())
        .push(build_anatomy(anatomy_items))
        .push(build_pills())
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
            "c-item-details",
            "C05",
            "Item Details",
            r##"Reference page for a single endpoint, RPC, schema, or command. A method/kind pill anchors the symbol below the title; a one-sentence tagline explains it; an optional structured request-body table, a responses list, and paired example panels stack below in fixed order. Used as the destination from <a href="#c-item-list" class="text-ink-700 underline decoration-line decoration-1 underline-offset-[3px] hover:text-ink-900">Item List</a> rows."##,
            ANATOMY_ITEMS,
        )));
    }
}
