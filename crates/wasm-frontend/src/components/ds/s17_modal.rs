//! 17 — Modal.

/// Render this section.
pub(crate) fn render() -> String {
    let content = r##"<div class="relative rounded-lg p-8 md:p-12 overflow-hidden bg-canvas">
          <!-- Simulate the page beneath: a faint preview of content -->
          <div class="absolute inset-0 p-6 select-none pointer-events-none" aria-hidden="true">
            <div class="h-3 w-40 rounded mb-3" style="background: var(--c-ink-300);"></div>
            <div class="h-2 w-72 rounded mb-2" style="background: var(--c-line);"></div>
            <div class="h-2 w-64 rounded mb-2" style="background: var(--c-line);"></div>
            <div class="h-2 w-56 rounded" style="background: var(--c-line);"></div>
          </div>
          <!-- Scrim: always-dark, regardless of theme, so it darkens the page -->
          <div class="absolute inset-0" style="background: rgba(15, 15, 17, 0.55); backdrop-filter: blur(2px);"></div>
          <div class="relative mx-auto max-w-md bg-surface border border-line rounded-lg shadow-tooltip">
            <div class="flex items-start justify-between p-5 border-b border-lineSoft">
              <div>
                <div class="text-[15px] font-semibold tracking-tight">Confirm action</div>
                <div class="text-[12px] text-ink-500 mt-1">Lorem ipsum dolor sit amet</div>
              </div>
              <button class="text-ink-500 hover:text-ink-900">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75"
                  stroke-linecap="round" stroke-linejoin="round">
                  <path d="M18 6 6 18" />
                  <path d="m6 6 12 12" />
                </svg>
              </button>
            </div>
            <div class="p-5 text-[14px] text-ink-700 leading-relaxed">
              Pellentesque habitant morbi tristique senectus et netus et malesuada fames
              ac turpis egestas. Vestibulum tortor quam.
            </div>
            <div class="flex items-center justify-end gap-2 p-4 border-t border-lineSoft bg-canvas rounded-b-lg">
              <button
                class="h-8 px-3 rounded-lg border border-line bg-surface text-[13px] hover:bg-surfaceMuted">Cancel</button>
              <button
                class="h-8 px-3 rounded-lg bg-surfaceMuted text-ink-900 text-[13px] hover:bg-ink-300">Confirm</button>
            </div>
          </div>
        </div>"##;
    super::section(
        "modal",
        "17",
        "Modal",
        "Centered dialog over a 50% ink scrim. 8px radius, 1px gray border, 24px padding. Header / body / footer rhythm.",
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
