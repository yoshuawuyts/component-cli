//! C02 — On This Page.

use html::content::Navigation;
use html::text_content::Division;

const SVG_UP: &str = concat!(
    r#"<svg class="h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">"#,
    include_str!("../../../../../vendor/lucide/chevron-up.svg"),
    "</svg>"
);

/// TOC link entries: (label, class_suffix).
pub(crate) const TOC_LINKS: &[(&str, &str)] = &[
    ("Overview", ""),
    ("Subcommands", ""),
    ("add", " indent"),
    ("remove", " indent active"),
    ("list", " indent"),
    ("login", " indent"),
    ("publish", " indent"),
    ("Global flags", ""),
    ("Environment", ""),
    ("Exit codes", ""),
    ("Config files", ""),
];

pub(crate) const ANATOMY_ITEMS: &[&str] = &[
    r#"Two depth levels only: top-level entries and one indent (<code class="mono text-[12px]">.indent</code>, +12px)."#,
    r#"Hover lifts ink to <code class="mono text-[12px]">--c-ink-900</code> and tints the left rail to <code class="mono text-[12px]">--c-line</code>."#,
    r#"Active state uses a full <code class="mono text-[12px]">--c-ink-900</code> rail; the rail is the only marker — never combine with a background."#,
    r#"Drive the active state from a scroll-spy (<code class="mono text-[12px]">IntersectionObserver</code> with a top rootMargin) so the marker tracks the section currently in view."#,
    "The rail width is reserved (1.5px transparent border) on every row, so toggling active doesn\u{2019}t shift the label.",
    "End the rail with a quiet <strong>Back to top</strong> button \u{2014} 11px mono, ink-500, no border; hover reveals the surface-muted background.",
];

#[allow(dead_code)]
/// A single entry in the "On this page" table of contents.
pub(crate) struct TocEntry<'a> {
    /// Anchor href (e.g. `"#worlds"`).
    pub href: &'a str,
    /// Display label.
    pub label: &'a str,
    /// Whether this entry is indented (child item).
    pub indent: bool,
}

/// Render a "On this page" sidebar ToC from a list of entries.
///
/// Uses the same markup as the DS showcase: `toc-link` anchors with
/// `space-y-px`, a "Back to top" button, and an inline scrollspy script
/// that highlights the link whose section is currently in view.
pub(crate) fn on_this_page_nav(links: &[TocEntry<'_>]) -> String {
    let mut nav = Navigation::builder();
    nav.class("space-y-px");

    for entry in links {
        let href = entry.href.to_owned();
        let label = entry.label.to_owned();
        let cls = if entry.indent {
            "toc-link indent"
        } else {
            "toc-link"
        };
        nav.anchor(|a| a.href(href).class(cls).text(label));
    }

    let mut wrapper = Division::builder();
    wrapper.division(|lbl| {
        lbl.class("mono uppercase tracking-wider text-[10px] text-ink-500 mb-2 px-2.5")
            .text("On this page")
    });
    wrapper.push(nav.build());
    wrapper.text(
        r#"<div class="px-2.5 mt-4"><button type="button" class="inline-flex items-center gap-1.5 h-7 px-2 rounded-md text-[11px] mono text-ink-500 hover:bg-surfaceMuted hover:text-ink-900 transition-colors" onclick="window.scrollTo({top:0,behavior:'smooth'})">"#,
    );
    wrapper.text(format!("{SVG_UP} Back to top"));
    wrapper.text("</button></div>");

    // Scrollspy script: highlights the toc-link whose target section is
    // nearest the top of the viewport.
    wrapper.text(SCROLLSPY_SCRIPT);

    wrapper.build().to_string()
}

/// Inline scrollspy script for the "On this page" ToC.
const SCROLLSPY_SCRIPT: &str = r##"<script>(function(){var links=document.querySelectorAll('.toc-link[href^="#"]');if(!links.length)return;var observer=new IntersectionObserver(function(entries){entries.forEach(function(e){var id=e.target.id;if(!id)return;var link=document.querySelector('.toc-link[href="#'+id+'"]');if(!link)return;if(e.isIntersecting){link.classList.add('active')}else{link.classList.remove('active')}})},{rootMargin:'-80px 0px -70% 0px',threshold:0});links.forEach(function(link){var href=link.getAttribute('href');if(!href)return;var target=document.querySelector(href);if(target)observer.observe(target)})})()</script>"##;

/// Render this section.
pub(crate) fn render(
    section_id: &str,
    num: &str,
    title: &str,
    desc: &str,
    toc_links: &[(&str, &str)],
    anatomy_items: &[&str],
) -> String {
    // TOC demo nav
    let mut nav = Navigation::builder();
    nav.class("space-y-px");
    for (label, cls_suffix) in toc_links {
        let cls = format!("toc-link{cls_suffix}");
        let label = (*label).to_owned();
        let a = html::inline_text::Anchor::builder()
            .href("#c-toc".to_owned())
            .class(cls)
            .text(label)
            .build();
        nav.push(a);
    }
    let nav = nav.build();

    // Anatomy UL
    let mut anatomy_ul = html::text_content::UnorderedList::builder();
    anatomy_ul.class(
        "text-[13px] text-ink-700 leading-relaxed space-y-1.5 pl-5 list-disc marker:text-ink-400",
    );
    for item in anatomy_items {
        let item = (*item).to_owned();
        anatomy_ul.list_item(|li| li.paragraph(|p| p.text(item)));
    }

    let content = Division::builder()
        .class("space-y-6")
        // Live demo
        .division(|d| {
            d.class("border border-line rounded-lg bg-canvas p-4 max-w-[240px]")
                .division(|lbl| {
                    lbl.class("mono uppercase tracking-wider text-[10px] text-ink-500 mb-2 px-2.5")
                        .text("On this page")
                })
                .push(nav)
                .division(|bottom| {
                    bottom.class("px-2.5 mt-4")
                        .button(|b| {
                            b.type_("button")
                                .class("inline-flex items-center gap-1.5 h-7 px-2 rounded-md text-[11px] mono text-ink-500 hover:bg-surfaceMuted hover:text-ink-900 transition-colors")
                                .text(SVG_UP)
                                .text(" Back to top")
                        })
                })
        })
        // States
        .division(|d| {
            d.division(|l| l.class("text-[12px] text-ink-500 mb-3").text("States"))
                .division(|grid| {
                    grid.class("grid grid-cols-3 gap-4 max-w-[480px]")
                        .division(|s| {
                            s.division(|l| l.class("text-[11px] mono uppercase tracking-wider text-ink-500 mb-2").text("Default"))
                                .anchor(|a| a.href("#c-toc".to_owned()).class("toc-link").text("Section title"))
                        })
                        .division(|s| {
                            s.division(|l| l.class("text-[11px] mono uppercase tracking-wider text-ink-500 mb-2").text("Hover"))
                                // Raw HTML: Anchor::style() creates a <style> child, not an inline style attribute.
                                .text(r##"<a href="#c-toc" class="toc-link" style="color:var(--c-ink-900);border-left-color:var(--c-line);">Section title</a>"##)
                        })
                        .division(|s| {
                            s.division(|l| l.class("text-[11px] mono uppercase tracking-wider text-ink-500 mb-2").text("Active"))
                                .anchor(|a| a.href("#c-toc".to_owned()).class("toc-link active").text("Section title"))
                        })
                })
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
            "c-toc",
            "C02",
            "On This Page",
            "Right-rail table of contents for long reference pages. A 1.5px left border lights up on hover and active state \u{2014} the only visual cue, no background fills.",
            TOC_LINKS,
            ANATOMY_ITEMS,
        )));
    }
}
