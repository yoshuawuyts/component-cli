//! Navigation bar component.

use crate::components::search_bar;

/// A breadcrumb segment: (label, optional href).
pub(crate) struct Crumb {
    /// Display text.
    pub label: String,
    /// Link target, or `None` for the current (last) segment.
    pub href: Option<String>,
}

/// Render the site navigation bar with home link, breadcrumbs, and search.
#[must_use]
pub(crate) fn render(crumbs: &[Crumb]) -> String {
    let mut breadcrumb_html = String::new();
    for (i, crumb) in crumbs.iter().enumerate() {
        if i == 0 {
            breadcrumb_html.push(' ');
        } else {
            breadcrumb_html.push_str(r#" <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="inline-block text-ink-300 mx-1 align-[-1px]"><path d="m9 18 6-6-6-6"/></svg> "#);
        }
        if let Some(href) = &crumb.href {
            use std::fmt::Write;
            write!(
                breadcrumb_html,
                r#"<a href="{href}" class="text-ink-500 hover:text-ink-900 transition-colors">{label}</a>"#,
                label = crumb.label
            )
            .unwrap();
        } else {
            use std::fmt::Write;
            write!(
                breadcrumb_html,
                r#"<span class="text-ink-900">{label}</span>"#,
                label = crumb.label
            )
            .unwrap();
        }
    }

    let search = search_bar::compact("search-input");

    format!(
        r#"<nav class="w-full max-w-6xl mx-auto px-4 sm:px-6 md:px-8 pt-4 pb-3 flex items-center gap-4" aria-label="Main">
  <div class="flex items-center gap-1 text-[14px] text-ink-500 shrink-0">
    <a href="/" class="font-semibold text-ink-900 hover:text-accent transition-colors shrink-0">wasm</a>{breadcrumb_html}
  </div>
  <div class="flex-1 hidden sm:block max-w-md">{search}</div>
  <div class="flex items-center gap-3 sm:gap-4 shrink-0 ml-auto">
    <a href="/docs" class="text-[13px] text-ink-500 hover:text-ink-900 transition-colors">Docs</a>
    <a href="/downloads" class="text-[13px] text-ink-500 hover:text-ink-900 transition-colors hidden sm:inline">Downloads</a>
  </div>
</nav>"#,
    )
}
