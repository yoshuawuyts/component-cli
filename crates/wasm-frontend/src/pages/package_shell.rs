//! Shared page shell for the package detail page and its sub-pages
//! (interface, world, item).
//!
//! Provides a two-column layout: main content on the left, and a sidebar
//! on the right with version selector, install command, metadata,
//! dependencies, and dependents.

use html::text_content::Division;
use wasm_meta_registry_client::{KnownPackage, PackageVersion};

use crate::layout;

/// Context for rendering the package page sidebar.
pub(crate) struct SidebarContext<'a> {
    /// Package being displayed.
    pub pkg: &'a KnownPackage,
    /// Current version string.
    pub version: &'a str,
    /// Version detail (annotations, size, etc.) if available.
    pub version_detail: Option<&'a PackageVersion>,
    /// Packages that import this one.
    pub importers: &'a [KnownPackage],
    /// Packages that export this one.
    pub exporters: &'a [KnownPackage],
}

/// Render the shared page shell: two-column layout with sidebar,
/// wrapped in the HTML document layout.
#[must_use]
pub(crate) fn render_page(ctx: &SidebarContext<'_>, title: &str, body_content: &str) -> String {
    render_page_inner(ctx, title, body_content, &[], true)
}

/// Render the page shell with extra breadcrumb segments after the package name.
#[must_use]
pub(crate) fn render_page_with_crumbs(
    ctx: &SidebarContext<'_>,
    title: &str,
    body_content: &str,
    extra_crumbs: &[crate::nav::Crumb],
) -> String {
    render_page_inner(ctx, title, body_content, extra_crumbs, false)
}

