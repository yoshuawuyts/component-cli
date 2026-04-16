//! World detail page.

use crate::wit_doc::{WitDocument, WorldDoc, WorldItemDoc};
use html::text_content::{Division, ListItem, UnorderedList};
use wasm_meta_registry_client::{KnownPackage, PackageVersion};

use super::package_shell;

/// Render the world detail page.
#[must_use]
pub(crate) fn render(
    pkg: &KnownPackage,
    version: &str,
    version_detail: Option<&PackageVersion>,
    world: &WorldDoc,
    _doc: &WitDocument,
) -> String {
    let display_name = package_shell::display_name_for(pkg);
    let title = format!("{display_name} \u{2014} {}", world.name);

    let docs_md = world
        .docs
        .as_deref()
        .map(|d| crate::markdown::render_block(d, crate::markdown::DOC_CLASS))
        .unwrap_or_default();

    let fqn = format!("{display_name}/{}", world.name);
    let copy_icon = "<svg xmlns='http://www.w3.org/2000/svg' width='14' height='14' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><rect x='9' y='9' width='13' height='13' rx='2' ry='2'/><path d='M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1'/></svg>";
    let check_icon = "<svg xmlns='http://www.w3.org/2000/svg' width='14' height='14' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><polyline points='20 6 9 17 4 12'/></svg>";

    let header = format!(
        r#"<div class="max-w-3xl mb-6">
  <h2 class="text-3xl font-light tracking-display font-display flex items-baseline gap-2 group">
    <span class="text-wit-world">{world_name}</span>
    <button id="copy-fqn-btn" class="text-fg-faint hover:text-fg transition-opacity cursor-pointer opacity-0 group-hover:opacity-100" style="font-size:0.5em;vertical-align:middle" title="Copy item path to clipboard">{copy_icon}</button>
  </h2>
  <span class="text-sm text-fg-muted mt-1 block">World</span>
  <div class="mt-4">{docs_md}</div>
</div>
<script>
(function(){{
  var btn=document.getElementById('copy-fqn-btn');
  var copyIcon="{copy_icon}";
  var checkIcon="{check_icon}";
  btn.addEventListener('click',function(){{
    navigator.clipboard.writeText('{fqn}').then(function(){{
      btn.innerHTML=checkIcon;
      setTimeout(function(){{btn.innerHTML=copyIcon}},2000);
    }});
  }});
}})();
</script>"#,
        world_name = world.name,
    );

    let mut content = Division::builder();
    content.class("space-y-10 max-w-3xl");

    // Build a doc lookup from the API's enriched world data (has cross-package docs).
    let api_docs = build_api_doc_lookup(version_detail, &world.name);

    if !world.imports.is_empty() {
        content.push(render_item_section(
            "Imports",
            &world.imports,
            true,
            &api_docs,
        ));
    }
    if !world.exports.is_empty() {
        content.push(render_item_section(
            "Exports",
            &world.exports,
            false,
            &api_docs,
        ));
    }

    let body_html = format!("{header}{}", content.build());

    let ctx = package_shell::SidebarContext {
        pkg,
        version,
        version_detail,
        importers: &[],
        exporters: &[],
    };
    package_shell::render_page_with_crumbs(&ctx, &title, &body_html, &[])
}

/// Build a lookup map of interface name → doc string from the API's enriched
/// world data. This provides cross-package docs that the WIT parser can't.
fn build_api_doc_lookup(
    version_detail: Option<&PackageVersion>,
    world_name: &str,
) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    let Some(detail) = version_detail else {
        return map;
    };
    for world in &detail.worlds {
        if world.name != world_name {
            continue;
        }
        for iface in world.imports.iter().chain(world.exports.iter()) {
            if let Some(docs) = &iface.docs {
                let mut key = iface.package.clone();
                if let Some(name) = &iface.interface {
                    key.push('/');
                    key.push_str(name);
                }
                map.insert(key, docs.clone());
            }
        }
    }
    map
}

/// Render an imports or exports section, grouped by package namespace.
fn render_item_section(
    heading: &str,
    items: &[WorldItemDoc],
    is_import: bool,
    api_docs: &std::collections::HashMap<String, String>,
) -> Division {
    // Separate interface items (shared rendering) from non-interface items
    let mut iface_entries: Vec<package_shell::ImportExportEntry> = Vec::new();
    let mut other_items: Vec<&WorldItemDoc> = Vec::new();

    for item in items {
        match item {
            WorldItemDoc::Interface { name, url, docs } => {
                // Use WIT-parsed docs first, fall back to API-enriched docs.
                let name_no_ver = strip_version(name);
                let effective_docs = docs.clone().or_else(|| api_docs.get(name_no_ver).cloned());
                iface_entries.push(package_shell::ImportExportEntry {
                    label: name.clone(),
                    url: url.clone(),
                    docs: effective_docs,
                    item_kind: package_shell::WorldItemKind::Interface,
                });
            }
            _ => other_items.push(item),
        }
    }

    // If everything is an interface, use the shared renderer directly.
    if other_items.is_empty() {
        return package_shell::render_import_export_section(heading, &iface_entries);
    }

    // Mixed content: render heading + interfaces via shared code, then
    // append functions/types with custom rendering.
    let mut div = Division::builder();
    if iface_entries.is_empty() {
        div.heading_2(|h2| {
            h2.class("text-lg font-medium text-fg-muted mb-3 pb-2 border-b border-border")
                .text(heading.to_owned())
        });
    } else {
        div.push(package_shell::render_import_export_section(
            heading,
            &iface_entries,
        ));
    }

    let mut ul = UnorderedList::builder();
    for item in other_items {
        ul.push(render_world_item_row(item));
    }
    div.push(ul.build());
    div.build()
}

