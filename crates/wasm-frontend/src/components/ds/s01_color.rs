//! 01 — Color.

use html::text_content::Division;

/// A single color swatch entry.
pub(crate) struct Swatch {
    pub(crate) bg: &'static str,
    pub(crate) label: &'static str,
    pub(crate) hex: &'static str,
    pub(crate) oklch: &'static str,
    /// Optional paired ink text for categorical swatches.
    pub(crate) ink_class: &'static str,
}

impl Swatch {
    const fn new(
        bg: &'static str,
        label: &'static str,
        hex: &'static str,
        oklch: &'static str,
    ) -> Self {
        Self {
            bg,
            label,
            hex,
            oklch,
            ink_class: "",
        }
    }

    const fn cat(
        bg: &'static str,
        label: &'static str,
        hex: &'static str,
        oklch: &'static str,
        ink_class: &'static str,
    ) -> Self {
        Self {
            bg,
            label,
            hex,
            oklch,
            ink_class,
        }
    }
}

fn render_swatch(s: &Swatch) -> Division {
    let mut d = Division::builder();
    if s.ink_class.is_empty() {
        d.division(|sw| sw.class(format!("swatch {}", s.bg)));
    } else {
        d.division(|sw| {
            sw.class(format!("swatch {} flex items-end p-3", s.bg))
                .span(|sp| {
                    sp.class(format!("text-[12px] {} font-medium", s.ink_class))
                        .text("Aa")
                })
        });
    }
    d.division(|n| n.class("mt-2 text-[13px]").text(s.label))
        .division(|h| h.class("text-[12px] text-ink-500 mono").text(s.hex))
        .division(|o| o.class("text-[11px] text-ink-400 mono").text(s.oklch));
    d.build()
}

fn render_group(title: &'static str, grid_class: &'static str, swatches: &[Swatch]) -> Division {
    let title = title.to_owned();
    let mut grid = Division::builder();
    grid.class(grid_class);
    for s in swatches {
        grid.push(render_swatch(s));
    }
    Division::builder()
        .heading_3(|h| {
            h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
                .text(title)
        })
        .push(grid.build())
        .build()
}

pub(crate) const SURFACES: &[Swatch] = &[
    Swatch::new("bg-canvas", "Canvas", "#F4F4F5", "oklch(.967 .001 286)"),
    Swatch::new("bg-surface", "Surface", "#FFFFFF", "oklch(1 0 0)"),
    Swatch::new(
        "bg-surfaceMuted",
        "Surface Muted",
        "#E8E8EA",
        "oklch(.928 .003 286)",
    ),
];

pub(crate) const INK: &[Swatch] = &[
    Swatch::new("bg-ink-900", "Ink 900", "#18181B", "oklch(.210 .006 286)"),
    Swatch::new("bg-ink-700", "Ink 700", "#3F3F46", "oklch(.370 .013 286)"),
    Swatch::new("bg-ink-500", "Ink 500", "#71717A", "oklch(.552 .016 286)"),
    Swatch::new("bg-ink-400", "Ink 400", "#A1A1AA", "oklch(.705 .015 286)"),
    Swatch::new("bg-ink-300", "Ink 300", "#D4D4D8", "oklch(.871 .006 286)"),
];

pub(crate) const LINES: &[Swatch] = &[
    Swatch::new("bg-line", "Line", "#D4D4D8", "oklch(.871 .006 286)"),
    Swatch::new(
        "bg-lineSoft",
        "Line Soft",
        "#E4E4E7",
        "oklch(.920 .004 286)",
    ),
];

pub(crate) const SEMANTIC: &[Swatch] = &[
    Swatch::new("bg-positive", "Positive", "#1F8A4C", "oklch(.561 .149 149)"),
    Swatch::new(
        "bg-cat-pinkInk",
        "Negative (pinkInk)",
        "#9B4F5E",
        "oklch(.490 .080 13)",
    ),
];