/// Inner page shell renderer.
///
/// Uses a "golden layout": left sidebar with navigation and metadata,
/// right column for main content. The top nav bar is replaced by the
/// sidebar's own logo, breadcrumbs, and search.
fn render_page_inner(
    ctx: &SidebarContext<'_>,
    title: &str,
    body_content: &str,
    extra_crumbs: &[crate::nav::Crumb],
    is_root: bool,
) -> String {
    let pkg = ctx.pkg;
    let version = ctx.version;
    let display_name = display_name_for(pkg);

    // Build breadcrumbs (extra crumbs only — package name is in the navbar)
    let breadcrumb_html = render_breadcrumb_path(extra_crumbs);
    let trailing_slash = if is_root {
        ""
    } else {
        r#" <span class="text-fg-faint mx-0.5">/</span>"#
    };

    // Build sidebar metadata
    let sidebar_meta = render_sidebar(ctx, &display_name).to_string();

    // Build main content
    let content = body_content;

    // Top navbar with bunny, breadcrumbs, and links
    // Golden layout below: sidebar left, content right
    let pkg_url = url_base_for(pkg, version);
    let pkg_name_html = match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) if !is_root => {
            format!(
                r#"<a href="/{ns}" class="text-fg-muted hover:text-fg transition-colors">{ns}</a><span class="text-fg-faint">:</span><a href="{pkg_url}" class="text-fg-muted hover:text-fg transition-colors">{name}</a>"#
            )
        }
        (Some(ns), Some(_)) => {
            format!(
                r#"<a href="/{ns}" class="text-fg-muted hover:text-fg transition-colors">{ns}</a><span class="text-fg-faint">:</span>"#
            )
        }
        _ => {
            format!(
                r#"<a href="{pkg_url}" class="text-fg-muted hover:text-fg transition-colors">{display_name}</a>"#
            )
        }
    };
    let body = format!(
        r#"<style>
  .page-grid {{
    display: grid;
    grid-template-columns: 260px 1fr;
    grid-template-rows: auto 1fr;
    grid-template-areas:
      "sidebar topbar"
      "sidebar main";
    gap: 0 2.5rem;
    min-height: 100vh;
  }}
  @media (min-width: 1280px) {{
    .page-grid {{
      grid-template-columns: 260px 1fr auto;
      grid-template-areas:
        "sidebar main rightbar"
        "sidebar main rightbar";
    }}
    .page-grid .topbar {{
      display: none;
    }}
    .page-grid .rightbar {{
      display: block;
    }}
  }}
  .sidebar-scroll {{
    scrollbar-width: none;
  }}
  .sidebar-scroll::-webkit-scrollbar {{
    display: none;
  }}
</style>
<div class="page-grid pt-3 xl:pt-6">
  <aside class="space-y-5 sidebar-scroll font-mono" style="grid-area:sidebar;position:sticky;top:1.5rem;align-self:start;display:flex;flex-direction:column;height:calc(100vh - 3rem);overflow-y:auto;padding-right:0.75rem;padding-left:0.75rem">
    <div class="space-y-5 flex-1">
    <div class="flex justify-center mb-4"><a href="/" id="bunny" aria-label="Home" role="link" class="text-lg font-mono font-medium text-fg hover:text-accent transition-colors" style="cursor:pointer;display:inline-block;width:12ch;text-align:left">(๑╹ᆺ╹)</a></div>
    {sidebar_meta}
    </div>
    <p class="text-sm text-fg-faint pb-6">Made by <a href="https://yosh.is" class="hover:text-fg transition-colors">Yosh Wuyts</a><br>Intended to be donated to the <a href="https://bytecodealliance.org" class="hover:text-fg transition-colors">Bytecode Alliance</a></p>
  </aside>
  <div class="topbar flex items-center justify-end gap-4 pb-2 pr-4" style="grid-area:topbar;align-self:start">
    <a href="/docs" class="text-sm text-fg-muted hover:text-fg transition-colors">Docs</a>
    <a href="/downloads" class="text-sm text-fg-muted hover:text-fg transition-colors">Downloads</a>
    <form action="/search" method="get" class="relative flex">
      <input type="search" name="q" placeholder="Search…" aria-label="Search" class="w-48 px-3 pr-12 py-1.5 text-sm border-2 border-fg bg-page text-fg focus:outline-none" id="search-input">
      <span class="absolute right-3 top-1/2 -translate-y-1/2 text-sm font-mono pointer-events-none opacity-30" aria-hidden="true">[ / ]</span>
    </form>
  </div>
  <div style="grid-area:main;min-width:0" class="pr-4">
    <div class="flex flex-wrap items-baseline text-lg font-light tracking-display font-display font-display mb-2">
      {pkg_name_html}{breadcrumb_html}{trailing_slash}
    </div>
    {content}
  </div>
  <aside class="rightbar hidden pr-4" style="grid-area:rightbar;position:sticky;top:1.5rem;align-self:start">
    <div class="flex items-center gap-4">
      <a href="/docs" class="text-sm text-fg-muted hover:text-fg transition-colors">Docs</a>
      <a href="/downloads" class="text-sm text-fg-muted hover:text-fg transition-colors">Downloads</a>
      <form action="/search" method="get" class="relative flex">
        <input type="search" name="q" placeholder="Search…" aria-label="Search" class="w-36 px-3 pr-10 py-1.5 text-sm border-2 border-fg bg-page text-fg focus:outline-none" id="search-input-lg">
        <span class="absolute right-3 top-1/2 -translate-y-1/2 text-sm font-mono pointer-events-none opacity-30" aria-hidden="true">[ / ]</span>
      </form>
    </div>
  </aside>
</div>
<script>
(function(){{
  var b=document.getElementById('bunny');
  if(!b)return;
  var anims=[
    ['(๑╹ᆺ╹)','(๑°ᆺ°)!','(๑◉ᆺ◉)!!'],
    ['(๑╹ᆺ╹)','(๑°ᆺ°)♪','ヽ(๑≧ᆺ≦)ノ'],
    ['(๑╹ᆺ╹)','(๑╹ᆺ╹)>','(๑°ᆺ°)>>']
  ];
  var seq=anims[Math.floor(Math.random()*anims.length)];
  var timer=null;
  b.addEventListener('mouseenter',function(){{
    if(timer)return;
    b.textContent=seq[1];
    timer=setTimeout(function(){{
      b.textContent=seq[2];
    }},80);
  }});
  b.addEventListener('mouseleave',function(){{
    if(timer){{clearTimeout(timer);timer=null;}}
    b.textContent=seq[0];
  }});
}})();
</script>"#
    );

    layout::document_full_width(title, &body)
}

