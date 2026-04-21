//! Item detail page (type or function within an interface).

use crate::components::ds::page_header;
use crate::wit_doc::{FunctionDoc, TypeDoc, TypeKind, TypeRef, WitDocument};
use html::tables::{Table, TableRow};
use html::text_content::Division;
use wasm_meta_registry_client::{KnownPackage, PackageVersion};

use super::package_shell;

// ── Table styling constants ──────────────────────────────

const TABLE_CLASS: &str = "w-full text-[13px]";
const HEADER_ROW_CLASS: &str = "border-b border-line text-left text-ink-500";
const ROW_CLASS: &str = "border-b-2 border-line";
const NAME_CELL_CLASS: &str = "py-2 pr-4 font-mono text-accent";
const VALUE_CELL_CLASS: &str = "py-2 pr-4 font-mono text-ink-900";
const DESC_CELL_CLASS: &str = "py-2 text-ink-700";

/// Build a header row with N columns.
fn table_header(columns: &[&str]) -> TableRow {
    let mut tr = TableRow::builder();
    tr.class(HEADER_ROW_CLASS);
    let last = columns.len().saturating_sub(1);
    for (i, col) in columns.iter().enumerate() {
        let cls = if i == last {
            "py-2 font-medium"
        } else {
            "py-2 pr-4 font-medium"
        };
        tr.table_header(|th| th.class(cls).text(col.to_string()));
    }
    tr.build()
}

/// Build a data row from N cells. Each cell is `(class, text)`.
fn table_row(cells: &[(&str, &str)]) -> TableRow {
    let mut tr = TableRow::builder();
    tr.class(ROW_CLASS);
    for &(cls, text) in cells {
        let cls = cls.to_owned();
        let text = text.to_owned();
        tr.table_cell(|td| td.class(cls).text(text));
    }
    tr.build()
}

/// Render the item detail page for a type.
#[must_use]
pub(crate) fn render_type(
    pkg: &KnownPackage,
    version: &str,
    version_detail: Option<&PackageVersion>,
    iface_name: &str,
    ty: &TypeDoc,
    doc: &WitDocument,
) -> String {
    let display_name = package_shell::display_name_for(pkg);
    let title = format!("{display_name} \u{2014} {iface_name}::{}", ty.name);

    let kind_label = type_kind_label(&ty.kind);

    // Code block
    let code_block = render_type_definition(ty).to_string();

    // Description

    let header = page_header::page_header_block(
        kind_label,
        &ty.name,
        ty.docs.as_deref().unwrap_or("No description available."),
        Some(&code_block),
    )
    .to_string();

    // Type body content (fields, variants, etc.)
    let body = render_type_body(&ty.kind).to_string();

    let content = format!("{header}<div class=\"max-w-3xl\">{body}</div>");

    let nav = super::sidebar::render_sidebar(&super::sidebar::SidebarContext {
        display_name: &display_name,
        version,
        doc,
        active: super::sidebar::SidebarActive::Item(iface_name, &ty.name),
    });

    let ctx = package_shell::SidebarContext {
        pkg,
        version,
        version_detail,
        importers: &[],
        exporters: &[],
        nav_html: Some(nav.to_string()),
    };
    let iface_url = format!(
        "/{}/{version}/interface/{iface_name}",
        display_name.replace(':', "/")
    );
    let extra = vec![crate::components::ds::breadcrumb::Crumb {
        label: iface_name.to_owned(),
        href: Some(iface_url),
    }];
    package_shell::render_page_with_crumbs(&ctx, &title, &content, &extra)
}

/// Render the item detail page for a freestanding function.
#[must_use]
pub(crate) fn render_function(
    pkg: &KnownPackage,
    version: &str,
    version_detail: Option<&PackageVersion>,
    iface_name: &str,
    func: &FunctionDoc,
    doc: &WitDocument,
) -> String {
    let display_name = package_shell::display_name_for(pkg);
    let title = format!("{display_name} \u{2014} {iface_name}::{}", func.name);

    // Code block
    let code_block = render_function_definition(func).to_string();

    // Description

    let header = page_header::page_header_block(
        "Function",
        &func.name,
        func.docs.as_deref().unwrap_or("No description available."),
        Some(&code_block),
    )
    .to_string();

    let content = header;

    let nav = super::sidebar::render_sidebar(&super::sidebar::SidebarContext {
        display_name: &display_name,
        version,
        doc,
        active: super::sidebar::SidebarActive::Item(iface_name, &func.name),
    });

    let ctx = package_shell::SidebarContext {
        pkg,
        version,
        version_detail,
        importers: &[],
        exporters: &[],
        nav_html: Some(nav.to_string()),
    };
    let iface_url = format!(
        "/{}/{version}/interface/{iface_name}",
        display_name.replace(':', "/")
    );
    let extra = vec![crate::components::ds::breadcrumb::Crumb {
        label: iface_name.to_owned(),
        href: Some(iface_url),
    }];
    package_shell::render_page_with_crumbs(&ctx, &title, &content, &extra)
}

