//! Front page — landing experience matching `references/landing.html`.

// r[impl frontend.pages.home]

use wasm_meta_registry_client::{ApiError, KnownPackage, RegistryClient};

use crate::components::ds::{
    cta_strip::{self, CtaStrip},
    hero::{self, Hero, HeroCta, HeroCtaStyle},
    install_card::{self, InstallCard},
    link_list::{self, LeftStyle, LinkRow, RightStyle},
    metrics_strip::{self, Metric},
    principles_grid::{self, Principle},
    search_bar,
};
use crate::layout;

/// Fetch recent packages and render the home page.
pub(crate) async fn render(client: &RegistryClient) -> String {
    match client.fetch_recent_packages(1000).await {
        Ok(packages) => render_packages(&packages),
        Err(err) => render_error(&err),
    }
}

/// Render the home page with a list of packages.
fn render_packages(packages: &[KnownPackage]) -> String {
    let body = compose_body(&Stats::from_packages(packages), None);
    layout::document_landing("Home", &body)
}

/// Render the home page with an API error message — keep the chrome but
/// surface a small notice so visitors know the live data is unavailable.
fn render_error(err: &ApiError) -> String {
    let notice = format!(
        r#"<div class="mx-auto max-w-[1280px] px-4 md:px-8 pt-4"><div role="status" class="flex items-start gap-2 rounded-md border border-line bg-surfaceMuted px-3 py-2 text-[12px] text-ink-700"><span class="mono uppercase tracking-wider text-ink-500">Registry offline</span><span>Live package data is temporarily unavailable. Install the CLI below to get started — the registry is not required to use <code class="px-1 py-0.5 rounded-sm bg-surface text-ink-900 mono text-[0.875em]">wasm</code> locally. ({err})</span></div></div>"#,
        err = html_escape(&err.to_string()),
    );
    let body = compose_body(&Stats::default(), Some(&notice));
    layout::document_landing("Home", &body)
}

/// Minimal HTML escape for inline error text.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Aggregated landing-page statistics derived from the registry index.
#[derive(Default)]
struct Stats {
    /// Total number of indexed packages.
    package_count: usize,
    /// Number of distinct WIT namespaces (or repository owners as fallback).
    author_count: usize,
    /// Sum of release tag counts across all packages.
    version_count: usize,
    /// Top featured packages by version count, with a short description.
    featured: Vec<NamedRow>,
    /// Top namespaces by package count.
    namespaces: Vec<NamedRow>,
}

/// An owned row used to feed [`link_list::render`] from dynamic data.
struct NamedRow {
    left: String,
    right: String,
    href: String,
}

impl Stats {
    fn from_packages(packages: &[KnownPackage]) -> Self {
        use std::collections::BTreeMap;

        let package_count = packages.len();
        let version_count: usize = packages.iter().map(|p| p.tags.len()).sum();

        // Group by WIT namespace (preferred) or fall back to the first
        // segment of the repository path.
        let mut by_ns: BTreeMap<String, Vec<&KnownPackage>> = BTreeMap::new();
        for pkg in packages {
            let ns = pkg
                .wit_namespace
                .clone()
                .or_else(|| pkg.repository.split('/').next().map(str::to_owned))
                .unwrap_or_default();
            if ns.is_empty() {
                continue;
            }
            by_ns.entry(ns).or_default().push(pkg);
        }
        let author_count = by_ns.len();

        // Top namespaces by package count.
        let mut ns_rows: Vec<(String, usize)> = by_ns
            .iter()
            .map(|(ns, pkgs)| (ns.clone(), pkgs.len()))
            .collect();
        ns_rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        let namespaces: Vec<NamedRow> = ns_rows
            .into_iter()
            .take(5)
            .map(|(ns, count)| NamedRow {
                left: ns.clone(),
                right: format_count(count),
                href: format!("/{ns}"),
            })
            .collect();

        // Top featured packages: most release tags first, alphabetical tiebreak.
        let mut featured_pool: Vec<&KnownPackage> = packages.iter().collect();
        featured_pool.sort_by(|a, b| {
            b.tags
                .len()
                .cmp(&a.tags.len())
                .then_with(|| a.repository.cmp(&b.repository))
        });
        let featured: Vec<NamedRow> = featured_pool
            .into_iter()
            .take(5)
            .map(|p| {
                let label = match (&p.wit_namespace, &p.wit_name) {
                    (Some(ns), Some(name)) => format!("{ns}:{name}"),
                    _ => p.repository.clone(),
                };
                let href = match (&p.wit_namespace, &p.wit_name) {
                    (Some(ns), Some(name)) => format!("/{ns}/{name}"),
                    _ => format!("/{}", p.repository),
                };
                let right = p
                    .description
                    .clone()
                    .filter(|d| !d.trim().is_empty())
                    .unwrap_or_else(|| {
                        let n = p.tags.len();
                        if n == 1 {
                            "1 release".to_owned()
                        } else {
                            format!("{n} releases")
                        }
                    });
                NamedRow {
                    left: label,
                    right,
                    href,
                }
            })
            .collect();

        Self {
            package_count,
            author_count,
            version_count,
            featured,
            namespaces,
        }
    }
}

