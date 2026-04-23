//! Site footer — brand block + link columns + bottom legal row.

use html::content::Footer as FooterEl;

/// A single link in a footer column.
pub(crate) struct FooterLink {
    pub label: &'static str,
    pub href: &'static str,
}

/// A column of footer links.
pub(crate) struct FooterColumn {
    pub kicker: &'static str,
    pub links: &'static [FooterLink],
}

/// Configuration for [`render`].
pub(crate) struct Footer<'a> {
    /// Brand name shown at the top-left.
    pub brand: &'a str,
    /// Short lede paragraph under the brand name.
    pub lede: &'a str,
    /// System status text (e.g. `"All systems operational"`).
    pub status: &'a str,
    /// Right-hand link columns.
    pub columns: &'a [FooterColumn],
    /// Bottom-left mono copyright string.
    pub copyright: &'a str,
    /// Bottom-right legal links.
    pub legal: &'a [FooterLink],
}

/// Render the footer.
#[must_use]
pub(crate) fn render(footer: &Footer<'_>) -> String {
    let brand = footer.brand.to_owned();
    let lede = footer.lede.to_owned();
    let status = footer.status.to_owned();
    let copyright = footer.copyright.to_owned();

    FooterEl::builder()
        .class("page-grid-bleed border-t border-lineSoft")
        .division(|grid| {
            let grid = grid
                .class("mx-auto max-w-[1280px] px-4 md:px-8 py-10 grid grid-cols-2 md:grid-cols-5 gap-8 text-[13px]")
                .division(|brand_col| {
                    brand_col
                        .class("col-span-2")
                        .division(|d| {
                            d.class("flex items-center text-[15px] font-semibold tracking-tight")
                                .text(brand)
                        })
                        .paragraph(|p| {
                            p.class("mt-3 max-w-sm text-ink-500 leading-relaxed").text(lede)
                        })
                        .division(|d| {
                            d.class("mt-4 inline-flex items-center gap-2 text-[12px] text-ink-500 mono")
                                .span(|s| s.class("h-1.5 w-1.5 rounded-full bg-positive"))
                                .text(status)
                        })
                });
            for col in footer.columns {
                push_column(grid, col);
            }
            grid
        })
        .division(|bottom| {
            let mut bottom = bottom
                .class("mx-auto max-w-[1280px] px-4 md:px-8 py-5 border-t border-lineSoft flex items-center justify-between text-[12px] text-ink-500")
                .span(|s| s.class("mono").text(copyright));
            bottom = bottom.division(|legal| {
                let mut legal = legal.class("flex items-center gap-5");
                for link in footer.legal {
                    let label = link.label.to_owned();
                    let href = link.href.to_owned();
                    legal = legal.anchor(|a| {
                        a.href(href).class("hover:text-ink-900 no-underline").text(label)
                    });
                }
                legal
            });
            bottom
        })
        .build()
        .to_string()
}

fn push_column(parent: &mut html::text_content::builders::DivisionBuilder, col: &FooterColumn) {
    let kicker = col.kicker.to_owned();
    parent.division(|d| {
        let d = d.division(|k| {
            k.class("text-[12px] mono uppercase tracking-wider text-ink-500")
                .text(kicker)
        });
        d.unordered_list(|ul| {
            let mut ul = ul.class("mt-3 space-y-2 text-ink-700");
            for link in col.links {
                let label = link.label.to_owned();
                let href = link.href.to_owned();
                ul = ul.list_item(|li| {
                    li.anchor(|a| {
                        a.href(href)
                            .class("hover:text-ink-900 no-underline")
                            .text(label)
                    })
                });
            }
            ul
        })
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        const BROWSE: &[FooterLink] = &[
            FooterLink {
                label: "Packages",
                href: "/packages",
            },
            FooterLink {
                label: "Categories",
                href: "/categories",
            },
        ];
        const COMMUNITY: &[FooterLink] = &[
            FooterLink {
                label: "GitHub",
                href: "https://github.com/yoshuawuyts/wasm",
            },
            FooterLink {
                label: "Spec",
                href: "/spec",
            },
        ];
        const LEGAL: &[FooterLink] = &[
            FooterLink {
                label: "Privacy",
                href: "/privacy",
            },
            FooterLink {
                label: "Terms",
                href: "/terms",
            },
        ];
        let html = render(&Footer {
            brand: "wasm",
            lede: "The package manager for WebAssembly Components.",
            status: "All systems operational",
            columns: &[
                FooterColumn {
                    kicker: "Browse",
                    links: BROWSE,
                },
                FooterColumn {
                    kicker: "Community",
                    links: COMMUNITY,
                },
            ],
            copyright: "© 2026 wasm contributors",
            legal: LEGAL,
        });
        insta::assert_snapshot!(crate::components::ds::pretty_html(&html));
    }
}