/// Get the display label for a type kind.
fn type_kind_label(kind: &TypeKind) -> &'static str {
    match kind {
        TypeKind::Record { .. } => "Record",
        TypeKind::Variant { .. } => "Variant",
        TypeKind::Enum { .. } => "Enum",
        TypeKind::Flags { .. } => "Flags",
        TypeKind::Resource { .. } => "Resource",
        TypeKind::Alias(_) => "Type",
    }
}

/// Get the CSS color class for a type kind heading.
#[allow(dead_code)]
fn type_kind_color(kind: &TypeKind) -> &'static str {
    match kind {
        TypeKind::Record { .. } | TypeKind::Variant { .. } => "text-wit-struct",
        TypeKind::Enum { .. } | TypeKind::Flags { .. } => "text-wit-enum",
        TypeKind::Resource { .. } => "text-wit-resource",
        TypeKind::Alias(_) => "text-accent",
    }
}

/// Render the WIT definition code block for a type, with linked type refs.
fn render_type_definition(ty: &TypeDoc) -> Division {
    use super::wit_render::{self, CODE_BLOCK_CLASS};

    Division::builder()
        .class("mb-4")
        .push(
            html::text_content::PreformattedText::builder()
                .class(CODE_BLOCK_CLASS)
                .code(|c| {
                    wit_render::render_type_in_code(c, ty, "");
                    c
                })
                .build(),
        )
        .build()
}

/// Render the WIT definition code block for a function, with linked type refs.
fn render_function_definition(func: &FunctionDoc) -> Division {
    use super::wit_render::{self, CODE_BLOCK_CLASS};

    Division::builder()
        .class("mb-4")
        .push(
            html::text_content::PreformattedText::builder()
                .class(CODE_BLOCK_CLASS)
                .code(|c| {
                    wit_render::render_func_in_code(c, func, "");
                    c
                })
                .build(),
        )
        .build()
}

/// Render a function signature inline (no border/box), like docs.rs style.
fn render_function_signature(func: &FunctionDoc) -> Division {
    use super::wit_render;

    Division::builder()
        .class("mb-2 bg-surface px-3 py-2")
        .push(
            html::text_content::PreformattedText::builder()
                .class("text-[14px] font-mono text-ink-900 overflow-x-auto")
                .code(|c| {
                    wit_render::render_func_in_code(c, func, "");
                    c
                })
                .build(),
        )
        .build()
}

/// Render the body for a type based on its kind.
fn render_type_body(kind: &TypeKind) -> Division {
    match kind {
        TypeKind::Record { fields } => render_field_table("Fields", fields),
        TypeKind::Variant { cases } => render_variant_table(cases),
        TypeKind::Enum { cases } => render_enum_list(cases),
        TypeKind::Flags { flags } => render_flags_list(flags),
        TypeKind::Resource {
            constructor,
            methods,
            statics,
        } => render_resource_body(constructor.as_ref(), methods, statics),
        TypeKind::Alias(type_ref) => render_alias(type_ref),
    }
}

/// Render a table of record fields.
fn render_field_table(heading: &str, fields: &[crate::wit_doc::FieldDoc]) -> Division {
    let mut div = Division::builder();
    div.heading_2(|h2| {
        h2.class(crate::components::ds::typography::SECTION_CLASS)
            .text(heading.to_owned())
    });

    let mut table = Table::builder();
    table.class(TABLE_CLASS);
    table.push(table_header(&["Name", "Type", "Description"]));
    for field in fields {
        table.push(render_field_row(
            &field.name,
            &field.ty,
            field.docs.as_deref(),
        ));
    }
    div.push(table.build());
    div.build()
}

/// Render a single field/param row.
fn render_field_row(name: &str, ty: &TypeRef, docs: Option<&str>) -> TableRow {
    TableRow::builder()
        .class(ROW_CLASS)
        .table_cell(|td| td.class(NAME_CELL_CLASS).text(name.to_owned()))
        .table_cell(|td| {
            td.class(VALUE_CELL_CLASS)
                .push(super::wit_render::render_type_ref(ty))
        })
        .table_cell(|td| {
            td.class(DESC_CELL_CLASS)
                .text(crate::markdown::render_inline(docs.unwrap_or("")))
        })
        .build()
}

