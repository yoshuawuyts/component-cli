//! Shared sidebar components for detail pages.
//!
//! Provides a navigation sidebar showing sibling interfaces/worlds and
//! package metadata, matching the layout of the package detail page.

use crate::components::nav_list::{self, NavState};
use crate::wit_doc::WitDocument;
use html::content::Aside;
use html::text_content::{Division, UnorderedList};

/// Context needed to render the detail page sidebar.
pub(crate) struct SidebarContext<'a> {
    /// The package display name (e.g. `"wasi:cli"`).
    pub display_name: &'a str,
    /// The current version string.
    pub version: &'a str,
    /// The parsed WIT document for navigation links.
    pub doc: &'a WitDocument,
    /// Which sidebar item is currently active.
    pub active: SidebarActive<'a>,
}

/// Which item in the sidebar is currently active.
pub(crate) enum SidebarActive<'a> {
    /// An interface page (name of the interface).
    Interface(&'a str),
    /// An item within an interface (interface name, item name).
    Item(&'a str, #[allow(dead_code)] &'a str),
    /// A world page (name of the world).
    World(&'a str),
}

/// Render the sidebar for a detail page.
pub(crate) fn render_sidebar(ctx: &SidebarContext<'_>) -> Aside {
    let pkg_url = format!("/{}/{}", ctx.display_name.replace(':', "/"), ctx.version);

    let mut aside = Aside::builder();
    aside.class("space-y-4");
    aside.push(render_nav_card(ctx, &pkg_url));
    aside.build()
}

/// Render the navigation card with interfaces and worlds.
fn render_nav_card(ctx: &SidebarContext<'_>, pkg_url: &str) -> Division {
    use crate::components::detail_row;

    let mut card = Division::builder();
    card.class("text-[13px]");

    // Package link at top
    card.division(|d| {
        d.class("mb-3 pb-3 border-b-[1.5px] border-rule")
            .anchor(|a| {
                a.href(pkg_url.to_owned())
                    .class("text-accent hover:underline font-medium text-[13px]")
                    .text(ctx.display_name.to_owned())
            })
    });

    // Worlds section
    if !ctx.doc.worlds.is_empty() {
        card.division(|d| {
            d.class("mb-3")
                .push(detail_row::section_label("Worlds"));
            let mut ul = UnorderedList::builder();
            ul.class("space-y-px");
            for world in &ctx.doc.worlds {
                let state = if matches!(ctx.active, SidebarActive::World(name) if name == world.name) {
                    NavState::Active
                } else {
                    NavState::Inactive
                };
                ul.push(nav_list::item(&world.name, &world.url, state));
            }
            d.push(ul.build());
            d
        });
    }

    // Interfaces section
    if !ctx.doc.interfaces.is_empty() {
        if !ctx.doc.worlds.is_empty() {
            card.push(detail_row::section_rule());
        }
        card.division(|d| {
            d.push(detail_row::section_label("Interfaces"));
            let mut ul = UnorderedList::builder();
            ul.class("space-y-px");
            for iface in &ctx.doc.interfaces {
                let is_active = matches!(
                    ctx.active,
                    SidebarActive::Interface(name) if name == iface.name
                ) || matches!(
                    ctx.active,
                    SidebarActive::Item(iface_name, _) if iface_name == iface.name
                );
                let state = if is_active {
                    NavState::Active
                } else {
                    NavState::Inactive
                };
                ul.push(nav_list::item(&iface.name, &iface.url, state));
            }
            d.push(ul.build());
            d
        });
    }

    card.build()
}