/// Render breadcrumb segments as inline HTML.
fn render_breadcrumb_path(crumbs: &[crate::nav::Crumb]) -> String {
    use std::fmt::Write;
    let mut html = String::new();
    for crumb in crumbs {
        html.push_str(r#" <span class="text-fg-faint mx-0.5">/</span> "#);
        if let Some(href) = &crumb.href {
            write!(
                html,
                r#"<a href="{href}" class="text-fg-muted hover:text-fg transition-colors">{label}</a>"#,
                label = crumb.label
            )
            .unwrap();
        } else {
            write!(
                html,
                r#"<span class="text-fg">{label}</span>"#,
                label = crumb.label
            )
            .unwrap();
        }
    }
    html
}

/// Render the right sidebar with all package metadata.
fn render_sidebar(ctx: &SidebarContext<'_>, display_name: &str) -> Division {
    let pkg = ctx.pkg;
    let version = ctx.version;
    let version_detail = ctx.version_detail;
    let annotations = version_detail.and_then(|d| d.annotations.as_ref());

    let mut sidebar = Division::builder();
    sidebar
        .class("space-y-4")
        .style("font-size:0.75rem;line-height:1.125rem");

    // Metadata (including version selector)
    sidebar.division(|wrapper| {
        wrapper.class("");
        let mut meta = Division::builder();
        meta.class("space-y-3 border-2 border-fg p-3");

        // Version selector inside metadata
        if !pkg.tags.is_empty() {
            let url_name = match (&pkg.wit_namespace, &pkg.wit_name) {
                (Some(ns), Some(name)) => format!("{ns}/{name}"),
                _ => pkg.repository.clone(),
            };
            meta.push(render_version_select(pkg, version, &url_name));
        }

        {
            let registry_url = format!("https://{}/{}", pkg.registry, pkg.repository);
            let registry_display = friendly_registry_name(&pkg.registry);
            meta.push(meta_link_row("Registry", &registry_display, &registry_url));
        }
        if let Some(source) = annotations.and_then(|a| a.source.as_deref()) {
            meta.push(meta_link_row(
                "Repository",
                &friendly_repo_name(source),
                source,
            ));
        } else {
            let repo_url = format!("https://{}/{}", pkg.registry, pkg.repository);
            let repo_display = friendly_repo_name(&repo_url);
            meta.push(meta_link_row("Repository", &repo_display, &repo_url));
        }
        if let Some(license) = annotations.and_then(|a| a.licenses.as_deref()) {
            meta.push(meta_row("License", license));
        }
        if let Some(size) = version_detail.and_then(|d| d.size_bytes) {
            meta.push(meta_row("Size", &format_size(size)));
        }
        if let Some(created) = version_detail.and_then(|d| d.created_at.as_deref()) {
            meta.push(meta_row("Published", &format_date(created)));
        }
        if let Some(docs_url) = annotations.and_then(|a| a.documentation.as_deref()) {
            meta.push(meta_link_row("Docs", &abbreviate_url(docs_url), docs_url));
        }
        let source = annotations.and_then(|a| a.source.as_deref());
        if let Some(url) = annotations.and_then(|a| a.url.as_deref())
            && source != Some(url)
        {
            meta.push(meta_link_row("Homepage", &abbreviate_url(url), url));
        }
        wrapper.push(meta.build());
        wrapper
    });

    // Install command (after metadata)
    let install_cmd = render_install_command(display_name, version);
    sidebar.division(|wrapper| {
        wrapper
            .class("")
            .division(|label| {
                label
                    .class("text-sm font-medium text-fg-muted mb-1")
                    .text("Install")
            })
            .push(install_cmd)
    });

    // Imports & Exports (from version detail worlds)
    if let Some(detail) = ctx.version_detail {
        let (imports, exports) = collect_imports_exports(detail, pkg);
        if !imports.is_empty() {
            sidebar.push(build_iface_sidebar_section(
                "Imports",
                &imports,
                ctx.version,
            ));
        }
        if !exports.is_empty() {
            sidebar.push(build_iface_sidebar_section(
                "Exports",
                &exports,
                ctx.version,
            ));
        }
    }

    // Dependents
    let total_dependents = ctx.importers.len() + ctx.exporters.len();
    if total_dependents > 0 {
        sidebar.division(|wrapper| {
            wrapper.class("").heading_3(|h3| {
                h3.class("text-sm font-medium text-fg-muted mb-1")
                    .text(format!("Dependents ({total_dependents})"))
            });
            wrapper.division(|div| {
                div.class("border-2 border-fg p-3");

                let mut all: Vec<&KnownPackage> =
                    ctx.importers.iter().chain(ctx.exporters.iter()).collect();
                all.sort_by(|a, b| a.repository.cmp(&b.repository));
                all.dedup_by(|a, b| a.repository == b.repository);

                let mut ul = html::text_content::UnorderedList::builder();
                ul.class("space-y-1");
                for dep_pkg in all.iter().take(10) {
                    let name = match (&dep_pkg.wit_namespace, &dep_pkg.wit_name) {
                        (Some(ns), Some(n)) => format!("{ns}:{n}"),
                        _ => dep_pkg.repository.clone(),
                    };
                    ul.list_item(|li| {
                        li.class("text-sm");
                        match (&dep_pkg.wit_namespace, &dep_pkg.wit_name) {
                            (Some(ns), Some(n)) => {
                                li.anchor(|a| {
                                    a.href(format!("/{ns}/{n}"))
                                        .class("text-accent hover:underline font-mono")
                                        .text(name.clone())
                                });
                            }
                            _ => {
                                li.span(|s| s.class("text-fg font-mono").text(name.clone()));
                            }
                        }
                        li
                    });
                }
                div.push(ul.build());

                if all.len() > 10 {
                    div.paragraph(|p| {
                        p.class("text-sm text-fg-faint mt-1")
                            .text(format!("and {} more", all.len() - 10))
                    });
                }
                div
            });
            wrapper
        });
    }

    sidebar.build()
}

/// Collect deduplicated import and export refs from all worlds.
///
/// Internal interfaces (belonging to the same package) are listed with
/// their full path (e.g. `wasi:clocks/wall-clock`) and no version.
/// External packages are grouped by package name with version
/// (e.g. `wasi:io@0.2.11`). Internal items are sorted first.
fn collect_imports_exports(
    detail: &PackageVersion,
    pkg: &KnownPackage,
) -> (Vec<String>, Vec<String>) {
    let display_name = display_name_for(pkg);
    let mut internal_imports = std::collections::BTreeSet::new();
    let mut external_imports = std::collections::BTreeSet::new();
    let mut internal_exports = std::collections::BTreeSet::new();
    let mut external_exports = std::collections::BTreeSet::new();

    for world in &detail.worlds {
        for iface in &world.imports {
            if iface.package == display_name {
                internal_imports.insert(format_internal_label(iface));
            } else {
                external_imports.insert(format_iface_label(iface));
            }
        }
        for iface in &world.exports {
            if iface.package == display_name {
                internal_exports.insert(format_internal_label(iface));
            } else {
                external_exports.insert(format_iface_label(iface));
            }
        }
    }

    let imports: Vec<String> = internal_imports
        .into_iter()
        .chain(external_imports)
        .collect();
    let exports: Vec<String> = internal_exports
        .into_iter()
        .chain(external_exports)
        .collect();

    (imports, exports)
}

/// Format an internal interface ref as "package/interface" (no version).
fn format_internal_label(iface: &wasm_meta_registry_client::WitInterfaceRef) -> String {
    let mut s = iface.package.clone();
    if let Some(name) = &iface.interface {
        s.push('/');
        s.push_str(name);
    }
    s
}

/// Format an interface ref as "package@version" (grouped by package, no sub-interface).
fn format_iface_label(iface: &wasm_meta_registry_client::WitInterfaceRef) -> String {
    let mut s = iface.package.clone();
    if let Some(v) = &iface.version {
        s.push('@');
        s.push_str(v);
    }
    s
}

/// Build a sidebar section listing interface refs (imports or exports).
fn build_iface_sidebar_section(heading: &str, items: &[String], version: &str) -> Division {
    let heading = heading.to_string();
    let version = version.to_string();
    let mut wrapper = Division::builder();
    wrapper.class("").heading_3(|h3| {
        h3.class("text-sm font-medium text-fg-muted mb-1")
            .text(heading)
    });
    wrapper.division(|div| {
        div.class("border-2 border-fg p-3");
        let mut ul = html::text_content::UnorderedList::builder();
        ul.class("space-y-1");
        for label in items {
            ul.list_item(|li| {
                li.class("font-mono text-sm");
                let (pkg_part, ver_suffix) = match label.split_once('@') {
                    Some((p, v)) => (p, Some(v)),
                    None => (label.as_str(), None),
                };
                match pkg_part.split_once(':') {
                    Some((ns, name)) => {
                        // Internal refs contain '/' (e.g. "wasi:clocks/wall-clock")
                        // and link to the interface page; external refs link to
                        // the package page.
                        let href = if let Some((pkg_name, iface)) = name.split_once('/') {
                            format!("/{ns}/{pkg_name}/{version}/interface/{iface}")
                        } else if let Some(v) = ver_suffix {
                            format!("/{ns}/{name}/{v}")
                        } else {
                            format!("/{ns}/{name}")
                        };
                        li.anchor(|a| {
                            a.href(href).class("text-accent hover:underline");
                            a.span(|s| s.text(pkg_part.to_string()));
                            if let Some(v) = ver_suffix {
                                a.span(|s| s.class("text-fg-faint ml-1").text(format!("@{v}")));
                            }
                            a
                        });
                    }
                    None => {
                        li.span(|s| s.class("text-fg").text(pkg_part.to_string()));
                        if let Some(v) = ver_suffix {
                            li.span(|s| s.class("text-fg-faint ml-1").text(format!("@{v}")));
                        }
                    }
                }
                li
            });
        }
        div.push(ul.build());
        div
    });
    wrapper.build()
}

/// Compute the display name from package WIT metadata.
pub(crate) fn display_name_for(pkg: &KnownPackage) -> String {
    match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) => format!("{ns}:{name}"),
        _ => pkg.repository.clone(),
    }
}

