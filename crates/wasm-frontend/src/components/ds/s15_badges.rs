//! 15 — Badges.

/// Render this section.
pub(crate) fn render() -> String {
    let content = r##"<div class="space-y-6">
          <div>
            <h3 class="text-[13px] mono uppercase tracking-wider text-ink-500 mb-3">Status</h3>
            <div class="flex flex-wrap items-center gap-2 text-[12px] font-medium">
              <span class="inline-flex items-center gap-1.5 px-2 h-6 rounded-pill bg-cat-green text-cat-greenInk">
                <span class="h-1.5 w-1.5 rounded-full bg-cat-greenInk"></span>Active
              </span>
              <span class="inline-flex items-center gap-1.5 px-2 h-6 rounded-pill bg-cat-cream text-cat-creamInk">
                <span class="h-1.5 w-1.5 rounded-full bg-cat-creamInk"></span>Pending
              </span>
              <span class="inline-flex items-center gap-1.5 px-2 h-6 rounded-pill bg-cat-pink text-cat-pinkInk">
                <span class="h-1.5 w-1.5 rounded-full bg-cat-pinkInk"></span>Failed
              </span>
              <span class="inline-flex items-center gap-1.5 px-2 h-6 rounded-pill bg-cat-blue text-cat-blueInk">
                <span class="h-1.5 w-1.5 rounded-full bg-cat-blueInk"></span>Info
              </span>
              <span class="inline-flex items-center px-2 h-6 rounded-pill bg-surfaceMuted text-ink-700">Draft</span>
            </div>
          </div>
          <div>
            <h3 class="text-[13px] mono uppercase tracking-wider text-ink-500 mb-3">Counts</h3>
            <div class="flex flex-wrap items-center gap-2 text-[12px] font-medium">
              <span
                class="inline-flex items-center px-1.5 min-w-[20px] h-5 rounded-pill bg-ink-700 text-canvas justify-center">3</span>
              <span
                class="inline-flex items-center px-1.5 min-w-[20px] h-5 rounded-pill bg-surfaceMuted text-ink-700 border border-line justify-center">12</span>
              <span
                class="inline-flex items-center px-1.5 min-w-[20px] h-5 rounded-pill bg-cat-pink text-cat-pinkInk justify-center">99+</span>
            </div>
          </div>
          <div>
            <h3 class="text-[13px] mono uppercase tracking-wider text-ink-500 mb-3">Tag</h3>
            <div class="flex flex-wrap items-center gap-2 text-[12px]">
              <span class="inline-flex items-center gap-1 px-2 h-6 rounded-md border border-line text-ink-700">
                Tellus
                <button class="text-ink-400 hover:text-ink-900">
                  <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                    stroke-linecap="round" stroke-linejoin="round">
                    <path d="M18 6 6 18" />
                    <path d="m6 6 12 12" />
                  </svg>
                </button>
              </span>
              <span
                class="inline-flex items-center gap-1 px-2 h-6 rounded-md border border-line text-ink-700">Convallis</span>
            </div>
          </div>
        </div>"##;
    super::section(
        "badges",
        "15",
        "Badges",
        "Compact pill labels. Use categorical pairs for status; ink for counts and metadata.",
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
