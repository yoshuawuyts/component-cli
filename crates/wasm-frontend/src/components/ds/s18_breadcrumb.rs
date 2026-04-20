//! 18 — Breadcrumb & Pagination.

/// Render this section.
pub(crate) fn render() -> String {
    let content = r##"<div class="space-y-8">
          <div>
            <h3 class="text-[13px] mono uppercase tracking-wider text-ink-500 mb-3">Breadcrumb</h3>
            <nav class="flex items-center gap-1.5 text-[13px] text-ink-500">
              <a href="#" class="hover:text-ink-900">Tellus</a>
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                stroke-linecap="round" stroke-linejoin="round" class="text-ink-300">
                <path d="m9 18 6-6-6-6" />
              </svg>
              <a href="#" class="hover:text-ink-900">Pellentesque</a>
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                stroke-linecap="round" stroke-linejoin="round" class="text-ink-300">
                <path d="m9 18 6-6-6-6" />
              </svg>
              <span class="text-ink-900 font-medium">Vestibulum ante</span>
            </nav>
          </div>
          <div>
            <h3 class="text-[13px] mono uppercase tracking-wider text-ink-500 mb-3">Pagination</h3>
            <div class="inline-flex items-center gap-1 text-[13px]">
              <button
                class="h-8 w-8 grid place-items-center rounded-md border border-line bg-surface text-ink-500 hover:bg-surfaceMuted">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                  stroke-linecap="round" stroke-linejoin="round">
                  <path d="m15 18-6-6 6-6" />
                </svg>
              </button>
              <button
                class="h-8 w-8 grid place-items-center rounded-md border border-line bg-surface hover:bg-surfaceMuted">1</button>
              <button class="h-8 w-8 grid place-items-center rounded-md bg-ink-900 text-canvas font-medium">2</button>
              <button
                class="h-8 w-8 grid place-items-center rounded-md border border-line bg-surface hover:bg-surfaceMuted">3</button>
              <span class="px-1 text-ink-400">…</span>
              <button
                class="h-8 w-8 grid place-items-center rounded-md border border-line bg-surface hover:bg-surfaceMuted">12</button>
              <button
                class="h-8 w-8 grid place-items-center rounded-md border border-line bg-surface text-ink-500 hover:bg-surfaceMuted">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                  stroke-linecap="round" stroke-linejoin="round">
                  <path d="m9 18 6-6-6-6" />
                </svg>
              </button>
            </div>
          </div>
        </div>"##;
    super::section(
        "breadcrumb",
        "19",
        "Breadcrumb &<br />Pagination",
        "Navigation context. Breadcrumb uses chevron separators and dims all but the current item. Pagination is square-buttoned for compact toolbars.",
        content,
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
