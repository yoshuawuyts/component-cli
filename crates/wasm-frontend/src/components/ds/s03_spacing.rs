//! 03 — Spacing & Radius.

use html::text_content::Division;

const SPACING: &[(&str, &str, &str)] = &[
    ("4", "xs", "4px"),
    ("8", "sm", "8px"),
    ("12", "md", "12px"),
    ("16", "lg", "16px"),
    ("24", "xl", "24px"),
    ("32", "2xl", "32px"),
    ("48", "3xl", "48px"),
];

const RADII: &[(&str, &str, &str)] = &[
    ("2px", "", "sm \u{2014} 2px"),
    ("4px", "", "md \u{2014} 4px (inputs, bars)"),
    ("5px", "", "lg \u{2014} 5px (buttons, cards)"),
    ("", "rounded-pill", "pill \u{2014} 9999px"),
];

fn spacing_row(value: &str, label: &str, width: &str) -> Division {
    let width = width.to_owned();
    Division::builder()
        .class("flex items-center gap-4")
        .division(|d| d.class("h-3 bg-ink-900").style(format!("width:{width}px")))
        .span(|s| s.class("text-[13px] mono w-12").text(value.to_owned()))
        .span(|s| s.class("text-[12px] text-ink-500").text(label.to_owned()))
        .build()
}

/// Render this section.
pub(crate) fn render() -> String {
    let mut scale = Division::builder();
    scale.class("space-y-2");
    for (value, label, _) in SPACING {
        scale.push(spacing_row(value, label, value));
    }

    let mut radii_grid = Division::builder();
    radii_grid.class("grid grid-cols-2 md:grid-cols-4 gap-4");
    for (radius, extra_class, label) in RADII {
        let cls = if extra_class.is_empty() {
            "h-16 bg-surfaceMuted".to_owned()
        } else {
            format!("h-16 bg-surfaceMuted {extra_class}")
        };
        let style = if radius.is_empty() {
            String::new()
        } else {
            format!("border-radius:{radius}")
        };
        let label = label.to_owned();
        let label = (*label).to_owned();
        radii_grid.division(|d| {
            d.division(|s| {
                let s = s.class(cls.clone());
                if style.is_empty() {
                    s
                } else {
                    s.style(style.clone())
                }
            })
            .division(|l| l.class("mt-2 text-[13px]").text(label))
        });
    }

    let content = Division::builder()
        .class("space-y-10")
        .division(|d| {
            d.heading_3(|h| {
                h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
                    .text("Spacing scale")
            })
            .push(scale.build())
        })
        .division(|d| {
            d.heading_3(|h| {
                h.class("text-[13px] mono uppercase tracking-wider text-ink-500 mb-3")
                    .text("Radius")
            })
            .push(radii_grid.build())
        })
        .build()
        .to_string();

    super::section(
        "spacing",
        "03",
        "Spacing & Radius",
        "4px base scale. Radii stay small for a precise, instrumental feel; pills used for selection chips only.",
        &content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(render());
    }
}