/// Compute the URL base for sub-page links.
pub(crate) fn url_base_for(pkg: &KnownPackage, version: &str) -> String {
    match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) => format!("/{ns}/{name}/{version}"),
        _ => format!("/{}/{version}", pkg.repository),
    }
}

/// Render the version selector dropdown.
fn render_version_select(pkg: &KnownPackage, current_version: &str, url_name: &str) -> Division {
    let script_body = format!(
        "document.getElementById('version-select').addEventListener('change',function(){{\
        var p=window.location.pathname;\
        var base='/{url_name}/';\
        var rest=p.indexOf(base)===0?p.slice(base.length):'';\
        var slash=rest.indexOf('/');\
        var sub=slash>=0?rest.slice(slash):'';\
        window.location.href=base+this.value+sub\
        }})"
    );

    Division::builder()
        .class("flex items-center justify-between gap-3")
        .span(|s| s.class("text-fg-muted text-sm").text("Version"))
        .push({
            let mut s = html::forms::Select::builder();
            s.id("version-select").name("version").class(
                "bg-transparent text-fg text-sm cursor-pointer border-0 outline-none text-right",
            );
            for tag in &pkg.tags {
                let is_current = tag == current_version;
                if is_current {
                    s.option(|opt| opt.value(tag.clone()).text(tag.clone()).selected(true));
                } else {
                    s.option(|opt| opt.value(tag.clone()).text(tag.clone()));
                }
            }
            s.build()
        })
        .script(|s| s.text(script_body))
        .build()
}