/// Compose the full landing page body. `notice_html` is rendered above
/// the hero when present (for example, a registry-offline banner).
fn compose_body(stats: &Stats, notice_html: Option<&str>) -> String {
    let install = install_card::render(&InstallCard {
        platforms: &["macOS", "Linux", "Windows"],
        filename: "install.sh",
        snippet_html: &install_snippet(),
        sha: "9e4a…c0f1",
        copy_command: "curl -sSf https://wasm.dev/install.sh | sh",
    });

    let hero_html = hero::render(&Hero {
        kicker: &["v0.4.0", "Stable · WASI 0.2"],
        title: "The package manager for components.",
        lede: "Resolve, vendor, and compose WebAssembly components from any registry. \
               Reproducible builds, semantic versioning, and an append-only index — so \
               the dependency you shipped is the dependency you keep.",
        ctas: &[
            HeroCta {
                label: "Get started",
                href: "/docs",
                style: HeroCtaStyle::Primary,
            },
            HeroCta {
                label: "Browse packages",
                href: "/all",
                style: HeroCtaStyle::Secondary,
            },
            HeroCta {
                label: "Source on GitHub",
                href: "https://github.com/yoshuawuyts/component-cli",
                style: HeroCtaStyle::Ghost,
            },
        ],
        right: &install,
    });

    let pkg_count = format_count(stats.package_count);
    let author_count = format_count(stats.author_count);
    let version_count = format_count(stats.version_count);
    let metrics_html = metrics_strip::render(&[
        Metric {
            label: "Packages",
            value: &pkg_count,
            delta: None,
            verified: false,
        },
        Metric {
            label: "Authors",
            value: &author_count,
            delta: None,
            verified: false,
        },
        Metric {
            label: "Versions published",
            value: &version_count,
            delta: None,
            verified: false,
        },
        Metric {
            label: "Index integrity",
            value: "100%",
            delta: Some("verified"),
            verified: true,
        },
    ]);

    let explore_html = render_explore(stats);

    let principles_html = principles_grid::render(
        "Why wasm",
        "Built for components.",
        "A package manager designed around the WebAssembly Component Model — not \
         retrofitted from an older ecosystem.",
        PRINCIPLES,
    );

    let cta_html = cta_strip::render(&CtaStrip {
        kicker: "For maintainers",
        title: "Publish your component.",
        body_html: "Add your namespace to a registry config and run \
                    <code class=\"px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]\">wasm publish</code>. \
                    The index is append-only and signed end-to-end.",
        primary_label: "Open the publishing guide",
        primary_href: "/docs",
        secondary_label: "Read the spec",
        secondary_href: "/docs",
    });

    let notice_html = notice_html.unwrap_or("");
    format!(
        r#"{notice_html}
{hero_html}
{metrics_html}
<div class="bg-surface pt-12 md:pt-16 pb-14 md:pb-20">
{explore_html}
{principles_html}
{cta_html}
</div>"#
    )
}

