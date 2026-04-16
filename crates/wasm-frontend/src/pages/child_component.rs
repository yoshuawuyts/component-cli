//! Detail page for a child module or component inside a Wasm component.

use html::text_content::{Division, UnorderedList};
use wasm_meta_registry_client::{ComponentSummary, KnownPackage, PackageVersion};

use super::package_shell;

/// Render the detail page for a child module or component.
#[must_use]
pub(crate) fn render(
    pkg: &KnownPackage,
    version: &str,
    version_detail: Option<&PackageVersion>,
    child: &ComponentSummary,
    display_name: &str,
) -> String {
    let pkg_display = package_shell::display_name_for(pkg);
    let kind = child.kind.as_deref().unwrap_or("module");
    let title = format!("{pkg_display} \u{2014} {display_name}");

    // Header
    let kind_color = if kind == "component" {
        "text-wit-world"
    } else {
        "text-wit-iface"
    };

    let size_text = child
        .size_bytes
        .map(super::package::format_size)
        .unwrap_or_default();

    let subtitle = if size_text.is_empty() {
        kind.to_owned()
    } else {
        format!("{kind} ({size_text})")
    };

    let header = format!(
        r#"<div class="max-w-3xl mb-6">
  <h2 class="text-3xl font-light tracking-display font-display">
    <span class="{kind_color}">{display_name}</span>
  </h2>
  <span class="text-sm text-fg-muted mt-1 block">{subtitle}</span>
</div>"#,
    );

    let mut body = format!("{header}<div class=\"space-y-10 max-w-3xl pt-4 pb-12\">");

    // Producers
    if !child.producers.is_empty() {
        body.push_str(&render_producers_section(&child.producers));
    }

    // Bill of materials
    if !child.bill_of_materials.is_empty() {
        body.push_str(&render_bom_section(&child.bill_of_materials));
    }

    body.push_str("</div>");

    let ctx = package_shell::SidebarContext {
        pkg,
        version,
        version_detail,
        importers: &[],
        exporters: &[],
    };
    package_shell::render_page_with_crumbs(&ctx, &title, &body, &[])
}

/// Render producers as a list (matching package page style).
fn render_producers_section(producers: &[wasm_meta_registry_client::ProducerEntry]) -> String {
    let mut div = Division::builder();
    div.heading_2(|h2| {
        h2.class("text-lg font-medium text-fg-muted mb-3 pb-2 border-b border-border")
            .text("Producers")
    });

    let mut ul = UnorderedList::builder();
    for entry in producers {
        let name = entry.name.clone();
        let version = entry.version.clone();
        let field = entry.field.clone();
        let tooltip = if version.is_empty() {
            name.clone()
        } else {
            format!("{name} {version}")
        };
        ul.list_item(|li| {
            li.class("py-1 flex justify-between gap-4");
            li.span(|s| {
                s.class("font-mono text-base min-w-0 truncate")
                    .title(tooltip);
                s.span(|n| n.class("text-accent").text(name));
                if !version.is_empty() {
                    s.span(|v| v.class("text-fg-muted ml-1").text(version));
                }
                s
            });
            li.span(|s| {
                s.class("text-sm text-fg-muted shrink-0 whitespace-nowrap")
                    .text(field)
            });
            li
        });
    }
    div.push(ul.build());
    div.build().to_string()
}

/// Render dependencies as a list with links to crates.io.
fn render_bom_section(deps: &[wasm_meta_registry_client::BomEntry]) -> String {
    let mut div = Division::builder();
    div.heading_2(|h2| {
        h2.class("text-lg font-medium text-fg-muted mb-3 pb-2 border-b border-border")
            .text(format!("Dependencies ({})", deps.len()))
    });

    let mut ul = UnorderedList::builder();
    for dep in deps {
        let name = dep.name.clone();
        let version = dep.version.clone();
        let href = format!("https://crates.io/crates/{name}");
        ul.list_item(|li| {
            li.class("py-1 flex justify-between");
            li.anchor(|a| {
                a.href(href)
                    .class("font-mono text-base text-accent hover:underline")
                    .text(name)
            });
            li.span(|s| s.class("font-mono text-sm text-fg-muted").text(version));
            li
        });
    }
    div.push(ul.build());
    div.build().to_string()
}