pub(crate) const CATEGORICAL: &[Swatch] = &[
    Swatch::cat(
        "bg-cat-blue",
        "Blue",
        "#D6E4FF / #3D5A99",
        "oklch(.910 .046 264) / oklch(.430 .102 263)",
        "text-cat-blueInk",
    ),
    Swatch::cat(
        "bg-cat-pink",
        "Pink",
        "#FBD9DF / #9B4F5E",
        "oklch(.910 .037 9) / oklch(.490 .080 13)",
        "text-cat-pinkInk",
    ),
    Swatch::cat(
        "bg-cat-green",
        "Green",
        "#D2ECD8 / #3F7A52",
        "oklch(.918 .039 148) / oklch(.503 .089 149)",
        "text-cat-greenInk",
    ),
    Swatch::cat(
        "bg-cat-peach",
        "Peach",
        "#F8E2C2 / #8E6529",
        "oklch(.911 .049 79) / oklch(.499 .089 70)",
        "text-cat-peachInk",
    ),
    Swatch::cat(
        "bg-cat-lilac",
        "Lilac",
        "#D5C8EF / #5A3D8C",
        "oklch(.852 .051 287) / oklch(.395 .120 287)",
        "text-cat-lilacInk",
    ),
    Swatch::cat(
        "bg-cat-cream",
        "Cream",
        "#F4ECC2 / #7A6A2A",
        "oklch(.937 .055 100) / oklch(.490 .073 96)",
        "text-cat-creamInk",
    ),
    Swatch::cat(
        "bg-cat-teal",
        "Teal",
        "#BFE3EE / #1F6F87",
        "oklch(.890 .037 215) / oklch(.470 .080 224)",
        "text-cat-tealInk",
    ),
    Swatch::cat(
        "bg-cat-rust",
        "Rust",
        "#F4D2C0 / #9F5536",
        "oklch(.880 .045 50) / oklch(.510 .104 42)",
        "text-cat-rustInk",
    ),
    Swatch::cat(
        "bg-cat-plum",
        "Plum",
        "#E8C5E8 / #7E2E7E",
        "oklch(.851 .065 322) / oklch(.408 .135 326)",
        "text-cat-plumInk",
    ),
    Swatch::cat(
        "bg-cat-slate",
        "Slate",
        "#DADCE0 / #535A66",
        "oklch(.882 .005 264) / oklch(.450 .015 257)",
        "text-cat-slateInk",
    ),
];

/// A group of swatches with a title and grid layout.
pub(crate) struct SwatchGroup {
    pub(crate) title: &'static str,
    pub(crate) grid_class: &'static str,
    pub(crate) swatches: &'static [Swatch],
}

/// Render this section.
pub(crate) fn render(
    section_id: &str,
    num: &str,
    title: &str,
    desc: &str,
    groups: &[SwatchGroup],
) -> String {
    let mut content = Division::builder();
    content.class("space-y-10");
    for group in groups {
        content.push(render_group(group.title, group.grid_class, group.swatches));
    }
    let content = content.build().to_string();

    super::section(section_id, num, title, desc, &content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(crate::components::ds::pretty_html(&render(
            "colors",
            "01",
            "Color",
            "Neutral surfaces and ink form the structural base. Pastel categoricals encode chart series with paired ink tones for legibility.",
            &[
                SwatchGroup {
                    title: "Surfaces",
                    grid_class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                    swatches: SURFACES
                },
                SwatchGroup {
                    title: "Ink",
                    grid_class: "grid grid-cols-2 md:grid-cols-5 gap-4",
                    swatches: INK
                },
                SwatchGroup {
                    title: "Lines",
                    grid_class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                    swatches: LINES
                },
                SwatchGroup {
                    title: "Semantic",
                    grid_class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                    swatches: SEMANTIC
                },
                SwatchGroup {
                    title: "Categorical",
                    grid_class: "grid grid-cols-2 md:grid-cols-5 gap-4",
                    swatches: CATEGORICAL
                },
            ],
        )));
    }
}
