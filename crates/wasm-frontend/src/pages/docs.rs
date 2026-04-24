//! Documentation page.

use html::text_content::Division;

use crate::layout;

/// Render the documentation page.
#[must_use]
pub(crate) fn render() -> String {
    let body = Division::builder()
        .class("pt-8 max-w-[65ch]")
        .heading_1(|h1| {
            h1.class(format!("{} mb-6", crate::components::ds::typography::H1_CLASS))
                .text("Documentation")
        })
        .paragraph(|p| {
            p.class(crate::components::ds::typography::BODY_CLASS)
                .text("Documentation is coming soon.")
        })
        .heading_2(|h2| {
            h2.class(crate::components::ds::typography::H2_CLASS)
                .text("About")
        })
        .paragraph(|p| {
            p.class(crate::components::ds::typography::BODY_CLASS)
                .text("The WebAssembly Package Registry is a discovery service for WebAssembly components and interfaces. It indexes packages from OCI registries and provides a browsable frontend for exploring the ecosystem.")
        })
        .paragraph(|p| {
            p.class(format!("{} mt-4", crate::components::ds::typography::BODY_CLASS))
                .text("This frontend is itself a WebAssembly component, compiled to ")
                .code(|c| {
                    c.class("bg-surfaceMuted px-1.5 py-0.5 text-[14px]")
                        .text("wasm32-wasip2")
                })
                .text(" and served via ")
                .code(|c| {
                    c.class("bg-surfaceMuted px-1.5 py-0.5 text-[14px]")
                        .text("wasi:http")
                })
                .text(".")
        })
        .build();

    layout::document_with_nav("Docs", &body.to_string())
}
