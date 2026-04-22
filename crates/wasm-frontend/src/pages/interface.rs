//! Interface detail page.

use crate::components::ds::sigil as s;
use crate::components::ds::{item_list, page_header, section_group};
use crate::wit_doc::{FunctionDoc, InterfaceDoc, TypeDoc, TypeKind, WitDocument};
use html::text_content::Division;
use wasm_meta_registry_client::{KnownPackage, PackageVersion};

use super::package_shell;

/// Render the interface detail page.
#[must_use]
pub(crate) fn render(
    pkg: &KnownPackage,
    version: &str,
    version_detail: Option<&PackageVersion>,
    iface: &InterfaceDoc,
    doc: &WitDocument,
) -> String {
    let display_name = package_shell::display_name_for(pkg);
    let title = format!("{display_name} — {}", iface.name);

    // Interface content — heading + docs in a two-column row

    let header_row = page_header::page_header_block(
        &format!("v{version} \u{00b7} Interface"),
        &iface.name,
        iface.docs.as_deref().unwrap_or("No description available."),
        None,
    )
    .to_string();

    // Grouped type and function sections
    let mut content = Division::builder();
    content.class("space-y-10 pt-8");
    let mut toc: Vec<(String, String)> = Vec::new();

    let resources: Vec<&TypeDoc> = iface
        .types
        .iter()
        .filter(|t| matches!(t.kind, TypeKind::Resource { .. }))
        .collect();
    let records: Vec<&TypeDoc> = iface
        .types
        .iter()
        .filter(|t| matches!(t.kind, TypeKind::Record { .. }))
        .collect();
    let variants: Vec<&TypeDoc> = iface
        .types
        .iter()
        .filter(|t| matches!(t.kind, TypeKind::Variant { .. }))
        .collect();
    let enums: Vec<&TypeDoc> = iface
        .types
        .iter()
        .filter(|t| matches!(t.kind, TypeKind::Enum { .. }))
        .collect();
    let flags: Vec<&TypeDoc> = iface
        .types
        .iter()
        .filter(|t| matches!(t.kind, TypeKind::Flags { .. }))
        .collect();
    let aliases: Vec<&TypeDoc> = iface
        .types
        .iter()
        .filter(|t| matches!(t.kind, TypeKind::Alias(_)))
        .collect();

    if !resources.is_empty() {
        toc.push(("#resources".to_owned(), "Resources".to_owned()));
        content.division(|d| {
            d.id("resources".to_owned())
                .push(render_type_section("Resources", &resources))
        });
    }
    if !records.is_empty() {
        toc.push(("#records".to_owned(), "Records".to_owned()));
        content.division(|d| {
            d.id("records".to_owned())
                .push(render_type_section("Records", &records))
        });
    }
    if !variants.is_empty() {
        toc.push(("#variants".to_owned(), "Variants".to_owned()));
        content.division(|d| {
            d.id("variants".to_owned())
                .push(render_type_section("Variants", &variants))
        });
    }
    if !enums.is_empty() {
        toc.push(("#enums".to_owned(), "Enums".to_owned()));
        content.division(|d| {
            d.id("enums".to_owned())
                .push(render_type_section("Enums", &enums))
        });
    }
    if !flags.is_empty() {
        toc.push(("#flags".to_owned(), "Flags".to_owned()));
        content.division(|d| {
            d.id("flags".to_owned())
                .push(render_type_section("Flags", &flags))
        });
    }
    if !aliases.is_empty() {
        toc.push(("#type-aliases".to_owned(), "Type Aliases".to_owned()));
        content.division(|d| {
            d.id("type-aliases".to_owned())
                .push(render_type_section("Type Aliases", &aliases))
        });
    }
    if !iface.functions.is_empty() {
        toc.push(("#functions".to_owned(), "Functions".to_owned()));
        content.division(|d| {
            d.id("functions".to_owned())
                .push(render_function_section(&iface.functions))
        });
    }

    let body_html = content.build().to_string();

    // Build "On this page" ToC
    let toc_html = if toc.is_empty() {
        None
    } else {
        use crate::components::ds::on_this_page::TocEntry;
        let links: Vec<TocEntry<'_>> = toc
            .iter()
            .map(|(href, label)| TocEntry {
                href: href.as_str(),
                label: label.as_str(),
                indent: false,
            })
            .collect();
        Some(crate::components::ds::on_this_page::on_this_page_nav(
            &links,
        ))
    };

    // Build nav card with interface items for the sidebar
    let nav = super::sidebar::render_sidebar(&super::sidebar::SidebarContext {
        display_name: &display_name,
        version,
        versions: &pkg.tags,
        doc,
        active: super::sidebar::SidebarActive::Interface(&iface.name),
        annotations: version_detail.and_then(|d| d.annotations.as_ref()),
        kind_label: package_shell::kind_label_for(pkg),
        description: pkg.description.as_deref(),
        registry: &pkg.registry,
        repository: &pkg.repository,
        digest: version_detail.map(|d| d.digest.as_str()),
    });

    let ctx = package_shell::SidebarContext {
        pkg,
        version,
        version_detail,
        importers: &[],
        exporters: &[],
        nav_html: Some(nav.to_string()),
    };
    package_shell::render_page_with_crumbs(
        &ctx,
        &title,
        &header_row,
        &body_html,
        &[],
        toc_html.as_deref(),
    )
}

