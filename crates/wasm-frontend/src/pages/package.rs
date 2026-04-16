//! Package detail page.

// r[impl frontend.pages.package-detail]

use crate::wit_doc::WitDocument;
use html::content::Section;
use html::text_content::{Division, ListItem, UnorderedList};
use wasm_meta_registry_client::{KnownPackage, PackageVersion};

use super::package_shell;

/// Render the package detail page for a given package and version.
#[must_use]
pub(crate) fn render(
    pkg: &KnownPackage,
    version: &str,
    version_detail: Option<&PackageVersion>,
    importers: &[KnownPackage],
    exporters: &[KnownPackage],
) -> String {
    let display_name = package_shell::display_name_for(pkg);
    let url_base = package_shell::url_base_for(pkg, version);
    let wit_doc = version_detail.and_then(|d| try_parse_wit(d, &url_base));

    // Package heading
    let kind_label = match pkg.kind {
        Some(wasm_meta_registry_client::PackageKind::Interface) => "Interface Types",
        Some(wasm_meta_registry_client::PackageKind::Component) => "Component",
        _ => "Package",
    };
    let pkg_name = pkg.wit_name.as_deref().unwrap_or(&display_name);

    let docs_md = pkg
        .description
        .as_deref()
        .map(|desc| crate::markdown::render_block(desc, crate::markdown::DOC_CLASS))
        .unwrap_or_default();

    let copy_icon = "<svg xmlns='http://www.w3.org/2000/svg' width='14' height='14' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><rect x='9' y='9' width='13' height='13' rx='2' ry='2'/><path d='M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1'/></svg>";
    let check_icon = "<svg xmlns='http://www.w3.org/2000/svg' width='14' height='14' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><polyline points='20 6 9 17 4 12'/></svg>";

    let header = format!(
        r#"<div class="max-w-3xl mb-6">
  <h2 class="text-3xl font-light tracking-display font-display flex items-baseline gap-2 group">
    <span class="text-accent">{pkg_name}</span>
    <button id="copy-fqn-btn" class="text-fg-faint hover:text-fg transition-opacity cursor-pointer opacity-0 group-hover:opacity-100" style="font-size:0.5em;vertical-align:middle" title="Copy item path to clipboard">{copy_icon}</button>
  </h2>
  <span class="text-sm text-fg-muted mt-1 block">{kind_label}</span>
  <div class="mt-4">{docs_md}</div>
</div>
<script>
(function(){{
  var btn=document.getElementById('copy-fqn-btn');
  var copyIcon="{copy_icon}";
  var checkIcon="{check_icon}";
  btn.addEventListener('click',function(){{
    navigator.clipboard.writeText('{display_name}').then(function(){{
      btn.innerHTML=checkIcon;
      setTimeout(function(){{btn.innerHTML=copyIcon}},2000);
    }});
  }});
}})();
</script>"#,
    );

    let wit_content = if let Some(detail) = version_detail {
        render_wit_content_with_doc(detail, &url_base, wit_doc.as_ref(), pkg, version).to_string()
    } else {
        String::new()
    };

    let body_html =
        format!("{header}<div class=\"space-y-10 max-w-3xl pt-4 pb-12\">{wit_content}</div>");

    let shell_ctx = package_shell::SidebarContext {
        pkg,
        version,
        version_detail,
        importers,
        exporters,
    };
    package_shell::render_page(&shell_ctx, &display_name, &body_html)
}

/// Render the WIT content section for a package version.
///
/// When a pre-parsed `WitDocument` is available, show interfaces and worlds
/// as navigable cards.  Otherwise fall back to the world summaries that the
/// registry extracted at index time plus the raw WIT text block.
fn render_wit_content_with_doc(
    detail: &PackageVersion,
    _url_base: &str,
    doc: Option<&WitDocument>,
    pkg: &KnownPackage,
    version: &str,
) -> Section {
    let mut section = Section::builder();
    section.class("space-y-10");

    if let Some(doc) = doc {
        if !doc.worlds.is_empty() {
            section.push(render_world_overview(doc));
        }
        if !doc.interfaces.is_empty() {
            section.push(render_interface_overview(doc));
        }
    } else {
        // Fallback: prefer component-level imports/exports (from wasm-metadata,
        // which include docs) over world summaries (from DB, no docs).
        let has_component_imports = detail
            .components
            .iter()
            .any(|c| !c.imports.is_empty() || !c.exports.is_empty());

        if has_component_imports {
            for comp in &detail.components {
                if !comp.imports.is_empty() {
                    section.push(render_iface_ref_list("Imports", &comp.imports));
                }
                if !comp.exports.is_empty() {
                    section.push(render_iface_ref_list("Exports", &comp.exports));
                }
            }
        } else if !detail.worlds.is_empty() {
            section.push(render_world_summaries(detail));
        }

        // Only show the raw WIT text if it's genuine WIT (not lossy
        // debug output that contains patterns like `type foo: "type"`
        // or `interface-Id { idx: 0 }`).
        if let Some(wit_text) = &detail.wit_text
            && !is_lossy_wit(wit_text)
        {
            section.push(render_raw_wit(wit_text));
        }
    }

    // Component children: list modules and nested components as navigable sections.
    for comp in &detail.components {
        let url_base = package_shell::url_base_for(pkg, version);

        // Modules section
        let modules: Vec<&wasm_meta_registry_client::ComponentSummary> = comp
            .children
            .iter()
            .filter(|ch| ch.kind.as_deref() == Some("module"))
            .collect();
        if !modules.is_empty() {
            section.push(render_children_overview(
                "Modules", &modules, &url_base, "module",
            ));
        }

        // Nested components section
        let components: Vec<&wasm_meta_registry_client::ComponentSummary> = comp
            .children
            .iter()
            .filter(|ch| ch.kind.as_deref() == Some("component"))
            .collect();
        if !components.is_empty() {
            section.push(render_children_overview(
                "Components",
                &components,
                &url_base,
                "component",
            ));
        }

        // Root toolchain
        if !comp.producers.is_empty() {
            section.push(render_producers(&comp.producers));
        }
    }

    section.build()
}

