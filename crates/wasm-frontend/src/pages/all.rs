//! All packages listing page.

// r[impl frontend.pages.all]

use html::inline_text::Anchor;
use html::text_content::Division;
use wasm_meta_registry_client::KnownPackage;

use crate::api_client::ApiClient;
use crate::layout;

/// Fetch all packages and render a paginated list.
pub(crate) async fn render(client: &ApiClient) -> String {
    let packages = client.fetch_all_packages(0, 100).await;

    let mut body = Division::builder();

    // Page header with count
    body.division(|div| {
        div.class("flex items-baseline justify-between pb-6 border-b border-gray-200 mb-6")
            .heading_1(|h1| {
                h1.class("text-3xl font-bold tracking-tight")
                    .text("All Packages")
            })
            .span(|s| {
                s.class("text-sm text-gray-400")
                    .text(format!("{} packages", packages.len()))
            })
    });

    if packages.is_empty() {
        body.division(|div| {
            div.class("py-16 text-center").paragraph(|p| {
                p.class("text-gray-500")
                    .text("No packages found. The registry may still be syncing.")
            })
        });
    } else {
        // Table-style header
        body.division(|div| {
            div.class("hidden sm:flex items-baseline gap-3 px-2 pb-2 text-xs text-gray-400 uppercase tracking-wide")
                .span(|s| s.class("w-48 shrink-0").text("Package"))
                .span(|s| s.class("w-20 shrink-0").text("Version"))
                .span(|s| s.text("Description"))
        });

        let mut list = Division::builder();
        list.class("divide-y divide-gray-100");
        for pkg in &packages {
            list.push(render_row(pkg));
        }
        body.push(list.build());
    }

    layout::document("All Packages", &body.build().to_string())
}

/// Render a single package row.
fn render_row(pkg: &KnownPackage) -> Anchor {
    let display_name = match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) => format!("{ns}:{name}"),
        _ => pkg.repository.clone(),
    };

    let href = match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) => format!("/{ns}/{name}"),
        _ => "#".to_string(),
    };

    let description = pkg.description.as_deref().unwrap_or("");

    let version = pkg.tags.first().map_or("—", String::as_str);

    Anchor::builder()
        .href(href)
        .class(
            "flex items-baseline gap-3 py-3 hover:bg-gray-50 -mx-2 px-2 rounded transition-colors",
        )
        .span(|s| {
            s.class("w-48 shrink-0 font-semibold text-accent truncate")
                .text(display_name)
        })
        .span(|s| {
            s.class("w-20 shrink-0 text-sm text-gray-400")
                .text(version.to_owned())
        })
        .span(|s| {
            s.class("text-sm text-gray-500 truncate")
                .text(description.to_owned())
        })
        .build()
}
