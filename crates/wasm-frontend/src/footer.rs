//! Site footer wrapper — delegates to the design-system footer component
//! with the default content used across all pages.

use crate::components::ds::footer::{Footer, FooterColumn, FooterLink};

const BROWSE: &[FooterLink] = &[
    FooterLink {
        label: "Packages",
        href: "/all",
    },
    FooterLink {
        label: "Authors",
        href: "/all",
    },
    FooterLink {
        label: "Registries",
        href: "/about",
    },
    FooterLink {
        label: "Changelog",
        href: "/about",
    },
];

const DEVELOP: &[FooterLink] = &[
    FooterLink {
        label: "Documentation",
        href: "/docs",
    },
    FooterLink {
        label: "CLI reference",
        href: "/docs",
    },
    FooterLink {
        label: "Spec",
        href: "/docs",
    },
    FooterLink {
        label: "Design system",
        href: "/design-system",
    },
];

const COMMUNITY: &[FooterLink] = &[
    FooterLink {
        label: "GitHub",
        href: "https://github.com/yoshuawuyts/component-cli",
    },
    FooterLink {
        label: "Status",
        href: "/health",
    },
];

const COLUMNS: &[FooterColumn] = &[
    FooterColumn {
        kicker: "Browse",
        links: BROWSE,
    },
    FooterColumn {
        kicker: "Develop",
        links: DEVELOP,
    },
    FooterColumn {
        kicker: "Community",
        links: COMMUNITY,
    },
];

/// Render the site footer.
#[must_use]
pub(crate) fn render() -> String {
    crate::components::ds::footer::render(&Footer {
        brand: "wasm",
        lede: "A package manager and registry for WebAssembly components. Made by Yosh Wuyts and contributors. To be donated to the Bytecode Alliance.",
        status: "All systems operational",
        columns: COLUMNS,
    })
}
