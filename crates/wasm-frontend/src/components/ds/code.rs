//! 08 — Code Samples.

use html::text_content::Division;

/// Pre block: plain TOML example.
const PRE_PLAIN: &str = r#"<pre class="id-code"><span class="h">[package]</span>
<span class="k">name</span>    <span class="p">=</span> <span class="s">"example.com/hello-world"</span>
<span class="k">version</span> <span class="p">=</span> <span class="s">"0.1.0"</span>
<span class="k">authors</span> <span class="p">=</span> <span class="p">[</span><span class="s">"Lorem Ipsum &lt;lorem@example.com&gt;"</span><span class="p">]</span>

<span class="h">[dependencies]</span>
<span class="k">"wasi:http"</span>   <span class="p">=</span> <span class="s">"0.2"</span>
<span class="k">"wasi:cli"</span>    <span class="p">=</span> <span class="s">"0.2"</span></pre>"#;

/// Pre block: curl example.
const PRE_CURL: &str = r#"<pre class="id-code"><span class="v">curl</span> <span class="f">-X</span> POST \
  <span class="f">-H</span> <span class="s">"Authorization: Bearer $TOKEN"</span> \
  <span class="f">-H</span> <span class="s">"Content-Type: application/json"</span> \
  <span class="f">-d</span> <span class="s">'{"version":"0.2.4","digest":"sha256:1f08…"}'</span> \
  https://registry.example.com/v1/packages/wasi/http</pre>"#;

/// Pre block: curl request (paired).
const PRE_REQUEST: &str = r#"<pre class="id-code"><span class="v">curl</span> <span class="f">-X</span> POST \
  <span class="f">-H</span> <span class="s">"Authorization: Bearer $TOKEN"</span> \
  <span class="f">-H</span> <span class="s">"Content-Type: application/json"</span> \
  <span class="f">-d</span> <span class="s">'{
    "version": "0.2.4",
    "digest":  "sha256:1f08…"
  }'</span> \
  https://registry.example.com/v1/packages/wasi/http</pre>"#;

/// Pre block: JSON response (paired).
const PRE_RESPONSE: &str = r#"<pre class="id-code">{
  <span class="k">"version"</span><span class="p">:</span> <span class="s">"0.2.4"</span><span class="p">,</span>
  <span class="k">"digest"</span><span class="p">:</span>  <span class="s">"sha256:1f08…"</span><span class="p">,</span>
  <span class="k">"yanked"</span><span class="p">:</span>  <span class="n">false</span><span class="p">,</span>
  <span class="k">"published_at"</span><span class="p">:</span> <span class="s">"2026-04-18T08:11:02Z"</span>
}</pre>"#;

/// Token legend entries: (token_class, token_style, token_text, description).
pub(crate) const TOKENS: &[(&str, &str, &str, &str)] = &[
    (
        "mono w-12",
        "color:var(--color-wit-struct)",
        ".k .f",
        "keys / keywords / shell flags",
    ),
    (
        "mono w-12",
        "color:var(--color-wit-resource)",
        ".s",
        "strings",
    ),
    (
        "mono w-12",
        "color:var(--color-wit-func)",
        ".n",
        "numbers / booleans",
    ),
    ("mono w-12", "color:var(--color-wit-world)", ".ty", "types"),
    (
        "mono w-12",
        "color:var(--color-wit-iface)",
        ".fn .at",
        "functions / attributes",
    ),
    (
        "mono w-12",
        "color:var(--color-wit-module);font-weight:600",
        ".h",
        "section headers",
    ),
    (
        "mono w-12 text-ink-900",
        "font-weight:600",
        ".v",
        "shell verb",
    ),
    ("mono w-12 text-ink-400", "", ".p", "punctuation"),
    (
        "mono w-12 text-ink-500",
        "font-style:italic",
        ".c",
        "comments",
    ),
];

/// Anatomy items — prose with inline `<code>`.
pub(crate) const ANATOMY_ITEMS: &[&str] = &[
    r#"Quiet inset panel — <code class="mono text-[12px]">--c-surface</code> background, <code class="mono text-[12px]">--c-line-soft</code> hairline border, 5px radius. Inverts with the rest of the page in dark mode."#,
    "12.5px monospace, line-height 1.55, padding 14\u{00d7}16. Horizontal overflow scrolls rather than wrapping \u{2014} code never reflows.",
    r#"Tokens are short class names on <code class="mono text-[12px]">&lt;span&gt;</code>: <code class="mono text-[12px]">.k</code>, <code class="mono text-[12px]">.s</code>, <code class="mono text-[12px]">.n</code>, <code class="mono text-[12px]">.ty</code>, <code class="mono text-[12px]">.fn</code>, <code class="mono text-[12px]">.at</code>, <code class="mono text-[12px]">.h</code>, <code class="mono text-[12px]">.v</code>, <code class="mono text-[12px]">.f</code>, <code class="mono text-[12px]">.p</code>, <code class="mono text-[12px]">.c</code>. Untagged characters fall back to the panel's default <code class="mono text-[12px]">--c-ink-900</code>."#,
    r#"Caption above each panel uses <code class="mono text-[12px]">12px ink-500</code>; request/response labels use <code class="mono text-[12px]">11px mono uppercase ink-500</code> with <code class="mono text-[12px]">tracking-wider</code> for parity with the rest of the system."#,
    r#"Tabbed variant: <code class="mono text-[12px]">.id-lang-tabs</code> sits flush against the panel; the active tab shares the panel's dark surface and zeros its bottom-left corner so the two read as one piece."#,
];

