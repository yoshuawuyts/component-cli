//! C05 — Item Details.

/// Render this section.
pub(crate) fn render() -> String {
    let inner = r##"
          <!-- Live demo: HTTP POST (cornerstone — full feature set) -->
          <div>
            <div class="text-[12px] text-ink-500 mb-3">Cornerstone · POST with auth and request body</div>
            <div class="rounded-lg border border-line bg-canvas p-5 md:p-6">
              <article class="space-y-5">
                <!-- Title — page-level h2 + aux (since/source/man) -->
                <header>
                  <div class="id-title-head">
                    <h2>
                      <span>publish a package version</span>
                      <a href="#c-item-details" class="id-anchor">§</a>
                    </h2>
                    <div class="id-aux">
                      <span class="id-since-tag">v1.4.0</span>
                      <a href="#" class="id-src-link">source</a>
                      <a href="#" class="id-src-link">spec</a>
                    </div>
                  </div>
                </header>
                <!-- Header — method pill, path, auth tag pushed right -->
                <div class="id-header">
                  <span class="id-method id-method-post">POST</span>
                  <span class="id-path">
                    <span class="seg">/v1/packages</span><span class="sl">/</span><span class="par">{registry}</span><span
                      class="sl">/</span><span class="par">{*repository}</span>
                  </span>
                  <span class="id-auth-tag">Auth required</span>
                </div>
                <!-- Tagline — what this symbol does, in one sentence -->
                <p class="id-page-tagline">
                  Push a new version of a package to a registry. The body
                  references an OCI manifest already uploaded via the standard
                  distribution endpoints.
                </p>
              </article>
            </div>
          </div>

          <!-- Anatomy / rules -->
          <div>
            <div class="text-[12px] text-ink-500 mb-3">Anatomy</div>
            <ul class="text-[13px] text-ink-700 leading-relaxed space-y-1.5 pl-5 list-disc marker:text-ink-400">
              <li>
                <p><strong>Container</strong> — the whole article sits inside a
                  <code class="mono text-[12px]">rounded-lg border border-line bg-canvas</code> card with
                  <code class="mono text-[12px]">p-5 md:p-6</code> padding, mirroring the bordered surfaces
                  used by the search trigger and the navbar mockups. The card frames the symbol so it
                  reads as a discrete reference unit, not loose page chrome.</p>
              </li>
              <li>
                <p><strong>Title</strong> — page-level <code class="mono text-[12px]">&lt;h2&gt;</code> in
                  <code class="mono text-[12px]">.id-title-head</code> with an aux row on the right
                  (<code class="mono text-[12px]">.id-since-tag</code> for the version a method/endpoint was
                  introduced, plus <code class="mono text-[12px]">.id-src-link</code> entries for source / spec
                  / man). Always the first element on the page.
                </p>
              </li>
              <li>
                <p><strong>Header (signature)</strong> — sits directly below the title so readers see what
                  the symbol <em>is</em> before reading what it <em>does</em>. Pill
                  (<code class="mono text-[12px]">.id-method</code> for HTTP,
                  <code class="mono text-[12px]">.id-kind</code> for RPC) on the left, mono path
                  (<code class="mono text-[12px]">.id-path</code>) center, optional
                  <code class="mono text-[12px]">.id-auth-tag</code> pushed right via
                  <code class="mono text-[12px]">margin-left: auto</code>. Hairline-bordered band on
                  <code class="mono text-[12px]">canvas</code> — distinct from cards, anchored to the
                  article.
                </p>
              </li>
              <li>
                <p><strong>Tagline</strong> — a single-paragraph
                  <code class="mono text-[12px]">.id-page-tagline</code> below the signature, capped at
                  <code class="mono text-[12px]">72ch</code>. What this thing does, in one sentence. Acts as
                  the bridge from the signature to the structured sections below.
                </p>
              </li>
              <li>
                <p><strong>Path coloring</strong> — static segments stay
                  <code class="mono text-[12px] text-ink-900">ink-900</code>, parameters
                  (<code class="mono text-[12px] text-ink-500">{registry}</code>) take
                  <code class="mono text-[12px]" style="color:var(--c-cat-plum-ink);font-weight:600;">plum-ink</code>
                  so the eye can scan placeholders without reading every character. Slashes drop to
                  <code class="mono text-[12px] text-ink-500">ink-400</code> as low-contrast punctuation.
                </p>
              </li>
              <li>
                <p><strong>Request body</strong> (optional) — structured table, not a code block. Each row:
                  a name + type/required line on the left
                  (<code class="mono text-[12px]">w-[180px]</code>, mono name + 11.5px ink-500 mono meta) and
                  a description on the right. Hairline rules between rows. Use this whenever the body
                  has more than two fields or any optional fields — code blocks become hard to scan past
                  ~3 keys.</p>
              </li>
              <li>
                <p><strong>Responses</strong> — divided list. HTTP uses
                  <code class="mono text-[12px]">.id-http-status</code> pills (2xx green / 3xx blue /
                  4xx peach / 5xx pink). RPC uses <code class="mono text-[12px]">.id-status-dot</code> +
                  fixed-width mono code (<code class="mono text-[12px]">w-20</code> so descriptions align).
                </p>
              </li>
              <li>
                <p><strong>Example</strong> — for HTTP, paired request/response code panels in a 2-col grid.
                  For RPC, language tabs above a single panel; inactive tabs are
                  <code class="mono text-[12px]">is-soon</code> placeholders so users see where future
                  bindings will land.
                </p>
              </li>
              <li>
                <p>Sections always appear in this order: <strong>title → header → tagline → request
                    body → responses → example</strong>. Optional sections (request body, auth tag, example,
                  since-tag, source/spec links) drop cleanly without reordering or backfilling — predictability
                  across pages matters more than local optimization.</p>
              </li>
            </ul>
          </div>

          <!-- Pill variants -->
          <div>
            <div class="text-[12px] text-ink-500 mb-3">Header pill variants</div>
            <div class="space-y-3">
              <div class="flex flex-wrap items-center gap-3">
                <span class="text-[11px] mono uppercase tracking-wider text-ink-500 w-16">HTTP</span>
                <span class="id-method id-method-get">GET</span>
                <span class="id-method id-method-post">POST</span>
                <span class="id-method id-method-put">PUT</span>
                <span class="id-method id-method-patch">PATCH</span>
                <span class="id-method id-method-delete">DELETE</span>
              </div>
              <div class="flex flex-wrap items-center gap-3">
                <span class="text-[11px] mono uppercase tracking-wider text-ink-500 w-16">gRPC</span>
                <span class="id-kind id-kind-unary">UNARY</span>
                <span class="id-kind id-kind-server">SERVER</span>
                <span class="id-kind id-kind-client">CLIENT</span>
                <span class="id-kind id-kind-bidi">BIDI</span>
              </div>
              <div class="flex flex-wrap items-center gap-3">
                <span class="text-[11px] mono uppercase tracking-wider text-ink-500 w-16">Tag</span>
                <span class="id-auth-tag" style="margin-left:0;">Auth required</span>
              </div>
            </div>
            <p class="mt-3 text-[12px] text-ink-500">Method pill (<code class="mono text-[12px]">.id-method-*</code>)
              and kind pill (<code class="mono text-[12px]">.id-kind-*</code>) share shape and slot —
              swap by surface, not by stacking. The auth tag is the only header element that uses
              <code class="mono text-[12px]">cream</code> + the rounded-pill shape, reserving cream for
              "you need credentials" semantics across the design system.
            </p>
          </div>
        "##;
    let content = html::text_content::Division::builder()
        .class("space-y-8")
        .text(inner)
        .build()
        .to_string();
    super::section(
        "c-item-details",
        "C05",
        "Item Details",
        "Reference page for a single endpoint, RPC, schema, or command. A method/kind pill anchors the symbol below the title; a one-sentence tagline explains it; an optional structured request-body table, a responses list, and paired example panels stack below in fixed order. Used as the destination from <a href=\"#c-item-list\" class=\"text-ink-700 underline decoration-line decoration-1 underline-offset-[3px] hover:text-ink-900\">Item List</a> rows.",
        &content,
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
