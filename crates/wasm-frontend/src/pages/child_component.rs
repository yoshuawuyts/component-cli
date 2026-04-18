//! Detail page for a child module or component inside a Wasm component.

use crate::components::section_group;
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
        "text-wit-module"
    };

    let size_text = child
        .size_bytes
        .map(super::package::format_size)
        .unwrap_or_default();

    let lang_text = if child.languages.is_empty() {
        String::new()
    } else {
        child.languages.join(", ")
    };

    let mut subtitle_parts = vec![kind.to_owned()];
    if !size_text.is_empty() {
        subtitle_parts.push(size_text);
    }
    if !lang_text.is_empty() {
        subtitle_parts.push(lang_text);
    }
    let subtitle = subtitle_parts.join(" \u{2014} ");

    let header = format!(
        r#"<div class="max-w-3xl mb-6">
  <h2 class="text-[28px] sm:text-[36px] font-semibold tracking-tight font-mono">
    <span class="{kind_color}">{display_name}</span>
  </h2>
  <span class="text-[13px] text-ink-500 mt-1 block">{subtitle}</span>
</div>"#,
    );

    let mut body = format!("{header}<div class=\"space-y-10 max-w-3xl pt-4 pb-12\">");

    // WIT imports
    if !child.imports.is_empty() {
        let entries: Vec<package_shell::ImportExportEntry> = child
            .imports
            .iter()
            .map(package_shell::iface_ref_to_entry)
            .collect();
        body.push_str(
            &package_shell::render_import_export_section("Imports", &entries).to_string(),
        );
    }

    // WIT exports
    if !child.exports.is_empty() {
        let entries: Vec<package_shell::ImportExportEntry> = child
            .exports
            .iter()
            .map(package_shell::iface_ref_to_entry)
            .collect();
        body.push_str(
            &package_shell::render_import_export_section("Exports", &entries).to_string(),
        );
    }

    // Producers
    if !child.producers.is_empty() {
        body.push_str(&render_producers_section(&child.producers));
    }

    // Dependencies
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
        nav_html: None,
    };
    package_shell::render_page_with_crumbs(&ctx, &title, &body, &[])
}

/// Render producers as a list, excluding language entries (shown in subtitle).
fn render_producers_section(producers: &[wasm_meta_registry_client::ProducerEntry]) -> String {
    // Filter out language entries — those are shown in the subtitle.
    let filtered: Vec<_> = producers.iter().filter(|e| e.field != "language").collect();
    if filtered.is_empty() {
        return String::new();
    }

    let mut div = Division::builder();
    div.push(section_group::header("Producers", filtered.len()));

    let mut ul = UnorderedList::builder();
    for entry in &filtered {
        let name = entry.name.clone();
        let version = entry.version.clone();
        // Strip parenthesized info from display, keep in tooltip.
        let display_version = version
            .split_once(" (")
            .map_or_else(|| version.clone(), |(before, _)| before.to_owned());
        let tooltip = if version.is_empty() {
            name.clone()
        } else {
            format!("{name} {version}")
        };
        ul.list_item(|li| {
            li.class("py-1");
            li.span(|s| {
                s.class("text-[14px] min-w-0 truncate").title(tooltip);
                s.span(|n| n.class("text-accent").text(name));
                if !display_version.is_empty() {
                    s.span(|v| {
                        v.class("text-ink-400 ml-1")
                            .text(format!("@{display_version}"))
                    });
                }
                s
            });
            li
        });
    }
    div.push(ul.build());
    div.build().to_string()
}