/// Render the "Explore the ecosystem" section: kicker, search input, and
/// two link lists (Featured / Namespaces).
fn render_explore(stats: &Stats) -> String {
    let pkg_count = format_count(stats.package_count);
    let author_count = format_count(stats.author_count);
    let ns_count = format_count(stats.namespaces.len());
    let search_html = search_bar::landing(&pkg_count).to_string();

    let featured_rows: Vec<LinkRow<'_>> = stats
        .featured
        .iter()
        .map(|r| LinkRow {
            left: &r.left,
            right: &r.right,
            href: &r.href,
        })
        .collect();
    let namespace_rows: Vec<LinkRow<'_>> = stats
        .namespaces
        .iter()
        .map(|r| LinkRow {
            left: &r.left,
            right: &r.right,
            href: &r.href,
        })
        .collect();
    let featured = link_list::render(
        "Featured",
        &featured_rows,
        &LeftStyle::Mono,
        &RightStyle::Description,
    );
    let namespaces = link_list::render(
        "Namespaces",
        &namespace_rows,
        &LeftStyle::Mono,
        &RightStyle::Count,
    );

    format!(
        r#"<section class="mx-auto max-w-[1280px] px-4 md:px-8">
  <div class="max-w-2xl">
    <div class="text-[12px] mono uppercase tracking-wider text-ink-500">Explore</div>
    <h2 class="mt-2 text-[28px] md:text-[32px] font-semibold tracking-tight">The ecosystem.</h2>
    <p class="mt-3 text-[14px] text-ink-700 leading-relaxed">
      <span class="mono tabular-nums text-ink-900">{pkg_count}</span> packages from
      <span class="mono tabular-nums text-ink-900">{author_count}</span> authors across
      <span class="mono tabular-nums text-ink-900">{ns_count}</span> namespaces.
    </p>
  </div>

  {search_html}

  <div class="mt-10 grid md:grid-cols-2 gap-x-12 gap-y-10">
    {featured}
    {namespaces}
  </div>

  <div class="mt-6 text-right text-[13px]">
    <a href="/all" class="text-ink-900 hover:underline no-underline">Browse all packages →</a>
  </div>
</section>"#
    )
}

/// Format a count with a thin space as the thousands separator (e.g.
/// `1248 -> "1 248"`), matching the visual style in `landing.html`.
fn format_count(n: usize) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push('\u{2009}');
        }
        out.push(c);
    }
    out.chars().rev().collect()
}

/// Build the install card shell snippet using the helper spans.
fn install_snippet() -> String {
    let mut s = String::new();
    s.push_str(&install_card::prompt(
        "curl -sSf https://wasm.dev/install.sh | sh",
    ));
    s.push_str("\n\n");
    s.push_str(&install_card::prompt(
        "wasm <span class=\"font-semibold\">init</span> hello-world",
    ));
    s.push('\n');
    s.push_str(&install_card::muted("  Created hello-world/wasm.toml"));
    s.push_str("\n\n");
    s.push_str(&install_card::prompt(
        "wasm <span class=\"font-semibold\">add</span> wasi:http@0.2",
    ));
    s.push('\n');
    s.push_str(&install_card::muted("  Resolved 4 packages in 312 ms"));
    s.push('\n');
    s.push_str(&install_card::positive("  ✓ Locked"));
    s.push_str(" wasi:http ");
    s.push_str(&install_card::muted("0.2.3"));
    s.push('\n');
    s.push_str(&install_card::positive("  ✓ Locked"));
    s.push_str(" wasi:io   ");
    s.push_str(&install_card::muted("0.2.3"));
    s.push_str("\n\n");
    s.push_str(&install_card::prompt(
        "wasm <span class=\"font-semibold\">build</span>",
    ));
    s.push('\n');
    s.push_str(&install_card::positive("  ✓ Compiled"));
    s.push_str(" hello-world ");
    s.push_str(&install_card::muted("v0.1.0"));
    s.push(' ');
    s.push_str("<span class=\"text-ink-400\">(2.18s)</span>");
    s
}

const SHIELD_SVG: &str = r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>"#;
const STACK_SVG: &str = concat!(
    r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../vendor/lucide/layers.svg"),
    "</svg>",
);
const GLOBE_SVG: &str = r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="9"/><path d="M3 12h18"/><path d="M12 3a14 14 0 0 1 0 18a14 14 0 0 1 0-18z"/></svg>"#;
const GRID_SVG: &str = r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="7" rx="1"/><rect x="14" y="3" width="7" height="7" rx="1"/><rect x="3" y="14" width="7" height="7" rx="1"/><rect x="14" y="14" width="7" height="7" rx="1"/></svg>"#;