/// Render the install command section with a copy button.
fn render_install_command(display_name: &str, version: &str) -> Division {
    let command = format!("wasm install {display_name}@{version}");

    let copy_icon = "<svg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><rect x='9' y='9' width='13' height='13' rx='2' ry='2'/><path d='M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1'/></svg>";
    let check_icon = "<svg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><polyline points='20 6 9 17 4 12'/></svg>";

    let script = format!(
        "(function(){{\
        var btn=document.getElementById('copy-install-btn');\
        var copyIcon=\"{copy_icon}\";\
        var checkIcon=\"{check_icon}\";\
        btn.innerHTML=copyIcon;\
        btn.addEventListener('click',function(){{\
        navigator.clipboard.writeText('{command}').then(function(){{\
        btn.innerHTML=checkIcon;\
        setTimeout(function(){{btn.innerHTML=copyIcon}},2000)\
        }})}})}})()",
    );

    Division::builder()
        .division(|div| {
            div.class(
                "flex items-center gap-2 border-2 border-fg \
                 px-3 py-2 font-mono text-sm text-fg",
            )
            .code(|code| {
                code.class("flex-1 select-all overflow-hidden whitespace-nowrap text-ellipsis")
                    .text(command)
            })
            .button(|btn| {
                btn.id("copy-install-btn")
                    .class("shrink-0 text-fg-muted hover:text-fg transition-opacity cursor-pointer")
            })
            .script(|s| s.text(script))
        })
        .build()
}