/// Strip version suffix from a qualified name.
///
/// `"wasi:cli/environment@0.2.11"` → `"wasi:cli/environment"`
fn strip_version(name: &str) -> &str {
    name.split('@').next().unwrap_or(name)
}

/// Render a single world item row.
fn render_world_item_row(item: &WorldItemDoc) -> ListItem {
    let mut li = ListItem::builder();
    li.class("py-1");

    match item {
        WorldItemDoc::Interface {
            name,
            url: Some(url),
            ..
        } => {
            li.anchor(|a| {
                a.href(url.clone())
                    .class("block font-mono text-wit-iface hover:underline text-base")
                    .text(name.to_owned())
            });
        }
        WorldItemDoc::Interface {
            name, url: None, ..
        } => {
            li.span(|s| {
                s.class("block font-mono text-fg text-base")
                    .text(name.to_owned())
            });
        }
        WorldItemDoc::Function(func) => {
            let sig = format_function_signature(func);
            li.code(|c| c.class("block font-mono text-base text-wit-func").text(sig));
            if let Some(docs) = &func.docs {
                li.paragraph(|p| {
                    p.class("text-base text-fg-secondary mt-1")
                        .text(crate::markdown::render_inline(&first_sentence(docs)))
                });
            }
        }
        WorldItemDoc::Type(ty) => {
            li.span(|s| {
                s.class("block font-mono text-base")
                    .span(|s2| s2.class("text-fg-muted").text("type "))
                    .span(|s2| s2.class("text-accent").text(ty.name.clone()))
            });
            if let Some(docs) = &ty.docs {
                li.paragraph(|p| {
                    p.class("text-base text-fg-secondary mt-1")
                        .text(crate::markdown::render_inline(&first_sentence(docs)))
                });
            }
        }
    }

    li.build()
}

/// Format a function signature.
fn format_function_signature(func: &crate::wit_doc::FunctionDoc) -> String {
    let params: Vec<String> = func
        .params
        .iter()
        .filter(|p| p.name != "self")
        .map(|p| format!("{}: {}", p.name, format_type_ref_short(&p.ty)))
        .collect();
    let ret = func
        .result
        .as_ref()
        .map(|r| format!(" -> {}", format_type_ref_short(r)))
        .unwrap_or_default();
    format!("{}({}){ret}", func.name, params.join(", "))
}

/// Format a `TypeRef` as a short inline string.
fn format_type_ref_short(ty: &crate::wit_doc::TypeRef) -> String {
    match ty {
        crate::wit_doc::TypeRef::Primitive { name }
        | crate::wit_doc::TypeRef::Named { name, .. } => name.clone(),
        crate::wit_doc::TypeRef::List { ty } => {
            format!("list<{}>", format_type_ref_short(ty))
        }
        crate::wit_doc::TypeRef::Option { ty } => {
            format!("option<{}>", format_type_ref_short(ty))
        }
        crate::wit_doc::TypeRef::Result { ok, err } => {
            let ok_s = ok
                .as_ref()
                .map_or_else(|| "_".to_owned(), |t| format_type_ref_short(t));
            let err_s = err
                .as_ref()
                .map_or_else(|| "_".to_owned(), |t| format_type_ref_short(t));
            format!("result<{ok_s}, {err_s}>")
        }
        crate::wit_doc::TypeRef::Tuple { types } => {
            let inner: Vec<String> = types.iter().map(format_type_ref_short).collect();
            format!("tuple<{}>", inner.join(", "))
        }
        crate::wit_doc::TypeRef::Handle {
            handle_kind,
            resource_name,
            ..
        } => match handle_kind {
            crate::wit_doc::HandleKind::Own => resource_name.clone(),
            crate::wit_doc::HandleKind::Borrow => format!("borrow<{resource_name}>"),
        },
        crate::wit_doc::TypeRef::Future { ty } => match ty {
            Some(t) => format!("future<{}>", format_type_ref_short(t)),
            None => "future".to_owned(),
        },
        crate::wit_doc::TypeRef::Stream { ty } => match ty {
            Some(t) => format!("stream<{}>", format_type_ref_short(t)),
            None => "stream".to_owned(),
        },
    }
}

/// Extract the first sentence from a doc comment.
fn first_sentence(text: &str) -> String {
    text.split_once("\n\n").map_or_else(
        || text.trim().to_owned(),
        |(first, _)| first.trim().to_owned(),
    )
}