/// Render a section listing child modules or components as navigable links.
fn render_children_overview(
    heading: &str,
    children: &[&wasm_meta_registry_client::ComponentSummary],
    url_base: &str,
    kind: &str,
) -> Division {
    let mut div = Division::builder();
    div.heading_2(|h2| {
        h2.class("text-lg font-medium text-fg-muted mb-3 pb-2 border-b border-border")
            .text(heading.to_owned())
    });

    let mut ul = UnorderedList::builder();
    for (i, child) in children.iter().enumerate() {
        let fallback = format!("{kind}[{i}]");
        let name = child.name.as_deref().unwrap_or(&fallback);
        let href = if kind == "module" {
            format!("{url_base}/module/{name}")
        } else {
            format!("{url_base}/component/{i}")
        };
        let size = child.size_bytes.map(format_size).unwrap_or_default();

        let link_class = if kind == "component" {
            "font-mono text-base font-medium text-wit-world hover:underline"
        } else {
            "font-mono text-base font-medium text-wit-iface hover:underline"
        };

        ul.list_item(|li| {
            li.class("py-1 flex justify-between");
            li.anchor(|a| {
                a.href(href)
                    .class(link_class.to_owned())
                    .text(name.to_owned())
            });
            if !size.is_empty() {
                li.span(|s| s.class("text-sm text-fg-muted").text(size));
            }
            li
        });
    }
    div.push(ul.build());
    div.build()
}

/// Try parsing the WIT text into a rich document model.
fn try_parse_wit(detail: &PackageVersion, url_base: &str) -> Option<WitDocument> {
    let wit_text = detail.wit_text.as_deref()?;
    let dep_urls = build_dep_urls(&detail.dependencies);
    crate::wit_doc::parse_wit_doc(wit_text, url_base, &dep_urls).ok()
}

/// Build the `dep_urls` mapping from a package's declared dependencies.
///
/// Maps `"namespace:name"` → `"/namespace/name/version"` for each
/// dependency that has a version.
fn build_dep_urls(
    deps: &[wasm_meta_registry_client::PackageDependencyRef],
) -> std::collections::HashMap<String, String> {
    deps.iter()
        .filter_map(|dep| {
            let version = dep.version.as_deref()?;
            let url = format!("/{}/{version}", dep.package.replace(':', "/"));
            Some((dep.package.clone(), url))
        })
        .collect()
}

/// Render the interfaces overview section.
fn render_interface_overview(doc: &WitDocument) -> Division {
    let mut container = Division::builder();
    container.class("space-y-1");
    container.heading_2(|h2| {
        h2.class("text-lg font-medium text-fg-muted mb-2 pb-2 border-b border-border")
            .text("Interfaces")
    });

    let mut ul = UnorderedList::builder();
    ul.class("");
    for iface in &doc.interfaces {
        ul.push(render_interface_row(iface));
    }
    container.push(ul.build());
    container.build()
}

/// Render a single interface row: linked name + doc excerpt.
fn render_interface_row(iface: &crate::wit_doc::InterfaceDoc) -> ListItem {
    let mut li = ListItem::builder();
    li.class("py-2 flex gap-4");

    li.division(|left| {
        left.class("shrink-0 w-44").anchor(|a| {
            a.href(iface.url.clone())
                .class("font-mono text-base font-medium text-wit-iface hover:underline")
                .text(iface.name.clone())
        })
    });

    // Right: doc excerpt
    if let Some(docs) = &iface.docs {
        li.division(|right| {
            right
                .class("text-base leading-relaxed text-fg-secondary min-w-0")
                .text(crate::markdown::render_inline(&first_sentence(docs)))
        });
    }

    li.build()
}