/// Render a section of types grouped by kind.
fn render_type_section(heading: &str, types: &[&TypeDoc]) -> Division {
    let items: Vec<item_list::DynItemRow> = types
        .iter()
        .map(|ty| {
            let desc = ty
                .docs
                .as_deref()
                .map(|d| crate::markdown::render_inline(&first_sentence(d)))
                .unwrap_or_default();
            item_list::DynItemRow {
                sigil_bg: wit_kind_sigil_bg(&ty.kind).to_owned(),
                sigil_color: wit_kind_sigil_color(&ty.kind).to_owned(),
                sigil_text: wit_kind_sigil_text(&ty.kind).to_owned(),
                name: ty.name.clone(),
                href: ty.url.clone(),
                desc,
                meta: String::new(),
                deprecated: false,
                id: None,
            }
        })
        .collect();
    let mut div = Division::builder();
    div.push(item_list::render_dyn_item_list(heading, &items));
    div.build()
}

/// Render the freestanding functions section.
fn render_function_section(functions: &[FunctionDoc]) -> Division {
    let items: Vec<item_list::DynItemRow> = functions
        .iter()
        .map(|func| {
            let desc = func
                .docs
                .as_deref()
                .map(|d| crate::markdown::render_inline(&first_sentence(d)))
                .unwrap_or_default();
            item_list::DynItemRow {
                sigil_bg: s::FUNC.bg.to_owned(),
                sigil_color: s::FUNC.color.to_owned(),
                sigil_text: s::FUNC.text.to_owned(),
                name: func.name.clone(),
                href: func.url.clone(),
                desc,
                meta: String::new(),
                deprecated: false,
                id: None,
            }
        })
        .collect();
    let mut div = Division::builder();
    div.push(item_list::render_dyn_item_list("Functions", &items));
    div.build()
}

/// Convert a WIT stability to the component enum.
#[allow(dead_code)]
fn wit_stability(stability: &crate::wit_doc::Stability) -> section_group::Stability {
    match stability {
        crate::wit_doc::Stability::Stable { .. } => section_group::Stability::Stable,
        crate::wit_doc::Stability::Unstable { .. } => section_group::Stability::Unstable,
        crate::wit_doc::Stability::Unknown => section_group::Stability::Unknown,
    }
}

/// Get the CSS color class for a type kind.
///
/// Palette (OKLCH-based, same hue family as the design system):
/// - Records/Variants: blue-violet (hue 260) — structural data types
/// - Enums/Flags: teal (hue 180) — enumerable values
/// - Resources: amber (hue 70) — managed handles
/// - Aliases: default accent — pass-through types
/// - Functions: indigo (hue 240) — callable items
#[allow(dead_code)]
fn wit_kind_to_color(kind: &TypeKind) -> section_group::ItemColor {
    match kind {
        TypeKind::Record { .. } | TypeKind::Variant { .. } => section_group::ItemColor::Struct,
        TypeKind::Enum { .. } | TypeKind::Flags { .. } => section_group::ItemColor::Enum,
        TypeKind::Resource { .. } => section_group::ItemColor::Resource,
        TypeKind::Alias(_) => section_group::ItemColor::Accent,
    }
}

/// Sigil background color for a WIT type kind.
fn wit_kind_sigil_bg(kind: &TypeKind) -> &'static str {
    match kind {
        TypeKind::Record { .. } => s::RECORD.bg,
        TypeKind::Variant { .. } => s::VARIANT.bg,
        TypeKind::Enum { .. } => s::ENUM.bg,
        TypeKind::Flags { .. } => s::FLAGS.bg,
        TypeKind::Resource { .. } => s::RESOURCE.bg,
        TypeKind::Alias(_) => s::TYPE.bg,
    }
}

/// Sigil text color for a WIT type kind.
fn wit_kind_sigil_color(kind: &TypeKind) -> &'static str {
    match kind {
        TypeKind::Record { .. } => s::RECORD.color,
        TypeKind::Variant { .. } => s::VARIANT.color,
        TypeKind::Enum { .. } => s::ENUM.color,
        TypeKind::Flags { .. } => s::FLAGS.color,
        TypeKind::Resource { .. } => s::RESOURCE.color,
        TypeKind::Alias(_) => s::TYPE.color,
    }
}

/// Sigil character for a WIT type kind.
fn wit_kind_sigil_text(kind: &TypeKind) -> &'static str {
    match kind {
        TypeKind::Record { .. } => s::RECORD.text,
        TypeKind::Variant { .. } => s::VARIANT.text,
        TypeKind::Enum { .. } => s::ENUM.text,
        TypeKind::Flags { .. } => s::FLAGS.text,
        TypeKind::Resource { .. } => s::RESOURCE.text,
        TypeKind::Alias(_) => s::TYPE.text,
    }
}

/// Extract the first sentence from a doc comment.
fn first_sentence(text: &str) -> String {
    // Split on paragraph break first, then on single newline for tighter excerpts
    let first_para = text.split_once("\n\n").map_or(text, |(first, _)| first);
    // Within that paragraph, take only the first line
    let first_line = first_para
        .split_once('\n')
        .map_or(first_para, |(first, _)| first);
    first_line.trim().to_owned()
}
/// Render the full interface definition as a WIT code block.
#[allow(dead_code)]
fn render_interface_definition(iface: &InterfaceDoc) -> Division {
    use super::wit_render::{self, CODE_BLOCK_CLASS};

    Division::builder()
        .class("mb-8")
        .push(
            html::text_content::PreformattedText::builder()
                .class(CODE_BLOCK_CLASS)
                .code(|c| {
                    c.span(|s| s.class("text-ink-500").text("interface "))
                        .span(|s| {
                            s.class("text-wit-iface font-medium")
                                .text(iface.name.clone())
                        })
                        .text(" {\n".to_owned());

                    for ty in &iface.types {
                        wit_render::render_type_in_code(c, ty, "  ");
                        c.text("\n\n".to_owned());
                    }

                    for func in &iface.functions {
                        wit_render::render_func_in_code(c, func, "  ");
                        c.text("\n".to_owned());
                    }

                    c.text("}".to_owned())
                })
                .build(),
        )
        .build()
}
