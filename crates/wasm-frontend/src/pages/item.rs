//! Item detail page (type or function within an interface).

use crate::wit_doc::{FunctionDoc, TypeDoc, TypeKind, TypeRef, WitDocument};
use html::tables::{Table, TableRow};
use html::text_content::Division;
use wasm_meta_registry_client::{KnownPackage, PackageVersion};

use super::package_shell;

/// Render the item detail page for a type.
#[must_use]
pub(crate) fn render_type(
    pkg: &KnownPackage,
    version: &str,
    version_detail: Option<&PackageVersion>,
    iface_name: &str,
    ty: &TypeDoc,
    _doc: &WitDocument,
) -> String {
    let display_name = package_shell::display_name_for(pkg);
    let title = format!("{display_name} \u{2014} {iface_name}::{}", ty.name);

    let mut outer = Division::builder();

    // Item heading: "record datetime" style
    let kind_color = type_kind_color(&ty.kind);
    outer.heading_2(|h2| {
        h2.class("text-2xl font-light tracking-display mb-4")
            .span(|s| {
                s.class("text-fg-muted")
                    .text(format!("{} ", type_kind_label(&ty.kind)))
            })
            .span(|s| s.class(kind_color).text(ty.name.clone()))
    });

    // WIT definition block
    outer.push(render_type_definition(ty));

    // Description after code
    if let Some(docs) = &ty.docs {
        outer.paragraph(|p| {
            p.class("text-fg leading-relaxed mb-8 max-w-[65ch]")
                .text(docs.clone())
        });
    }

    // Type body content
    outer.push(render_type_body(&ty.kind));

    let ctx = package_shell::SidebarContext {
        pkg,
        version,
        version_detail,
        importers: &[],
        exporters: &[],
    };
    let pkg_url = package_shell::url_base_for(pkg, version);
    let pkg_label = pkg.wit_name.as_deref().unwrap_or(&display_name);
    let iface_url = format!(
        "/{}/{version}/interface/{iface_name}",
        display_name.replace(':', "/")
    );
    let extra = vec![
        crate::nav::Crumb {
            label: pkg_label.to_owned(),
            href: Some(pkg_url),
        },
        crate::nav::Crumb {
            label: iface_name.to_owned(),
            href: Some(iface_url),
        },
    ];
    package_shell::render_page_with_crumbs(&ctx, &title, &outer.build(), extra)
}

/// Render the item detail page for a freestanding function.
#[must_use]
pub(crate) fn render_function(
    pkg: &KnownPackage,
    version: &str,
    version_detail: Option<&PackageVersion>,
    iface_name: &str,
    func: &FunctionDoc,
    _doc: &WitDocument,
) -> String {
    let display_name = package_shell::display_name_for(pkg);
    let title = format!("{display_name} \u{2014} {iface_name}::{}", func.name);

    let mut outer = Division::builder();

    // Item heading
    outer.heading_2(|h2| {
        h2.class("text-2xl font-light tracking-display mb-4")
            .span(|s| s.class("text-fg-muted").text("function "))
            .span(|s| s.class("text-wit-func").text(func.name.clone()))
    });

    // WIT definition block
    outer.push(render_function_definition(func));

    // Description after code
    if let Some(docs) = &func.docs {
        outer.paragraph(|p| {
            p.class("text-fg leading-relaxed mb-8 max-w-[65ch]")
                .text(docs.clone())
        });
    }

    // Function detail content
    outer.push(render_function_detail(func));

    let ctx = package_shell::SidebarContext {
        pkg,
        version,
        version_detail,
        importers: &[],
        exporters: &[],
    };
    let pkg_url = package_shell::url_base_for(pkg, version);
    let pkg_label = pkg.wit_name.as_deref().unwrap_or(&display_name);
    let iface_url = format!(
        "/{}/{version}/interface/{iface_name}",
        display_name.replace(':', "/")
    );
    let extra = vec![
        crate::nav::Crumb {
            label: pkg_label.to_owned(),
            href: Some(pkg_url),
        },
        crate::nav::Crumb {
            label: iface_name.to_owned(),
            href: Some(iface_url),
        },
    ];
    package_shell::render_page_with_crumbs(&ctx, &title, &outer.build(), extra)
}