/// Render the worlds overview section.
fn render_world_overview(doc: &WitDocument) -> Division {
    let mut container = Division::builder();
    container.class("space-y-1");
    container.heading_2(|h2| {
        h2.class("text-lg font-medium text-fg-muted mb-2 pb-2 border-b border-border")
            .text("Worlds")
    });

    let mut ul = UnorderedList::builder();
    ul.class("");
    for world in &doc.worlds {
        ul.push(render_world_row(world));
    }
    container.push(ul.build());
    container.build()
}

/// Render a single world row: linked name + doc excerpt.
fn render_world_row(world: &crate::wit_doc::WorldDoc) -> ListItem {
    let mut li = ListItem::builder();
    li.class("py-1 flex gap-4");

    li.division(|left| {
        left.class("shrink-0 w-44").anchor(|a| {
            a.href(world.url.clone())
                .class("font-mono text-base font-medium text-wit-world hover:underline")
                .text(world.name.clone())
        })
    });

    // Right: doc excerpt
    if let Some(docs) = &world.docs {
        li.division(|right| {
            right
                .class("text-base leading-relaxed text-fg-secondary min-w-0")
                .text(crate::markdown::render_inline(&first_sentence(docs)))
        });
    }

    li.build()
}

/// Render raw WIT text in a pre-formatted code block (fallback).
fn render_raw_wit(wit_text: &str) -> Division {
    Division::builder()
        .heading_2(|h2| {
            h2.class("text-lg font-medium text-fg-muted mb-3")
                .text("WIT Definition")
        })
        .push(
            html::text_content::PreformattedText::builder()
                .class("border-2 border-fg p-4 overflow-x-auto text-base leading-relaxed")
                .code(|code| code.class("text-fg").text(wit_text.to_owned()))
                .build(),
        )
        .build()
}

/// Render world summaries from pre-extracted `PackageVersion` data (fallback
/// when the WIT text cannot be parsed into a rich document).
fn render_world_summaries(detail: &PackageVersion) -> Division {
    let mut container = Division::builder();
    container.class("space-y-8");

    for world in &detail.worlds {
        container.division(|world_div| {
            if world.name != "root" {
                world_div.heading_2(|h2| {
                    h2.class("text-lg font-medium text-fg-muted mb-3")
                        .text(format!("world {}", world.name))
                });
            }

            if let Some(desc) = &world.description {
                world_div.paragraph(|p| {
                    p.class("text-fg-secondary text-base mb-3")
                        .text(crate::markdown::render_inline(desc))
                });
            }

            if !world.imports.is_empty() {
                world_div.push(render_iface_ref_list("Imports", &world.imports));
            }
            if !world.exports.is_empty() {
                world_div.push(render_iface_ref_list("Exports", &world.exports));
            }
            world_div
        });
    }

    container.build()
}

/// Render a list of WIT interface references (fallback), styled like world
/// imports/exports with clickable links. Includes version to disambiguate
/// duplicates.
fn render_iface_ref_list(
    label: &str,
    interfaces: &[wasm_meta_registry_client::WitInterfaceRef],
) -> Division {
    let items: Vec<package_shell::ImportExportEntry> = interfaces
        .iter()
        .map(package_shell::iface_ref_to_entry)
        .collect();

    let mut div = Division::builder();
    div.class("mb-4");
    div.push(package_shell::render_import_export_section(label, &items));
    div.build()
}

/// Format a byte size into a human-readable string.
pub(crate) fn format_size(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = 1024 * KIB;
    #[allow(clippy::cast_precision_loss)]
    match bytes {
        b if b >= MIB => format!("{:.1} MiB", b as f64 / MIB as f64),
        b if b >= KIB => format!("{:.1} KiB", b as f64 / KIB as f64),
        b => format!("{b} B"),
    }
}

/// Render producer toolchain entries as a list.
fn render_producers(producers: &[wasm_meta_registry_client::ProducerEntry]) -> Division {
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
    div.build()
}

/// Extract the first sentence from a doc comment for summary display.
fn first_sentence(text: &str) -> String {
    text.split_once("\n\n").map_or_else(
        || text.trim().to_owned(),
        |(first, _)| first.trim().to_owned(),
    )
}

/// Detect whether WIT text is the lossy hand-rolled format rather than
/// genuine parseable WIT.  The lossy format contains debug patterns like
/// `type foo: "type"` and `interface-Id { idx: 0 }`.
fn is_lossy_wit(text: &str) -> bool {
    text.contains(": \"type\"")
        || text.contains(": \"record\"")
        || text.contains(": \"variant\"")
        || text.contains("interface-Id {")
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_meta_registry_client::PackageDependencyRef;

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
            wit_namespace: Some("wasi".to_string()),
            wit_name: Some("demo".to_string()),
            dependencies: vec![PackageDependencyRef {
                package: "wasi:io".to_string(),
                version: Some("0.2.0".to_string()),
            }],
        }
    }

    #[test]
    fn dependency_versions_shown_in_sidebar() {
        let pkg = sample_pkg();
        let html = render(&pkg, "1.0.0", None, &[], &[]);
        assert!(html.contains("wasi:io"));
        assert!(html.contains("@0.2.0"));
    }
}
