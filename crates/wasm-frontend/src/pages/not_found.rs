//! 404 Not Found page.

// r[impl frontend.pages.not-found]

use html::text_content::Division;

use crate::components::ds::link_button;
use crate::layout;

/// Render a user-friendly 404 page.
#[must_use]
pub(crate) fn render() -> String {
    let body = Division::builder()
        .class("pt-16 pb-20 max-w-lg")
        .heading_1(|h1| {
            h1.class(
                "text-[24px] sm:text-[36px] font-semibold tracking-tight font-mono text-accent",
            )
            .text("Page not found")
        })
        .paragraph(|p| {
            p.class("text-ink-700 mt-3").text(
                "The package, interface, or item you're looking for \
                     doesn't exist — or it may have been published under \
                     a different version.",
            )
        })
        .division(|actions| {
            actions
                .class("mt-8 flex flex-wrap gap-3 text-[13px]")
                .push(link_button::render(
                    &link_button::Variant::Primary,
                    "/",
                    "Browse packages",
                ))
                .push(link_button::render(
                    &link_button::Variant::Outline,
                    "/search",
                    "Search",
                ))
        })
        .build();

    layout::document_with_nav("Not Found", &body.to_string())
}
