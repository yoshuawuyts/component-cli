//! C02 — On This Page.

/// Render this section.
pub(crate) fn render() -> String {
    let content = r##"<div class="space-y-6">
          <!-- Live demo -->
          <div class="border border-line rounded-lg bg-canvas p-4 max-w-[240px]">
            <div class="mono uppercase tracking-wider text-[10px] text-ink-500 mb-2 px-2.5">On this page</div>
            <nav class="space-y-px">
              <a href="#c-toc" class="toc-link">Overview</a>
              <a href="#c-toc" class="toc-link">Subcommands</a>
              <a href="#c-toc" class="toc-link indent">add</a>
              <a href="#c-toc" class="toc-link indent active">remove</a>
              <a href="#c-toc" class="toc-link indent">list</a>
              <a href="#c-toc" class="toc-link indent">login</a>
              <a href="#c-toc" class="toc-link indent">publish</a>
              <a href="#c-toc" class="toc-link">Global flags</a>
              <a href="#c-toc" class="toc-link">Environment</a>
              <a href="#c-toc" class="toc-link">Exit codes</a>
              <a href="#c-toc" class="toc-link">Config files</a>
            </nav>
            <div class="px-2.5 mt-4">
              <button type="button"
                class="inline-flex items-center gap-1.5 h-7 px-2 rounded-md text-[11px] mono text-ink-500 hover:bg-surfaceMuted hover:text-ink-900 transition-colors">
                <svg class="h-3 w-3" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5"
                  stroke-linecap="round">
                  <path d="m3 7.5 3-3 3 3" />
                </svg>
                Back to top
              </button>
            </div>
          </div>

          <!-- States -->
          <div>
            <div class="text-[12px] text-ink-500 mb-3">States</div>
            <div class="grid grid-cols-3 gap-4 max-w-[480px]">
              <div>
                <div class="text-[11px] mono uppercase tracking-wider text-ink-500 mb-2">Default</div>
                <a href="#c-toc" class="toc-link">Section title</a>
              </div>
              <div>
                <div class="text-[11px] mono uppercase tracking-wider text-ink-500 mb-2">Hover</div>
                <a href="#c-toc" class="toc-link"
                  style="color:var(--c-ink-900);border-left-color:var(--c-line);">Section title</a>
              </div>
              <div>
                <div class="text-[11px] mono uppercase tracking-wider text-ink-500 mb-2">Active</div>
                <a href="#c-toc" class="toc-link active">Section title</a>
              </div>
            </div>
          </div>

          <!-- Anatomy / rules -->
          <div>
            <div class="text-[12px] text-ink-500 mb-3">Anatomy</div>
            <ul class="text-[13px] text-ink-700 leading-relaxed space-y-1.5 pl-5 list-disc marker:text-ink-400">
              <li>
                <p>Two depth levels only: top-level entries and one indent
                  (<code class="mono text-[12px]">.indent</code>, +12px).</p>
              </li>
              <li>
                <p>Hover lifts ink to <code class="mono text-[12px]">--c-ink-900</code> and tints the left rail to
                  <code class="mono text-[12px]">--c-line</code>.
                </p>
              </li>
              <li>
                <p>Active state uses a full <code class="mono text-[12px]">--c-ink-900</code> rail; the rail is the
                  only marker — never combine with a background.</p>
              </li>
              <li>
                <p>Drive the active state from a scroll-spy
                  (<code class="mono text-[12px]">IntersectionObserver</code> with a top rootMargin) so the marker
                  tracks the section currently in view.</p>
              </li>
              <li>
                <p>The rail width is reserved (1.5px transparent border) on every row, so toggling active
                  doesn't shift the label.</p>
              </li>
              <li>
                <p>End the rail with a quiet <strong>Back to top</strong> button — 11px mono, ink-500, no border;
                  hover reveals the surface-muted background.</p>
              </li>
            </ul>
          </div>
        </div>"##;
    super::section(
        "c-toc",
        "C02",
        "On This Page",
        "Right-rail table of contents for long reference pages. A 1.5px left border lights up on hover and active state \u{2014} the only visual cue, no background fills.",
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