/// Get the display label for a type kind.
fn type_kind_label(kind: &TypeKind) -> &'static str {
    match kind {
        TypeKind::Record { .. } => "record",
        TypeKind::Variant { .. } => "variant",
        TypeKind::Enum { .. } => "enum",
        TypeKind::Flags { .. } => "flags",
        TypeKind::Resource { .. } => "resource",
        TypeKind::Alias(_) => "type",
    }
}

/// Get the CSS color class for a type kind heading.
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
        .class("mb-6")
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
        .class("mb-6")
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
        h2.class("text-sm font-medium text-fg-muted uppercase tracking-wide mb-3")
            .text(heading.to_owned())
    });

    let mut table = Table::builder();
    table.class("w-full text-sm");
    table.table_row(|tr| {
        tr.class("border-b-2 border-fg text-left text-fg-muted")
            .table_header(|th| th.class("py-2 pr-4 font-medium").text("Name"))
            .table_header(|th| th.class("py-2 pr-4 font-medium").text("Type"))
            .table_header(|th| th.class("py-2 font-medium").text("Description"))
    });
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
        .class("border-b-2 border-fg/50")
        .table_cell(|td| {
            td.class("py-2 pr-4 font-mono text-accent")
                .text(name.to_owned())
        })
        .table_cell(|td| {
            td.class("py-2 pr-4 font-mono text-fg")
                .push(super::wit_render::render_type_ref(ty))
        })
        .table_cell(|td| {
            td.class("py-2 text-fg-secondary")
                .text(docs.unwrap_or("").to_owned())
        })
        .build()
}

/// Render a variant cases table.
fn render_variant_table(cases: &[crate::wit_doc::CaseDoc]) -> Division {
    let mut div = Division::builder();
    div.heading_2(|h2| {
        h2.class("text-sm font-medium text-fg-muted uppercase tracking-wide mb-3")
            .text("Cases")
    });

    let mut table = Table::builder();
    table.class("w-full text-sm");
    table.table_row(|tr| {
        tr.class("border-b-2 border-fg text-left text-fg-muted")
            .table_header(|th| th.class("py-2 pr-4 font-medium").text("Case"))
            .table_header(|th| th.class("py-2 pr-4 font-medium").text("Payload"))
            .table_header(|th| th.class("py-2 font-medium").text("Description"))
    });
    for case in cases {
        table.table_row(|tr| {
            tr.class("border-b-2 border-fg/50")
                .table_cell(|td| {
                    td.class("py-2 pr-4 font-mono text-accent")
                        .text(case.name.clone())
                })
                .table_cell(|td| {
                    td.class("py-2 pr-4 font-mono text-fg");
                    if let Some(t) = &case.ty {
                        td.push(super::wit_render::render_type_ref(t));
                    } else {
                        td.text("\u{2014}".to_owned());
                    }
                    td
                })
                .table_cell(|td| {
                    td.class("py-2 text-fg-secondary")
                        .text(case.docs.clone().unwrap_or_default())
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
        h2.class("text-sm font-medium text-fg-muted uppercase tracking-wide mb-3")
            .text("Cases")
    });
    let mut table = Table::builder();
    table.class("w-full text-sm");
    table.table_row(|tr| {
        tr.class("border-b-2 border-fg text-left text-fg-muted")
            .table_header(|th| th.class("py-2 pr-4 font-medium").text("Case"))
            .table_header(|th| th.class("py-2 font-medium").text("Description"))
    });
    for case in cases {
        table.table_row(|tr| {
            tr.class("border-b-2 border-fg/50")
                .table_cell(|td| {
                    td.class("py-2 pr-4 font-mono text-accent")
                        .text(case.name.clone())
                })
                .table_cell(|td| {
                    td.class("py-2 text-fg-secondary")
                        .text(case.docs.clone().unwrap_or_default())
                })
        });
    }
    div.push(table.build());
    div.build()
}

/// Render a flags list.
fn render_flags_list(flags: &[crate::wit_doc::FlagDoc]) -> Division {
    let mut div = Division::builder();
    div.heading_2(|h2| {
        h2.class("text-sm font-medium text-fg-muted uppercase tracking-wide mb-3")
            .text("Flags")
    });
    let mut table = Table::builder();
    table.class("w-full text-sm");
    table.table_row(|tr| {
        tr.class("border-b-2 border-fg text-left text-fg-muted")
            .table_header(|th| th.class("py-2 pr-4 font-medium").text("Flag"))
            .table_header(|th| th.class("py-2 font-medium").text("Description"))
    });
    for flag in flags {
        table.table_row(|tr| {
            tr.class("border-b-2 border-fg/50")
                .table_cell(|td| {
                    td.class("py-2 pr-4 font-mono text-accent")
                        .text(flag.name.clone())
                })
                .table_cell(|td| {
                    td.class("py-2 text-fg-secondary")
                        .text(flag.docs.clone().unwrap_or_default())
                })
        });
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
                h2.class("text-sm font-medium text-fg-muted uppercase tracking-wide mb-3")
                    .text("Constructor")
            })
            .push(render_function_detail(ctor))
        });
    }
    if !methods.is_empty() {
        div.division(|d| {
            d.heading_2(|h2| {
                h2.class("text-sm font-medium text-fg-muted uppercase tracking-wide mb-3")
                    .text("Methods")
            });
            for func in methods {
                d.push(render_function_detail(func));
            }
            d
        });
    }
    if !statics.is_empty() {
        div.division(|d| {
            d.heading_2(|h2| {
                h2.class("text-sm font-medium text-fg-muted uppercase tracking-wide mb-3")
                    .text("Static Functions")
            });
            for func in statics {
                d.push(render_function_detail(func));
            }
            d
        });
    }

    div.build()
}

