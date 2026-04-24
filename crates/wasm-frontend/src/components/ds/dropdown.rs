//! 16 — Dropdown.

use html::text_content::Division;

const SVG_EDIT: &str = concat!(
    r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/square-pen.svg"),
    "</svg>"
);
const SVG_COPY: &str = concat!(
    r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/copy.svg"),
    "</svg>"
);
const SVG_SHARE: &str = concat!(
    r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/share.svg"),
    "</svg>"
);
const SVG_DELETE: &str = concat!(
    r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/trash-2.svg"),
    "</svg>"
);

#[allow(dead_code)]
/// Render a single dropdown menu item.
pub(crate) fn menu_item(
    icon_svg: &str,
    label: &str,
    shortcut: Option<&str>,
    danger: bool,
) -> Division {
    let icon_svg = icon_svg.to_owned();
    let label = label.to_owned();
    let text_class = if danger {
        "text-negative"
    } else {
        "text-ink-700"
    };
    let hover_class = if danger {
        "hover:bg-cat-pink hover:text-cat-pinkInk"
    } else {
        "hover:bg-surfaceMuted hover:text-ink-900"
    };
    let class = format!(
        "flex items-center gap-2.5 px-3 h-8 rounded-md {text_class} text-[13px] {hover_class} cursor-pointer w-full"
    );
    let mut div = Division::builder();
    div.button(|b| {
        let mut b = b.type_("button").class(class).text(icon_svg).text(label);
        if let Some(hint) = shortcut {
            let hint = hint.to_owned();
            b = b.span(|s| s.class("ml-auto mono text-[11px] text-ink-400").text(hint));
        }
        b
    });
    div.build()
}

#[allow(dead_code)]
/// Render a dropdown menu container with items.
pub(crate) fn dropdown_menu(items_html: &str) -> Division {
    let items_html = items_html.to_owned();
    Division::builder()
        .class("w-48 bg-surface rounded-lg border border-line shadow-tooltip p-1 text-[13px]")
        .text(items_html)
        .build()
}

/// Render this section.
pub(crate) fn render(section_id: &str, num: &str, title: &str, desc: &str) -> String {
    let content = Division::builder()
        .class("p-12 bg-canvas border border-line rounded-lg flex items-start justify-center")
        .division(|menu| {
            menu.class("w-56 rounded-md bg-surface border border-line shadow-tooltip py-1 text-[13px]")
                .division(|lbl| {
                    lbl.class("px-3 py-1.5 text-[11px] mono uppercase tracking-wider text-ink-400")
                        .text("Aenean")
                })
                .button(|b| {
                    b.class("w-full text-left px-3 h-8 hover:bg-surfaceMuted flex items-center gap-2")
                        .text(SVG_EDIT)
                        .text(" Edit lorem")
                })
                .button(|b| {
                    b.class("w-full text-left px-3 h-8 hover:bg-surfaceMuted flex items-center gap-2")
                        .text(SVG_COPY)
                        .text(" Duplicate")
                })
                .division(|sep| sep.class("my-1 border-t border-lineSoft"))
                .button(|b| {
                    b.class("w-full text-left px-3 h-8 hover:bg-surfaceMuted flex items-center gap-2")
                        .text(SVG_SHARE)
                        .text(" Share")
                        .span(|s| s.class("ml-auto text-[11px] mono text-ink-400").text("\u{2318}S"))
                })
                .division(|sep| sep.class("my-1 border-t border-lineSoft"))
                .button(|b| {
                    b.class("w-full text-left px-3 h-8 hover:bg-surfaceMuted flex items-center gap-2 text-negative")
                        .text(SVG_DELETE)
                        .text(" Delete")
                })
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
            "dropdown",
            "16",
            "Dropdown",
            "Floating menu on white. 1px gray border + tooltip-grade shadow. Section dividers separate logical groups.",
        )));
    }
}
