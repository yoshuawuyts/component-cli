//! 19 — Progress & Spinner.

/// Render this section.
pub(crate) fn render() -> String {
    let content = r##"<div class="space-y-8">
          <div>
            <h3 class="text-[13px] mono uppercase tracking-wider text-ink-500 mb-3">Progress bar</h3>
            <div class="space-y-2 max-w-md">
              <div>
                <div class="flex justify-between text-[12px] text-ink-500 mb-1"><span>Aenean lectus</span><span
                    class="mono">68%</span></div>
                <div class="h-1.5 w-full rounded-pill bg-surfaceMuted overflow-hidden">
                  <div class="h-full bg-ink-900 rounded-pill" style="width:68%"></div>
                </div>
              </div>
              <div>
                <div class="flex justify-between text-[12px] text-ink-500 mb-1"><span>Pellentesque</span><span
                    class="mono">24%</span></div>
                <div class="h-1.5 w-full rounded-pill bg-surfaceMuted overflow-hidden">
                  <div class="h-full bg-cat-greenInk rounded-pill" style="width:24%"></div>
                </div>
              </div>
            </div>
          </div>
          <div>
            <h3 class="text-[13px] mono uppercase tracking-wider text-ink-500 mb-3">Spinner</h3>
            <style>
              @keyframes ds-spin {
                to {
                  transform: rotate(360deg);
                }
              }

              .ds-spinner {
                animation: ds-spin 0.8s linear infinite;
              }

              @keyframes ds-pulse {

                0%,
                100% {
                  opacity: 1;
                }

                50% {
                  opacity: .5;
                }
              }

              .ds-skel {
                animation: ds-pulse 1.4s ease-in-out infinite;
              }
            </style>
            <div class="flex items-center gap-4">
              <svg class="ds-spinner text-ink-900" width="16" height="16" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
                <path d="M21 12a9 9 0 1 1-6.2-8.55" />
              </svg>
              <svg class="ds-spinner text-ink-500" width="20" height="20" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2" stroke-linecap="round">
                <path d="M21 12a9 9 0 1 1-6.2-8.55" />
              </svg>
              <svg class="ds-spinner text-ink-300" width="28" height="28" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="1.75" stroke-linecap="round">
                <path d="M21 12a9 9 0 1 1-6.2-8.55" />
              </svg>
            </div>
          </div>
          <div>
            <h3 class="text-[13px] mono uppercase tracking-wider text-ink-500 mb-3">Skeleton</h3>
            <div class="max-w-md space-y-2">
              <div class="ds-skel h-4 w-2/3 rounded bg-surfaceMuted"></div>
              <div class="ds-skel h-3 w-full rounded bg-surfaceMuted"></div>
              <div class="ds-skel h-3 w-5/6 rounded bg-surfaceMuted"></div>
            </div>
          </div>
        </div>"##;
    super::section(
        "progress",
        "19",
        "Progress & Spinner",
        "Determinate progress as a 6px ink track. Indeterminate as a 16px spinner (CSS animation). Skeleton shimmer for placeholder content.",
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
