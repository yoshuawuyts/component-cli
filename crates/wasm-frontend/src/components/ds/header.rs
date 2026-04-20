//! Page header.

use html::content::Header;

/// The sun icon SVG (visible in dark mode).
const SUN_ICON: &str = r#"<svg class="theme-icon-sun h-3.5 w-3.5" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"><circle cx="8" cy="8" r="3" /><path d="M8 1.5v1.5M8 13v1.5M1.5 8h1.5M13 8h1.5M3.3 3.3l1.05 1.05M11.65 11.65l1.05 1.05M3.3 12.7l1.05-1.05M11.65 4.35l1.05-1.05" /></svg>"#;

/// The moon icon SVG (visible in light mode).
const MOON_ICON: &str = r#"<svg class="theme-icon-moon h-3.5 w-3.5" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M13.5 9.5A5.5 5.5 0 0 1 6.5 2.5a5.5 5.5 0 1 0 7 7Z" /></svg>"#;

/// Render the page header.
pub(crate) fn render() -> String {
    Header::builder()
        .class("pt-8 md:pt-12 pb-8 md:pb-12")
        .division(|div| {
            div.class("flex items-center gap-2 text-[12px] text-ink-500 mono uppercase tracking-wider")
                .span(|s| s.text("v1.0"))
                .span(|s| s.class("h-1 w-1 rounded-full bg-ink-300"))
                .span(|s| s.text("Foundations \u{00b7} Components \u{00b7} Patterns"))
                .span(|s| s.class("ml-auto"))
                .button(|btn| {
                    btn.id("theme-toggle".to_owned())
                        .type_("button")
                        .aria_label("Toggle color theme".to_owned())
                        .title("Toggle color theme".to_owned())
                        .class("inline-flex items-center gap-1.5 h-7 px-2.5 rounded-md border border-line bg-surface text-ink-700 hover:bg-surfaceMuted hover:text-ink-900 transition-colors")
                        .text(format!("{SUN_ICON}{MOON_ICON}"))
                        .span(|s| s.class("theme-label text-[11px]").text("Theme"))
                })
        })
        .heading_1(|h1| {
            h1.class("mt-3 text-[36px] md:text-[44px] leading-[1.05] font-semibold tracking-tight")
                .text("Design System")
        })
        .paragraph(|p| {
            p.class("mt-3 max-w-2xl text-[15px] text-ink-700 leading-relaxed")
                .text("A quiet, data-forward visual language built around soft rules, neutral ink, and a categorical pastel palette. Optimized for dense dashboards and analytical interfaces.")
        })
        .build()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(render());
    }
}
