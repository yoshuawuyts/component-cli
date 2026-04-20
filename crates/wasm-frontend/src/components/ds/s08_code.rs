//! 08 — Code Samples.

/// Render this section.
pub(crate) fn render() -> String {
    let content = r##"<div class="space-y-12">

          <!-- Plain · single panel -->
          <div>
            <div class="text-[12px] text-ink-500 mb-3">Plain · single panel</div>
            <pre class="id-code"><span class="h">[package]</span>
<span class="k">name</span>    <span class="p">=</span> <span class="s">"example.com/hello-world"</span>
<span class="k">version</span> <span class="p">=</span> <span class="s">"0.1.0"</span>
<span class="k">authors</span> <span class="p">=</span> <span class="p">[</span><span class="s">"Lorem Ipsum &lt;lorem@example.com&gt;"</span><span class="p">]</span>

<span class="h">[dependencies]</span>
<span class="k">"wasi:http"</span>   <span class="p">=</span> <span class="s">"0.2"</span>
<span class="k">"wasi:cli"</span>    <span class="p">=</span> <span class="s">"0.2"</span></pre>
            <p class="mt-3 text-[12px] text-ink-500">Default form. Used for one-off TOML, JSON, or
              shell snippets where the language is obvious from context.</p>
          </div>

          <!-- Tabbed · multiple languages -->
          <div>
            <div class="text-[12px] text-ink-500 mb-3">Tabbed · same call, multiple languages</div>
            <div class="id-lang-tabs">
              <span class="id-lang-tab is-active"><span class="dot"></span>curl</span>
              <span class="id-lang-tab"><span class="dot"></span>Rust</span>
              <span class="id-lang-tab is-soon"><span class="dot"></span>Python</span>
              <span class="id-lang-tab is-soon"><span class="dot"></span>TypeScript</span>
            </div>
            <div class="id-lang-panel">
              <pre class="id-code"><span class="v">curl</span> <span class="f">-X</span> POST \
  <span class="f">-H</span> <span class="s">"Authorization: Bearer $TOKEN"</span> \
  <span class="f">-H</span> <span class="s">"Content-Type: application/json"</span> \
  <span class="f">-d</span> <span class="s">'{"version":"0.2.4","digest":"sha256:1f08…"}'</span> \
  https://registry.example.com/v1/packages/wasi/http</pre>
            </div>
            <p class="mt-3 text-[12px] text-ink-500">Tab strip uses
              <code class="mono text-[12px]">.id-lang-tabs</code> + <code class="mono text-[12px]">.id-lang-tab</code>.
              Active tab fuses with the panel (shared surface, top-left radius zeroed). The
              <code class="mono text-[12px]">.is-soon</code> modifier dims tabs whose bindings haven't
              landed yet — users see where future languages will appear without the link being live.
            </p>
          </div>

          <!-- Paired · request / response -->
          <div>
            <div class="text-[12px] text-ink-500 mb-3">Paired · request and response</div>
            <div class="grid md:grid-cols-2 gap-4">
              <div>
                <div class="text-[11px] mono uppercase tracking-wider text-ink-500 mb-2">Request</div>
                <pre class="id-code"><span class="v">curl</span> <span class="f">-X</span> POST \
  <span class="f">-H</span> <span class="s">"Authorization: Bearer $TOKEN"</span> \
  <span class="f">-H</span> <span class="s">"Content-Type: application/json"</span> \
  <span class="f">-d</span> <span class="s">'{
    "version": "0.2.4",
    "digest":  "sha256:1f08…"
  }'</span> \
  https://registry.example.com/v1/packages/wasi/http</pre>
              </div>
              <div>
                <div class="text-[11px] mono uppercase tracking-wider text-ink-500 mb-2">Response · 201</div>
                <pre class="id-code">{
  <span class="k">"version"</span><span class="p">:</span> <span class="s">"0.2.4"</span><span class="p">,</span>
  <span class="k">"digest"</span><span class="p">:</span>  <span class="s">"sha256:1f08…"</span><span class="p">,</span>
  <span class="k">"yanked"</span><span class="p">:</span>  <span class="n">false</span><span class="p">,</span>
  <span class="k">"published_at"</span><span class="p">:</span> <span class="s">"2026-04-18T08:11:02Z"</span>
}</pre>
              </div>
            </div>
            <p class="mt-3 text-[12px] text-ink-500">Two panels in a 2-col grid; each carries its own
              11px ink-500 mono uppercase label above. Use for HTTP / RPC reference where the
              response shape is part of the contract. Stacks to single column on narrow viewports.</p>
          </div>

          <!-- Token legend -->
          <div>
            <div class="text-[12px] text-ink-500 mb-3">Token palette</div>
            <div class="rounded-lg border border-line bg-canvas p-4">
              <div class="grid grid-cols-2 md:grid-cols-3 gap-x-6 gap-y-2.5 text-[12px]">
                <div class="flex items-center gap-3"><span class="mono w-12" style="color:var(--color-wit-struct)">.k
                    .f</span><span class="text-ink-500">keys / keywords / shell flags</span></div>
                <div class="flex items-center gap-3"><span class="mono w-12"
                    style="color:var(--color-wit-resource)">.s</span><span class="text-ink-500">strings</span></div>
                <div class="flex items-center gap-3"><span class="mono w-12"
                    style="color:var(--color-wit-func)">.n</span><span class="text-ink-500">numbers / booleans</span>
                </div>
                <div class="flex items-center gap-3"><span class="mono w-12"
                    style="color:var(--color-wit-world)">.ty</span><span class="text-ink-500">types</span></div>
                <div class="flex items-center gap-3"><span class="mono w-12" style="color:var(--color-wit-iface)">.fn
                    .at</span><span class="text-ink-500">functions / attributes</span></div>
                <div class="flex items-center gap-3"><span class="mono w-12"
                    style="color:var(--color-wit-module);font-weight:600">.h</span><span class="text-ink-500">section
                    headers</span></div>
                <div class="flex items-center gap-3"><span class="mono w-12 text-ink-900"
                    style="font-weight:600">.v</span><span class="text-ink-500">shell verb</span></div>
                <div class="flex items-center gap-3"><span class="mono w-12 text-ink-400">.p</span><span
                    class="text-ink-500">punctuation</span></div>
                <div class="flex items-center gap-3"><span class="mono w-12 text-ink-500"
                    style="font-style:italic">.c</span><span class="text-ink-500">comments</span></div>
              </div>
            </div>
            <p class="mt-3 text-[12px] text-ink-500">Each token references the same
              <code class="mono text-[12px]">--color-wit-*</code> variables we use for WIT diagrams
              — module pink, world purple, interface sky, function green, struct indigo, resource
              amber. Both light and dark themes carry calibrated hex pairs so the panel never feels
              over-saturated on paper or muddy on graphite.
            </p>
          </div>

          <!-- Anatomy / rules -->
          <div>
            <div class="text-[12px] text-ink-500 mb-3">Anatomy</div>
            <ul class="text-[13px] text-ink-700 leading-relaxed space-y-1.5 pl-5 list-disc marker:text-ink-400">
              <li>
                <p>Quiet inset panel — <code class="mono text-[12px]">--c-surface</code> background,
                  <code class="mono text-[12px]">--c-line-soft</code> hairline border, 5px radius.
                  Inverts with the rest of the page in dark mode.
                </p>
              </li>
              <li>
                <p>12.5px monospace, line-height 1.55, padding 14×16. Horizontal overflow scrolls
                  rather than wrapping — code never reflows.</p>
              </li>
              <li>
                <p>Tokens are short class names on <code class="mono text-[12px]">&lt;span&gt;</code>:
                  <code class="mono text-[12px]">.k</code>, <code class="mono text-[12px]">.s</code>,
                  <code class="mono text-[12px]">.n</code>, <code class="mono text-[12px]">.ty</code>,
                  <code class="mono text-[12px]">.fn</code>, <code class="mono text-[12px]">.at</code>,
                  <code class="mono text-[12px]">.h</code>, <code class="mono text-[12px]">.v</code>,
                  <code class="mono text-[12px]">.f</code>, <code class="mono text-[12px]">.p</code>,
                  <code class="mono text-[12px]">.c</code>. Untagged characters fall back to the panel's
                  default <code class="mono text-[12px]">--c-ink-900</code>.
                </p>
              </li>
              <li>
                <p>Caption above each panel uses <code class="mono text-[12px]">12px ink-500</code>;
                  request/response labels use <code class="mono text-[12px]">11px mono uppercase
                ink-500</code> with <code class="mono text-[12px]">tracking-wider</code> for parity
                  with the rest of the system.</p>
              </li>
              <li>
                <p>Tabbed variant: <code class="mono text-[12px]">.id-lang-tabs</code> sits flush against
                  the panel; the active tab shares the panel's dark surface and zeros its bottom-left
                  corner so the two read as one piece.</p>
              </li>
            </ul>
          </div>
        </div>"##;
    super::section(
        "code",
        "08",
        "Code Samples",
        "One panel \u{2014} <code class=\"mono text-[12px]\">pre.id-code</code> \u{2014} sitting on <code class=\"mono text-[12px]\">--c-surface</code>, with token colours pulled from the theme-aware <code class=\"mono text-[12px]\">--color-wit-*</code> palette so chroma stays balanced on both light and dark pages. Three forms: a plain block, a tabbed multi-language block, and a paired request / response grid.",
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
