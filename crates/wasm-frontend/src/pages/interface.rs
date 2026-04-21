//! Interface detail page.

use crate::components::ds::{page_header, section_group};
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
    content.class("space-y-6 max-w-3xl");
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
        content.push(render_type_section("Resources", &resources));
    }
    if !records.is_empty() {
        content.push(render_type_section("Records", &records));
    }
    if !variants.is_empty() {
        content.push(render_type_section("Variants", &variants));
    }
    if !enums.is_empty() {
        content.push(render_type_section("Enums", &enums));
    }
    if !flags.is_empty() {
        content.push(render_type_section("Flags", &flags));
    }
    if !aliases.is_empty() {
        content.push(render_type_section("Type Aliases", &aliases));
    }
    if !iface.functions.is_empty() {
        content.push(render_function_section(&iface.functions));
    }

    let body_html = format!("{header_row}{}", content.build());

    // Build nav card with interface items for the sidebar
    let nav = super::sidebar::render_sidebar(&super::sidebar::SidebarContext {
        display_name: &display_name,
        version,
        doc,
        active: super::sidebar::SidebarActive::Interface(&iface.name),
    });

    let ctx = package_shell::SidebarContext {
        pkg,
        version,
        version_detail,
        importers: &[],
        exporters: &[],
        nav_html: Some(nav.to_string()),
    };
    package_shell::render_page_with_crumbs(&ctx, &title, &body_html, &[])
}

/// Render a section of types grouped by kind.
fn render_type_section(heading: &str, types: &[&TypeDoc]) -> Division {
    let mut div = Division::builder();
    div.class("pt-6 first:pt-0");
    div.push(section_group::header(heading, types.len()));

    for ty in types {
        let desc = ty
            .docs
            .as_deref()
            .map(|d| crate::markdown::render_inline(&first_sentence(d)))
            .unwrap_or_default();
        div.push(section_group::item_row(
            &ty.name,
            &ty.url,
            &wit_kind_to_color(&ty.kind),
            &wit_stability(&ty.stability),
            &desc,
        ));
    }
    div.build()
}

/// Render the freestanding functions section.
fn render_function_section(functions: &[FunctionDoc]) -> Division {
    let mut div = Division::builder();
    div.class("pt-6 first:pt-0");
    div.push(section_group::header("Functions", functions.len()));

    for func in functions {
        let desc = func
            .docs
            .as_deref()
            .map(|d| crate::markdown::render_inline(&first_sentence(d)))
            .unwrap_or_default();
        div.push(section_group::item_row(
            &func.name,
            &func.url,
            &section_group::ItemColor::Func,
            &wit_stability(&func.stability),
            &desc,
        ));
    }
    div.build()
}

/// Convert a WIT stability to the component enum.
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
fn wit_kind_to_color(kind: &TypeKind) -> section_group::ItemColor {
    match kind {
        TypeKind::Record { .. } | TypeKind::Variant { .. } => section_group::ItemColor::Struct,
        TypeKind::Enum { .. } | TypeKind::Flags { .. } => section_group::ItemColor::Enum,
        TypeKind::Resource { .. } => section_group::ItemColor::Resource,
        TypeKind::Alias(_) => section_group::ItemColor::Accent,
    }
}

/// Extract the first sentence from a doc comment.
fn first_sentence(text: &str) -> String {
    text.split_once("\n\n").map_or_else(
        || text.trim().to_owned(),
        |(first, _)| first.trim().to_owned(),
    )
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
