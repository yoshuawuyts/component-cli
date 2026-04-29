//! Shared sigil definitions.
//!
//! Centralises the color + letter combinations used across the sidebar,
//! item lists, and detail pages so every callsite references one constant.

/// A sigil definition: background color, text color, and letter.
pub(crate) struct Sigil {
    /// CSS background color value (e.g. `"var(--c-cat-green)"`).
    pub bg: &'static str,
    /// CSS text color value (e.g. `"var(--c-cat-green-ink)"`).
    pub color: &'static str,
    /// Single-character label (always rendered uppercase by CSS).
    pub text: &'static str,
}

/// Return the sigil for a WIT type kind.
pub(crate) fn for_type_kind(kind: &crate::wit_doc::TypeKind) -> &'static Sigil {
    use crate::wit_doc::TypeKind;
    match kind {
        TypeKind::Record { .. } => &RECORD,
        TypeKind::Variant { .. } => &VARIANT,
        TypeKind::Enum { .. } => &ENUM,
        TypeKind::Flags { .. } => &FLAGS,
        TypeKind::Resource { .. } => &RESOURCE,
        TypeKind::Alias(_) => &TYPE,
    }
}

/// World — cream "W".
pub(crate) const WORLD: Sigil = Sigil {
    bg: "var(--c-cat-cream)",
    color: "var(--c-cat-cream-ink)",
    text: "W",
};

/// Interface — rust "I".
pub(crate) const IFACE: Sigil = Sigil {
    bg: "var(--c-cat-rust)",
    color: "var(--c-cat-rust-ink)",
    text: "I",
};

/// Function — green "F".
pub(crate) const FUNC: Sigil = Sigil {
    bg: "var(--c-cat-green)",
    color: "var(--c-cat-green-ink)",
    text: "F",
};

/// Type (record, variant, alias, etc.) — blue "T".
pub(crate) const TYPE: Sigil = Sigil {
    bg: "var(--c-cat-blue)",
    color: "var(--c-cat-blue-ink)",
    text: "T",
};

/// Resource — peach "R".
pub(crate) const RESOURCE: Sigil = Sigil {
    bg: "var(--c-cat-peach)",
    color: "var(--c-cat-peach-ink)",
    text: "R",
};

/// Record — lilac "R".
pub(crate) const RECORD: Sigil = Sigil {
    bg: "var(--c-cat-lilac)",
    color: "var(--c-cat-lilac-ink)",
    text: "R",
};

/// Variant — teal "V".
pub(crate) const VARIANT: Sigil = Sigil {
    bg: "var(--c-cat-teal)",
    color: "var(--c-cat-teal-ink)",
    text: "V",
};

/// Enum — teal "E".
pub(crate) const ENUM: Sigil = Sigil {
    bg: "var(--c-cat-teal)",
    color: "var(--c-cat-teal-ink)",
    text: "E",
};

/// Flags — lilac "F".
pub(crate) const FLAGS: Sigil = Sigil {
    bg: "var(--c-cat-lilac)",
    color: "var(--c-cat-lilac-ink)",
    text: "F",
};

/// Package / root — plum "·".
#[allow(dead_code)]
pub(crate) const ROOT: Sigil = Sigil {
    bg: "var(--c-cat-plum)",
    color: "var(--c-cat-plum-ink)",
    text: "\u{00b7}",
};

/// Module — pink "M".
pub(crate) const MODULE: Sigil = Sigil {
    bg: "var(--c-cat-pink)",
    color: "var(--c-cat-pink-ink)",
    text: "M",
};

/// Dependency — plum, layers icon.
pub(crate) const DEPENDENCY: Sigil = Sigil {
    bg: "var(--c-cat-plum)",
    color: "var(--c-cat-plum-ink)",
    text: concat!(
        r#"<svg style="width:12px;height:12px" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">"#,
        include_str!("../../../../../vendor/lucide/layers.svg"),
        "</svg>"
    ),
};

/// Nested component — plum "C".
pub(crate) const COMPONENT: Sigil = Sigil {
    bg: "var(--c-cat-plum)",
    color: "var(--c-cat-plum-ink)",
    text: "C",
};

// ── Design-system showcase ──────────────────────────────────────────

use html::text_content::Division;

/// All sigils for the design-system gallery: (label, sigil ref).
pub(crate) const ALL: &[(&str, &Sigil)] = &[
    ("World", &WORLD),
    ("Interface", &IFACE),
    ("Function", &FUNC),
    ("Type", &TYPE),
    ("Resource", &RESOURCE),
    ("Record", &RECORD),
    ("Flags", &FLAGS),
    ("Variant", &VARIANT),
    ("Enum", &ENUM),
    ("Module", &MODULE),
    ("Component", &COMPONENT),
    ("Dependency", &DEPENDENCY),
    ("Root", &ROOT),
];

/// Render the sigils design-system section.
pub(crate) fn render(
    section_id: &str,
    num: &str,
    title: &str,
    desc: &str,
    sigils: &[(&str, &Sigil)],
) -> String {
    let mut wrapper = Division::builder();
    wrapper.class("space-y-6");

    // Gallery grid
    wrapper.division(|lbl| lbl.class("text-[12px] text-ink-500 mb-3").text("Legend"));
    wrapper.division(|gallery| {
        gallery.class("flex flex-wrap gap-4");
        for (label, sigil) in sigils {
            let label = (*label).to_owned();
            // Raw HTML: Span::style() creates a <style> child, not an inline style attribute.
            let sigil_html = format!(
                r#"<span class="sigil" style="background:{};color:{};">{}</span>"#,
                sigil.bg, sigil.color, sigil.text,
            );
            gallery.division(|cell| {
                cell.class("flex flex-col items-center gap-2")
                    .text(sigil_html)
                    .span(|s| s.class("text-[11px] text-ink-500").text(label))
            });
        }
        gallery
    });

    // Inline usage example — body text
    wrapper.division(|demo| {
        let world_sigil = format!(
            r#"<span class="sigil" style="background:{};color:{};">{}</span>"#,
            WORLD.bg, WORLD.color, WORLD.text,
        );
        demo.division(|lbl| {
            lbl.class("text-[12px] text-ink-500 mb-3")
                .text("Inline with text")
        })
        .division(|row| {
            row.class("flex items-center gap-2 text-[14px] text-ink-700")
                .text(world_sigil)
                .span(|s| s.class("font-medium").text("wasi:http/handler"))
        })
    });

    // Inline usage example — heading text
    wrapper.division(|demo| {
        let iface_sigil = format!(
            r#"<span class="sigil" style="background:{};color:{};width:22px;height:22px;font-size:12px;">{}</span>"#,
            IFACE.bg, IFACE.color, IFACE.text,
        );
        demo.division(|lbl| {
            lbl.class("text-[12px] text-ink-500 mb-3")
                .text("Inline with heading")
        })
        .division(|row| {
            row.class(
                "flex items-center gap-[6px] text-[24px] font-semibold tracking-tight text-ink-900",
            )
            .text(iface_sigil)
            .span(|s| s.text("wasi:http/handler"))
        })
    });

    let content = wrapper.build().to_string();
    super::section(section_id, num, title, desc, &content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(crate::components::ds::pretty_html(&render(
            "sigils",
            "25",
            "Sigils",
            "18\u{00d7}18px rounded squares with a single monospace letter, used to classify items by kind in sidebars, item lists, and detail pages. Each sigil pairs a categorical background with its ink counterpart for 4.5:1 contrast.",
            ALL,
        )));
    }
}