/// Tab strip description.
const TAB_DESC: &str = r#"Tab strip uses <code class="mono text-[12px]">.id-lang-tabs</code> + <code class="mono text-[12px]">.id-lang-tab</code>. Active tab fuses with the panel (shared surface, top-left radius zeroed). The <code class="mono text-[12px]">.is-soon</code> modifier dims tabs whose bindings haven't landed yet — users see where future languages will appear without the link being live."#;

/// Token palette description.
const TOKEN_DESC: &str = r#"Each token references the same <code class="mono text-[12px]">--color-wit-*</code> variables we use for WIT diagrams — module pink, world purple, interface sky, function green, struct indigo, resource amber. Both light and dark themes carry calibrated hex pairs so the panel never feels over-saturated on paper or muddy on graphite."#;

#[allow(dead_code)]
/// Render a `<pre>` code block with the given content.
pub(crate) fn code_block(content: &str) -> String {
    let content = content.to_owned();
    html::text_content::PreformattedText::builder()
        .class("id-code")
        .text(content)
        .build()
        .to_string()
}

/// Class string for a standard code block.
pub(crate) const CODE_BLOCK_CLASS: &str = "id-code";

/// Render this section.
pub(crate) fn render(
    section_id: &str,
    num: &str,
    title: &str,
    desc: &str,
    tokens: &[(&str, &str, &str, &str)],
    anatomy_items: &[&str],
) -> String {
    // Token legend grid
    let mut token_grid = Division::builder();
    token_grid.class("grid grid-cols-2 md:grid-cols-3 gap-x-6 gap-y-2.5 text-[12px]");
    for (cls, style, token, desc) in tokens {
        let cls = (*cls).to_owned();
        let style = (*style).to_owned();
        let token = (*token).to_owned();
        let desc = (*desc).to_owned();
        let entry = Division::builder()
            .class("flex items-center gap-3")
            .span(|s| {
                let s = s.class(cls).text(token);
                if style.is_empty() { s } else { s.style(style) }
            })
            .span(|s| s.class("text-ink-500").text(desc))
            .build();
        token_grid.push(entry);
    }

    // Anatomy list
    let mut anatomy_ul = html::text_content::UnorderedList::builder();
    anatomy_ul.class(
        "text-[13px] text-ink-700 leading-relaxed space-y-1.5 pl-5 list-disc marker:text-ink-400",
    );
    for item in anatomy_items {
        let item = (*item).to_owned();
        anatomy_ul.list_item(|li| li.paragraph(|p| p.text(item)));
    }

    let content = Division::builder()
        .class("space-y-12")
        // Plain · single panel
        .division(|d| {
            d.division(|l| l.class("text-[12px] text-ink-500 mb-3").text("Plain \u{00b7} single panel"))
                .text(PRE_PLAIN.to_owned())
                .paragraph(|p| p.class("mt-3 text-[12px] text-ink-500").text(
                    "Default form. Used for one-off TOML, JSON, or shell snippets where the language is obvious from context."
                ))
        })
        // Tabbed · multiple languages
        .division(|d| {
            d.division(|l| l.class("text-[12px] text-ink-500 mb-3").text("Tabbed \u{00b7} same call, multiple languages"))
                .division(|tabs| {
                    tabs.class("id-lang-tabs")
                        .span(|s| s.class("id-lang-tab is-active").span(|dot| dot.class("dot")).text("curl"))
                        .span(|s| s.class("id-lang-tab").span(|dot| dot.class("dot")).text("Rust"))
                        .span(|s| s.class("id-lang-tab is-soon").span(|dot| dot.class("dot")).text("Python"))
                        .span(|s| s.class("id-lang-tab is-soon").span(|dot| dot.class("dot")).text("TypeScript"))
                })
                .division(|panel| panel.class("id-lang-panel").text(PRE_CURL.to_owned()))
                .paragraph(|p| p.class("mt-3 text-[12px] text-ink-500").text(TAB_DESC))
        })
        // Paired · request and response
        .division(|d| {
            d.division(|l| l.class("text-[12px] text-ink-500 mb-3").text("Paired \u{00b7} request and response"))
                .division(|grid| {
                    grid.class("grid md:grid-cols-2 gap-4")
                        .division(|req| {
                            req.division(|l| l.class("text-[11px] mono uppercase tracking-wider text-ink-500 mb-2").text("Request"))
                                .text(PRE_REQUEST.to_owned())
                        })
                        .division(|res| {
                            res.division(|l| l.class("text-[11px] mono uppercase tracking-wider text-ink-500 mb-2").text("Response \u{00b7} 201"))
                                .text(PRE_RESPONSE.to_owned())
                        })
                })
                .paragraph(|p| p.class("mt-3 text-[12px] text-ink-500").text(
                    "Two panels in a 2-col grid; each carries its own 11px ink-500 mono uppercase label above. Use for HTTP / RPC reference where the response shape is part of the contract. Stacks to single column on narrow viewports."
                ))
        })
        // Token palette
        .division(|d| {
            d.division(|l| l.class("text-[12px] text-ink-500 mb-3").text("Token palette"))
                .division(|card| card.class("rounded-lg border border-line bg-canvas p-4").push(token_grid.build()))
                .paragraph(|p| p.class("mt-3 text-[12px] text-ink-500").text(TOKEN_DESC))
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
            "code",
            "08",
            "Code Samples",
            "One panel \u{2014} <code class=\"mono text-[12px]\">pre.id-code</code> \u{2014} sitting on <code class=\"mono text-[12px]\">--c-surface</code>, with token colours pulled from the theme-aware <code class=\"mono text-[12px]\">--color-wit-*</code> palette so chroma stays balanced on both light and dark pages. Three forms: a plain block, a tabbed multi-language block, and a paired request / response grid.",
            TOKENS,
            ANATOMY_ITEMS,
        )));
    }
}
