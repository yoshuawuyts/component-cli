//! Footer component.

use html::content::Footer;

/// Render the site footer.
#[must_use]
pub(crate) fn render() -> String {
    Footer::builder()
        .class("mt-8")
        .division(|div| {
            div.class("max-w-6xl mx-auto px-6 sm:px-8 py-4 text-sm text-fg-muted")
                .paragraph(|p| p.text("Made by Yosh Wuyts"))
        })
        .build()
        .to_string()
}
