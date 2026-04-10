//! Footer component.

use html::content::Footer;

/// Render the site footer.
#[must_use]
pub(crate) fn render() -> String {
    Footer::builder()
        .class("mt-16")
        .division(|div| {
            div.class("max-w-6xl mx-auto px-4 py-8 text-sm text-fg-muted")
                .paragraph(|p| {
                    p.text("\u{00a9} 2025 wasm registry")
                })
        })
        .build()
        .to_string()
}