/// Render a type alias.
fn render_alias(type_ref: &TypeRef) -> Division {
    Division::builder()
        .heading_2(|h2| {
            h2.class("text-sm font-medium text-fg-muted uppercase tracking-wide mb-3")
                .text("Definition")
        })
        .paragraph(|p| {
            p.class("font-mono text-fg")
                .push(super::wit_render::render_type_ref(type_ref))
        })
        .build()
}

/// Render function detail: signature + param table.
fn render_function_detail(func: &FunctionDoc) -> Division {
    let mut div = Division::builder();
    div.class("mb-6");

    // Param table (skip `self`)
    let visible_params: Vec<_> = func.params.iter().filter(|p| p.name != "self").collect();
    if !visible_params.is_empty() {
        let mut table = Table::builder();
        table.class("w-full text-sm mt-3");
        table.table_row(|tr| {
            tr.class("border-b-2 border-fg text-left text-fg-muted")
                .table_header(|th| th.class("py-2 pr-4 font-medium").text("Parameter"))
                .table_header(|th| th.class("py-2 font-medium").text("Type"))
        });
        for param in &visible_params {
            table.table_row(|tr| {
                tr.class("border-b-2 border-fg/50")
                    .table_cell(|td| {
                        td.class("py-2 pr-4 font-mono text-accent")
                            .text(param.name.clone())
                    })
                    .table_cell(|td| {
                        td.class("py-2 font-mono text-fg")
                            .push(super::wit_render::render_type_ref(&param.ty))
                    })
            });
        }
        div.push(table.build());
    }

    // Return type
    if let Some(ret) = &func.result {
        div.division(|d| {
            d.class("mt-3 text-sm")
                .span(|s| s.class("text-fg-muted").text("Returns: "))
                .span(|s| {
                    s.class("font-mono text-fg")
                        .push(super::wit_render::render_type_ref(ret))
                })
        });
    }

    div.build()
}
