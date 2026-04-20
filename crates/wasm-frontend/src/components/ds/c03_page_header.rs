//! C03 — Page Header.

/// Render this section.
pub(crate) fn render() -> String {
    let content = r##"<div class="space-y-10">
          <!-- Install variant: kicker + title + tagline + copyable install command -->
          <div>
            <div class="text-[12px] text-ink-500 mb-3">Package · with install command</div>
            <div class="border border-line rounded-lg bg-canvas px-6 py-8">
              <div class="flex items-center gap-2 text-[12px] text-ink-500 mono uppercase tracking-wider">
                <span>v0.4.2</span>
                <span class="h-1 w-1 rounded-full bg-ink-300"></span>
                <span>wasi:http</span>
                <span class="h-1 w-1 rounded-full bg-ink-300"></span>
                <span>Apache-2.0</span>
              </div>
              <h1 class="mt-3 text-[36px] md:text-[44px] leading-[1.05] font-semibold tracking-tight">
                wasi-http-handler
              </h1>
              <p class="mt-3 max-w-2xl text-[15px] text-ink-700 leading-relaxed">
                A composable HTTP request handler component implementing the
                <span class="mono text-[13px]">wasi:http/incoming-handler</span>
                interface. Drop-in compatible with any wasi:http host.
              </p>
              <!-- Metadata strip with install command -->
              <div class="mt-6 flex flex-wrap items-center gap-x-6 gap-y-3 text-[13px]">
                <div class="inline-flex items-center gap-2">
                  <span class="text-[11px] mono uppercase tracking-wider text-ink-500">Install</span>
                  <div class="flex">
                    <span
                      class="inline-flex items-center px-2.5 h-7 rounded-l-md border border-r-0 border-line bg-surfaceMuted text-[12.5px] text-ink-500 mono select-none"
                      aria-hidden="true">$</span>
                    <code
                      class="inline-flex items-center px-2.5 h-7 border border-line bg-surface mono text-[12.5px] text-ink-900 whitespace-nowrap">wasm install wasi:http-handler</code>
                    <button type="button"
                      class="inline-flex items-center justify-center w-7 h-7 rounded-r-md border border-l-0 border-line bg-surface text-ink-500 hover:text-ink-900 hover:bg-surfaceMuted focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-[-2px] focus-visible:outline-ink-900"
                      aria-label="Copy install command">
                      <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor"
                        stroke-width="1.4" aria-hidden="true">
                        <rect x="4" y="4" width="9" height="9" rx="1.2" />
                        <path d="M4 11H3.2A1.2 1.2 0 0 1 2 9.8V3.2A1.2 1.2 0 0 1 3.2 2h6.6A1.2 1.2 0 0 1 11 3.2V4" />
                      </svg>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Anatomy / rules -->
          <div>
            <div class="text-[12px] text-ink-500 mb-3">Anatomy</div>
            <ul class="text-[13px] text-ink-700 leading-relaxed space-y-1.5 pl-5 list-disc marker:text-ink-400">
              <li>
                <p><strong>Kicker</strong> — 12px ink-500 mono uppercase, dot-separated tokens (version,
                  category, format). Sets context before the title.</p>
              </li>
              <li>
                <p><strong>Title</strong> — 36/44px semibold, tight tracking, leading 1.05. The page's anchor.</p>
              </li>
              <li>
                <p><strong>Tagline</strong> — 15px ink-700, max-w-2xl, single paragraph. One sentence
                  describing what the page <em>is</em>, not what it does.</p>
              </li>
              <li>
                <p><strong>Metadata strip</strong> (optional) — labelled key/value pairs separated by
                  horizontal gap, wrapping to a new row at narrow widths. Each label is 11px ink-500 mono
                  uppercase; each value uses the appropriate token (mono pill, link, or status badge).</p>
              </li>
              <li>
                <p>Vertical rhythm: <code class="mono text-[12px]">pt-8 md:pt-12 pb-8 md:pb-12</code>, with
                  a strong <code class="mono text-[12px]">.rule</code> divider beneath separating the header from
                  page content.</p>
              </li>
            </ul>
          </div>
        </div>"##;
    super::section(
        "c-page-header",
        "C03",
        "Page Header",
        "Top-of-page identification block: a kicker, a large title, an optional tagline, and an optional metadata strip. Used to anchor reference and documentation pages.",
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