/// Render dependencies as package URLs with links to crates.io.
fn render_bom_section(deps: &[wasm_meta_registry_client::BomEntry]) -> String {
    let mut div = Division::builder();
    div.push(section_group::header("Dependencies", deps.len()));

    let mut ul = UnorderedList::builder();
    for dep in deps {
        let name = dep.name.clone();
        let version = dep.version.clone();
        let source = dep.source.as_deref().unwrap_or("crates.io");
        let (purl_type, href) = match source {
            "crates.io" | "registry" => (
                "cargo",
                Some(format!("https://crates.io/crates/{name}/{version}")),
            ),
            _ => ("generic", None),
        };
        let purl = format!("pkg:{purl_type}/{name}@{version}");
        ul.list_item(|li| {
            li.class("py-1");
            if let Some(url) = href {
                li.anchor(|a| {
                    a.href(url).class("text-[14px] hover:underline");
                    a.span(|s| s.class("text-ink-500").text(format!("pkg:{purl_type}/")));
                    a.span(|s| s.class("text-accent").text(name));
                    a.span(|s| s.class("text-ink-400 ml-1").text(format!("@{version}")));
                    a
                })
                .title(purl);
            } else {
                li.span(|s| {
                    s.class("text-[14px]");
                    s.span(|ps| ps.class("text-ink-500").text(format!("pkg:{purl_type}/")));
                    s.span(|ns| ns.class("text-ink-900").text(name));
                    s.span(|vs| vs.class("text-ink-400 ml-1").text(format!("@{version}")));
                    s
                })
                .title(purl);
            }
            li
        });
    }
    div.push(ul.build());
    div.build().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_meta_registry_client::{BomEntry, ProducerEntry, WitInterfaceRef};

    fn sample_pkg() -> KnownPackage {
        KnownPackage {
            registry: "ghcr.io".to_string(),
            repository: "example/pkg".to_string(),
            kind: None,
            description: None,
            tags: vec!["1.0.0".to_string()],
            signature_tags: vec![],
            attestation_tags: vec![],
            last_seen_at: "2026-01-01T00:00:00Z".to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            wit_namespace: Some("example".to_string()),
            wit_name: Some("pkg".to_string()),
            dependencies: vec![],
        }
    }

    fn sample_child(kind: &str) -> ComponentSummary {
        ComponentSummary {
            name: Some("child".into()),
            description: None,
            targets: vec![],
            producers: vec![
                ProducerEntry {
                    field: "language".into(),
                    name: "Rust".into(),
                    version: "1.82.0".into(),
                },
                ProducerEntry {
                    field: "processed-by".into(),
                    name: "wit-component".into(),
                    version: "0.220.0 (extra)".into(),
                },
            ],
            kind: Some(kind.into()),
            size_bytes: Some(4096),
            languages: vec!["Rust".into()],
            children: vec![],
            source: None,
            homepage: None,
            licenses: None,
            authors: None,
            revision: None,
            component_version: None,
            bill_of_materials: vec![
                BomEntry {
                    name: "serde".into(),
                    version: "1.0.0".into(),
                    source: Some("crates.io".into()),
                },
                BomEntry {
                    name: "custom".into(),
                    version: "0.1.0".into(),
                    source: Some("git".into()),
                },
            ],
            imports: vec![WitInterfaceRef {
                package: "wasi:io".into(),
                interface: Some("streams".into()),
                version: Some("0.2.0".into()),
                docs: None,
            }],
            exports: vec![WitInterfaceRef {
                package: "wasi:http".into(),
                interface: Some("incoming-handler".into()),
                version: Some("0.2.0".into()),
                docs: None,
            }],
        }
    }

    #[test]
    fn render_module_uses_module_color() {
        let pkg = sample_pkg();
        let child = sample_child("module");
        let html = render(&pkg, "1.0.0", None, &child, "inner");
        assert!(html.contains("text-wit-module"));
        assert!(html.contains("inner"));
        assert!(html.contains("Imports"));
        assert!(html.contains("Exports"));
        assert!(html.contains("Producers"));
        assert!(html.contains("Dependencies"));
    }

    #[test]
    fn render_component_uses_world_color() {
        let pkg = sample_pkg();
        let child = sample_child("component");
        let html = render(&pkg, "1.0.0", None, &child, "inner");
        assert!(html.contains("text-wit-world"));
    }

    #[test]
    fn render_empty_sections_are_skipped() {
        let pkg = sample_pkg();
        let mut child = sample_child("module");
        // Only language producers — filtered out of producers section.
        child.producers = vec![ProducerEntry {
            field: "language".into(),
            name: "Rust".into(),
            version: String::new(),
        }];
        child.bill_of_materials = vec![];
        child.imports = vec![];
        child.exports = vec![];
        child.languages = vec![];
        child.size_bytes = None;
        let html = render(&pkg, "1.0.0", None, &child, "inner");
        assert!(!html.contains(">Producers<"));
        assert!(!html.contains(">Dependencies<"));
    }
}
