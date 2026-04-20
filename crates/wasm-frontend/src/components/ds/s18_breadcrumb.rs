//! 18 — Breadcrumb & Pagination.

/// Render the HTML for this section.
pub(crate) fn render() -> &'static str {
    r##"
    <section id="breadcrumb" class="pt-12 md:pt-16">
      <div class="grid md:grid-cols-[200px_1fr] gap-6 md:gap-12">
        <div>
          <div class="text-[12px] mono uppercase tracking-wider text-ink-500">19</div>
          <h2 class="mt-2 text-[24px] font-semibold tracking-tight">Breadcrumb &<br />Pagination</h2>
          <p class="mt-2 text-[13px] text-ink-500 leading-relaxed">
            Navigation context. Breadcrumb uses chevron separators and dims all but the
            current item. Pagination is square-buttoned for compact toolbars.
          </p>
        </div>
        <div class="space-y-8">
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
        </div>
      </div>
    </section>
"##
}