/// Render a variant cases table.
fn render_variant_table(cases: &[crate::wit_doc::CaseDoc]) -> Division {
    let mut div = Division::builder();
    div.heading_2(|h2| {
        h2.class(crate::components::ds::typography::SECTION_CLASS)
            .text("Cases")
    });

    let mut table = Table::builder();
    table.class(TABLE_CLASS);
    table.push(table_header(&["Case", "Payload", "Description"]));
    for case in cases {
        table.table_row(|tr| {
            tr.class(ROW_CLASS)
                .table_cell(|td| td.class(NAME_CELL_CLASS).text(case.name.clone()))
                .table_cell(|td| {
                    td.class(VALUE_CELL_CLASS);
                    if let Some(t) = &case.ty {
                        td.push(super::wit_render::render_type_ref(t));
                    } else {
                        td.text("\u{2014}".to_owned());
                    }
                    td
                })
                .table_cell(|td| {
                    td.class(DESC_CELL_CLASS)
                        .text(crate::markdown::render_inline(
                            case.docs.as_deref().unwrap_or(""),
                        ))
                })
        });
    }
    div.push(table.build());
    div.build()
}

/// Render an enum cases list.
fn render_enum_list(cases: &[crate::wit_doc::EnumCaseDoc]) -> Division {
    let mut div = Division::builder();
    div.heading_2(|h2| {
        h2.class(crate::components::ds::typography::SECTION_CLASS)
            .text("Cases")
    });
    let mut table = Table::builder();
    table.class(TABLE_CLASS);
    table.push(table_header(&["Case", "Description"]));
    for case in cases {
        table.push(table_row(&[
            (NAME_CELL_CLASS, &case.name),
            (
                DESC_CELL_CLASS,
                &crate::markdown::render_inline(case.docs.as_deref().unwrap_or("")),
            ),
        ]));
    }
    div.push(table.build());
    div.build()
}

/// Render a flags list.
fn render_flags_list(flags: &[crate::wit_doc::FlagDoc]) -> Division {
    let mut div = Division::builder();
    div.heading_2(|h2| {
        h2.class(crate::components::ds::typography::SECTION_CLASS)
            .text("Flags")
    });
    let mut table = Table::builder();
    table.class(TABLE_CLASS);
    table.push(table_header(&["Flag", "Description"]));
    for flag in flags {
        table.push(table_row(&[
            (NAME_CELL_CLASS, &flag.name),
            (
                DESC_CELL_CLASS,
                &crate::markdown::render_inline(flag.docs.as_deref().unwrap_or("")),
            ),
        ]));
    }
    div.push(table.build());
    div.build()
}

/// Render a resource body with constructor, methods, and statics.
fn render_resource_body(
    constructor: Option<&FunctionDoc>,
    methods: &[FunctionDoc],
    statics: &[FunctionDoc],
) -> Division {
    let mut div = Division::builder();
    div.class("space-y-6");

    if let Some(ctor) = constructor {
        div.division(|d| {
            d.heading_2(|h2| {
                h2.class(crate::components::ds::typography::SECTION_CLASS)
                    .text("Constructor")
            })
            .push(render_function_signature(ctor));
            if let Some(docs) = &ctor.docs {
                d.text(crate::markdown::render_block(
                    docs,
                    "text-[15px] text-ink-700 leading-relaxed prose-doc",
                ));
            }
            d
        });
    }
    if !methods.is_empty() {
        div.division(|d| {
            d.heading_2(|h2| {
                h2.class(crate::components::ds::typography::SECTION_CLASS)
                    .text("Methods")
            });
            for func in methods {
                d.division(|m| {
                    m.class("py-3 border-b border-lineSoft");
                    m.push(render_function_signature(func));
                    if let Some(docs) = &func.docs {
                        m.text(crate::markdown::render_block(
                            docs,
                            "text-[15px] text-ink-700 leading-relaxed prose-doc",
                        ));
                    }
                    m
                });
            }
            d
        });
    }
    if !statics.is_empty() {
        div.division(|d| {
            d.heading_2(|h2| {
                h2.class(crate::components::ds::typography::SECTION_CLASS)
                    .text("Static Functions")
            });
            for func in statics {
                d.division(|m| {
                    m.class("py-3 border-b border-lineSoft");
                    m.push(render_function_signature(func));
                    if let Some(docs) = &func.docs {
                        m.text(crate::markdown::render_block(
                            docs,
                            "text-[15px] text-ink-700 leading-relaxed prose-doc",
                        ));
                    }
                    m
                });
            }
            d
        });
    }

    div.build()
}

/// Render a type alias (no-op — the code block already shows the definition).
fn render_alias(_type_ref: &TypeRef) -> Division {
    Division::builder().build()
}