/// Render a label: value metadata row.
fn meta_row(label: &str, value: &str) -> Division {
    Division::builder()
        .class("flex items-baseline justify-between gap-3")
        .span(|s| {
            s.class("text-fg-muted text-sm shrink-0")
                .text(label.to_owned())
        })
        .span(|s| {
            s.class("text-fg text-sm font-mono text-right")
                .text(value.to_owned())
        })
        .build()
}

/// Render a label: linked-value metadata row.
fn meta_link_row(label: &str, text: &str, href: &str) -> Division {
    Division::builder()
        .class("flex items-baseline justify-between gap-3")
        .span(|s| {
            s.class("text-fg-muted text-sm shrink-0")
                .text(label.to_owned())
        })
        .anchor(|a| {
            a.href(href.to_owned())
                .class("text-accent hover:underline font-mono text-sm text-right truncate")
                .text(text.to_owned())
        })
        .build()
}

/// Format a byte count as a human-readable size string.
#[allow(clippy::cast_precision_loss)]
fn format_size(bytes: i64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;

    let bytes = bytes as f64;
    if bytes < KIB {
        format!("{bytes} B")
    } else if bytes < MIB {
        format!("{:.1} KiB", bytes / KIB)
    } else if bytes < GIB {
        format!("{:.1} MiB", bytes / MIB)
    } else {
        format!("{:.1} GiB", bytes / GIB)
    }
}

/// Abbreviate a URL for display (strip scheme and trailing slash).
fn abbreviate_url(url: &str) -> String {
    url.strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url)
        .trim_end_matches('/')
        .to_owned()
}

/// Return a friendly display name for a known OCI registry, or the full host/path.
fn friendly_registry_name(registry: &str) -> String {
    match registry {
        "ghcr.io" => "GitHub Packages".to_owned(),
        "registry-1.docker.io" | "docker.io" => "Docker Hub".to_owned(),
        "mcr.microsoft.com" => "Microsoft MCR".to_owned(),
        _ => registry.to_owned(),
    }
}

/// Return a friendly display name for a known repository host, or the abbreviated URL.
fn friendly_repo_name(url: &str) -> String {
    let stripped = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    if stripped.starts_with("github.com/") {
        "GitHub".to_owned()
    } else if stripped.starts_with("gitlab.com/") {
        "GitLab".to_owned()
    } else if stripped.starts_with("codeberg.org/") {
        "Codeberg".to_owned()
    } else {
        abbreviate_url(url)
    }
}

/// Format an ISO 8601 timestamp as a short date (YYYY-MM-DD).
fn format_date(iso: &str) -> String {
    iso.split('T').next().unwrap_or(iso).to_owned()
}