const PRINCIPLES: &[Principle<'static>] = &[
    Principle {
        bg_class: "bg-cat-blue",
        fg_class: "text-cat-blueInk",
        icon_svg: SHIELD_SVG,
        title: "Reproducible by default",
        body: "Every dependency is locked by content hash. The build you ship today \
               is the build you can ship in five years.",
    },
    Principle {
        bg_class: "bg-cat-green",
        fg_class: "text-cat-greenInk",
        icon_svg: STACK_SVG,
        title: "Federated registries",
        body: "Pull from any OCI-compatible registry — host your own, mirror upstream, \
               or compose private and public packages in one manifest.",
    },
    Principle {
        bg_class: "bg-cat-peach",
        fg_class: "text-cat-peachInk",
        icon_svg: GLOBE_SVG,
        title: "Semantic versioning",
        body: "Versions are strict semver — breaking changes require a major bump, \
               and once published, a version cannot be overwritten, only yanked.",
    },
    Principle {
        bg_class: "bg-cat-lilac",
        fg_class: "text-cat-lilacInk",
        icon_svg: GRID_SVG,
        title: "Compose, don't link",
        body: "Components are wired together at the WIT interface boundary — \
               no shared globals, no symbol clashes, no surprise side effects.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    // r[verify frontend.pages.home]
    #[test]
    fn format_count_inserts_thin_spaces() {
        assert_eq!(format_count(73), "73");
        assert_eq!(format_count(1248), "1\u{2009}248");
        assert_eq!(format_count(1_234_567), "1\u{2009}234\u{2009}567");
    }

    #[test]
    fn install_snippet_contains_expected_commands() {
        let snippet = install_snippet();
        assert!(snippet.contains("install.sh"));
        assert!(snippet.contains("wasi:http"));
    }

    fn pkg(ns: &str, name: &str, tags: &[&str], description: Option<&str>) -> KnownPackage {
        KnownPackage {
            registry: "ghcr.io".into(),
            repository: format!("{ns}/{name}"),
            kind: None,
            description: description.map(str::to_owned),
            tags: tags.iter().map(|s| (*s).to_owned()).collect(),
            signature_tags: vec![],
            attestation_tags: vec![],
            last_seen_at: String::new(),
            created_at: String::new(),
            wit_namespace: Some(ns.into()),
            wit_name: Some(name.into()),
            dependencies: vec![],
        }
    }

    #[test]
    fn stats_aggregate_counts_and_authors() {
        let packages = vec![
            pkg("wasi", "http", &["0.1.0", "0.2.0", "0.2.1"], Some("HTTP")),
            pkg("wasi", "io", &["0.2.0"], None),
            pkg("ba", "sqlite", &["0.1.0", "0.2.0"], Some("SQLite")),
        ];
        let stats = Stats::from_packages(&packages);
        assert_eq!(stats.package_count, 3);
        assert_eq!(stats.author_count, 2);
        assert_eq!(stats.version_count, 6);
    }

    #[test]
    fn stats_featured_sorted_by_version_count() {
        let packages = vec![
            pkg("wasi", "io", &["0.2.0"], None),
            pkg("wasi", "http", &["0.1.0", "0.2.0", "0.2.1"], Some("HTTP")),
            pkg("ba", "sqlite", &["0.1.0", "0.2.0"], Some("SQLite")),
        ];
        let stats = Stats::from_packages(&packages);
        assert_eq!(stats.featured[0].left, "wasi:http");
        assert_eq!(stats.featured[0].right, "HTTP");
        assert_eq!(stats.featured[0].href, "/wasi/http");
        assert_eq!(stats.featured[1].left, "ba:sqlite");
        // Falls back to release count when description is missing.
        assert_eq!(stats.featured[2].right, "1 release");
    }

    #[test]
    fn stats_top_namespaces_sorted_by_package_count() {
        let packages = vec![
            pkg("wasi", "http", &["0.1.0"], None),
            pkg("wasi", "io", &["0.1.0"], None),
            pkg("wasi", "cli", &["0.1.0"], None),
            pkg("ba", "sqlite", &["0.1.0"], None),
        ];
        let stats = Stats::from_packages(&packages);
        assert_eq!(stats.namespaces[0].left, "wasi");
        assert_eq!(stats.namespaces[0].right, "3");
        assert_eq!(stats.namespaces[0].href, "/wasi");
        assert_eq!(stats.namespaces[1].left, "ba");
        assert_eq!(stats.namespaces[1].right, "1");
    }
}
